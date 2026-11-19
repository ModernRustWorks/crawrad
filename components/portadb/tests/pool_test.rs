use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use portadb::pool::{Pool, PoolOptions};
use portadb::{ConnectOptions, Engine, Error, Executor, params};

#[tokio::test]
async fn test_connection_pool_lifecycle_and_concurrency() {
    let options = ConnectOptions::new("sqlite:file:pool_test1?mode=memory&cache=shared")
        .with_max_connections(5);

    let engine = Engine::connect(&options).await.expect("connect with pool");

    // 1. Initial status
    let status = engine.pool_status().await.expect("pool status");
    assert_eq!(status.max, 5);

    // 2. Initialize schema
    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS pool_counter (id INTEGER PRIMARY KEY, val INTEGER);",
            &params![],
        )
        .await
        .expect("create table");

    engine
        .execute(
            "INSERT INTO pool_counter (id, val) VALUES (1, 0);",
            &params![],
        )
        .await
        .expect("insert init");

    // 3. Concurrent operations acquiring pooled connections
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let eng = engine.clone();
        let counter = Arc::clone(&success_count);
        handles.push(tokio::spawn(async move {
            let mut conn = eng.acquire_pooled().await.expect("acquire from pool");
            let rows = conn
                .query("SELECT val FROM pool_counter WHERE id = 1;", &params![])
                .await
                .expect("query counter");
            assert_eq!(rows.len(), 1);
            counter.fetch_add(1, Ordering::SeqCst);
            // conn dropped here and recycled back to pool
        }));
    }

    for h in handles {
        h.await.expect("task join");
    }

    assert_eq!(success_count.load(Ordering::SeqCst), 10);

    // 4. Verify connections are idle and available in pool
    tokio::time::sleep(Duration::from_millis(50)).await;
    let final_status = engine.pool_status().await.expect("pool status");
    assert!(final_status.idle > 0);
    assert_eq!(final_status.active, 0);
}

#[tokio::test]
async fn test_pool_exhaustion_timeout() {
    let options = ConnectOptions::new("sqlite:file:pool_test2?mode=memory&cache=shared");
    let registry = portadb::registry();
    let driver = registry
        .find_for(options.database, options.engine.as_deref())
        .expect("driver found");

    let pool_options = PoolOptions::new()
        .with_max_connections(2)
        .with_acquire_timeout(Duration::from_millis(100));

    let pool = Pool::new(driver, options, pool_options)
        .await
        .expect("create pool");

    // Check out all 2 connections
    let _conn1 = pool.acquire().await.expect("checkout 1");
    let _conn2 = pool.acquire().await.expect("checkout 2");

    let status = pool.status().await;
    assert_eq!(status.active, 2);
    assert_eq!(status.idle, 0);

    // 3rd acquire must timeout
    let result = pool.acquire().await;
    match result {
        Err(Error::ConnectionFailed(msg)) => {
            assert!(msg.contains("timed out"));
        }
        other => panic!("expected timeout ConnectionFailed, got: {:?}", other.err()),
    }
}

#[tokio::test]
async fn test_pool_discard_dirty_connection() {
    let options = ConnectOptions::new("sqlite:file:pool_test3?mode=memory&cache=shared");
    let registry = portadb::registry();
    let driver = registry
        .find_for(options.database, options.engine.as_deref())
        .expect("driver found");

    let pool_options = PoolOptions::new().with_max_connections(2);
    let pool = Pool::new(driver, options, pool_options)
        .await
        .expect("create pool");

    // Acquire and discard
    let conn = pool.acquire().await.expect("acquire");
    conn.discard();

    tokio::time::sleep(Duration::from_millis(20)).await;

    // Discarded connection must not be in idle queue
    let status = pool.status().await;
    assert_eq!(status.idle, 0);
    assert_eq!(status.active, 0);

    // But its permit was freed, so we can acquire a fresh connection immediately
    let fresh = pool.acquire().await.expect("acquire fresh");
    assert_eq!(pool.status().await.active, 1);
    drop(fresh);
}

#[tokio::test]
async fn test_pool_idle_timeout_expiration() {
    let options = ConnectOptions::new("sqlite:file:pool_test4?mode=memory&cache=shared");
    let registry = portadb::registry();
    let driver = registry
        .find_for(options.database, options.engine.as_deref())
        .expect("driver found");

    // Very short idle timeout: 30ms
    let pool_options = PoolOptions::new()
        .with_max_connections(2)
        .with_idle_timeout(Duration::from_millis(30));

    let pool = Pool::new(driver, options, pool_options)
        .await
        .expect("create pool");

    // Acquire and return to idle
    let conn = pool.acquire().await.expect("acquire");
    drop(conn);

    tokio::time::sleep(Duration::from_millis(20)).await;
    let status_before = pool.status().await;
    assert_eq!(status_before.idle, 1);

    // Sleep past idle timeout
    tokio::time::sleep(Duration::from_millis(40)).await;

    // Next acquire will see the entry has expired, discard it, and create a fresh one
    let fresh = pool.acquire().await.expect("acquire after expire");
    assert_eq!(pool.status().await.active, 1);
    drop(fresh);
}

#[tokio::test]
async fn test_pooled_connection_transaction_commit_and_rollback() {
    let options = ConnectOptions::new("sqlite:file:pool_test5?mode=memory&cache=shared")
        .with_max_connections(2);
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS pool_tx_test (id INTEGER PRIMARY KEY, note TEXT);",
            &params![],
        )
        .await
        .expect("create table");

    // 1. Transaction Commit on pooled connection
    {
        let mut conn = engine.acquire_pooled().await.expect("acquire");
        let mut tx = conn.begin().await.expect("begin tx");
        tx.execute(
            "INSERT INTO pool_tx_test (id, note) VALUES (1, 'committed');",
            &params![],
        )
        .await
        .expect("insert in tx");
        tx.commit().await.expect("commit tx");
    }

    // 2. Transaction Rollback on pooled connection
    {
        let mut conn = engine.acquire_pooled().await.expect("acquire");
        let mut tx = conn.begin().await.expect("begin tx");
        tx.execute(
            "INSERT INTO pool_tx_test (id, note) VALUES (2, 'rolled_back');",
            &params![],
        )
        .await
        .expect("insert in tx");
        tx.rollback().await.expect("rollback tx");
    }

    // 3. Verify only committed row is present
    let rows = engine
        .query("SELECT id, note FROM pool_tx_test ORDER BY id;", &params![])
        .await
        .expect("query");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String>("note").unwrap(), "committed");
}
