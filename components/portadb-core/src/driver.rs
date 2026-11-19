use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::ops::ControlFlow;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

use crate::capability::Capabilities;
use crate::database::Database;
use crate::dyn_driver::DynDriver;
use crate::error::Error;
use crate::options::ConnectOptions;
use crate::params::NamedParams;
use crate::row::Row;
use crate::row_ref::RowVisitor;

/// Pinned boxed future returned by asynchronous driver methods.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// The SPI (Service Provider Interface) contract implemented by database access engines.
///
/// An engine (such as `rusqlite`, `sqlx`, `tokio-postgres`, `diesel`) is responsible
/// for establishing low-level connections to supported databases.
pub trait Driver: Send + Sync + 'static {
    /// Active connection type managed by this engine driver.
    type Connection: DriverConnection;

    /// Canonical name of this engine library (e.g. "rusqlite", "sqlx").
    fn name(&self) -> &'static str;

    /// Checks if this engine supports the specified RDBMS.
    fn supports_database(&self, database: Database) -> bool;

    /// Returns the capability matrix supported by this driver engine.
    fn capabilities(&self) -> Capabilities {
        Capabilities::relational_default()
    }

    /// Connects to the database and returns an active [`DriverConnection`].
    fn connect(
        &self,
        options: &ConnectOptions,
    ) -> impl Future<Output = Result<Self::Connection, Error>> + Send;
}

/// Active connection handle managed by a specific engine driver.
pub trait DriverConnection: Send + Sync + 'static {
    /// Executes a command returning the count of affected rows.
    fn execute(
        &mut self,
        sql: &str,
        params: &NamedParams,
    ) -> impl Future<Output = Result<u64, Error>> + Send;

    /// Queries the database returning rows.
    fn query(
        &mut self,
        sql: &str,
        params: &NamedParams,
    ) -> impl Future<Output = Result<Vec<Row>, Error>> + Send;

    /// Hands each row to `on_row` as soon as the engine yields it, without
    /// collecting the result; `on_row` returns `ControlFlow::Break(())` to stop
    /// early. Returns how many rows were handed over.
    fn query_each<F: FnMut(Row) -> ControlFlow<()> + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        mut on_row: F,
    ) -> impl Future<Output = Result<u64, Error>> + Send {
        async move {
            let rows = self.query(sql, params).await?;
            let mut count = 0u64;
            for row in rows {
                count += 1;
                if on_row(row).is_break() {
                    break;
                }
            }
            Ok(count)
        }
    }

    /// Hands each row as a borrowed [`RowRef`](crate::RowRef) to `visitor` as soon as the engine yields it,
    /// without collecting or allocating intermediate rows. Returns how many rows were handed over.
    fn query_each_ref<V: RowVisitor + Send>(
        &mut self,
        sql: &str,
        params: &NamedParams,
        visitor: &mut V,
    ) -> impl Future<Output = Result<u64, Error>> + Send {
        async move {
            let mut callback_err: Option<Error> = None;
            let count = {
                let err_ref = &mut callback_err;
                let on_row = move |row: Row| match visitor.visit(&row) {
                    Ok(cf) => cf,
                    Err(e) => {
                        *err_ref = Some(e);
                        ControlFlow::Break(())
                    }
                };
                self.query_each(sql, params, on_row).await?
            };
            if let Some(err) = callback_err {
                return Err(err);
            }
            Ok(count)
        }
    }

    /// Begins a database transaction.
    fn begin_transaction(
        &mut self,
    ) -> impl Future<Output = Result<Box<dyn DriverTransaction + '_>, Error>> + Send;

    /// Checks health of the connection.
    fn ping(&mut self) -> impl Future<Output = Result<(), Error>> + Send {
        async move { Ok(()) }
    }

    /// Closes the connection.
    fn close(&mut self) -> impl Future<Output = Result<(), Error>> + Send;

    /// Downcast reference for raw driver escape hatch.
    fn as_any(&self) -> &dyn Any;

    /// Downcast mutable reference for raw driver escape hatch.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Active transaction handle managed by a specific engine driver.
pub trait DriverTransaction: Send + Sync {
    /// Executes a command within a transaction.
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>>;

    /// Queries within a transaction.
    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>>;

    /// Commits the transaction.
    fn commit<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>>;

    /// Rolls back the transaction.
    fn rollback<'a>(&'a mut self) -> BoxFuture<'a, Result<(), Error>>;
}

/// Dynamic, thread-safe registry of database drivers.
///
/// Implements the Open-Closed Principle (OCP): new database engines can be registered
/// at runtime without modifying or recompiling core engine files.
#[derive(Default)]
pub struct DriverRegistry {
    drivers: RwLock<HashMap<String, Arc<dyn DynDriver>>>,
}

impl DriverRegistry {
    /// Creates a new empty driver registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            drivers: RwLock::new(HashMap::new()),
        }
    }

    /// Registers a driver instance in this registry.
    pub fn register(&self, driver: Arc<dyn DynDriver>) {
        let name = driver.name().to_ascii_lowercase();
        let mut map = self.drivers.write().expect("lock driver registry");
        map.insert(name, driver);
    }

    /// Looks up a driver by its canonical name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<Arc<dyn DynDriver>> {
        let map = self.drivers.read().expect("lock driver registry");
        map.get(&name.to_ascii_lowercase()).cloned()
    }

    /// Resolves the most appropriate driver for a target database and optional engine preference.
    pub fn find_for(
        &self,
        database: Database,
        engine_preference: Option<&str>,
    ) -> Option<Arc<dyn DynDriver>> {
        let map = self.drivers.read().expect("lock driver registry");

        // 1. If explicit engine name requested, find matching driver that supports the database
        if let Some(pref) = engine_preference {
            let key = pref.to_ascii_lowercase();
            if let Some(driver) = map.get(&key)
                && driver.supports_database(database)
            {
                return Some(Arc::clone(driver));
            }
            return None;
        }

        // 2. Deterministic preference order when multiple engines support the same database
        let preferred_order = match database {
            Database::Sqlite => &["rusqlite", "sqlx", "memory"][..],
            Database::Postgres => &["sqlx", "tokio-postgres"][..],
            Database::Redis => &["redis"][..],
            _ => &[],
        };

        for &preferred_name in preferred_order {
            if let Some(driver) = map.get(preferred_name)
                && driver.supports_database(database)
            {
                return Some(Arc::clone(driver));
            }
        }

        // 3. Fallback: find any registered driver supporting the target database
        for driver in map.values() {
            if driver.supports_database(database) {
                return Some(Arc::clone(driver));
            }
        }

        None
    }
}
