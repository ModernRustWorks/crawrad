use std::fmt;

/// Supported Relational Database Management Systems (RDBMS).
///
/// This represents the **Database axis** (e.g. SQLite, PostgreSQL, MySQL, SQL Server),
/// which is completely independent from the **Engine axis** (the access library e.g. `sqlx`, `rusqlite`, `tokio-postgres`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Database {
    /// SQLite embedded database engine.
    Sqlite,
    /// PostgreSQL relational database.
    Postgres,
    /// MySQL / MariaDB relational database.
    MySql,
    /// Microsoft SQL Server relational database.
    SqlServer,
    /// Redis in-memory key-value data structure store.
    Redis,
}

impl Database {
    /// Returns the canonical name of the database system.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
            Self::MySql => "mysql",
            Self::SqlServer => "sqlserver",
            Self::Redis => "redis",
        }
    }

    /// Infers the database type from a connection URL scheme or file extension.
    ///
    /// Recognizes SQLite URLs including `:memory:`, standard prefixes (`sqlite:`),
    /// file URIs (`file:`), and common database file extensions (`.db`, `.sqlite`, `.sqlite3`).
    /// In SQLite, `:memory:` connections are private to each connection handle;
    /// to share an in-memory database across connections on purpose, an explicit
    /// named URI such as `file:shared?mode=memory&cache=shared` is used.
    #[must_use]
    pub fn from_url(url: &str) -> Option<Self> {
        let lower = url.to_ascii_lowercase();
        if lower.starts_with("sqlite:")
            || lower.starts_with(":memory:")
            || lower.starts_with("file:")
            || lower.ends_with(".db")
            || lower.ends_with(".sqlite")
            || lower.ends_with(".sqlite3")
        {
            Some(Self::Sqlite)
        } else if lower.starts_with("postgres:") || lower.starts_with("postgresql:") {
            Some(Self::Postgres)
        } else if lower.starts_with("mysql:") || lower.starts_with("mariadb:") {
            Some(Self::MySql)
        } else if lower.starts_with("mssql:") || lower.starts_with("sqlserver:") {
            Some(Self::SqlServer)
        } else if lower.starts_with("redis:") || lower.starts_with("rediss:") {
            Some(Self::Redis)
        } else {
            None
        }
    }
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
#[path = "../tests/database/mod.rs"]
mod test;
