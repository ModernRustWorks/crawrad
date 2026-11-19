use std::ops::ControlFlow;
use std::sync::Arc;

use crate::capability::{Capabilities, Feature};
use crate::database::Database;
use crate::dialect::{Dialect, dialect_for};
use crate::driver::{Driver, DriverConnection};
use crate::dyn_driver::{DynConnection, DynDriver};
use crate::error::Error;
use crate::executor::{BoxFuture, Executor};
use crate::options::ConnectOptions;
use crate::params::NamedParams;
use crate::row::Row;
use crate::row_ref::{FromRow, RowRef, RowVisitor};
use crate::transaction::Transaction;

/// An active database connection handle.
///
/// Implements unified execution across all supported drivers and databases.
/// Can be acquired from an [`Engine`](crate::Engine) or opened directly.
pub struct Connection<C: DriverConnection = DynConnection> {
    driver_conn: C,
    database: Database,
    engine_name: &'static str,
    dialect: &'static (dyn Dialect + Send + Sync),
    capabilities: Capabilities,
}

impl Connection {
    /// Instantiates a dynamic connection using an explicit driver instance.
    pub async fn open(driver: impl Driver, options: &ConnectOptions) -> Result<Self, Error> {
        Self::open_arc(Arc::new(driver), options).await
    }

    /// Instantiates a dynamic connection using an `Arc<dyn DynDriver>`.
    pub async fn open_arc(
        driver: Arc<dyn DynDriver>,
        options: &ConnectOptions,
    ) -> Result<Self, Error> {
        if !driver.supports_database(options.database) {
            return Err(Error::ConfigurationError(format!(
                "engine '{}' does not support target database '{}'",
                driver.name(),
                options.database
            )));
        }

        let database = options.database;
        let engine_name = driver.name();
        let dialect = dialect_for(database);
        let capabilities = driver.capabilities();
        let driver_conn = driver.connect(options).await?;

        Ok(Self {
            driver_conn,
            database,
            engine_name,
            dialect,
            capabilities,
        })
    }

    /// Connects using the driver resolved from the global [`DriverRegistry`].
    pub async fn connect(options: &ConnectOptions) -> Result<Self, Error> {
        let registry = crate::registry();
        let engine_choice = options.engine.as_deref();

        let driver = registry
            .find_for(options.database, engine_choice)
            .ok_or_else(|| {
                let engine_name = engine_choice.unwrap_or("default");
                Error::ConfigurationError(format!(
                    "no registered driver available for database '{}' with engine '{}'",
                    options.database, engine_name
                ))
            })?;

        Self::open_arc(driver, options).await
    }
}

impl<C: DriverConnection> Connection<C> {
    /// Instantiates a statically typed connection using a driver known at compile time.
    ///
    /// This is the path taken by generated applications where the database engine is known
    /// ahead of time, eliminating dynamic dispatch and allocation overhead.
    pub async fn open_typed<D: Driver<Connection = C>>(
        driver: &D,
        options: &ConnectOptions,
    ) -> Result<Self, Error> {
        if !driver.supports_database(options.database) {
            return Err(Error::ConfigurationError(format!(
                "engine '{}' does not support target database '{}'",
                driver.name(),
                options.database
            )));
        }

        let database = options.database;
        let engine_name = driver.name();
        let dialect = dialect_for(database);
        let capabilities = driver.capabilities();
        let driver_conn = driver.connect(options).await?;

        Ok(Self {
            driver_conn,
            database,
            engine_name,
            dialect,
            capabilities,
        })
    }

    /// Executes a SQL statement with named parameters, returning the count of affected rows.
    pub async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        self.driver_conn.execute(sql, params).await
    }

    /// Queries the database returning a vector of [`Row`]s.
    pub async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        self.driver_conn.query(sql, params).await
    }

    /// Hands each row to `on_row` as soon as the engine yields it, without
    /// collecting the result; `on_row` returns `ControlFlow::Break(())` to stop
    /// early. Returns how many rows were handed over.
    pub async fn query_each(
        &mut self,
        sql: &str,
        params: &NamedParams,
        on_row: impl FnMut(Row) -> ControlFlow<()> + Send,
    ) -> Result<u64, Error> {
        self.driver_conn.query_each(sql, params, on_row).await
    }

    /// Hands each row as a borrowed [`RowRef`] to `visitor` as soon as the engine yields it,
    /// without collecting or allocating intermediate rows. Returns how many rows were handed over.
    pub async fn query_each_ref<V: RowVisitor + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> Result<u64, Error> {
        self.driver_conn.query_each_ref(sql, params, visitor).await
    }

    /// Queries the database returning a vector of decoded items of type `T`.
    ///
    /// If the query yields zero rows, returns `Ok(vec![])` without constructing a plan.
    pub async fn query_as<T: FromRow + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
    ) -> Result<Vec<T>, Error> {
        let mut items = Vec::new();
        self.query_each_as(sql, params, |item| {
            items.push(item);
            ControlFlow::Continue(())
        })
        .await?;
        Ok(items)
    }

    /// Hands each decoded item of type `T` to `on_row` as soon as the engine yields it,
    /// without collecting or allocating intermediate rows.
    /// `on_row` returns `ControlFlow::Break(())` to stop early. Returns how many rows were handed over.
    ///
    /// The decoding plan is built lazily on the first row. If the query yields zero rows,
    /// `T::plan` is never invoked and `Ok(0)` is returned.
    pub async fn query_each_as<T: FromRow>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        mut on_row: impl FnMut(T) -> ControlFlow<()> + Send,
    ) -> Result<u64, Error> {
        struct AsVisitor<'a, T: FromRow, F> {
            plan: Option<T::Plan>,
            on_row: &'a mut F,
        }

        impl<'a, T: FromRow, F: FnMut(T) -> ControlFlow<()>> RowVisitor for AsVisitor<'a, T, F> {
            fn visit<R: RowRef>(&mut self, row: &R) -> Result<ControlFlow<()>, Error> {
                let plan = match &mut self.plan {
                    Some(p) => p,
                    None => {
                        let new_plan = T::plan(row.columns())?;
                        self.plan.insert(new_plan)
                    }
                };
                let item = T::from_row(plan, row)?;
                Ok((self.on_row)(item))
            }
        }

        let mut visitor = AsVisitor {
            plan: None,
            on_row: &mut on_row,
        };
        self.driver_conn
            .query_each_ref(sql, params, &mut visitor)
            .await
    }

    /// Begins a database transaction.
    pub async fn begin(&mut self) -> Result<Transaction<'_>, Error> {
        let tx = self.driver_conn.begin_transaction().await?;
        Ok(Transaction::new(tx))
    }

    /// Begins a database transaction (alias for [`Connection::begin`]).
    pub async fn begin_transaction(&mut self) -> Result<Transaction<'_>, Error> {
        self.begin().await
    }

    /// Verifies the connection is alive and healthy.
    pub async fn ping(&mut self) -> Result<(), Error> {
        self.driver_conn.ping().await
    }

    /// Gracefully closes the connection.
    pub async fn close(mut self) -> Result<(), Error> {
        self.driver_conn.close().await
    }

    /// Returns the target database RDBMS type (e.g. SQLite, PostgreSQL).
    #[must_use]
    pub fn database(&self) -> Database {
        self.database
    }

    /// Returns the name of the underlying engine library (e.g. "rusqlite", "sqlx").
    #[must_use]
    pub fn engine_name(&self) -> &'static str {
        self.engine_name
    }

    /// Returns the SQL dialect for this connection's target database.
    #[must_use]
    pub fn dialect(&self) -> &'static (dyn Dialect + Send + Sync) {
        self.dialect
    }

    /// Returns the capabilities matrix supported by this connection.
    #[must_use]
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    /// Checks whether this connection supports a specific capability feature.
    #[must_use]
    pub fn supports(&self, feature: Feature) -> bool {
        self.capabilities.supports(feature)
    }

    /// Ensures that this connection supports the specified feature, returning [`Error::UnsupportedFeature`] otherwise.
    pub fn require_feature(&self, feature: Feature) -> Result<(), Error> {
        self.capabilities.require(feature, self.engine_name)
    }

    /// Downcast escape hatch: immutably access the underlying concrete driver type.
    pub fn downcast_raw<T: 'static>(&self) -> Result<&T, Error> {
        self.driver_conn
            .as_any()
            .downcast_ref::<T>()
            .ok_or(Error::DowncastFailed)
    }

    /// Downcast escape hatch: mutably access the underlying concrete driver type.
    pub fn downcast_raw_mut<T: 'static>(&mut self) -> Result<&mut T, Error> {
        self.driver_conn
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or(Error::DowncastFailed)
    }
}

impl<C: DriverConnection> Executor for Connection<C> {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(self.driver_conn.execute(sql, params))
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>> {
        Box::pin(self.driver_conn.query(sql, params))
    }
}

#[cfg(test)]
#[path = "../tests/connection/mod.rs"]
mod test;
