#[cfg(feature = "rusqlite")]
mod rusqlite_connection_tests {
    use crate::connection::Connection;
    use crate::database::Database;
    use crate::error::Error;
    use crate::options::ConnectOptions;
    use crate::params::NamedParams;

    #[tokio::test]
    async fn test_db_connection_open_and_execute() {
        let options = ConnectOptions::new(":memory:");
        let mut conn = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .expect("open connection");

        assert_eq!(conn.database(), Database::Sqlite);
        assert_eq!(conn.engine_name(), "rusqlite");

        conn.execute(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);",
            &NamedParams::new(),
        )
        .await
        .expect("create table");

        let mut params = NamedParams::new();
        params.insert("id", 1i64);
        params.insert("name", "Alice");

        let affected = conn
            .execute("INSERT INTO test (id, name) VALUES (:id, :name);", &params)
            .await
            .expect("insert");
        assert_eq!(affected, 1);

        let rows = conn
            .query("SELECT id, name FROM test WHERE id = :id;", &params)
            .await
            .expect("query");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get::<i64>("id").unwrap(), 1);
        assert_eq!(rows[0].get::<String>("name").unwrap(), "Alice");
    }

    #[tokio::test]
    async fn test_db_connection_transaction_commit_and_rollback() {
        let options = ConnectOptions::new(":memory:");
        let mut conn = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .expect("open connection");

        conn.execute(
            "CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance REAL);",
            &NamedParams::new(),
        )
        .await
        .unwrap();

        // Commit test
        {
            let mut tx = conn.begin_transaction().await.unwrap();
            let mut p = NamedParams::new();
            p.insert("id", 1i64);
            p.insert("b", 100.0f64);
            tx.execute("INSERT INTO accounts VALUES (:id, :b);", &p)
                .await
                .unwrap();
            tx.commit().await.unwrap();
        }

        let rows = conn
            .query("SELECT * FROM accounts WHERE id = 1;", &NamedParams::new())
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);

        // Rollback test
        {
            let mut tx = conn.begin_transaction().await.unwrap();
            let mut p = NamedParams::new();
            p.insert("id", 2i64);
            p.insert("b", 200.0f64);
            tx.execute("INSERT INTO accounts VALUES (:id, :b);", &p)
                .await
                .unwrap();
            tx.rollback().await.unwrap();
        }

        let rows = conn
            .query("SELECT * FROM accounts WHERE id = 2;", &NamedParams::new())
            .await
            .unwrap();
        assert_eq!(rows.len(), 0);

        // Auto-rollback on drop test
        {
            let mut tx = conn.begin_transaction().await.unwrap();
            let mut p = NamedParams::new();
            p.insert("id", 3i64);
            p.insert("b", 300.0f64);
            tx.execute("INSERT INTO accounts VALUES (:id, :b);", &p)
                .await
                .unwrap();
            // tx dropped here
        }

        let rows = conn
            .query("SELECT * FROM accounts WHERE id = 3;", &NamedParams::new())
            .await
            .unwrap();
        assert_eq!(rows.len(), 0);
    }

    #[tokio::test]
    async fn test_db_connection_unsupported_database() {
        let options = ConnectOptions::new("postgres://localhost/db");
        let res = Connection::open(crate::drivers::RusqliteDriver::new(), &options).await;
        assert!(matches!(res, Err(Error::ConfigurationError(_))));
    }

    #[tokio::test]
    async fn test_db_connection_rusqlite_memory_private() {
        let options = ConnectOptions::new(":memory:");
        let mut conn1 = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .unwrap();
        let mut conn2 = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .unwrap();

        conn1
            .execute(
                "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                &NamedParams::new(),
            )
            .await
            .unwrap();

        let rows = conn2
            .query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='t';",
                &NamedParams::new(),
            )
            .await
            .unwrap();
        assert!(rows.is_empty());

        assert!(
            conn2
                .query("SELECT * FROM t;", &NamedParams::new())
                .await
                .is_err()
        );
        assert_eq!(
            conn2
                .execute(
                    "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                    &NamedParams::new()
                )
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn test_db_connection_rusqlite_shared_named() {
        let options = ConnectOptions::new("file:conn_rusqlite_shared?mode=memory&cache=shared");
        let mut conn1 = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .unwrap();
        let mut conn2 = Connection::open(crate::drivers::RusqliteDriver::new(), &options)
            .await
            .unwrap();

        conn1
            .execute(
                "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                &NamedParams::new(),
            )
            .await
            .unwrap();
        conn1
            .execute("INSERT INTO t VALUES (77);", &NamedParams::new())
            .await
            .unwrap();

        let rows = conn2
            .query("SELECT * FROM t;", &NamedParams::new())
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get::<i64>("id").unwrap(), 77);
    }
}

#[cfg(feature = "sqlx-sqlite")]
mod sqlx_connection_tests {
    use crate::connection::Connection;
    use crate::database::Database;
    use crate::options::ConnectOptions;
    use crate::params::NamedParams;

    #[tokio::test]
    async fn test_db_connection_sqlx_memory_private() {
        let options = ConnectOptions::new("sqlite::memory:");
        let mut conn1 = Connection::open(crate::drivers::SqlxDriver::new(), &options)
            .await
            .unwrap();
        let mut conn2 = Connection::open(crate::drivers::SqlxDriver::new(), &options)
            .await
            .unwrap();

        assert_eq!(conn1.database(), Database::Sqlite);
        assert_eq!(conn1.engine_name(), "sqlx");

        conn1
            .execute(
                "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                &NamedParams::new(),
            )
            .await
            .unwrap();

        let rows = conn2
            .query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name='t';",
                &NamedParams::new(),
            )
            .await
            .unwrap();
        assert!(rows.is_empty());

        assert!(
            conn2
                .query("SELECT * FROM t;", &NamedParams::new())
                .await
                .is_err()
        );
        assert_eq!(
            conn2
                .execute(
                    "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                    &NamedParams::new()
                )
                .await
                .unwrap(),
            0
        );
    }

    #[tokio::test]
    async fn test_db_connection_sqlx_shared_named() {
        let options = ConnectOptions::new("file:conn_sqlx_shared?mode=memory&cache=shared");
        let mut conn1 = Connection::open(crate::drivers::SqlxDriver::new(), &options)
            .await
            .unwrap();
        let mut conn2 = Connection::open(crate::drivers::SqlxDriver::new(), &options)
            .await
            .unwrap();

        conn1
            .execute(
                "CREATE TABLE t (id INTEGER PRIMARY KEY);",
                &NamedParams::new(),
            )
            .await
            .unwrap();
        conn1
            .execute("INSERT INTO t VALUES (88);", &NamedParams::new())
            .await
            .unwrap();

        let rows = conn2
            .query("SELECT * FROM t;", &NamedParams::new())
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].get::<i64>("id").unwrap(), 88);
    }
}
