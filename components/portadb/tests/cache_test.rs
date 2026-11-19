use std::sync::Arc;
use std::time::Duration;

use portadb::cache::{CacheProvider, cache_key};
use portadb::{ConnectOptions, Engine, MemoryCache, NamedParams, params};

#[tokio::test]
async fn test_query_result_caching_and_zero_copy_sharing() {
    let options = ConnectOptions::new("sqlite:file:cache_test1?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    // Enable memory cache
    engine.set_cache(MemoryCache::new());

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS cache_products (id INT, title TEXT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute(
            "INSERT INTO cache_products VALUES (1, 'Rust Book'), (2, 'Makepad Guide');",
            &params![],
        )
        .await
        .expect("insert");

    let sql = "SELECT * FROM cache_products ORDER BY id ASC";
    let p = params![];

    // First call: populates cache
    let first_rows = engine
        .query_cached(sql, &p, Duration::from_secs(60))
        .await
        .expect("query cached 1");
    assert_eq!(first_rows.len(), 2);

    // Second call: returns cached Arc pointer
    let second_rows = engine
        .query_cached(sql, &p, Duration::from_secs(60))
        .await
        .expect("query cached 2");
    assert_eq!(second_rows.len(), 2);

    // Verify zero-copy: both Arc pointers point to the identical memory allocation!
    assert!(Arc::ptr_eq(&first_rows, &second_rows));
}

#[tokio::test]
async fn test_cache_ttl_expiration() {
    let options = ConnectOptions::new("sqlite:file:cache_test2?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine.set_cache(MemoryCache::new());

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS cache_ttl_test (id INT, val TEXT);",
            &params![],
        )
        .await
        .expect("create");

    engine
        .execute(
            "INSERT INTO cache_ttl_test VALUES (1, 'original');",
            &params![],
        )
        .await
        .expect("insert");

    let sql = "SELECT * FROM cache_ttl_test WHERE id = 1";
    let p = params![];

    // 1. Cache with short 40ms TTL
    let first = engine
        .query_cached(sql, &p, Duration::from_millis(40))
        .await
        .expect("query cached");
    assert_eq!(first.len(), 1);

    // 2. Query immediately before TTL expires: hit
    let cached = engine
        .query_cached(sql, &p, Duration::from_millis(40))
        .await
        .expect("query cached again");
    assert!(Arc::ptr_eq(&first, &cached));

    // 3. Insert new row
    engine
        .execute("INSERT INTO cache_ttl_test VALUES (2, 'new');", &params![])
        .await
        .expect("insert new");

    // 4. Sleep past TTL (50ms > 40ms)
    tokio::time::sleep(Duration::from_millis(50)).await;

    // 5. Query after TTL: cache miss, fresh query executed
    let fresh = engine
        .query_cached(
            "SELECT * FROM cache_ttl_test ORDER BY id ASC",
            &p,
            Duration::from_millis(40),
        )
        .await
        .expect("fresh query");
    assert_eq!(fresh.len(), 2);
}

#[tokio::test]
async fn test_cache_key_determinism_and_isolation() {
    let mut p1 = NamedParams::new();
    p1.insert("alpha", 1i64);
    p1.insert("beta", "test");

    // Insert in reverse order
    let mut p2 = NamedParams::new();
    p2.insert("beta", "test");
    p2.insert("alpha", 1i64);

    let k1 = cache_key("SELECT * FROM tbl", &p1);
    let k2 = cache_key("SELECT * FROM tbl", &p2);

    // Must be identical regardless of insertion order
    assert_eq!(k1, k2);

    // Different query text or params produces different keys
    let k3 = cache_key("SELECT * FROM other", &p1);
    assert_ne!(k1, k3);

    let mut p3 = NamedParams::new();
    p3.insert("alpha", 2i64);
    p3.insert("beta", "test");
    let k4 = cache_key("SELECT * FROM tbl", &p3);
    assert_ne!(k1, k4);
}

#[tokio::test]
async fn test_cache_manual_invalidation_and_clear() {
    let cache = MemoryCache::new();

    let rows = Arc::new(Vec::new());
    cache.set("k1", Arc::clone(&rows), Duration::from_secs(60));
    cache.set("k2", Arc::clone(&rows), Duration::from_secs(60));

    assert!(cache.get("k1").is_some());
    assert!(cache.get("k2").is_some());

    // Invalidate single key
    cache.invalidate("k1");
    assert!(cache.get("k1").is_none());
    assert!(cache.get("k2").is_some());

    // Clear all
    cache.clear();
    assert!(cache.get("k2").is_none());
}
