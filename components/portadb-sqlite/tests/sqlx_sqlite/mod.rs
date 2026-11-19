use crate::Driver as SqlxDriver;
use portadb_core::database::Database;
use portadb_core::driver::{Driver as _, DriverConnection};
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;

#[tokio::test]
async fn test_sqlx_driver_lifecycle() {
    let driver = SqlxDriver::new();
    assert_eq!(driver.name(), "sqlx");
    assert!(driver.supports_database(Database::Sqlite));
    assert!(!driver.supports_database(Database::Postgres));

    let options = ConnectOptions::new("sqlite::memory:");
    let mut conn = driver.connect(&options).await.unwrap();

    let affected = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS sqlx_users (id INTEGER PRIMARY KEY, name TEXT);",
            &NamedParams::new(),
        )
        .await
        .unwrap();
    assert_eq!(affected, 0);

    let mut p = NamedParams::new();
    p.insert("id", 20i64);
    p.insert("name", "Charlie");
    let affected = conn
        .execute("INSERT INTO sqlx_users (id, name) VALUES (:id, :name);", &p)
        .await
        .unwrap();
    assert_eq!(affected, 1);

    let rows = conn
        .query("SELECT id, name FROM sqlx_users WHERE id = :id;", &p)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 20);
    assert_eq!(rows[0].get::<String>("name").unwrap(), "Charlie");

    conn.close().await.unwrap();
}

#[tokio::test]
async fn test_sqlx_two_memory_connections_are_private() {
    let driver = SqlxDriver::new();
    let options = ConnectOptions::new("sqlite::memory:");

    let mut conn1 = driver.connect(&options).await.unwrap();
    let mut conn2 = driver.connect(&options).await.unwrap();

    // 1. Create table `t` in conn1
    conn1
        .execute(
            "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

    // conn2 does NOT see table `t`: query fails or sqlite_master has no `t`
    let master_rows = conn2
        .query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 't';",
            &NamedParams::new(),
        )
        .await
        .unwrap();
    assert!(
        master_rows.is_empty(),
        "conn2 must not see table `t` created in conn1"
    );

    let query_res = conn2.query("SELECT * FROM t;", &NamedParams::new()).await;
    assert!(
        query_res.is_err(),
        "querying `t` in conn2 must fail because table does not exist"
    );

    // conn2 can create its own table `t` without error
    let affected = conn2
        .execute(
            "CREATE TABLE t (id INTEGER PRIMARY KEY, count INTEGER);",
            &NamedParams::new(),
        )
        .await
        .unwrap();
    assert_eq!(affected, 0);
}

#[tokio::test]
async fn test_sqlx_one_memory_connection_retains_data_across_queries() {
    let driver = SqlxDriver::new();
    let options = ConnectOptions::new("sqlite::memory:");
    let mut conn = driver.connect(&options).await.unwrap();

    conn.execute(
        "CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT);",
        &NamedParams::new(),
    )
    .await
    .unwrap();

    let mut p1 = NamedParams::new();
    p1.insert("id", 1i64);
    p1.insert("name", "alpha");
    conn.execute("INSERT INTO items (id, name) VALUES (:id, :name);", &p1)
        .await
        .unwrap();

    let mut p2 = NamedParams::new();
    p2.insert("id", 2i64);
    p2.insert("name", "beta");
    conn.execute("INSERT INTO items (id, name) VALUES (:id, :name);", &p2)
        .await
        .unwrap();

    // Multiple queries (repeated and concurrent) all see the inserted data
    for _ in 0..10 {
        let rows = conn
            .query(
                "SELECT id, name FROM items ORDER BY id;",
                &NamedParams::new(),
            )
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get::<i64>("id").unwrap(), 1);
        assert_eq!(rows[0].get::<String>("name").unwrap(), "alpha");
        assert_eq!(rows[1].get::<i64>("id").unwrap(), 2);
        assert_eq!(rows[1].get::<String>("name").unwrap(), "beta");
    }

    // Concurrent queries on cloned connection handles verify pooled/concurrent queries see data
    let raw = conn.raw_conn().clone();
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let r = std::sync::Arc::clone(&raw);
            tokio::spawn(async move {
                let mut guard = r.lock().await;
                let c = guard.as_mut().unwrap();
                let rows = sqlx::query("SELECT id, name FROM items ORDER BY id;")
                    .fetch_all(c)
                    .await
                    .unwrap();
                assert_eq!(rows.len(), 2);
            })
        })
        .collect();

    for h in handles {
        h.await.unwrap();
    }
}

#[tokio::test]
async fn test_sqlx_explicit_named_shared_memory_shares_database() {
    let driver = SqlxDriver::new();
    let options = ConnectOptions::new("file:sqlx_shared_test?mode=memory&cache=shared");

    let mut conn1 = driver.connect(&options).await.unwrap();
    let mut conn2 = driver.connect(&options).await.unwrap();

    conn1
        .execute(
            "CREATE TABLE shared_t (id INTEGER PRIMARY KEY, label TEXT);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

    let mut p = NamedParams::new();
    p.insert("id", 200i64);
    p.insert("label", "sqlx_shared");
    conn1
        .execute("INSERT INTO shared_t (id, label) VALUES (:id, :label);", &p)
        .await
        .unwrap();

    // conn2 opened with the same named shared URL DOES see shared_t and its rows
    let rows = conn2
        .query("SELECT id, label FROM shared_t WHERE id = :id;", &p)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 200);
    assert_eq!(rows[0].get::<String>("label").unwrap(), "sqlx_shared");
}
