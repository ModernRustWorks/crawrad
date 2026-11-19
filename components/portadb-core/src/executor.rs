use std::future::Future;
use std::pin::Pin;

use crate::error::Error;
use crate::params::NamedParams;
use crate::row::Row;

/// Pinned boxed future returned by asynchronous executor methods.
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Unified trait for executing commands and queries against a database.
///
/// Implemented by both [`Connection`](crate::Connection) and [`Transaction`](crate::Transaction).
/// This allows repositories, services, and business rules to accept `&mut impl Executor`
/// and run seamlessly either standalone or within a transactional boundary.
///
/// # Examples
///
/// ```rust,no_run
/// use portadb_core::{Error, Executor, params};
///
/// async fn insert_log(exec: &mut impl Executor, message: &str) -> Result<u64, Error> {
///     exec.execute(
///         "INSERT INTO logs (message) VALUES (:msg)",
///         &params![("msg", message)],
///     ).await
/// }
/// ```
pub trait Executor: Send {
    /// Executes a SQL statement with named parameters, returning the count of affected rows.
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>>;

    /// Queries the database returning a vector of [`Row`]s.
    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>>;
}
