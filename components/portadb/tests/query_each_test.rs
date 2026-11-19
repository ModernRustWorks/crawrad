#![cfg(all(feature = "rusqlite", feature = "sqlx-sqlite"))]

use std::ops::ControlFlow;
use std::sync::Arc;

use portadb::{
    ConnectOptions, Connection, Database, NamedParams, RusqliteDriver, SqlxDriver, Value,
};

async fn verify_engine_query_each(mut conn: Connection) {
    conn.execute(
        "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT);",
        &NamedParams::new(),
    )
    .await
    .expect("create table");

    for i in 1..=5 {
        let mut params = NamedParams::new();
        params.insert("id", Value::Integer(i));
        params.insert("name", Value::Text(format!("name_{i}")));
        conn.execute("INSERT INTO t (id, name) VALUES (:id, :name);", &params)
            .await
            .expect("insert row");
    }

    // 1. query_each collecting ids -> [1, 2, 3, 4, 5], return value 5,
    // and rows share one column Arc (Arc::ptr_eq on shared_columns() of first and last row)
    let mut collected_ids = Vec::new();
    let mut collected_rows = Vec::new();
    let count = conn
        .query_each(
            "SELECT id, name FROM t ORDER BY id",
            &NamedParams::new(),
            |row| {
                let id: i64 = row.get("id").expect("get id");
                collected_ids.push(id);
                collected_rows.push(row);
                ControlFlow::Continue(())
            },
        )
        .await
        .expect("query_each");

    assert_eq!(count, 5);
    assert_eq!(collected_ids, vec![1, 2, 3, 4, 5]);
    assert_eq!(collected_rows.len(), 5);
    assert!(
        Arc::ptr_eq(
            collected_rows.first().unwrap().shared_columns(),
            collected_rows.last().unwrap().shared_columns()
        ),
        "all rows must share the same column Arc"
    );

    // 2. early stop: Break after the 2nd row -> returns 2 and callback ran exactly 2 times
    let mut callback_count = 0;
    let mut early_ids = Vec::new();
    let early_count = conn
        .query_each(
            "SELECT id, name FROM t ORDER BY id",
            &NamedParams::new(),
            |row| {
                callback_count += 1;
                let id: i64 = row.get("id").expect("get id");
                early_ids.push(id);
                if callback_count == 2 {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            },
        )
        .await
        .expect("query_each early stop");

    assert_eq!(early_count, 2);
    assert_eq!(callback_count, 2);
    assert_eq!(early_ids, vec![1, 2]);

    // 3. same result as query (compare Vec<Row> from query with rows collected by query_each)
    let query_rows = conn
        .query("SELECT id, name FROM t ORDER BY id", &NamedParams::new())
        .await
        .expect("query");
    assert_eq!(query_rows, collected_rows);
}

#[tokio::test]
async fn test_query_each_rusqlite() {
    let conn = Connection::open(RusqliteDriver::new(), &ConnectOptions::new(":memory:"))
        .await
        .expect("connect rusqlite");
    verify_engine_query_each(conn).await;
}

#[tokio::test]
async fn test_query_each_sqlx_sqlite() {
    let conn = Connection::open(SqlxDriver::new(), &ConnectOptions::new("sqlite::memory:"))
        .await
        .expect("connect sqlx");
    verify_engine_query_each(conn).await;
}

#[cfg(feature = "memory")]
#[tokio::test]
async fn test_query_each_memory_driver_default_body() {
    let options = ConnectOptions::new("memory://test_query_each_mem")
        .with_database(Database::Sqlite)
        .with_engine("memory");
    let mut conn = Connection::connect(&options)
        .await
        .expect("connect memory driver");

    conn.execute("CREATE TABLE t (id INT, name TEXT);", &NamedParams::new())
        .await
        .expect("create table");

    for i in 1..=5 {
        let mut params = NamedParams::new();
        params.insert("id", Value::Integer(i));
        params.insert("name", Value::Text(format!("name_{i}")));
        conn.execute("INSERT INTO t (id, name) VALUES (:id, :name)", &params)
            .await
            .expect("insert row");
    }

    // Default trait body feeds rows one by one
    let mut collected_ids = Vec::new();
    let count = conn
        .query_each("SELECT id, name FROM t", &NamedParams::new(), |row| {
            let id: i64 = row.get("id").expect("get id");
            collected_ids.push(id);
            ControlFlow::Continue(())
        })
        .await
        .expect("query_each");

    assert_eq!(count, 5);
    assert_eq!(collected_ids, vec![1, 2, 3, 4, 5]);

    // Proving early stop works on default trait body
    let mut callback_count = 0;
    let mut early_ids = Vec::new();
    let early_count = conn
        .query_each("SELECT id, name FROM t", &NamedParams::new(), |row| {
            callback_count += 1;
            let id: i64 = row.get("id").expect("get id");
            early_ids.push(id);
            if callback_count == 2 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .await
        .expect("query_each early stop");

    assert_eq!(early_count, 2);
    assert_eq!(callback_count, 2);
    assert_eq!(early_ids, vec![1, 2]);
}
