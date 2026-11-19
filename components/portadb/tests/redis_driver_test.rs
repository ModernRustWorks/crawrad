use portadb::{ConnectOptions, Database, Engine, Feature, params};

#[tokio::test]
async fn test_redis_engine_and_unsupported_features_abstraction() {
    let options = ConnectOptions::new("redis://127.0.0.1:6379/0");
    let engine = Engine::connect(&options)
        .await
        .expect("connect to redis engine");

    assert_eq!(engine.database(), Database::Redis);
    assert_eq!(engine.driver_name(), "redis");

    // 1. Capability inspection
    assert!(engine.supports(Feature::KeyValue));
    assert!(engine.supports(Feature::Transactions));
    assert!(!engine.supports(Feature::RelationalDdl));
    assert!(!engine.supports(Feature::Joins));
    let req_err = engine.require_feature(Feature::Joins).unwrap_err();
    assert!(req_err.is_unsupported());

    let mut conn = engine.acquire().await.expect("acquire redis connection");
    assert_eq!(conn.capabilities(), engine.capabilities());

    // 2. Ping
    conn.ping().await.expect("ping redis");

    // 3. SET key value with named parameters
    let affected = conn
        .execute(
            "SET :key :val",
            &params![("key", "session:user_123"), ("val", "auth_token_xyz")],
        )
        .await
        .expect("set key");
    assert_eq!(affected, 1);

    // 4. GET key with typed column access
    let rows = conn
        .query("GET :key", &params![("key", "session:user_123")])
        .await
        .expect("get key");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String>("value").unwrap(), "auth_token_xyz");

    // 5. EXISTS key
    let exists_rows = conn
        .query("EXISTS :key", &params![("key", "session:user_123")])
        .await
        .expect("exists key");
    assert_eq!(exists_rows.len(), 1);
    assert!(exists_rows[0].get::<bool>("exists").unwrap());

    // 6. Transaction with rollback (DISCARD)
    {
        let mut tx = conn.begin().await.expect("begin transaction");
        tx.execute(
            "SET :key :val",
            &params![("key", "session:temp"), ("val", "will_rollback")],
        )
        .await
        .expect("tx set");
        tx.rollback().await.expect("rollback");
    }

    let check_rollback = conn
        .query("GET :key", &params![("key", "session:temp")])
        .await
        .expect("query temp");
    assert_eq!(check_rollback.len(), 0);

    // 7. Transparent Tabular SQL Abstraction over Key-Value Store:
    // Callers can write standard SQL statements and the driver transparently
    // maps them to the underlying key-value store.
    conn.execute(
        "INSERT INTO kv (key, value) VALUES (:key, :value)",
        &params![("key", "cfg:timeout"), ("value", "30s")],
    )
    .await
    .expect("abstracted sql insert");

    let table_rows = conn
        .query("SELECT * FROM kv", &params![])
        .await
        .expect("abstracted sql select all");
    assert!(table_rows.len() >= 2);

    let specific_row = conn
        .query(
            "SELECT value FROM kv WHERE key = :key",
            &params![("key", "cfg:timeout")],
        )
        .await
        .expect("abstracted sql select by key");
    assert_eq!(specific_row.len(), 1);
    assert_eq!(specific_row[0].get::<String>("value").unwrap(), "30s");

    conn.execute(
        "DELETE FROM kv WHERE key = :key",
        &params![("key", "cfg:timeout")],
    )
    .await
    .expect("abstracted sql delete");

    // 8. DEL key
    let deleted = conn
        .execute("DEL :key", &params![("key", "session:user_123")])
        .await
        .expect("del key");
    assert_eq!(deleted, 1);

    // 9. Rejection of Truly Unsupported Relational Features:
    // Complex features that cannot be represented in a Key-Value engine
    // (such as relational DDL or multi-table JOINs) return typed Error::UnsupportedFeature.
    let unsupported_err = conn
        .execute("CREATE TABLE relational_table (id INT)", &params![])
        .await
        .unwrap_err();

    assert!(
        unsupported_err.is_unsupported(),
        "Expected is_unsupported() to be true, got: {unsupported_err:?}"
    );

    let join_err = conn
        .query(
            "SELECT * FROM users JOIN orders ON users.id = orders.user_id",
            &params![],
        )
        .await
        .unwrap_err();

    assert!(
        join_err.is_unsupported(),
        "Expected is_unsupported() for relational JOIN on Redis, got: {join_err:?}"
    );
}
