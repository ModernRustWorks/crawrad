use std::fmt;
use std::sync::Arc;
use std::time::Duration;

/// Telemetry metrics captured during database query or command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryMetrics {
    /// The executed SQL statement or database command.
    pub sql: String,
    /// Duration taken to execute the statement on the database engine.
    pub duration: Duration,
    /// Number of rows affected (for `execute`) or returned (for `query`).
    pub rows_count: usize,
    /// Whether the execution succeeded.
    pub success: bool,
    /// Error message if the execution failed.
    pub error: Option<String>,
}

impl fmt::Display for QueryMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.success { "OK" } else { "ERR" };
        write!(
            f,
            "[{status} in {:.2?}, {} rows] {}",
            self.duration, self.rows_count, self.sql
        )
    }
}

/// Trait for observing query execution metrics, latency, and slow queries.
pub trait QueryObserver: Send + Sync + 'static {
    /// Invoked immediately after a query or command finishes execution.
    fn on_query_executed(&self, metrics: &QueryMetrics);
}

impl<F> QueryObserver for F
where
    F: Fn(&QueryMetrics) + Send + Sync + 'static,
{
    fn on_query_executed(&self, metrics: &QueryMetrics) {
        (self)(metrics);
    }
}

impl<T: QueryObserver> QueryObserver for Arc<T> {
    fn on_query_executed(&self, metrics: &QueryMetrics) {
        (**self).on_query_executed(metrics);
    }
}

/// Telemetry manager holding registered query observers and alerting thresholds.
#[derive(Default, Clone)]
pub struct TelemetryManager {
    observers: Vec<Arc<dyn QueryObserver>>,
    slow_query_threshold: Option<Duration>,
}

impl TelemetryManager {
    /// Constructs a new empty telemetry manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
            slow_query_threshold: None,
        }
    }

    /// Sets the slow query threshold.
    /// Queries taking longer than this threshold will be marked or logged.
    pub fn set_slow_query_threshold(&mut self, threshold: Duration) {
        self.slow_query_threshold = Some(threshold);
    }

    /// Returns the active slow query threshold, if any.
    #[must_use]
    pub fn slow_query_threshold(&self) -> Option<Duration> {
        self.slow_query_threshold
    }

    /// Registers a new query observer.
    pub fn add_observer(&mut self, observer: impl QueryObserver) {
        self.observers.push(Arc::new(observer));
    }

    /// Registers a boxed query observer.
    pub fn add_observer_arc(&mut self, observer: Arc<dyn QueryObserver>) {
        self.observers.push(observer);
    }

    /// Notifies all registered observers that a query was executed.
    pub fn record_query(&self, metrics: &QueryMetrics) {
        for observer in &self.observers {
            observer.on_query_executed(metrics);
        }
    }

    /// Returns true if at least one observer is registered or a slow query threshold is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        !self.observers.is_empty() || self.slow_query_threshold.is_some()
    }
}

impl fmt::Debug for TelemetryManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TelemetryManager")
            .field("observer_count", &self.observers.len())
            .field("slow_query_threshold", &self.slow_query_threshold)
            .finish()
    }
}
