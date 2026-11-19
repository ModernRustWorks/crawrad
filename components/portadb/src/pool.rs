use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{Mutex, OwnedSemaphorePermit, Semaphore};

use crate::connection::Connection;
use crate::dyn_driver::DynDriver;
use crate::error::Error;
use crate::executor::{BoxFuture, Executor};
use crate::options::ConnectOptions;
use crate::params::NamedParams;
use crate::row::Row;

/// Configuration options for the connection pool.
#[derive(Debug, Clone)]
pub struct PoolOptions {
    /// Maximum number of concurrent connections allowed.
    pub max_connections: usize,
    /// Minimum number of idle connections to maintain.
    pub min_idle: usize,
    /// Maximum duration to wait when attempting to acquire a connection.
    pub acquire_timeout: Duration,
    /// Maximum duration an idle connection can remain unused before being retired.
    pub idle_timeout: Duration,
    /// Maximum lifetime of a connection regardless of activity.
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_idle: 1,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
        }
    }
}

impl PoolOptions {
    /// Creates a new `PoolOptions` with default parameters.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of concurrent connections.
    #[must_use]
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max.max(1);
        self
    }

    /// Sets the timeout for acquiring a connection from the pool.
    #[must_use]
    pub fn with_acquire_timeout(mut self, timeout: Duration) -> Self {
        self.acquire_timeout = timeout;
        self
    }

    /// Sets the idle timeout before an unused connection is recycled.
    #[must_use]
    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }
}

/// Real-time status metrics of the connection pool.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolStatus {
    /// Number of connections currently idle and available in the pool.
    pub idle: usize,
    /// Number of connections currently checked out and in active use.
    pub active: usize,
    /// Maximum allowed concurrent connections.
    pub max: usize,
}

struct IdleEntry {
    conn: Connection,
    last_used: Instant,
    created_at: Instant,
}

struct PoolInner {
    driver: Arc<dyn DynDriver>,
    connect_options: ConnectOptions,
    pool_options: PoolOptions,
    semaphore: Arc<Semaphore>,
    idle_queue: Mutex<VecDeque<IdleEntry>>,
}

/// A high-performance, asynchronous, non-blocking connection pool.
///
/// Thread-safe (`Send + Sync + Clone`), managing a pool of reusable connections
/// with concurrency limits governed by an internal asynchronous semaphore.
#[derive(Clone)]
pub struct Pool {
    inner: Arc<PoolInner>,
}

impl Pool {
    /// Instantiates a new connection pool for the given driver and options.
    pub async fn new(
        driver: Arc<dyn DynDriver>,
        connect_options: ConnectOptions,
        pool_options: PoolOptions,
    ) -> Result<Self, Error> {
        let max = pool_options.max_connections.max(1);
        let semaphore = Arc::new(Semaphore::new(max));
        let idle_queue = Mutex::new(VecDeque::with_capacity(max));

        let inner = Arc::new(PoolInner {
            driver,
            connect_options,
            pool_options,
            semaphore,
            idle_queue,
        });

        Ok(Self { inner })
    }

    /// Acquires an active connection from the pool.
    ///
    /// If an idle connection is available in the queue, it is reused.
    /// Otherwise, if the pool has not reached `max_connections`, a new connection is established.
    /// If `max_connections` is exhausted, this method asynchronously waits until a connection is returned
    /// or `acquire_timeout` expires.
    pub async fn acquire(&self) -> Result<PooledConnection, Error> {
        let timeout = self.inner.pool_options.acquire_timeout;

        // 1. Acquire permit with timeout
        let permit = tokio::time::timeout(timeout, self.inner.semaphore.clone().acquire_owned())
            .await
            .map_err(|_| {
                Error::ConnectionFailed(format!(
                    "timed out after {timeout:?} waiting for available connection permit from pool"
                ))
            })?
            .map_err(|_| Error::ConnectionLost("pool semaphore closed".to_string()))?;

        // 2. Try to pop an idle connection from the queue
        let mut queue = self.inner.idle_queue.lock().await;
        while let Some(entry) = queue.pop_front() {
            let now = Instant::now();
            let is_idle_expired =
                now.duration_since(entry.last_used) > self.inner.pool_options.idle_timeout;
            let is_lifetime_expired = self
                .inner
                .pool_options
                .max_lifetime
                .is_some_and(|max| now.duration_since(entry.created_at) > max);

            if is_idle_expired || is_lifetime_expired {
                // Connection expired; discard and continue checking queue
                continue;
            }

            return Ok(PooledConnection {
                conn: Some(entry.conn),
                pool: Arc::clone(&self.inner),
                created_at: entry.created_at,
                _permit: permit,
                is_dirty: false,
            });
        }
        drop(queue);

        // 3. No idle connection available; establish a fresh one
        let conn =
            Connection::open_arc(Arc::clone(&self.inner.driver), &self.inner.connect_options)
                .await?;

        Ok(PooledConnection {
            conn: Some(conn),
            pool: Arc::clone(&self.inner),
            created_at: Instant::now(),
            _permit: permit,
            is_dirty: false,
        })
    }

    /// Returns real-time metrics of the connection pool.
    pub async fn status(&self) -> PoolStatus {
        let idle = self.inner.idle_queue.lock().await.len();
        let available_permits = self.inner.semaphore.available_permits();
        let max = self.inner.pool_options.max_connections;
        let active = max.saturating_sub(available_permits);

        PoolStatus { idle, active, max }
    }

    /// Returns the pool configuration options.
    #[must_use]
    pub fn options(&self) -> &PoolOptions {
        &self.inner.pool_options
    }
}

/// A smart RAII wrapper around a pooled connection.
///
/// Implements `Deref` and `DerefMut` to [`Connection`], and implements [`Executor`].
/// When dropped, the underlying connection is automatically returned to the pool's idle queue
/// unless explicitly discarded via [`PooledConnection::discard`].
pub struct PooledConnection {
    conn: Option<Connection>,
    pool: Arc<PoolInner>,
    created_at: Instant,
    _permit: OwnedSemaphorePermit,
    is_dirty: bool,
}

impl PooledConnection {
    /// Marks this connection as broken or dirty so it will not be returned to the pool upon drop.
    pub fn discard(mut self) {
        self.is_dirty = true;
        self.conn.take();
    }
}

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().expect("pooled connection active")
    }
}

impl DerefMut for PooledConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().expect("pooled connection active")
    }
}

impl Executor for PooledConnection {
    fn execute<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<u64, Error>> {
        Box::pin(self.deref_mut().execute(sql, params))
    }

    fn query<'a>(
        &'a mut self,
        sql: &'a str,
        params: &'a NamedParams,
    ) -> BoxFuture<'a, Result<Vec<Row>, Error>> {
        Box::pin(self.deref_mut().query(sql, params))
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if self.is_dirty {
            return;
        }

        if let Some(conn) = self.conn.take() {
            let pool = Arc::clone(&self.pool);
            let created_at = self.created_at;
            let entry = IdleEntry {
                conn,
                last_used: Instant::now(),
                created_at,
            };

            // Non-blocking try_lock: if available return immediately; otherwise spawn recycling task
            if let Ok(mut queue) = pool.idle_queue.try_lock() {
                queue.push_back(entry);
            } else {
                tokio::spawn(async move {
                    let mut queue = pool.idle_queue.lock().await;
                    queue.push_back(entry);
                });
            }
        }
    }
}
