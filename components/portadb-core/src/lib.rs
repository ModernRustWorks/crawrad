//! # portadb-core
//!
//! The types and contracts of PortaDB: values, rows, parameters, dialects, the driver SPI
//! and the [`DriverRegistry`]. It knows no database: the providers (`portadb-sqlite`,
//! `portadb-postgres`, ...) implement [`Driver`], and the engine crate `portadb` puts
//! them together for applications.

#![forbid(unsafe_code)]

pub mod capability;
pub mod database;
pub mod dialect;
pub mod driver;
pub mod dyn_driver;
pub mod error;
pub mod executor;
pub mod options;
pub mod params;
pub mod query;
pub mod row;
pub mod row_ref;
pub mod value;

pub use capability::{Capabilities, Feature};
pub use database::Database;
pub use dialect::{
    Dialect, MySqlDialect, PostgresDialect, PreparedQuery, SqlServerDialect, SqliteDialect,
    dialect_for,
};
pub use driver::{Driver, DriverConnection, DriverRegistry, DriverTransaction};
pub use dyn_driver::{DynConnection, DynDriver, DynDriverConnection};
pub use error::Error;
pub use executor::{BoxFuture, Executor};
pub use options::ConnectOptions;
pub use params::NamedParams;
pub use row::Row;
pub use row_ref::{FromRow, FromValueRef, RowRef, RowVisitor, ValueRef, decode};
pub use value::{FromValue, Value};
