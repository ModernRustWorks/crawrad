use std::fmt;

use crate::error::Error;

/// Specific engine or database capabilities that may or may not be supported by a given backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feature {
    /// ACID multi-statement transactions (`begin`, `commit`, `rollback`).
    Transactions,
    /// Nested transaction savepoints (`SAVEPOINT`, `ROLLBACK TO`).
    Savepoints,
    /// Relational Data Definition Language (`CREATE TABLE`, `ALTER TABLE`, `DROP TABLE`).
    RelationalDdl,
    /// Complex multi-table relational joins (`INNER JOIN`, `LEFT JOIN`).
    Joins,
    /// Standard SQL queries (`SELECT`, `WHERE`, `GROUP BY`, `ORDER BY`).
    SqlQueries,
    /// Native or abstracted key-value commands (`GET`, `SET`, `DEL`, `EXISTS`, `KEYS`).
    KeyValue,
    /// `RETURNING` clause on `INSERT`, `UPDATE`, or `DELETE` statements.
    Returning,
    /// Parameterized prepared statements.
    PreparedStatements,
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transactions => write!(f, "transactions"),
            Self::Savepoints => write!(f, "savepoints"),
            Self::RelationalDdl => write!(f, "relational DDL"),
            Self::Joins => write!(f, "relational joins"),
            Self::SqlQueries => write!(f, "SQL queries"),
            Self::KeyValue => write!(f, "key-value commands"),
            Self::Returning => write!(f, "returning clause"),
            Self::PreparedStatements => write!(f, "prepared statements"),
        }
    }
}

/// Feature capabilities descriptor for a database engine or connection.
///
/// Allows client code, ORM layers, and UI data-binding controls to inspect
/// engine capabilities programmatically before executing commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    /// Whether the engine supports ACID multi-statement transactions.
    pub transactions: bool,
    /// Whether the engine supports nested savepoints.
    pub savepoints: bool,
    /// Whether the engine supports relational DDL (`CREATE TABLE`, etc.).
    pub relational_ddl: bool,
    /// Whether the engine supports relational joins.
    pub joins: bool,
    /// Whether the engine supports standard SQL queries.
    pub sql_queries: bool,
    /// Whether the engine supports key-value operations.
    pub key_value: bool,
    /// Whether the engine supports `RETURNING` clauses.
    pub returning: bool,
    /// Whether the engine supports parameterized prepared statements.
    pub prepared_statements: bool,
}

impl Capabilities {
    /// Default capabilities for standard relational RDBMS (SQLite, PostgreSQL, MySQL, SQL Server).
    #[must_use]
    pub const fn relational_default() -> Self {
        Self {
            transactions: true,
            savepoints: true,
            relational_ddl: true,
            joins: true,
            sql_queries: true,
            key_value: false,
            returning: false,
            prepared_statements: true,
        }
    }

    /// Capabilities for SQLite.
    #[must_use]
    pub const fn sqlite() -> Self {
        Self {
            transactions: true,
            savepoints: true,
            relational_ddl: true,
            joins: true,
            sql_queries: true,
            key_value: false,
            returning: true,
            prepared_statements: true,
        }
    }

    /// Capabilities for PostgreSQL.
    #[must_use]
    pub const fn postgres() -> Self {
        Self {
            transactions: true,
            savepoints: true,
            relational_ddl: true,
            joins: true,
            sql_queries: true,
            key_value: false,
            returning: true,
            prepared_statements: true,
        }
    }

    /// Capabilities for the In-Memory mock driver.
    #[must_use]
    pub const fn memory() -> Self {
        Self {
            transactions: true,
            savepoints: false,
            relational_ddl: true,
            joins: false,
            sql_queries: true,
            key_value: false,
            returning: false,
            prepared_statements: true,
        }
    }

    /// Capabilities for Redis key-value store.
    #[must_use]
    pub const fn redis() -> Self {
        Self {
            transactions: true, // Abstracted via MULTI/EXEC/DISCARD
            savepoints: false,
            relational_ddl: false,
            joins: false,
            sql_queries: true, // Tabular abstraction over keys/values
            key_value: true,
            returning: false,
            prepared_statements: true,
        }
    }

    /// Checks if a specific feature is supported.
    #[must_use]
    pub const fn supports(&self, feature: Feature) -> bool {
        match feature {
            Feature::Transactions => self.transactions,
            Feature::Savepoints => self.savepoints,
            Feature::RelationalDdl => self.relational_ddl,
            Feature::Joins => self.joins,
            Feature::SqlQueries => self.sql_queries,
            Feature::KeyValue => self.key_value,
            Feature::Returning => self.returning,
            Feature::PreparedStatements => self.prepared_statements,
        }
    }

    /// Ensures that a specific feature is supported, returning [`Error::UnsupportedFeature`] otherwise.
    pub fn require(&self, feature: Feature, engine_name: &str) -> Result<(), Error> {
        if self.supports(feature) {
            Ok(())
        } else {
            Err(Error::UnsupportedFeature(format!(
                "engine '{engine_name}' does not support feature '{feature}'"
            )))
        }
    }
}

impl Default for Capabilities {
    fn default() -> Self {
        Self::relational_default()
    }
}
