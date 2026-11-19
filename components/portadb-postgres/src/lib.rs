use std::any::Any;
use std::future::Future;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::sync::Arc;

use sqlx::{Column, Row as _, TypeInfo, ValueRef};

use portadb_core::database::Database;
use portadb_core::dialect::{Dialect, PostgresDialect};
use portadb_core::driver::{Driver as DriverTrait, DriverConnection, DriverTransaction};
use portadb_core::error::Error;
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;
use portadb_core::row::Row;
use portadb_core::row_ref::{RowRef, RowVisitor, ValueRef as PortaDbValueRef};
use portadb_core::value::Value;

/// Driver for PostgreSQL using the `sqlx` engine library.
#[derive(Debug, Default, Clone, Copy)]
pub struct SqlxPostgresDriver;

/// Standard alias for PostgreSQL driver.
pub type Driver = SqlxPostgresDriver;
/// Standard alias for PostgreSQL connection.
pub type Connection = SqlxPostgresConnection;
/// Standard alias for PostgreSQL transaction.
pub type Transaction<'a> = SqlxPostgresTransaction<'a>;

impl SqlxPostgresDriver {
    /// Creates a new `SqlxPostgresDriver` instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl DriverTrait for SqlxPostgresDriver {
    type Connection = SqlxPostgresConnection;

    fn name(&self) -> &'static str {
        "sqlx-postgres"
    }

    fn supports_database(&self, database: Database) -> bool {
        matches!(database, Database::Postgres)
    }

    fn capabilities(&self) -> portadb_core::capability::Capabilities {
        portadb_core::capability::Capabilities::postgres()
    }

    async fn connect(&self, options: &ConnectOptions) -> Result<Self::Connection, Error> {
        use sqlx::Connection;
        let conn = sqlx::PgConnection::connect(&options.url)
            .await
            .map_err(map_sqlx_postgres_error)?;

        Ok(SqlxPostgresConnection {
            inner: Arc::new(tokio::sync::Mutex::new(Some(conn))),
        })
    }
}

/// Active connection handle backed by `sqlx::PgConnection`.
pub struct SqlxPostgresConnection {
    inner: Arc<tokio::sync::Mutex<Option<sqlx::PgConnection>>>,
}

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

struct SqlxPostgresRowRef<'a> {
    columns: &'a [String],
    row: &'a sqlx::postgres::PgRow,
}

/// Borrowed row view over a PostgreSQL row.
///
/// Unlike the owned `Row` conversion which formats unsupported PostgreSQL types
/// as `Value::Text("<unsupported pg type: ...>")`, typed borrowed decoding returns
/// [`Error::TypeMismatch`] because a borrowed view cannot construct or own formatted strings.
impl<'a> RowRef for SqlxPostgresRowRef<'a> {
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
        let raw_val = self
            .row
            .try_get_raw(index)
            .map_err(map_sqlx_postgres_error)?;
        if raw_val.is_null() {
            return Ok(PortaDbValueRef::Null);
        }
        let type_info = self.row.columns()[index].type_info();
        let type_name = type_info.name();

        match type_name {
            "BOOL" => {
                let b: bool = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Boolean(b))
            }
            "INT2" => {
                let n: i16 = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Integer(i64::from(n)))
            }
            "INT4" => {
                let n: i32 = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Integer(i64::from(n)))
            }
            "INT8" => {
                let n: i64 = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Integer(n))
            }
            "FLOAT4" => {
                let f: f32 = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Real(f64::from(f)))
            }
            "FLOAT8" => {
                let f: f64 = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Real(f))
            }
            "TEXT" | "VARCHAR" | "CHAR" | "NAME" => {
                let s: &str = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Text(s))
            }
            "BYTEA" => {
                let b: &[u8] = self.row.try_get(index).map_err(map_sqlx_postgres_error)?;
                Ok(PortaDbValueRef::Blob(b))
            }
            _ => {
                if let Ok(s) = self.row.try_get::<&str, _>(index) {
                    Ok(PortaDbValueRef::Text(s))
                } else if let Ok(n) = self.row.try_get::<i64, _>(index) {
                    Ok(PortaDbValueRef::Integer(n))
                } else {
                    Err(Error::TypeMismatch {
                        column: self.columns[index].clone(),
                        expected: "supported postgres type",
                        found: type_name.to_string(),
                    })
                }
            }
        }
    }
}

impl SqlxPostgresConnection {
    /// Escape hatch: provides access to the underlying [`sqlx::PgConnection`] mutex.
    #[must_use]
    pub fn raw_conn(&self) -> &Arc<tokio::sync::Mutex<Option<sqlx::PgConnection>>> {
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
        let prepared = PostgresDialect.prepare_query(sql, params)?;
        let mut guard = self.inner.lock().await;
        let conn = guard
            .as_mut()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut query = sqlx::query(&prepared.sql);
        for p in &prepared.params {
            query = bind_postgres_param(query, p);
        }

        use futures_util::TryStreamExt as _;
        use sqlx::Executor as _;
        let mut stream = conn.fetch(query);

        let mut columns: Option<Arc<[String]>> = None;
        let mut count = 0u64;

        while let Some(pg_row) = stream.try_next().await.map_err(map_sqlx_postgres_error)? {
            let cols = columns.get_or_insert_with(|| {
                let column_names: Vec<String> = pg_row
                    .columns()
                    .iter()
                    .map(|c| c.name().to_string())
                    .collect();
                column_names.into()
            });

            count += 1;
            match sink {
                RowSink::Owned(on_row) => {
                    let row = convert_sqlx_postgres_row(&pg_row, cols)?;
                    if on_row(row).is_break() {
                        break;
                    }
                }
                RowSink::Ref(visitor) => {
                    let row_ref = SqlxPostgresRowRef {
                        columns: cols,
                        row: &pg_row,
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

impl DriverConnection for SqlxPostgresConnection {
    async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        let prepared = PostgresDialect.prepare_query(sql, params)?;
        let mut guard = self.inner.lock().await;
        let conn = guard
            .as_mut()
            .ok_or_else(|| Error::ConnectionLost("connection is closed".to_string()))?;

        let mut query = sqlx::query(&prepared.sql);
        for p in &prepared.params {
            query = bind_postgres_param(query, p);
        }

        use sqlx::Executor as _;
        let result = conn.execute(query).await.map_err(map_sqlx_postgres_error)?;

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
            conn.execute("BEGIN;")
                .await
                .map_err(map_sqlx_postgres_error)?;
        }

        let tx: Box<dyn DriverTransaction + '_> = Box::new(SqlxPostgresTransaction {
            conn: self,
            is_active: true,
        });
        Ok(tx)
    }

    async fn close(&mut self) -> Result<(), Error> {
        let mut guard = self.inner.lock().await;
        if let Some(conn) = guard.take() {
            use sqlx::Connection;
            conn.close().await.map_err(map_sqlx_postgres_error)?;
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

/// Transaction handle backed by `sqlx::PgConnection`.
pub struct SqlxPostgresTransaction<'a> {
    conn: &'a mut SqlxPostgresConnection,
    is_active: bool,
}

impl DriverTransaction for SqlxPostgresTransaction<'_> {
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
            conn.execute("COMMIT;")
                .await
                .map_err(map_sqlx_postgres_error)?;
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
            conn.execute("ROLLBACK;")
                .await
                .map_err(map_sqlx_postgres_error)?;
            Ok(())
        })
    }
}

impl Drop for SqlxPostgresTransaction<'_> {
    fn drop(&mut self) {
        if self.is_active {
            self.is_active = false;
            let inner_clone = Arc::clone(&self.conn.inner);
            tokio::spawn(async move {
                if let Ok(mut guard) = inner_clone.try_lock()
                    && let Some(conn) = guard.as_mut()
                {
                    use sqlx::Executor as _;
                    let _ = conn.execute("ROLLBACK;").await;
                }
            });
        }
    }
}

fn bind_postgres_param<'q>(
    query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
    val: &'q Value,
) -> sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments> {
    match val {
        Value::Null => query.bind(Option::<String>::None),
        Value::Integer(i) => query.bind(*i),
        Value::Real(r) => query.bind(*r),
        Value::Text(s) => query.bind(s.as_str()),
        Value::Blob(b) => query.bind(b.as_slice()),
        Value::Boolean(b) => query.bind(*b),
    }
}

fn convert_sqlx_postgres_row(
    pg_row: &sqlx::postgres::PgRow,
    columns: &Arc<[String]>,
) -> Result<Row, Error> {
    let col_count = columns.len();
    let mut values = Vec::with_capacity(col_count);

    for i in 0..col_count {
        let raw_val = pg_row.try_get_raw(i).map_err(map_sqlx_postgres_error)?;
        let v = if raw_val.is_null() {
            Value::Null
        } else {
            let type_info = pg_row.columns()[i].type_info();
            let type_name = type_info.name();

            match type_name {
                "BOOL" => {
                    let b: bool = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Boolean(b)
                }
                "INT2" => {
                    let n: i16 = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Integer(i64::from(n))
                }
                "INT4" => {
                    let n: i32 = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Integer(i64::from(n))
                }
                "INT8" => {
                    let n: i64 = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Integer(n)
                }
                "FLOAT4" => {
                    let f: f32 = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Real(f64::from(f))
                }
                "FLOAT8" => {
                    let f: f64 = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Real(f)
                }
                "TEXT" | "VARCHAR" | "CHAR" | "NAME" => {
                    let s: String = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Text(s)
                }
                "BYTEA" => {
                    let b: Vec<u8> = pg_row.try_get(i).map_err(map_sqlx_postgres_error)?;
                    Value::Blob(b)
                }
                _ => {
                    if let Ok(s) = pg_row.try_get::<String, _>(i) {
                        Value::Text(s)
                    } else if let Ok(n) = pg_row.try_get::<i64, _>(i) {
                        Value::Integer(n)
                    } else {
                        Value::Text(format!("<unsupported pg type: {type_name}>"))
                    }
                }
            }
        };
        values.push(v);
    }

    Ok(Row::with_columns(Arc::clone(columns), values))
}

fn map_sqlx_postgres_error(err: sqlx::Error) -> Error {
    match err {
        sqlx::Error::Database(db_err) => {
            let message = db_err.message().to_string();
            let constraint = db_err.constraint().map(std::string::ToString::to_string);

            if let Some(code) = db_err.code() {
                // PostgreSQL error codes:
                // 23505: unique_violation
                // 23503: foreign_key_violation
                if code == "23505" {
                    return Error::UniqueViolation {
                        constraint,
                        message,
                    };
                } else if code == "23503" {
                    return Error::ForeignKeyViolation {
                        constraint,
                        message,
                    };
                }
            }

            if message.contains("duplicate key value") || message.contains("unique constraint") {
                Error::UniqueViolation {
                    constraint,
                    message,
                }
            } else if message.contains("foreign key constraint") {
                Error::ForeignKeyViolation {
                    constraint,
                    message,
                }
            } else {
                Error::QueryError(message)
            }
        }
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
            Error::ConnectionLost("sqlx postgres pool closed or timed out".to_string())
        }
        sqlx::Error::Io(io_err) => Error::ConnectionLost(io_err.to_string()),
        _ => Error::from_engine(err),
    }
}
