use crate::database::Database;
use std::time::Duration;

/// Configuration options for database connectivity and engine selection.
///
/// Keeps connection parameters decoupled from code logic, allowing
/// the target database and the driving engine to be specified via configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectOptions {
    /// Database connection URL or connection string.
    pub url: String,
    /// Target database management system (e.g. SQLite, PostgreSQL).
    pub database: Database,
    /// Specific access engine requested (e.g. "rusqlite", "sqlx", "tokio-postgres").
    pub engine: Option<String>,
    /// Optional maximum number of connections (for pooled engines).
    pub max_connections: Option<u32>,
    /// Optional timeout for establishing a connection.
    pub connect_timeout: Option<Duration>,
}

impl ConnectOptions {
    /// Creates a new `ConnectOptions` with the specified database URL.
    ///
    /// Automatically infers the target [`Database`] and extracts any `?engine=` parameter from the URL.
    pub fn new(url: impl Into<String>) -> Self {
        let raw_url = url.into();
        let database = Database::from_url(&raw_url).unwrap_or(Database::Sqlite);
        let engine = extract_engine_from_url(&raw_url);

        Self {
            url: raw_url,
            database,
            engine,
            max_connections: None,
            connect_timeout: None,
        }
    }

    /// Explicitly specifies the target database type.
    #[must_use]
    pub fn with_database(mut self, database: Database) -> Self {
        self.database = database;
        self
    }

    /// Explicitly specifies which engine should handle this connection (e.g. "rusqlite", "sqlx").
    #[must_use]
    pub fn with_engine(mut self, engine: impl Into<String>) -> Self {
        self.engine = Some(engine.into());
        self
    }

    /// Sets the maximum connection pool size.
    #[must_use]
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Sets the connection timeout.
    #[must_use]
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }
}

fn extract_engine_from_url(url: &str) -> Option<String> {
    if let Some(pos) = url.find('?') {
        let query = &url[pos + 1..];
        for pair in query.split('&') {
            if let Some((k, v)) = pair.split_once('=')
                && k.eq_ignore_ascii_case("engine")
                && !v.is_empty()
            {
                return Some(v.to_string());
            }
        }
    }
    None
}
