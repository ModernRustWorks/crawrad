#![cfg(any(feature = "rusqlite", feature = "sqlx-sqlite"))]

use portadb::{ConnectOptions, Connection, Error, Value, params};

/// The EXACT same user business logic function.
///
/// Notice the signature: it accepts the unified [`Connection`] abstraction directly.
/// No generics, no trait parameters, and not a single line of this function knows or cares
/// whether the connection was instantiated with `rusqlite` or `sqlx`.
async fn run_complete_business_flow(conn: &mut Connection) -> Result<(), Error> {
    // 1. DDL: Create table
    conn.execute(
        "CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            sku TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            price REAL NOT NULL,
            description TEXT,
            stock INTEGER NOT NULL
        )",
        &params![],
    )
    .await?;

    // 2. Insert with named parameters (testing numbers, strings, and NULL)
    let affected = conn
        .execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 1i64),
                ("sku", "SKU-RUST-001"),
                ("title", "Rust Programming Guide"),
                ("price", 49.99f64),
                ("desc", "Comprehensive guide to safe systems programming"),
                ("stock", 100i64),
            ],
        )
        .await?;
    assert_eq!(affected, 1);

    // Insert a product with SQL NULL for description
    let affected = conn
        .execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 2i64),
                ("sku", "SKU-RUST-002"),
                ("title", "Minimal Widget"),
                ("price", 9.95f64),
                ("desc", Value::Null),
                ("stock", 50i64),
            ],
        )
        .await?;
    assert_eq!(affected, 1);

    // 3. Unique Constraint Violation: Verify typed error handling (NO STRING MATCHING)
    let err = conn
        .execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 3i64),
                ("sku", "SKU-RUST-001"), // Duplicate SKU!
                ("title", "Duplicate Product"),
                ("price", 10.0f64),
                ("desc", Value::Null),
                ("stock", 5i64),
            ],
        )
        .await
        .unwrap_err();

    assert!(
        err.is_unique_violation(),
        "Expected UniqueViolation typed error, got {err:?}"
    );

    // 4. Query with named parameter
    let rows = conn
        .query(
            "SELECT id, sku, title, price, description, stock
             FROM products WHERE sku = :sku",
            &params![("sku", "SKU-RUST-001")],
        )
        .await?;
    assert_eq!(rows.len(), 1);

    let row = &rows[0];
    assert_eq!(row.get::<i64>("id")?, 1);
    assert_eq!(row.get::<String>("sku")?, "SKU-RUST-001");
    assert_eq!(row.get::<String>("title")?, "Rust Programming Guide");
    assert!((row.get::<f64>("price")? - 49.99).abs() < f64::EPSILON);
    assert_eq!(
        row.get_opt::<String>("description")?,
        Some("Comprehensive guide to safe systems programming".to_string())
    );
    assert_eq!(row.get::<i64>("stock")?, 100);

    // 5. Verification of Absent vs NULL distinction:
    // Case A: Querying a column that was NOT selected in the query
    let missing_err = row.get::<String>("non_existent_column").unwrap_err();
    assert!(
        matches!(missing_err, Error::ColumnNotFound { .. }),
        "Expected ColumnNotFound, got {missing_err:?}"
    );
    let missing_opt_err = row.get_opt::<String>("non_existent_column").unwrap_err();
    assert!(
        matches!(missing_opt_err, Error::ColumnNotFound { .. }),
        "Expected ColumnNotFound for get_opt, got {missing_opt_err:?}"
    );

    // Case B: Querying product 2 where description is SQL NULL
    let rows_null = conn
        .query(
            "SELECT id, description FROM products WHERE id = :id",
            &params![("id", 2i64)],
        )
        .await?;
    assert_eq!(rows_null.len(), 1);
    let row_null = &rows_null[0];

    // Column is present: get_opt returns Ok(None)
    assert_eq!(row_null.get_opt::<String>("description")?, None);

    // Column is present: get (non-optional) returns Err(NullValue)
    let null_err = row_null.get::<String>("description").unwrap_err();
    assert!(
        matches!(null_err, Error::NullValue { .. }),
        "Expected NullValue, got {null_err:?}"
    );

    // 6. Transactions: Explicit Commit
    {
        let mut tx = conn.begin_transaction().await?;
        tx.execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 10i64),
                ("sku", "SKU-TX-001"),
                ("title", "Committed Transaction Item"),
                ("price", 15.0f64),
                ("desc", "Should persist"),
                ("stock", 20i64),
            ],
        )
        .await?;
        tx.commit().await?;
    }

    // Verify item 10 persisted
    let check = conn
        .query(
            "SELECT id FROM products WHERE id = :id",
            &params![("id", 10i64)],
        )
        .await?;
    assert_eq!(check.len(), 1);

    // 7. Transactions: Explicit Rollback
    {
        let mut tx = conn.begin_transaction().await?;
        tx.execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 20i64),
                ("sku", "SKU-TX-002"),
                ("title", "Rollback Item"),
                ("price", 25.0f64),
                ("desc", "Should not persist"),
                ("stock", 10i64),
            ],
        )
        .await?;
        tx.rollback().await?;
    }

    // Verify item 20 does NOT exist
    let check = conn
        .query(
            "SELECT id FROM products WHERE id = :id",
            &params![("id", 20i64)],
        )
        .await?;
    assert_eq!(check.len(), 0);

    // 8. Transactions: Automatic Rollback on Drop (Scope Exit without commit/rollback)
    {
        let mut tx = conn.begin_transaction().await?;
        tx.execute(
            "INSERT INTO products (id, sku, title, price, description, stock)
             VALUES (:id, :sku, :title, :price, :desc, :stock)",
            &params![
                ("id", 30i64),
                ("sku", "SKU-TX-003"),
                ("title", "Dropped Transaction Item"),
                ("price", 35.0f64),
                ("desc", "Should automatically rollback on drop"),
                ("stock", 5i64),
            ],
        )
        .await?;
        // Transaction dropped here without calling commit() or rollback()!
    }

    // Small yield to let any background async drop task finalize
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Verify item 30 was rolled back automatically and does NOT exist
    let check = conn
        .query(
            "SELECT id FROM products WHERE id = :id",
            &params![("id", 30i64)],
        )
        .await?;
    assert_eq!(check.len(), 0);

    Ok(())
}

#[cfg(feature = "rusqlite")]
#[tokio::test]
async fn test_identical_code_against_rusqlite() {
    // Instanciação: configurada para usar o engine rusqlite
    let options = ConnectOptions::new(":memory:").with_engine("rusqlite");
    let mut conn = portadb::connect(&options).await.expect("connect rusqlite");

    assert_eq!(conn.engine_name(), "rusqlite");

    // Executa a regra de negócio com a abstração UNICA DbConnection
    run_complete_business_flow(&mut conn)
        .await
        .expect("rusqlite execution succeeded");

    // Escape hatch test: Downcasting to concrete driver backend
    {
        let downcasted = conn.downcast_raw::<portadb::drivers::RusqliteConnection>();
        assert!(
            downcasted.is_ok(),
            "Downcast to RusqliteConnection must succeed"
        );

        let lock = downcasted.unwrap().raw_lock();
        assert!(
            lock.is_ok(),
            "Raw lock on rusqlite::Connection must succeed"
        );
    }

    conn.close().await.expect("close rusqlite");
}

#[cfg(feature = "sqlx-sqlite")]
#[tokio::test]
async fn test_identical_code_against_sqlx() {
    // Instanciação: configurada para usar o engine sqlx (MUDOU SOMENTE AQUI!)
    let options = ConnectOptions::new("sqlite::memory:").with_engine("sqlx");
    let mut conn = portadb::connect(&options).await.expect("connect sqlx");

    assert_eq!(conn.engine_name(), "sqlx");

    // Executa EXATAMENTE a mesma regra de negócio com a MESMA abstração UNICA DbConnection!
    run_complete_business_flow(&mut conn)
        .await
        .expect("sqlx execution succeeded");

    // Escape hatch test: Downcasting to concrete driver backend
    {
        let downcasted = conn.downcast_raw::<portadb::drivers::SqlxConnection>();
        assert!(
            downcasted.is_ok(),
            "Downcast to SqlxConnection must succeed"
        );

        // Downcast to wrong driver must fail cleanly with DowncastFailed
        let wrong_downcast = conn.downcast_raw::<String>();
        assert!(matches!(wrong_downcast, Err(Error::DowncastFailed)));
    }

    conn.close().await.expect("close sqlx");
}
