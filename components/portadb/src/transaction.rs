use crate::driver::DriverTransaction;
use crate::error::Error;
use crate::executor::{BoxFuture, Executor};
use crate::params::NamedParams;
use crate::row::Row;

/// Active database transaction wrapping the underlying engine driver transaction.
///
/// Guarantees automatic rollback on [`Drop`] if neither `commit` nor `rollback` was called.
pub struct Transaction<'a> {
    pub(crate) inner: Option<Box<dyn DriverTransaction + 'a>>,
}

impl<'a> Transaction<'a> {
    /// Constructs a new transaction handle.
    #[must_use]
    pub(crate) fn new(driver_tx: Box<dyn DriverTransaction + 'a>) -> Self {
        Self {
            inner: Some(driver_tx),
        }
    }

    /// Executes a SQL command within the transaction.
    pub async fn execute(&mut self, sql: &str, params: &NamedParams) -> Result<u64, Error> {
        let tx = self.inner.as_mut().ok_or(Error::TransactionAlreadyClosed)?;
        tx.execute(sql, params).await
    }

    /// Queries within the transaction.
    pub async fn query(&mut self, sql: &str, params: &NamedParams) -> Result<Vec<Row>, Error> {
        let tx = self.inner.as_mut().ok_or(Error::TransactionAlreadyClosed)?;
        tx.query(sql, params).await
    }

    /// Commits the transaction, persisting all changes.
    pub async fn commit(mut self) -> Result<(), Error> {
        let mut tx = self.inner.take().ok_or(Error::TransactionAlreadyClosed)?;
        tx.commit().await
    }

    /// Explicitly rolls back the transaction, discarding all changes.
    pub async fn rollback(mut self) -> Result<(), Error> {
        let mut tx = self.inner.take().ok_or(Error::TransactionAlreadyClosed)?;
        tx.rollback().await
    }
}

impl Executor for Transaction<'_> {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(async move {
            let tx = self.inner.as_mut().ok_or(Error::TransactionAlreadyClosed)?;
            tx.execute(sql, params).await
        })
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>> {
        Box::pin(async move {
            let tx = self.inner.as_mut().ok_or(Error::TransactionAlreadyClosed)?;
            tx.query(sql, params).await
        })
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        drop(self.inner.take());
    }
}
