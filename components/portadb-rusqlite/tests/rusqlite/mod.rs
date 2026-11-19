use crate::Driver as RusqliteDriver;
use portadb_core::database::Database;
use portadb_core::driver::{Driver as _, DriverConnection};
use portadb_core::options::ConnectOptions;
use portadb_core::params::NamedParams;

#[tokio::test]
async fn test_rusqlite_driver_lifecycle() {
    let driver = RusqliteDriver::new();
    assert_eq!(driver.name(), "rusqlite");
    assert!(driver.supports_database(Database::Sqlite));
    assert!(!driver.supports_database(Database::Postgres));

    let options = ConnectOptions::new(":memory:");
    let mut conn = driver.connect(&options).await.unwrap();

    let affected = conn
        .execute(
            "CREATE TABLE IF NOT EXISTS rusqlite_users (id INTEGER PRIMARY KEY, name TEXT);",
            &NamedParams::new(),
        )
        .await
        .unwrap();
    assert_eq!(affected, 0);

    let mut p = NamedParams::new();
    p.insert("id", 10i64);
    p.insert("name", "Bob");
    let affected = conn
        .execute(
            "INSERT INTO rusqlite_users (id, name) VALUES (:id, :name);",
            &p,
        )
        .await
        .unwrap();
    assert_eq!(affected, 1);

    let rows = conn
        .query("SELECT id, name FROM rusqlite_users WHERE id = :id;", &p)
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 10);
    assert_eq!(rows[0].get::<String>("name").unwrap(), "Bob");

    conn.close().await.unwrap();
}

#[tokio::test]
async fn test_rusqlite_execute_accepts_statements_that_answer_with_rows() {
    let driver = RusqliteDriver::new();
    let options = ConnectOptions::new("file:rusqlite_exec_rows?mode=memory&cache=shared");
    let mut conn = driver.connect(&options).await.unwrap();

    // A pragma that answers with a row used to fail with ExecuteReturnedResults
    conn.execute("PRAGMA journal_mode = MEMORY;", &NamedParams::new())
        .await
        .unwrap();

    conn.execute(
        "CREATE TABLE exec_rows (id INTEGER PRIMARY KEY, name TEXT);",
        &NamedParams::new(),
    )
    .await
    .unwrap();

    // RETURNING runs exactly once: one row inserted, not two
    let mut params = NamedParams::new();
    params.insert("name", "once");
    let affected = conn
        .execute(
            "INSERT INTO exec_rows (name) VALUES (:name) RETURNING id;",
            &params,
        )
        .await
        .unwrap();
    assert_eq!(affected, 1);

    let rows = conn
        .query("SELECT COUNT(*) FROM exec_rows;", &NamedParams::new())
        .await
        .unwrap();
    assert_eq!(rows[0].values()[0], portadb_core::value::Value::Integer(1));
}

#[tokio::test]
async fn test_rusqlite_two_memory_connections_are_private() {
    let driver = RusqliteDriver::new();
    let options = ConnectOptions::new(":memory:");

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
async fn test_rusqlite_one_memory_connection_retains_data_across_queries() {
    let driver = RusqliteDriver::new();
    let options = ConnectOptions::new(":memory:");
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

    // Multiple queries all see the inserted data
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
}

#[tokio::test]
async fn test_rusqlite_explicit_named_shared_memory_shares_database() {
    let driver = RusqliteDriver::new();
    let options = ConnectOptions::new("file:rusqlite_shared_test?mode=memory&cache=shared");

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
    p.insert("id", 100i64);
    p.insert("label", "shared_value");
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
    assert_eq!(rows[0].get::<i64>("id").unwrap(), 100);
    assert_eq!(rows[0].get::<String>("label").unwrap(), "shared_value");
}
