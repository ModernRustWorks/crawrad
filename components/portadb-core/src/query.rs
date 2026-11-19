use crate::dialect::{Dialect, SqliteDialect};
use crate::error::Error;
use crate::params::NamedParams;

pub use crate::dialect::PreparedQuery;
pub use crate::value::Value;

/// Rewrites a SQL string with `:name` parameters into positional `?` placeholders (SQLite style).
///
/// For multi-database dialect support, prefer [`Dialect::prepare_query`].
pub fn prepare_query(sql: &str, params: &NamedParams) -> Result<PreparedQuery, Error> {
    SqliteDialect.prepare_query(sql, params)
}

#[cfg(test)]
#[path = "../tests/query/mod.rs"]
mod test;
