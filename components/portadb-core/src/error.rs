use std::fmt;

/// Represents all errors that can occur within the portadb abstraction.
///
/// Every error is strongly typed so callers can branch on specific database
/// failure conditions (e.g., unique constraint violations, connection drops)
/// without parsing error strings or regex matching.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A unique constraint or primary key violation occurred.
    #[error("unique constraint violation: {message}")]
    UniqueViolation {
        /// Optional constraint name if reported by the database engine.
        constraint: Option<String>,
        /// Descriptive message from the engine.
        message: String,
    },

    /// A foreign key constraint violation occurred.
    #[error("foreign key constraint violation: {message}")]
    ForeignKeyViolation {
        /// Optional constraint name if reported by the database engine.
        constraint: Option<String>,
        /// Descriptive message from the engine.
        message: String,
    },

    /// An active database connection was severed or lost during an operation.
    #[error("database connection lost: {0}")]
    ConnectionLost(String),

    /// Failed to establish a new connection to the database.
    #[error("failed to connect to database: {0}")]
    ConnectionFailed(String),

    /// Requested column was not present in the query result set.
    #[error("column '{name}' was not found in result set")]
    ColumnNotFound {
        /// Name of the missing column.
        name: String,
    },

    /// Column was present, but its SQL value was NULL and a non-optional type was requested.
    #[error("column '{column}' is NULL, but a non-optional type was requested")]
    NullValue {
        /// Name of the column with NULL value.
        column: String,
    },

    /// Column value could not be converted to the target Rust type.
    #[error("type mismatch for column '{column}': expected {expected}, found {found}")]
    TypeMismatch {
        /// Name of the column.
        column: String,
        /// Expected Rust type name.
        expected: &'static str,
        /// Description of the actual value found.
        found: String,
    },

    /// An operation was attempted on a transaction that has already committed or rolled back.
    #[error("transaction has already been committed or rolled back")]
    TransactionAlreadyClosed,

    /// Syntax or semantic error in the SQL query execution.
    #[error("query execution error: {0}")]
    QueryError(String),

    /// Configuration error (e.g., invalid connection string, unsupported option).
    #[error("configuration error: {0}")]
    ConfigurationError(String),

    /// The requested operation or syntax is not supported by this backend engine (e.g. JOIN on Redis).
    #[error("feature or command not supported by engine: {0}")]
    UnsupportedFeature(String),

    /// Failed to downcast connection to the requested underlying raw engine type.
    #[error("downcast to raw engine failed: requested type does not match concrete driver")]
    DowncastFailed,

    /// Internal or driver-specific error.
    #[error("underlying engine error: {0}")]
    Engine(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Returns true if this error represents a unique or primary key constraint violation.
    #[must_use]
    pub fn is_unique_violation(&self) -> bool {
        matches!(self, Self::UniqueViolation { .. })
    }

    /// Returns true if this error represents a foreign key constraint violation.
    #[must_use]
    pub fn is_foreign_key_violation(&self) -> bool {
        matches!(self, Self::ForeignKeyViolation { .. })
    }

    /// Returns true if this error represents a lost connection.
    #[must_use]
    pub fn is_connection_lost(&self) -> bool {
        matches!(self, Self::ConnectionLost(_))
    }

    /// Returns true if this error represents an unsupported feature or command.
    #[must_use]
    pub fn is_unsupported(&self) -> bool {
        matches!(self, Self::UnsupportedFeature(_))
    }

    /// Creates an unsupported feature error with the provided message.
    pub fn unsupported(msg: impl fmt::Display) -> Self {
        Self::UnsupportedFeature(msg.to_string())
    }

    /// Creates a generic query error with the provided message.
    pub fn query_error(msg: impl fmt::Display) -> Self {
        Self::QueryError(msg.to_string())
    }

    /// Creates an engine error from any boxed standard error.
    pub fn from_engine<E>(err: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Engine(Box::new(err))
    }
}

#[cfg(test)]
#[path = "../tests/error/mod.rs"]
mod test;
