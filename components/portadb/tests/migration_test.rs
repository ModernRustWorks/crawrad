use portadb::{ConnectOptions, Engine, Migration, Migrator, params};

#[tokio::test]
async fn test_transactional_schema_migrations_and_rollback() {
    let options = ConnectOptions::new("sqlite:file:mig_tx_test1?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let migrator = Migrator::new()
        .add(Migration::new(
            1,
            "create_users",
            "CREATE TABLE IF NOT EXISTS mig_users (id INTEGER PRIMARY KEY, email TEXT NOT NULL);",
            "DROP TABLE mig_users;",
        ))
        .add(Migration::new(
            2,
            "add_status_column",
            "ALTER TABLE mig_users ADD COLUMN status TEXT DEFAULT 'active';",
            "ALTER TABLE mig_users DROP COLUMN status;",
        ));

    // 1. Run migrations
    let applied = migrator.run(&engine).await.expect("run migrations");
    assert_eq!(applied, 2);

    // 2. Insert data using migrated columns
    engine
        .execute(
            "INSERT INTO mig_users (id, email, status) VALUES (1, 'alice@example.com', 'verified');",
            &params![],
        )
        .await
        .expect("insert into migrated table");

    let rows = engine
        .query(
            "SELECT email, status FROM mig_users WHERE id = 1;",
            &params![],
        )
        .await
        .expect("query");
    assert_eq!(rows[0].get::<String>("status").unwrap(), "verified");

    // 3. Idempotency: running again applies 0 new migrations
    let applied_again = migrator.run(&engine).await.expect("run again");
    assert_eq!(applied_again, 0);

    // 4. Introspect schema discovering newly migrated tables
    let tables = engine.inspect_tables().await.expect("inspect tables");
    assert!(tables.contains(&"mig_users".to_string()));

    // 5. Introspect table schema column definitions
    let schema = engine
        .inspect_schema("mig_users")
        .await
        .expect("inspect schema");
    assert!(schema.has_column("id"));
    assert!(schema.has_column("email"));

    // 6. Rollback latest migration (version 2)
    let rolled_back = migrator.rollback_last(&engine).await.expect("rollback 2");
    assert_eq!(rolled_back, Some(2));

    // 7. Rollback version 1
    let rolled_back = migrator.rollback_last(&engine).await.expect("rollback 1");
    assert_eq!(rolled_back, Some(1));

    // 8. Rollback when no migrations remain
    let rolled_back = migrator
        .rollback_last(&engine)
        .await
        .expect("rollback empty");
    assert_eq!(rolled_back, None);
}

#[tokio::test]
async fn test_migrator_out_of_order_registration_is_sorted() {
    let migrator = Migrator::new()
        .add(Migration::new(
            30,
            "m30",
            "CREATE TABLE IF NOT EXISTS mig_order_30 (id INT);",
            "DROP TABLE mig_order_30;",
        ))
        .add(Migration::new(
            10,
            "m10",
            "CREATE TABLE IF NOT EXISTS mig_order_10 (id INT);",
            "DROP TABLE mig_order_10;",
        ))
        .add(Migration::new(
            20,
            "m20",
            "CREATE TABLE IF NOT EXISTS mig_order_20 (id INT);",
            "DROP TABLE mig_order_20;",
        ));

    let options = ConnectOptions::new("sqlite:file:mig_tx_test2?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let applied = migrator.run(&engine).await.expect("run");
    assert_eq!(applied, 3);

    // Verify history in database is strictly sorted 10, 20, 30
    let rows = engine
        .query(
            "SELECT version FROM _dataengine_migrations ORDER BY version ASC;",
            &params![],
        )
        .await
        .expect("query migrations");
    let versions: Vec<i64> = rows.iter().map(|r| r.get("version").unwrap()).collect();
    assert_eq!(versions, vec![10, 20, 30]);
}

#[tokio::test]
async fn test_migrator_atomic_rollback_on_failed_migration() {
    let options = ConnectOptions::new("sqlite:file:mig_tx_test3?mode=memory&cache=shared");
    let engine = Engine::connect(&options).await.expect("connect");

    let migrator = Migrator::new()
        .add(Migration::new(
            1,
            "good_migration",
            "CREATE TABLE IF NOT EXISTS mig_good (id INT);",
            "DROP TABLE mig_good;",
        ))
        .add(Migration::new(
            2,
            "broken_migration",
            "THIS IS INTENTIONALLY INVALID SQL SYNTAX ERROR;",
            "DROP TABLE mig_good;",
        ));

    // Run migrations must fail on broken migration
    let result = migrator.run(&engine).await;
    assert!(result.is_err(), "broken migration must return Err");

    // Atomic transaction ensures version 2 was NOT recorded
    let rows = engine
        .query(
            "SELECT version FROM _dataengine_migrations WHERE version = 2;",
            &params![],
        )
        .await
        .unwrap_or_default();
    assert_eq!(rows.len(), 0);
}
