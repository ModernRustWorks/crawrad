#[cfg(feature = "memory")]
mod tests {
    use portadb::{ConnectOptions, Database, Engine, params};

    #[tokio::test]
    async fn test_in_memory_driver_execution_and_rollback() {
        let options = ConnectOptions::new("memory://testdb")
            .with_database(Database::Sqlite)
            .with_engine("memory");

        let engine = Engine::connect(&options)
            .await
            .expect("connect to memory driver");

        assert_eq!(engine.driver_name(), "memory");

        // 1. Create table
        engine
            .execute("CREATE TABLE products (id INT, title TEXT);", &params![])
            .await
            .expect("create table");

        // 2. Insert rows
        let mut conn = engine.acquire().await.expect("acquire");
        conn.execute(
            "INSERT INTO products (id, title) VALUES (:id, :title)",
            &params![("id", 1i64), ("title", "Notebook")],
        )
        .await
        .expect("insert");

        // 3. Query
        let rows = conn
            .query(
                "SELECT * FROM products WHERE id = :id",
                &params![("id", 1i64)],
            )
            .await
            .expect("query");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get::<String>("title").unwrap(), "Notebook");

        // 4. Transaction with rollback
        {
            let mut tx = conn.begin().await.expect("begin transaction");
            tx.execute(
                "INSERT INTO products (id, title) VALUES (:id, :title)",
                &params![("id", 2i64), ("title", "Desk")],
            )
            .await
            .expect("insert");
            tx.rollback().await.expect("rollback");
        }

        // Verify row 2 was rolled back
        let rows_after = conn
            .query(
                "SELECT * FROM products WHERE id = :id",
                &params![("id", 2i64)],
            )
            .await
            .expect("query");
        assert_eq!(rows_after.len(), 0);
    }
}
