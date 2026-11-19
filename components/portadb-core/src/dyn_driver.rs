//! Dynamic driver adapters for runtime database engine dispatch.
//!
//! The dynamic driver adapter forms exist only where the engine is chosen at runtime
//! (such as via [`DriverRegistry`](crate::DriverRegistry), [`Engine`](crate::Engine),
//! or [`Pool`](crate::Pool)). Statically generated applications with compile-time known
//! engines use [`Connection<D::Connection>`](crate::Connection) directly.

use std::any::Any;
use std::ops::ControlFlow;

use crate::capability::Capabilities;
use crate::database::Database;
use crate::driver::{BoxFuture, Driver, DriverConnection, DriverTransaction};
use crate::error::Error;
use crate::options::ConnectOptions;
use crate::params::NamedParams;
use crate::row::Row;
use crate::row_ref::{OnRowRef, RowRef, RowVisitor};

/// Object-safe driver trait for runtime engine resolution.
pub trait DynDriver: Send + Sync + 'static {
    /// Canonical name of this engine library (e.g. "rusqlite", "sqlx").
    fn name(&self) -> &'static str;

    /// Checks if this engine supports the specified RDBMS.
    fn supports_database(&self, database: Database) -> bool;

    /// Returns the capability matrix supported by this driver engine.
    fn capabilities(&self) -> Capabilities;

    /// Connects to the database and returns a boxed [`DynDriverConnection`].
    fn connect<'a>(
        &'a self,
        options: &'a ConnectOptions,
    ) -> BoxFuture<'a, Result<DynConnection, Error>>;
}

/// Object-safe connection handle mirror of [`DriverConnection`] using boxed futures.
pub trait DynDriverConnection: Send + Sync + 'static {
    /// Executes a command returning the count of affected rows.
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>>;

    /// Queries the database returning rows.
    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>>;

    /// Hands each row to `on_row` as soon as the engine yields it.
    fn query_each<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
        on_row: &'a mut (dyn FnMut(Row) -> ControlFlow<()> + Send),
    ) -> BoxFuture<'a, Result<u64, Error>>;

    /// Hands each row as a borrowed [`RowRef`] to `on_row`.
    fn query_each_ref<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
        on_row: &'a mut OnRowRef<'a>,
    ) -> BoxFuture<'a, Result<u64, Error>>;

    /// Begins a database transaction.
    fn begin_transaction<'a>(
        &'a mut self,
    ) -> BoxFuture<'a, Result<Box<dyn DriverTransaction + 'a>, Error>>;

    /// Checks health of the connection.
    fn ping<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>>;

    /// Closes the connection.
    fn close<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>>;

    /// Downcast reference for raw driver escape hatch.
    fn as_any(&self) -> &dyn Any;

    /// Downcast mutable reference for raw driver escape hatch.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Dynamic boxed driver connection.
pub type DynConnection = Box<dyn DynDriverConnection>;

impl<D: Driver> DynDriver for D {
    fn name(&self) -> &'static str {
        Driver::name(self)
    }

    fn supports_database(&self, database: Database) -> bool {
        Driver::supports_database(self, database)
    }

    fn capabilities(&self) -> Capabilities {
        Driver::capabilities(self)
    }

    fn connect<'a>(
        &'a self,
        options: &'a ConnectOptions,
    ) -> BoxFuture<'a, Result<DynConnection, Error>> {
        Box::pin(async move {
            let conn = Driver::connect(self, options).await?;
            let dyn_conn: DynConnection = Box::new(conn);
            Ok(dyn_conn)
        })
    }
}

impl<C: DriverConnection> DynDriverConnection for C {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(DriverConnection::execute(self, sql, params))
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>> {
        Box::pin(DriverConnection::query(self, sql, params))
    }

    fn query_each<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
        on_row: &'a mut (dyn FnMut(Row) -> ControlFlow<()> + Send),
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(async move { DriverConnection::query_each(self, sql, params, on_row).await })
    }

    fn query_each_ref<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
        on_row: &'a mut OnRowRef<'a>,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(async move {
            struct DynVisitor<'b, 'c> {
                on_row: &'b mut OnRowRef<'c>,
            }
            impl<'b, 'c> RowVisitor for DynVisitor<'b, 'c> {
                fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, Error> {
                    (self.on_row)(row)
                }
            }
            let mut visitor = DynVisitor { on_row };
            DriverConnection::query_each_ref(self, sql, params, &mut visitor).await
        })
    }

    fn begin_transaction<'a>(
        &'a mut self,
    ) -> BoxFuture<'a, Result<Box<dyn DriverTransaction + 'a>, Error>> {
        Box::pin(DriverConnection::begin_transaction(self))
    }

    fn ping<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(DriverConnection::ping(self))
    }

    fn close<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>> {
        Box::pin(DriverConnection::close(self))
    }

    fn as_any(&self) -> &dyn Any {
        DriverConnection::as_any(self)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        DriverConnection::as_any_mut(self)
    }
}

impl DriverConnection for DynConnection {
    async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        self.as_mut().execute(sql, params).await
    }

    async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        self.as_mut().query(sql, params).await
    }

    async fn query_each<F: FnMut(Row) -> ControlFlow<()> + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        mut on_row: F,
    ) -> Result<u64, Error> {
        let mut dyn_on_row = |row: Row| on_row(row);
        self.as_mut().query_each(sql, params, &mut dyn_on_row).await
    }

    async fn query_each_ref<V: RowVisitor + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> Result<u64, Error> {
        let mut on_row =
            |row: &dyn RowRef| -> Result<ControlFlow<()>, Error> { visitor.visit(&row) };
        self.as_mut().query_each_ref(sql, params, &mut on_row).await
    }

    async fn begin_transaction(&mut self) -> Result<Box<dyn DriverTransaction + '_>, Error> {
        self.as_mut().begin_transaction().await
    }

    async fn ping(&mut self) -> Result<(), Error> {
        self.as_mut().ping().await
    }

    async fn close(&mut self) -> Result<(), Error> {
        self.as_mut().close().await
    }

    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self.as_mut().as_any_mut()
    }
}
