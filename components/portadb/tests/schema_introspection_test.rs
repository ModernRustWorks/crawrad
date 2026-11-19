use portadb::{ConnectOptions, DataType, Engine, params};

#[tokio::test]
async fn test_schema_introspection_sqlite() {
    let options = ConnectOptions::new("sqlite:file:schema_test_mem?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    // 1. Create a table with diverse column definitions
    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                balance REAL,
                avatar BLOB,
                is_active INTEGER DEFAULT 1
            );",
            &params![],
        )
        .await
        .expect("create accounts");

    // Create another table
    engine
        .execute(
            "CREATE TABLE IF NOT EXISTS audit_logs (
                log_id INTEGER PRIMARY KEY,
                action TEXT NOT NULL
            );",
            &params![],
        )
        .await
        .expect("create audit_logs");

    // 2. Discover tables
    let tables = engine.inspect_tables().await.expect("inspect tables");
    assert!(tables.contains(&"accounts".to_string()));
    assert!(tables.contains(&"audit_logs".to_string()));

    // 3. Inspect 'accounts' schema
    let schema = engine
        .inspect_schema("accounts")
        .await
        .expect("inspect accounts");

    assert!(schema.has_column("id"));
    assert!(schema.has_column("username"));
    assert!(schema.has_column("balance"));
    assert!(schema.has_column("avatar"));
    assert!(schema.has_column("is_active"));

    let id_col = schema.find_column("id").expect("id col");
    assert_eq!(id_col.data_type, DataType::Integer);

    let username_col = schema.find_column("username").expect("username col");
    assert_eq!(username_col.data_type, DataType::Text);
    assert!(
        !username_col.nullable,
        "username NOT NULL must not be nullable"
    );

    let balance_col = schema.find_column("balance").expect("balance col");
    assert_eq!(balance_col.data_type, DataType::Real);
    assert!(
        balance_col.nullable,
        "balance without NOT NULL must be nullable"
    );

    let avatar_col = schema.find_column("avatar").expect("avatar col");
    assert_eq!(avatar_col.data_type, DataType::Blob);

    // 4. Non-existent table returns empty schema without crashing
    let empty_schema = engine
        .inspect_schema("non_existent_table")
        .await
        .expect("inspect non existent");
    assert_eq!(empty_schema.columns().len(), 0);
}
