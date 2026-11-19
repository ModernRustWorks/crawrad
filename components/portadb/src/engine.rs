use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::builder::SelectBuilder;
use crate::cache::{CacheProvider, cache_key};
use crate::connection::Connection;
use crate::database::Database;
use crate::dialect::{Dialect, dialect_for};
use crate::driver::Driver;
use crate::dyn_driver::DynDriver;
use crate::error::Error;
use crate::options::ConnectOptions;
use crate::params::NamedParams;
#[cfg(feature = "pool")]
use crate::pool::{Pool, PoolOptions, PoolStatus, PooledConnection};
use crate::row::Row;
use crate::schema::SchemaInspector;
use crate::telemetry::{QueryMetrics, QueryObserver, TelemetryManager};
use crate::view::TableSchema;

struct EngineInner {
    options: ConnectOptions,
    driver: Arc<dyn DynDriver>,
    dialect: &'static (dyn Dialect + Send + Sync),
    database: Database,
    telemetry: RwLock<TelemetryManager>,
    cache: RwLock<Option<Arc<dyn CacheProvider>>>,
    #[cfg(feature = "pool")]
    pool: Option<Pool>,
}

/// The central Database Engine abstraction.
///
/// Coordinates connection configuration, driver resolution, connection pooling,
/// query caching, telemetry observability, and database dialect rules.
///
/// An `Engine` is thread-safe (`Send + Sync + Clone`) and represents a long-lived database access source.
#[derive(Clone)]
pub struct Engine {
    inner: Arc<EngineInner>,
}

impl Engine {
    /// Creates an `Engine` by resolving the appropriate driver from the global [`DriverRegistry`].
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

    /// Creates an `Engine` using an explicit driver instance.
    pub async fn open(driver: impl Driver, options: &ConnectOptions) -> Result<Self, Error> {
        Self::open_arc(Arc::new(driver), options).await
    }

    /// Creates an `Engine` from an `Arc<dyn DynDriver>`.
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
        let dialect = dialect_for(database);

        let mut telemetry = TelemetryManager::new();
        telemetry.set_slow_query_threshold(Duration::from_millis(100));

        #[cfg(feature = "pool")]
        let pool = {
            let max_conn = options.max_connections.map(|n| n as usize).unwrap_or(10);
            let pool_options = PoolOptions::new().with_max_connections(max_conn);
            Pool::new(Arc::clone(&driver), options.clone(), pool_options)
                .await
                .ok()
        };

        Ok(Self {
            inner: Arc::new(EngineInner {
                options: options.clone(),
                driver,
                dialect,
                database,
                telemetry: RwLock::new(telemetry),
                cache: RwLock::new(None),
                #[cfg(feature = "pool")]
                pool,
            }),
        })
    }

    /// Acquires an active connection from this engine (pooled if connection pooling is active).
    #[cfg(feature = "pool")]
    pub async fn acquire(&self) -> Result<PooledConnection, Error> {
        self.acquire_pooled().await
    }

    /// Acquires an active connection from this engine.
    #[cfg(not(feature = "pool"))]
    pub async fn acquire(&self) -> Result<Connection, Error> {
        Connection::open_arc(Arc::clone(&self.inner.driver), &self.inner.options).await
    }

    /// Explicitly opens a new, dedicated, unpooled connection directly from the driving engine.
    pub async fn open_connection(&self) -> Result<Connection, Error> {
        Connection::open_arc(Arc::clone(&self.inner.driver), &self.inner.options).await
    }

    /// Acquires a connection from the internal connection pool.
    #[cfg(feature = "pool")]
    pub async fn acquire_pooled(&self) -> Result<PooledConnection, Error> {
        if let Some(ref pool) = self.inner.pool {
            pool.acquire().await
        } else {
            Err(Error::ConfigurationError(
                "connection pooling is not active on this engine".to_string(),
            ))
        }
    }

    /// Returns a reference to the internal [`Pool`], if active.
    #[cfg(feature = "pool")]
    #[must_use]
    pub fn pool(&self) -> Option<&Pool> {
        self.inner.pool.as_ref()
    }

    /// Returns real-time metrics of the connection pool.
    #[cfg(feature = "pool")]
    pub async fn pool_status(&self) -> Option<PoolStatus> {
        if let Some(ref pool) = self.inner.pool {
            Some(pool.status().await)
        } else {
            None
        }
    }

    /// Executes a SQL statement directly on this engine, measuring telemetry.
    pub async fn execute(&self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        let start = Instant::now();
        let result = {
            #[cfg(feature = "pool")]
            if let Some(ref pool) = self.inner.pool {
                let mut conn = pool.acquire().await?;
                conn.execute(sql, params).await
            } else {
                let mut conn = self.acquire().await?;
                conn.execute(sql, params).await
            }
            #[cfg(not(feature = "pool"))]
            {
                let mut conn = self.acquire().await?;
                conn.execute(sql, params).await
            }
        };

        let duration = start.elapsed();
        self.record_telemetry(
            sql,
            duration,
            result.as_ref().map(|&n| n as usize),
            result.as_ref().err(),
        );
        result
    }

    /// Queries the database directly using an ephemeral connection, measuring telemetry.
    pub async fn query(&self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let start = Instant::now();
        let result = {
            #[cfg(feature = "pool")]
            if let Some(ref pool) = self.inner.pool {
                let mut conn = pool.acquire().await?;
                conn.query(sql, params).await
            } else {
                let mut conn = self.acquire().await?;
                conn.query(sql, params).await
            }
            #[cfg(not(feature = "pool"))]
            {
                let mut conn = self.acquire().await?;
                conn.query(sql, params).await
            }
        };

        let duration = start.elapsed();
        self.record_telemetry(
            sql,
            duration,
            result.as_ref().map(|r| r.len()),
            result.as_ref().err(),
        );
        result
    }

    /// Queries the database with multi-tier result caching.
    ///
    /// If an active [`CacheProvider`] contains fresh results for this query and parameters,
    /// they are returned as an `Arc<Vec<Row>>` without database I/O.
    /// Otherwise, the database is queried, cached, and returned.
    pub async fn query_cached(
        &self,
        sql: &str,
        params: &NamedParams,
        ttl: Duration,
    ) -> Result<Arc<Vec<Row>>, Error> {
        let key = cache_key(sql, params);

        // 1. Check cache
        if let Ok(guard) = self.inner.cache.read()
            && let Some(ref cache) = *guard
            && let Some(rows) = cache.get(&key)
        {
            return Ok(rows);
        }

        // 2. Query database
        let rows = self.query(sql, params).await?;
        let shared = Arc::new(rows);

        // 3. Store in cache
        if let Ok(guard) = self.inner.cache.read()
            && let Some(ref cache) = *guard
        {
            cache.set(&key, Arc::clone(&shared), ttl);
        }

        Ok(shared)
    }

    /// Executes a dynamic query built with the fluent [`SelectBuilder`].
    pub async fn execute_query(&self, builder: &SelectBuilder) -> Result<Vec<Row>, Error> {
        let (sql, params) = builder.build(self.dialect());
        self.query(&sql, &params).await
    }

    /// Sets the query result cache provider (e.g. [`MemoryCache`](crate::cache::MemoryCache)).
    pub fn set_cache(&self, cache: impl CacheProvider) {
        if let Ok(mut guard) = self.inner.cache.write() {
            *guard = Some(Arc::new(cache));
        }
    }

    /// Registers a query observer for telemetry, metric tracking, or profiling.
    pub fn add_observer(&self, observer: impl QueryObserver) {
        if let Ok(mut guard) = self.inner.telemetry.write() {
            guard.add_observer(observer);
        }
    }

    /// Configures the threshold duration above which queries trigger a slow query alert.
    pub fn set_slow_query_threshold(&self, threshold: Duration) {
        if let Ok(mut guard) = self.inner.telemetry.write() {
            guard.set_slow_query_threshold(threshold);
        }
    }

    fn record_telemetry(
        &self,
        sql: &str,
        duration: Duration,
        count: Result<usize, &Error>,
        err: Option<&Error>,
    ) {
        if let Ok(guard) = self.inner.telemetry.read()
            && guard.is_active()
        {
            let metrics = QueryMetrics {
                sql: sql.to_string(),
                duration,
                rows_count: count.unwrap_or(0),
                success: err.is_none(),
                error: err.map(|e| e.to_string()),
            };
            guard.record_query(&metrics);
        }
    }

    /// Discovers all user tables in the target database.
    pub async fn inspect_tables(&self) -> Result<Vec<String>, Error> {
        SchemaInspector::inspect_tables(self).await
    }

    /// Discovers the tabular schema definition of the given table.
    pub async fn inspect_schema(&self, table: &str) -> Result<TableSchema, Error> {
        SchemaInspector::inspect_table_schema(self, table).await
    }

    /// Checks the health and connectivity to the database.
    pub async fn ping(&self) -> Result<(), Error> {
        let mut conn = self.acquire().await?;
        conn.ping().await
    }

    /// Returns the target database type (e.g. SQLite, PostgreSQL).
    #[must_use]
    pub fn database(&self) -> Database {
        self.inner.database
    }

    /// Returns the canonical name of the active driving engine.
    #[must_use]
    pub fn driver_name(&self) -> &'static str {
        self.inner.driver.name()
    }

    /// Returns the SQL dialect associated with this engine's target database.
    #[must_use]
    pub fn dialect(&self) -> &'static (dyn Dialect + Send + Sync) {
        self.inner.dialect
    }

    /// Returns the active connection options.
    #[must_use]
    pub fn options(&self) -> &ConnectOptions {
        &self.inner.options
    }

    /// Returns the capabilities matrix supported by this engine.
    #[must_use]
    pub fn capabilities(&self) -> crate::capability::Capabilities {
        self.inner.driver.capabilities()
    }

    /// Checks whether this engine supports a specific capability feature.
    #[must_use]
    pub fn supports(&self, feature: crate::capability::Feature) -> bool {
        self.capabilities().supports(feature)
    }

    /// Ensures that this engine supports the specified feature, returning [`Error::UnsupportedFeature`] otherwise.
    pub fn require_feature(&self, feature: crate::capability::Feature) -> Result<(), Error> {
        self.capabilities().require(feature, self.driver_name())
    }
}
