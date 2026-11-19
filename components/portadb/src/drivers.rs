//! # Database Engine Drivers
//!
//! The providers, each its own crate, re-exported under the names they always had here.

#[cfg(feature = "memory")]
pub use portadb_memory as memory;
#[cfg(feature = "memory")]
pub use portadb_memory::{Connection as MemoryConnection, Driver as MemoryDriver};

#[cfg(feature = "rusqlite")]
pub use portadb_rusqlite as rusqlite;
#[cfg(feature = "rusqlite")]
pub use portadb_rusqlite::{Connection as RusqliteConnection, Driver as RusqliteDriver};

#[cfg(feature = "sqlx-sqlite")]
pub use portadb_sqlite as sqlx_sqlite;
#[cfg(feature = "sqlx-sqlite")]
pub use portadb_sqlite::{
    Connection as SqlxConnection, Connection as SqlxSqliteConnection, Driver as SqlxDriver,
    Driver as SqlxSqliteDriver,
};

#[cfg(feature = "sqlx-postgres")]
pub use portadb_postgres as sqlx_postgres;
#[cfg(feature = "sqlx-postgres")]
pub use portadb_postgres::{Connection as SqlxPostgresConnection, Driver as SqlxPostgresDriver};

pub use portadb_redis as redis;
pub use portadb_redis::{Connection as RedisConnection, Driver as RedisDriver};

// Compatibility module aliases
#[cfg(feature = "rusqlite")]
pub use portadb_rusqlite as rusqlite_driver;

#[cfg(feature = "sqlx-sqlite")]
pub use portadb_sqlite as sqlx_driver;
