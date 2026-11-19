#[cfg(feature = "rusqlite")]
mod tests {
    use portadb::{
        ColumnDef, ConnectOptions, DataType, Engine, Error, Executor, TableSchema, params,
    };

    /// Generic business function that accepts any [`Executor`] (Connection or Transaction).
    async fn insert_user(exec: &mut impl Executor, id: i64, name: &str) -> Result<u64, Error> {
        exec.execute(
            "INSERT INTO users (id, name) VALUES (:id, :name)",
            &params![("id", id), ("name", name)],
        )
        .await
    }

    #[tokio::test]
    async fn test_engine_lifecycle_and_executor_trait() {
        let options = ConnectOptions::new(":memory:").with_engine("rusqlite");
        let engine = Engine::connect(&options).await.expect("connect engine");

        // Health check
        engine.ping().await.expect("ping");

        // Acquire connection and use generic Executor
        let mut conn = engine.acquire().await.expect("acquire connection");

        conn.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
            &params![],
        )
        .await
        .expect("create table");

        insert_user(&mut conn, 1, "Alice")
            .await
            .expect("insert on conn");

        // Start transaction using begin() and use generic Executor
        let mut tx = conn.begin().await.expect("begin transaction");
        insert_user(&mut tx, 2, "Bob").await.expect("insert on tx");
        tx.commit().await.expect("commit");

        // Query through connection
        let rows = conn
            .query("SELECT id, name FROM users ORDER BY id", &params![])
            .await
            .expect("query");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get::<String>("name").unwrap(), "Alice");
        assert_eq!(rows[1].get::<String>("name").unwrap(), "Bob");

        // Fetch into reactive TableView
        let schema = TableSchema::new()
            .with_column(ColumnDef::new("id", DataType::Integer, false))
            .with_column(ColumnDef::new("name", DataType::Text, false));

        let view = portadb::TableView::fetch(
            &mut conn,
            "SELECT id, name FROM users ORDER BY id",
            &params![],
            schema,
        )
        .await
        .expect("fetch into TableView");

        assert_eq!(view.len(), 2);
        assert_eq!(view.selected_index(), Some(0));
        assert_eq!(view.get_value::<String>("name").unwrap(), "Alice");

        assert!(view.next());
        assert_eq!(view.selected_index(), Some(1));
        assert_eq!(view.get_value::<String>("name").unwrap(), "Bob");
    }
}
