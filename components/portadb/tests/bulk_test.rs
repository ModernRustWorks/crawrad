use portadb::{BulkLoader, ConnectOptions, Engine, Value, params};

#[tokio::test]
async fn test_bulk_loader_batch_insertions() {
    let options = ConnectOptions::new("sqlite:file:bulk_test1?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS sensor_data (id INT, device TEXT, temperature REAL);",
            &params![],
        )
        .await
        .expect("create");

    // Prepare 150 rows of data
    let mut rows_data = Vec::new();
    for i in 1..=150 {
        rows_data.push(vec![
            Value::Integer(i),
            Value::Text(format!("sensor_{}", i % 5)),
            Value::Real(25.0 + (i as f64) * 0.1),
        ]);
    }

    // Insert with batch size 50 (3 chunks)
    let loader =
        BulkLoader::new("sensor_data", &["id", "device", "temperature"]).with_batch_size(50);

    let inserted = loader
        .insert_all(&engine, &rows_data)
        .await
        .expect("bulk insert");
    assert_eq!(inserted, 150);

    let count_rows = engine
        .query("SELECT count(*) as total FROM sensor_data;", &params![])
        .await
        .expect("count query");
    assert_eq!(count_rows[0].get::<i64>("total").unwrap(), 150);
}

#[tokio::test]
async fn test_bulk_loader_empty_data() {
    let options = ConnectOptions::new("sqlite:file:bulk_test2?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS empty_bulk_test (id INT, val TEXT);",
            &params![],
        )
        .await
        .expect("create");

    let loader = BulkLoader::new("empty_bulk_test", &["id", "val"]).with_batch_size(10);
    let inserted = loader.insert_all(&engine, &[]).await.expect("insert empty");
    assert_eq!(inserted, 0);
}

#[tokio::test]
async fn test_bulk_loader_heterogeneous_values_and_nulls() {
    let options = ConnectOptions::new("sqlite:file:bulk_test3?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS bulk_mixed (id INT, note TEXT, active INT, payload BLOB);",
            &params![],
        )
        .await
        .expect("create");

    let rows_data = vec![
        vec![
            Value::Integer(1),
            Value::Null,
            Value::Boolean(true),
            Value::Blob(vec![0xDE, 0xAD]),
        ],
        vec![
            Value::Integer(2),
            Value::Text("non-null note".to_string()),
            Value::Boolean(false),
            Value::Null,
        ],
    ];

    let loader =
        BulkLoader::new("bulk_mixed", &["id", "note", "active", "payload"]).with_batch_size(1); // test batch size 1 (1 chunk per row)

    let inserted = loader
        .insert_all(&engine, &rows_data)
        .await
        .expect("insert mixed");
    assert_eq!(inserted, 2);

    let rows = engine
        .query(
            "SELECT id, note, active FROM bulk_mixed ORDER BY id ASC;",
            &params![],
        )
        .await
        .expect("query");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get::<Option<String>>("note").unwrap(), None);
    assert_eq!(
        rows[1].get::<Option<String>>("note").unwrap(),
        Some("non-null note".to_string())
    );
}

#[tokio::test]
async fn test_bulk_loader_batch_size_larger_than_dataset() {
    let options = ConnectOptions::new("sqlite:file:bulk_test4?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS bulk_single_chunk (id INT, val TEXT);",
            &params![],
        )
        .await
        .expect("create");

    let rows_data = vec![
        vec![Value::Integer(1), Value::Text("A".to_string())],
        vec![Value::Integer(2), Value::Text("B".to_string())],
        vec![Value::Integer(3), Value::Text("C".to_string())],
    ];

    // Batch size 1000 is much larger than 3 rows -> single batch
    let loader = BulkLoader::new("bulk_single_chunk", &["id", "val"]).with_batch_size(1000);
    let inserted = loader
        .insert_all(&engine, &rows_data)
        .await
        .expect("insert");
    assert_eq!(inserted, 3);

    let count_rows = engine
        .query(
            "SELECT count(*) as total FROM bulk_single_chunk;",
            &params![],
        )
        .await
        .expect("count");
    assert_eq!(count_rows[0].get::<i64>("total").unwrap(), 3);
}
