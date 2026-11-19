use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use portadb::{ConnectOptions, Engine, QueryMetrics, QueryObserver, params};

#[tokio::test]
async fn test_telemetry_metrics_and_observer() {
    let options = ConnectOptions::new("sqlite:file:telemetry_test1?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let metrics_log = Arc::new(Mutex::new(Vec::<QueryMetrics>::new()));
    let metrics_clone = Arc::clone(&metrics_log);

    // Register observer closure
    engine.add_observer(move |m: &QueryMetrics| {
        metrics_clone.lock().unwrap().push(m.clone());
    });
    engine.set_slow_query_threshold(Duration::from_millis(50));

    // Execute queries
    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS telemetry_basic (id INT, tag TEXT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute(
            "INSERT INTO telemetry_basic VALUES (:id, :tag);",
            &params![("id", 1i64), ("tag", "perf")],
        )
        .await
        .expect("insert");

    let rows = engine
        .query(
            "SELECT * FROM telemetry_basic WHERE id = :id;",
            &params![("id", 1i64)],
        )
        .await
        .expect("query");
    assert_eq!(rows.len(), 1);

    // Verify metrics recorded
    let logs = metrics_log.lock().unwrap().clone();
    assert_eq!(logs.len(), 3);
    assert!(logs[0].sql.contains("CREATE TABLE"));
    assert!(logs[0].success);
    assert!(logs[1].sql.contains("INSERT INTO"));
    assert_eq!(logs[1].rows_count, 1);
    assert!(logs[2].sql.contains("SELECT"));
    assert_eq!(logs[2].rows_count, 1);
}

#[tokio::test]
async fn test_telemetry_captures_query_errors() {
    let options = ConnectOptions::new("sqlite:file:telemetry_test2?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let error_metrics = Arc::new(Mutex::new(Vec::<QueryMetrics>::new()));
    let error_clone = Arc::clone(&error_metrics);

    engine.add_observer(move |m: &QueryMetrics| {
        if !m.success {
            error_clone.lock().unwrap().push(m.clone());
        }
    });

    // Execute a deliberately invalid query
    let result = engine
        .query(
            "SELECT * FROM non_existent_telemetry_table_xyz;",
            &params![],
        )
        .await;
    assert!(result.is_err(), "querying non-existent table must fail");

    let recorded_errors = error_metrics.lock().unwrap().clone();
    assert_eq!(recorded_errors.len(), 1);
    assert!(!recorded_errors[0].success);
    assert!(recorded_errors[0].error.is_some());
    assert_eq!(recorded_errors[0].rows_count, 0);
    assert!(
        recorded_errors[0]
            .sql
            .contains("non_existent_telemetry_table_xyz")
    );
}

struct CustomSlowObserver {
    query_count: AtomicUsize,
}

impl QueryObserver for CustomSlowObserver {
    fn on_query_executed(&self, _metrics: &QueryMetrics) {
        self.query_count.fetch_add(1, Ordering::SeqCst);
    }
}

#[tokio::test]
async fn test_telemetry_slow_query_observer_trait() {
    let options = ConnectOptions::new("sqlite:file:telemetry_test3?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let observer = Arc::new(CustomSlowObserver {
        query_count: AtomicUsize::new(0),
    });

    // Register custom struct implementing QueryObserver trait via Arc<T>
    engine.add_observer(observer.clone());

    engine.set_slow_query_threshold(Duration::from_millis(10));

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS telemetry_slow (id INT);",
            &params![],
        )
        .await
        .expect("execute");

    assert_eq!(observer.query_count.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_multiple_telemetry_observers_broadcasting() {
    let options = ConnectOptions::new("sqlite:file:telemetry_test4?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let counter_a = Arc::new(AtomicUsize::new(0));
    let counter_b = Arc::new(AtomicUsize::new(0));

    let a_clone = Arc::clone(&counter_a);
    engine.add_observer(move |_m: &QueryMetrics| {
        a_clone.fetch_add(1, Ordering::SeqCst);
    });

    let b_clone = Arc::clone(&counter_b);
    engine.add_observer(move |_m: &QueryMetrics| {
        b_clone.fetch_add(1, Ordering::SeqCst);
    });

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS telemetry_multi (id INT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute("INSERT INTO telemetry_multi VALUES (1);", &params![])
        .await
        .expect("insert");

    assert_eq!(counter_a.load(Ordering::SeqCst), 2);
    assert_eq!(counter_b.load(Ordering::SeqCst), 2);
}
