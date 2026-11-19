use std::any::Any;
use std::future::Future;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use portadb_core::database::Database;
use portadb_core::driver::{Driver as DriverTrait, DriverConnection, DriverTransaction};
use portadb_core::error::Error;
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;
use portadb_core::query::prepare_query;
use portadb_core::row::Row;
use portadb_core::row_ref::{RowRef, RowVisitor, ValueRef as PortaDbValueRef};
use portadb_core::value::Value;

/// Driver for SQLite using the `rusqlite` engine library.
#[derive(Debug, Default, Clone, Copy)]
pub struct Driver;

/// Backward-compatible alias for [`Driver`].
pub type RusqliteDriver = Driver;

/// Process-wide counter for generating unique private in-memory database names.
///
/// In SQLite, every `:memory:` connection is private by default. To preserve this
/// isolation semantics while allowing shared cache when explicitly desired, each
/// `:memory:` connection is assigned a unique URI
/// (`file:portadb_rusqlite_mem_{n}?mode=memory&cache=shared`).
/// Callers that intend to share an in-memory database across connections on purpose
/// specify an explicit named shared URI like `file:name?mode=memory&cache=shared`.
static NEXT_MEMORY_DB: AtomicU64 = AtomicU64::new(0);

impl Driver {
    /// Creates a new `Driver` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DriverTrait for Driver {
    type Connection = Connection;

    fn name(&self) -> &'static str {
        "rusqlite"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite)
    }

    fn capabilities(&self) -> portadb_core::capability::Capabilities {
        portadb_core::capability::Capabilities::sqlite()
    }

    async fn connect(&self, options: &ConnectOptions) -> Result<Self::Connection, Error> {
        let url = options.url.trim();
        let is_memory = url == ":memory:"
            || url.is_empty()
            || url == "sqlite::memory:"
            || url == "sqlite://:memory:";
        let conn = if is_memory {
            // Each `:memory:` connection gets its own private database via a unique name,
            // matching SQLite's standard private `:memory:` semantics.
            // Callers that need shared state across connections use an explicit named URL
            // (e.g. `file:shared_x?mode=memory&cache=shared`).
            let id = NEXT_MEMORY_DB.fetch_add(1, Ordering::Relaxed);
            let mem_url = format!("file:portadb_rusqlite_mem_{id}?mode=memory&cache=shared");
            rusqlite::Connection::open_with_flags(
                &mem_url,
                rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
                    | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                    | rusqlite::OpenFlags::SQLITE_OPEN_URI,
            )
        } else {
            let clean_path = url
                .strip_prefix("sqlite://")
                .or_else(|| url.strip_prefix("sqlite:"))
                .unwrap_or(url);
            if clean_path.starts_with("file:") {
                rusqlite::Connection::open_with_flags(
                    clean_path,
                    rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
                        | rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                        | rusqlite::OpenFlags::SQLITE_OPEN_URI,
                )
            } else {
                rusqlite::Connection::open(clean_path)
            }
        }
        .map_err(map_rusqlite_error)?;

        Ok(Connection {
            inner: Arc::new(Mutex::new(Some(conn))),
        })
    }
}

/// Active connection handle backed by `rusqlite`.
pub struct Connection {
    inner: Arc<Mutex<Option<rusqlite::Connection>>>,
}

/// Backward-compatible alias for [`Connection`].
pub type RusqliteConnection = Connection;

enum RowSink<'s, F, V> {
    Owned(F),
    Ref(&'s mut V),
}

struct NoVisitor;

impl RowVisitor for NoVisitor {
    fn visit<R: RowRef>(&mut self, _row: &R) -> Result<ControlFlow<()>, Error> {
        Ok(ControlFlow::Continue(()))
    }
}

struct RusqliteRowRef<'a, 'stmt> {
    columns: &'a Arc<[String]>,
    row: &'a rusqlite::Row<'stmt>,
}

impl<'a, 'stmt> RowRef for RusqliteRowRef<'a, 'stmt> {
    fn columns(&self) -> &[String] {
        self.columns
    }

    fn value(&self, index: usize) -> Result<PortaDbValueRef<'_>, Error> {
        if index >= self.columns.len() {
            return Err(Error::QueryError(format!(
                "column index {index} out of range (row has {} columns)",
                self.columns.len()
            )));
        }
        let val_ref = self.row.get_ref(index).map_err(map_rusqlite_error)?;
        match val_ref {
            rusqlite::types::ValueRef::Null => Ok(PortaDbValueRef::Null),
            rusqlite::types::ValueRef::Integer(i) => Ok(PortaDbValueRef::Integer(i)),
            rusqlite::types::ValueRef::Real(f) => Ok(PortaDbValueRef::Real(f)),
            rusqlite::types::ValueRef::Text(t) => {
                let s = std::str::from_utf8(t).map_err(|_| Error::TypeMismatch {
                    column: self.columns[index].clone(),
                    expected: "valid UTF-8 text",
                    found: "invalid UTF-8 bytes".to_string(),
                })?;
                Ok(PortaDbValueRef::Text(s))
            }
            rusqlite::types::ValueRef::Blob(b) => Ok(PortaDbValueRef::Blob(b)),
        }
    }
}

impl Connection {
    /// Escape hatch: safely locks and provides access to the underlying [`rusqlite::Connection`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::ConnectionLost`] if the mutex is poisoned.
    pub fn raw_lock(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, Option<rusqlite::Connection>>, Error> {
        self.inner
            .lock()
            .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))
    }

    fn step_internal<F, V>(
        &self,
        sql: &str,
        params: &NamedParams,
        sink: &mut RowSink<'_, F, V>,
    ) -> Result<u64, Error>
    where
        F: FnMut(Row) -> ControlFlow<()>,
        V: RowVisitor,
    {
        let prepared = prepare_query(sql, params)?;
        let guard = self
            .inner
            .lock()
            .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
        let conn = guard
            .as_ref()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut stmt = conn.prepare(&prepared.sql).map_err(map_rusqlite_error)?;
        let rusqlite_params: Vec<Box<dyn rusqlite::ToSql>> = prepared
            .params
            .iter()
            .map(|v| -> Box<dyn rusqlite::ToSql> {
                match v {
                    Value::Null => Box::new(rusqlite::types::Null),
                    Value::Integer(i) => Box::new(*i),
                    Value::Real(r) => Box::new(*r),
                    Value::Text(s) => Box::new(s.clone()),
                    Value::Blob(b) => Box::new(b.clone()),
                    Value::Boolean(b) => Box::new(*b),
                }
            })
            .collect();

        let slice_params: Vec<&dyn rusqlite::ToSql> = rusqlite_params
            .iter()
            .map(|b| &**b as &dyn rusqlite::ToSql)
            .collect();

        let col_count = stmt.column_count();
        let column_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect();
        let columns: Arc<[String]> = column_names.into();

        let mut rusqlite_rows = stmt
            .query(slice_params.as_slice())
            .map_err(map_rusqlite_error)?;
        let mut count = 0u64;

        while let Some(r) = rusqlite_rows.next().map_err(map_rusqlite_error)? {
            count += 1;
            match sink {
                RowSink::Owned(on_row) => {
                    let mut values = Vec::with_capacity(col_count);
                    for i in 0..col_count {
                        let val_ref = r.get_ref(i).map_err(map_rusqlite_error)?;
                        let v = match val_ref {
                            rusqlite::types::ValueRef::Null => Value::Null,
                            rusqlite::types::ValueRef::Integer(i) => Value::Integer(i),
                            rusqlite::types::ValueRef::Real(f) => Value::Real(f),
                            rusqlite::types::ValueRef::Text(t) => {
                                Value::Text(String::from_utf8_lossy(t).into_owned())
                            }
                            rusqlite::types::ValueRef::Blob(b) => Value::Blob(b.to_vec()),
                        };
                        values.push(v);
                    }
                    let row = Row::with_columns(Arc::clone(&columns), values);
                    if on_row(row).is_break() {
                        break;
                    }
                }
                RowSink::Ref(visitor) => {
                    let row_ref = RusqliteRowRef {
                        columns: &columns,
                        row: r,
                    };
                    if visitor.visit(&row_ref)?.is_break() {
                        break;
                    }
                }
            }
        }

        Ok(count)
    }
}

impl DriverConnection for Connection {
    async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        let prepared = prepare_query(sql, params)?;
        let guard = self
            .inner
            .lock()
            .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
        let conn = guard
            .as_ref()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut stmt = conn.prepare(&prepared.sql).map_err(map_rusqlite_error)?;
        let rusqlite_params: Vec<Box<dyn rusqlite::ToSql>> = prepared
            .params
            .iter()
            .map(|v| -> Box<dyn rusqlite::ToSql> {
                match v {
                    Value::Null => Box::new(rusqlite::types::Null),
                    Value::Integer(i) => Box::new(*i),
                    Value::Real(r) => Box::new(*r),
                    Value::Text(s) => Box::new(s.clone()),
                    Value::Blob(b) => Box::new(b.clone()),
                    Value::Boolean(b) => Box::new(*b),
                }
            })
            .collect();

        let slice_params: Vec<&dyn rusqlite::ToSql> = rusqlite_params
            .iter()
            .map(|b| &**b as &dyn rusqlite::ToSql)
            .collect();

        // A statement that answers with rows (`PRAGMA journal_mode = WAL`,
        // `INSERT ... RETURNING`) makes rusqlite's `execute` fail with
        // `ExecuteReturnedResults` after its first step already ran. Decide
        // BEFORE running it: such a statement is stepped to the end once, its
        // rows dropped, and the affected count read from the connection.
        if stmt.column_count() > 0 {
            let mut rows = stmt
                .query(slice_params.as_slice())
                .map_err(map_rusqlite_error)?;
            while rows.next().map_err(map_rusqlite_error)?.is_some() {}
            drop(rows);
            return Ok(conn.changes());
        }

        let affected = stmt
            .execute(slice_params.as_slice())
            .map_err(map_rusqlite_error)?;

        u64::try_from(affected).map_err(|e| Error::QueryError(e.to_string()))
    }

    async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let mut rows = Vec::new();
        let on_row = |row: Row| {
            rows.push(row);
            ControlFlow::Continue(())
        };
        let mut sink = RowSink::<_, NoVisitor>::Owned(on_row);
        self.step_internal(sql, params, &mut sink)?;
        Ok(rows)
    }

    async fn query_each<F: FnMut(Row) -> ControlFlow<()> + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        on_row: F,
    ) -> Result<u64, Error> {
        let mut sink = RowSink::<F, NoVisitor>::Owned(on_row);
        self.step_internal(sql, params, &mut sink)
    }

    async fn query_each_ref<V: RowVisitor + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> Result<u64, Error> {
        let mut sink = RowSink::<fn(Row) -> ControlFlow<()>, V>::Ref(visitor);
        self.step_internal(sql, params, &mut sink)
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        {
            let guard = self
                .inner
                .lock()
                .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
            let conn = guard
                .as_ref()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            conn.execute_batch("BEGIN TRANSACTION;")
                .map_err(map_rusqlite_error)?;
        }
        let tx: Box<dyn DriverTransaction + '_> = Box::new(Transaction {
            conn: self,
            is_active: true,
        });
        Ok(tx)
    }

    async fn close(&mut self) -> Result<(), Error> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
        if let Some(conn) = guard.take() {
            conn.close().map_err(|(_, err)| map_rusqlite_error(err))?;
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Transaction handle backed by `rusqlite`.
pub struct Transaction<'a> {
    conn: &'a mut Connection,
    is_active: bool,
}

/// Backward-compatible alias for [`Transaction`].
pub type RusqliteTransaction<'a> = Transaction<'a>;

impl DriverTransaction for Transaction<'_> {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<u64, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.execute(sql, params).await
        })
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.conn.query(sql, params).await
        })
    }

    fn commit<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            let guard = self
                .conn
                .inner
                .lock()
                .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
            let conn = guard
                .as_ref()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            conn.execute_batch("COMMIT TRANSACTION;")
                .map_err(map_rusqlite_error)?;
            Ok(())
        })
    }

    fn rollback<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            let guard = self
                .conn
                .inner
                .lock()
                .map_err(|_| Error::ConnectionLost("rusqlite mutex poisoned".to_string()))?;
            let conn = guard
                .as_ref()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            conn.execute_batch("ROLLBACK TRANSACTION;")
                .map_err(map_rusqlite_error)?;
            Ok(())
        })
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.is_active {
            self.is_active = false;
            if let Ok(guard) = self.conn.inner.lock()
                && let Some(conn) = guard.as_ref()
            {
                let _ = conn.execute_batch("ROLLBACK TRANSACTION;");
            }
        }
    }
}

fn map_rusqlite_error(err: rusqlite::Error) -> Error {
    match err {
        rusqlite::Error::SqliteFailure(ffi_err, msg) => {
            let code = ffi_err.extended_code;
            let message = msg.unwrap_or_else(|| ffi_err.to_string());
            if code == 2067
                || code == 1555
                || (ffi_err.code == rusqlite::ErrorCode::ConstraintViolation
                    && message.contains("UNIQUE"))
            {
                Error::UniqueViolation {
                    constraint: None,
                    message,
                }
            } else if code == 787
                || (ffi_err.code == rusqlite::ErrorCode::ConstraintViolation
                    && message.contains("FOREIGN KEY"))
            {
                Error::ForeignKeyViolation {
                    constraint: None,
                    message,
                }
            } else {
                Error::QueryError(message)
            }
        }
        _ => Error::from_engine(err),
    }
}

#[cfg(test)]
#[path = "../tests/rusqlite/mod.rs"]
mod test;
