//! # Connection
//!
//! A decoupled, driver-agnostic database connection engine abstraction for Rust.
//!
//! `portadb` allows applications to program against unified interfaces ([`Engine`], [`Connection`], and [`Executor`])
//! for database connectivity, query execution, transactions, and reactive UI data binding
//! without hard-coupling business logic to a specific driver crate (such as `rusqlite`, `sqlx`, or `tokio-postgres`).
//!
//! ## Core Architecture
//!
//! - **[`Engine`]**: The central façade representing the database access source, holding configuration,
//!   driver resolution, and dialect handling.
//! - **[`Connection`]** (alias `DbConnection`): An active connection handle for executing queries and commands.
//! - **[`Transaction`]** (alias `DbTransaction`): An atomic transaction scope with RAII automatic rollback on drop.
//! - **[`Executor`]**: Unified trait implemented by both `Connection` and `Transaction` for writing generic repositories.
//! - **[`Dialect`]**: Multi-RDBMS SQL syntax handling and parameter translation (`?`, `$1`, `@p1`).
//! - **[`Driver`] & [`DriverRegistry`]**: Open SPI for pluggable driver engines without modifying core code.
//! - **[`TableView`]** (alias `DataSource`): Observable tabular view for reactive UI controls and desktop data binding.
//! - **[`Dataset`]**: Relational data abstraction managing record editing lifecycle (`Browse`/`Edit`/`Insert`).
//!
//! ## Quickstart
//!
//! ```rust,no_run
//! use portadb::{ConnectOptions, Engine, params};
//!
//! # async fn run() -> Result<(), portadb::Error> {
//! let options = ConnectOptions::new("sqlite::memory:");
//! let engine = Engine::connect(&options).await?;
//!
//! engine.execute(
//!     "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
//!     &params![],
//! ).await?;
//!
//! engine.execute(
//!     "INSERT INTO users (id, name) VALUES (:id, :name)",
//!     &params![("id", 1i64), ("name", "Alice")],
//! ).await?;
//!
//! let rows = engine.query(
//!     "SELECT id, name FROM users WHERE id = :id",
//!     &params![("id", 1i64)],
//! ).await?;
//!
//! assert_eq!(rows.len(), 1);
//! let name: String = rows[0].get("name")?;
//! println!("User name: {name}");
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

pub use portadb_core::{
    capability, database, dialect, driver, dyn_driver, error, executor, options, params, query,
    row, row_ref, value,
};

pub mod builder;
pub mod drivers;
mod registry;
pub use registry::{register_driver, registry};
pub mod bulk;
pub mod cache;
pub mod connection;
pub mod engine;
pub mod migration;
#[cfg(feature = "pool")]
pub mod pool;
pub mod schema;
pub mod telemetry;
pub mod transaction;
pub mod view;

// Primary Public API
pub use builder::{Query, SelectBuilder};
pub use bulk::BulkLoader;
pub use cache::{CacheProvider, MemoryCache};
pub use capability::{Capabilities, Feature};
pub use connection::Connection;
pub use database::Database;
pub use dialect::{
    Dialect, MySqlDialect, PostgresDialect, PreparedQuery, SqlServerDialect, SqliteDialect,
    dialect_for,
};
pub use driver::{Driver, DriverConnection, DriverRegistry, DriverTransaction};
pub use dyn_driver::{DynConnection, DynDriver, DynDriverConnection};
pub use engine::Engine;
pub use error::Error;
pub use executor::{BoxFuture, Executor};
pub use migration::{Migration, Migrator};
pub use options::ConnectOptions;
pub use params::NamedParams;
#[cfg(feature = "pool")]
pub use pool::{Pool, PoolOptions, PoolStatus, PooledConnection};
pub use row::Row;
pub use row_ref::{FromRow, FromValueRef, RowRef, RowVisitor, ValueRef, decode};
pub use schema::SchemaInspector;
pub use telemetry::{QueryMetrics, QueryObserver, TelemetryManager};
pub use transaction::Transaction;
pub use value::{FromValue, Value};
pub use view::{
    ChangeKind, ChangeSignal, ColumnDef, DataCursor, DataSource, DataType, Dataset, DatasetEvent,
    DatasetSource, DatasetState, FieldDef, FieldType, SchemaDescriptor, TableEvent, TableSchema,
    TableSignal, TableState, TableView,
};

// Driver exports
#[cfg(feature = "memory")]
pub use drivers::MemoryDriver;
pub use drivers::RedisDriver;
#[cfg(feature = "rusqlite")]
pub use drivers::RusqliteDriver;
#[cfg(feature = "sqlx-sqlite")]
pub use drivers::SqlxDriver;
#[cfg(feature = "sqlx-postgres")]
pub use drivers::SqlxPostgresDriver;

/// Connects to a database using the provided configuration options.
///
/// Which engine executes the connection is determined strictly by configuration
/// ([`ConnectOptions::engine`] or connection URL query parameter `?engine=`), not in application logic.
pub async fn connect(options: &ConnectOptions) -> Result<Connection, Error> {
    Connection::connect(options).await
}

/// Creates a new [`Engine`] configured with the provided options.
pub async fn create_engine(options: &ConnectOptions) -> Result<Engine, Error> {
    Engine::connect(options).await
}
