use std::any::Any;
use std::future::Future;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use sqlx::{Column, Row as _, TypeInfo, ValueRef};

use portadb_core::database::Database;
use portadb_core::driver::{Driver as DriverTrait, DriverConnection, DriverTransaction};
use portadb_core::error::Error;
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;
use portadb_core::query::prepare_query;
use portadb_core::row::Row;
use portadb_core::row_ref::{RowRef, RowVisitor, ValueRef as PortaDbValueRef};
use portadb_core::value::Value;

/// Driver for SQLite using the `sqlx` engine library.
#[derive(Debug, Default, Clone, Copy)]
pub struct Driver;

/// Backward-compatible alias for [`Driver`].
pub type SqlxDriver = Driver;

/// Backward-compatible alias for [`Driver`].
pub type SqlxSqliteDriver = Driver;

/// Process-wide counter for generating unique private in-memory database names.
///
/// In SQLite every `:memory:` connection is private. Each `connect` here opens
/// ONE `SqliteConnection`, and a memory URL gets a process-unique name
/// (`sqlite:file:portadb_sqlx_mem_{n}?mode=memory&cache=shared`), so two
/// connections never see each other's tables. Callers that want to share an
/// in-memory database on purpose pass an explicit named URI such as
/// `file:name?mode=memory&cache=shared`.
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
        "sqlx"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Sqlite)
    }

    fn capabilities(&self) -> portadb_core::capability::Capabilities {
        portadb_core::capability::Capabilities::sqlite()
    }

    async fn connect(&self, options: &ConnectOptions) -> Result<Self::Connection, Error> {
        use sqlx::Connection as _;
        let url = normalize_sqlite_url(&options.url);
        let conn = sqlx::SqliteConnection::connect(&url)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Connection {
            inner: Arc::new(tokio::sync::Mutex::new(Some(conn))),
        })
    }
}

/// Active connection handle backed by `sqlx`.
pub struct Connection {
    inner: Arc<tokio::sync::Mutex<Option<sqlx::SqliteConnection>>>,
}

/// Backward-compatible alias for [`Connection`].
pub type SqlxConnection = Connection;

/// Backward-compatible alias for [`Connection`].
pub type SqlxSqliteConnection = Connection;

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

struct SqlxSqliteRowRef<'a> {
    columns: &'a [String],
    row: &'a sqlx::sqlite::SqliteRow,
}

impl<'a> RowRef for SqlxSqliteRowRef<'a> {
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
        let raw_val = self.row.try_get_raw(index).map_err(map_sqlx_error)?;
        if raw_val.is_null() {
            return Ok(PortaDbValueRef::Null);
        }
        let type_info = self.row.columns()[index].type_info();
        let type_name = type_info.name();

        match type_name {
            "INTEGER" | "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => {
                let n: i64 = self.row.try_get(index).map_err(map_sqlx_error)?;
                Ok(PortaDbValueRef::Integer(n))
            }
            "REAL" | "FLOAT" | "DOUBLE" => {
                let f: f64 = self.row.try_get(index).map_err(map_sqlx_error)?;
                Ok(PortaDbValueRef::Real(f))
            }
            "TEXT" | "VARCHAR" | "CHAR" => {
                let s: &str = self.row.try_get(index).map_err(map_sqlx_error)?;
                Ok(PortaDbValueRef::Text(s))
            }
            "BLOB" => {
                let b: &[u8] = self.row.try_get(index).map_err(map_sqlx_error)?;
                Ok(PortaDbValueRef::Blob(b))
            }
            "BOOLEAN" | "BOOL" => {
                let b: bool = self.row.try_get(index).map_err(map_sqlx_error)?;
                Ok(PortaDbValueRef::Boolean(b))
            }
            _ => {
                if let Ok(s) = self.row.try_get::<&str, _>(index) {
                    Ok(PortaDbValueRef::Text(s))
                } else if let Ok(n) = self.row.try_get::<i64, _>(index) {
                    Ok(PortaDbValueRef::Integer(n))
                } else if let Ok(r) = self.row.try_get::<f64, _>(index) {
                    Ok(PortaDbValueRef::Real(r))
                } else if let Ok(b) = self.row.try_get::<&[u8], _>(index) {
                    Ok(PortaDbValueRef::Blob(b))
                } else {
                    Ok(PortaDbValueRef::Null)
                }
            }
        }
    }
}

impl Connection {
    /// Escape hatch: returns a clone of the internal mutex-wrapped `sqlx::SqliteConnection`.
    #[must_use]
    pub fn raw_conn(&self) -> &Arc<tokio::sync::Mutex<Option<sqlx::SqliteConnection>>> {
        &self.inner
    }

    async fn step_internal<F, V>(
        &self,
        sql: &str,
        params: &NamedParams,
        sink: &mut RowSink<'_, F, V>,
    ) -> Result<u64, Error>
    where
        F: FnMut(Row) -> ControlFlow<()> + Send,
        V: RowVisitor + Send,
    {
        let prepared = prepare_query(sql, params)?;
        let mut guard = self.inner.lock().await;
        let conn = guard
            .as_mut()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut query = sqlx::query(&prepared.sql);
        for p in &prepared.params {
            query = bind_param(query, p);
        }

        use futures_util::TryStreamExt as _;
        use sqlx::Executor as _;
        let mut stream = conn.fetch(query);

        let mut columns: Option<Arc<[String]>> = None;
        let mut count = 0u64;

        while let Some(sqlx_row) = stream.try_next().await.map_err(map_sqlx_error)? {
            let cols = columns.get_or_insert_with(|| {
                let column_names: Vec<String> = sqlx_row
                    .columns()
                    .iter()
                    .map(|c| c.name().to_string())
                    .collect();
                column_names.into()
            });

            count += 1;
            match sink {
                RowSink::Owned(on_row) => {
                    let row = convert_sqlx_row(&sqlx_row, cols)?;
                    if on_row(row).is_break() {
                        break;
                    }
                }
                RowSink::Ref(visitor) => {
                    let row_ref = SqlxSqliteRowRef {
                        columns: cols,
                        row: &sqlx_row,
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
        let mut guard = self.inner.lock().await;
        let conn = guard
            .as_mut()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut query = sqlx::query(&prepared.sql);
        for p in &prepared.params {
            query = bind_param(query, p);
        }

        use sqlx::Executor as _;
        let result = conn.execute(query).await.map_err(map_sqlx_error)?;

        Ok(result.rows_affected())
    }

    async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let mut rows = Vec::new();
        let on_row = |row: Row| {
            rows.push(row);
            ControlFlow::Continue(())
        };
        let mut sink = RowSink::<_, NoVisitor>::Owned(on_row);
        self.step_internal(sql, params, &mut sink).await?;
        Ok(rows)
    }

    async fn query_each<F: FnMut(Row) -> ControlFlow<()> + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        on_row: F,
    ) -> Result<u64, Error> {
        let mut sink = RowSink::<F, NoVisitor>::Owned(on_row);
        self.step_internal(sql, params, &mut sink).await
    }

    async fn query_each_ref<V: RowVisitor + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> Result<u64, Error> {
        let mut sink = RowSink::<fn(Row) -> ControlFlow<()>, V>::Ref(visitor);
        self.step_internal(sql, params, &mut sink).await
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        {
            let mut guard = self.inner.lock().await;
            let conn = guard
                .as_mut()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            use sqlx::Executor as _;
            conn.execute("BEGIN TRANSACTION;")
                .await
                .map_err(map_sqlx_error)?;
        }

        let tx: Box<dyn DriverTransaction + '_> = Box::new(Transaction {
            conn: self,
            is_active: true,
        });
        Ok(tx)
    }

    async fn close(&mut self) -> Result<(), Error> {
        let mut guard = self.inner.lock().await;
        if let Some(conn) = guard.take() {
            use sqlx::Connection as _;
            conn.close().await.map_err(map_sqlx_error)?;
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

/// Active transaction handle backed by `sqlx`.
pub struct Transaction<'a> {
    conn: &'a mut Connection,
    is_active: bool,
}

/// Backward-compatible alias for [`Transaction`].
pub type SqlxTransaction<'a> = Transaction<'a>;

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
            let mut guard = self.conn.inner.lock().await;
            let conn = guard
                .as_mut()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            use sqlx::Executor as _;
            conn.execute("COMMIT TRANSACTION;")
                .await
                .map_err(map_sqlx_error)?;
            Ok(())
        })
    }

    fn rollback<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            if !self.is_active {
                return Err(Error::TransactionAlreadyClosed);
            }
            self.is_active = false;
            let mut guard = self.conn.inner.lock().await;
            let conn = guard
                .as_mut()
                .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;
            use sqlx::Executor as _;
            conn.execute("ROLLBACK TRANSACTION;")
                .await
                .map_err(map_sqlx_error)?;
            Ok(())
        })
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if self.is_active {
            self.is_active = false;
            let inner_clone = Arc::clone(&self.conn.inner);
            tokio::spawn(async move {
                if let Ok(mut guard) = inner_clone.try_lock()
                    && let Some(conn) = guard.as_mut()
                {
                    use sqlx::Executor as _;
                    let _ = conn.execute("ROLLBACK TRANSACTION;").await;
                }
            });
        }
    }
}

fn normalize_sqlite_url(url: &str) -> String {
    let trimmed = url.trim();
    if trimmed == ":memory:"
        || trimmed.is_empty()
        || trimmed == "sqlite::memory:"
        || trimmed == "sqlite://:memory:"
    {
        // A process-unique name keeps independent connections private, as
        // SQLite's own `:memory:` is.
        let id = NEXT_MEMORY_DB.fetch_add(1, Ordering::Relaxed);
        format!("sqlite:file:portadb_sqlx_mem_{id}?mode=memory&cache=shared")
    } else if !trimmed.starts_with("sqlite:") {
        format!("sqlite:{trimmed}")
    } else {
        trimmed.to_string()
    }
}

fn bind_param<'q>(
    query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    val: &'q Value,
) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
    match val {
        Value::Null => query.bind(Option::<String>::None),
        Value::Integer(i) => query.bind(*i),
        Value::Real(r) => query.bind(*r),
        Value::Text(s) => query.bind(s.as_str()),
        Value::Blob(b) => query.bind(b.as_slice()),
        Value::Boolean(b) => query.bind(*b),
    }
}

fn convert_sqlx_row(
    sqlx_row: &sqlx::sqlite::SqliteRow,
    columns: &Arc<[String]>,
) -> Result<Row, Error> {
    let col_count = columns.len();
    let mut values = Vec::with_capacity(col_count);

    for i in 0..col_count {
        let raw_val = sqlx_row.try_get_raw(i).map_err(map_sqlx_error)?;
        let val = if raw_val.is_null() {
            Value::Null
        } else {
            let type_info = sqlx_row.columns()[i].type_info();
            let type_name = type_info.name();

            match type_name {
                "INTEGER" | "INT" | "BIGINT" | "SMALLINT" | "TINYINT" => {
                    let n: i64 = sqlx_row.try_get(i).map_err(map_sqlx_error)?;
                    Value::Integer(n)
                }
                "REAL" | "FLOAT" | "DOUBLE" => {
                    let f: f64 = sqlx_row.try_get(i).map_err(map_sqlx_error)?;
                    Value::Real(f)
                }
                "TEXT" | "VARCHAR" | "CHAR" => {
                    let s: String = sqlx_row.try_get(i).map_err(map_sqlx_error)?;
                    Value::Text(s)
                }
                "BLOB" => {
                    let b: Vec<u8> = sqlx_row.try_get(i).map_err(map_sqlx_error)?;
                    Value::Blob(b)
                }
                "BOOLEAN" | "BOOL" => {
                    let b: bool = sqlx_row.try_get(i).map_err(map_sqlx_error)?;
                    Value::Boolean(b)
                }
                _ => {
                    if let Ok(s) = sqlx_row.try_get::<String, _>(i) {
                        Value::Text(s)
                    } else if let Ok(n) = sqlx_row.try_get::<i64, _>(i) {
                        Value::Integer(n)
                    } else if let Ok(r) = sqlx_row.try_get::<f64, _>(i) {
                        Value::Real(r)
                    } else if let Ok(b) = sqlx_row.try_get::<Vec<u8>, _>(i) {
                        Value::Blob(b)
                    } else {
                        Value::Null
                    }
                }
            }
        };
        values.push(val);
    }

    Ok(Row::with_columns(Arc::clone(columns), values))
}

fn map_sqlx_error(err: sqlx::Error) -> Error {
    match &err {
        sqlx::Error::Database(db_err) => {
            if db_err.is_unique_violation() {
                Error::UniqueViolation {
                    constraint: None,
                    message: db_err.message().to_string(),
                }
            } else if db_err.is_foreign_key_violation() {
                Error::ForeignKeyViolation {
                    constraint: None,
                    message: db_err.message().to_string(),
                }
            } else {
                Error::QueryError(db_err.message().to_string())
            }
        }
        sqlx::Error::Io(io_err) => Error::ConnectionLost(io_err.to_string()),
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => {
            Error::ConnectionLost(err.to_string())
        }
        sqlx::Error::Configuration(msg) => Error::ConfigurationError(msg.to_string()),
        _ => Error::from_engine(err),
    }
}

#[cfg(test)]
#[path = "../tests/sqlx_sqlite/mod.rs"]
mod test;
