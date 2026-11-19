mod tests {
    use std::sync::{Arc, Mutex};

    use portadb::{
        ConnectOptions, Dataset, DatasetEvent, DatasetSource, DatasetState, Engine, Error,
        TableEvent, Value, params,
    };

    async fn setup_people_db(db_name: &str) -> Engine {
        let options =
            ConnectOptions::new(format!("sqlite:file:{db_name}?mode=memory&cache=shared"));
        let engine = Engine::connect(&options).await.expect("connect to sqlite");

        engine
            .execute(
                "CREATE TABLE ds_people (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER, active BOOLEAN);",
                &params![],
            )
            .await
            .expect("create ds_people table");

        // Insert rows in reverse id order to verify that Dataset::open orders rows by primary key
        engine
            .execute(
                "INSERT INTO ds_people (id, name, age, active) VALUES \
                 (3, 'Charlie', 40, 1), \
                 (1, 'Alice', 30, 1), \
                 (2, 'Bob', 25, 0);",
                &params![],
            )
            .await
            .expect("seed ds_people rows");

        engine
    }

    #[tokio::test]
    async fn test_dataset_open() {
        let engine = setup_people_db("ds_open_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        // 3 rows, ordered by id
        assert_eq!(dataset.len(), 3);
        assert_eq!(dataset.position(), Some(0));

        // Schema primary key is id
        let schema = dataset.schema();
        let pk = schema.primary_key();
        assert_eq!(pk.len(), 1);
        assert_eq!(pk[0].name, "id");
        assert!(pk[0].primary_key);

        // First row is id 1
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(1));
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Alice".to_string())
        );

        // Assert database state with fresh query
        let db_rows = engine
            .query("SELECT id, name FROM ds_people ORDER BY id", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 3);
        assert_eq!(db_rows[0].get::<i64>("id").unwrap(), 1);
        assert_eq!(db_rows[1].get::<i64>("id").unwrap(), 2);
        assert_eq!(db_rows[2].get::<i64>("id").unwrap(), 3);
    }

    #[tokio::test]
    async fn test_dataset_navigation_and_view_sharing() {
        let engine = setup_people_db("ds_nav_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        assert_eq!(dataset.position(), Some(0));

        // Next
        assert!(dataset.next().unwrap());
        assert_eq!(dataset.position(), Some(1));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(2));

        // Last
        assert!(dataset.last().unwrap());
        assert_eq!(dataset.position(), Some(2));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(3));

        // Prior
        assert!(dataset.prior().unwrap());
        assert_eq!(dataset.position(), Some(1));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(2));

        // First
        assert!(dataset.first().unwrap());
        assert_eq!(dataset.position(), Some(0));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(1));

        // View shares cursor
        let view = dataset.view();
        assert_eq!(view.position(), Some(0));

        // Move the view, dataset sees it
        assert!(view.move_to(2));
        assert_eq!(dataset.position(), Some(2));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(3));

        // Move the dataset, view sees it
        assert!(dataset.move_to(1).unwrap());
        assert_eq!(view.position(), Some(1));

        // Assert database rows
        let db_rows = engine
            .query("SELECT COUNT(*) AS c FROM ds_people", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows[0].get::<i64>("c").unwrap(), 3);
    }

    #[tokio::test]
    async fn test_dataset_edit_set_field_post() {
        let engine = setup_people_db("ds_edit_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        let recorded_events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&recorded_events);
        dataset.view().subscribe(move |signal| {
            events_clone.lock().unwrap().push(signal.kind);
        });

        // Position on row 1 (Bob, id 2)
        dataset.move_to(1).unwrap();
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Bob".to_string())
        );

        // Enter Edit mode
        dataset.edit().expect("enter edit");
        assert_eq!(dataset.state(), DatasetState::Edit);

        dataset
            .set_field("name", "Bob Modified")
            .expect("set name field");
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Bob Modified".to_string())
        );

        // Post the change
        dataset.post().await.expect("post update");
        assert_eq!(dataset.state(), DatasetState::Browse);

        // field reads new value
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Bob Modified".to_string())
        );

        // View's row changed
        let cur_row = dataset.view().current_row().expect("current row");
        assert_eq!(
            cur_row.get_value("name").unwrap(),
            &Value::Text("Bob Modified".to_string())
        );

        // RowChanged event emitted for index 1
        let events = recorded_events.lock().unwrap().clone();
        assert!(
            events
                .iter()
                .any(|ev| matches!(ev, TableEvent::RowChanged { index: 1 })),
            "expected TableEvent::RowChanged {{ index: 1 }}"
        );

        // Assert database: only row 2 changed, others did not
        let db_rows = engine
            .query(
                "SELECT id, name, age, active FROM ds_people ORDER BY id",
                &params![],
            )
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 3);
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Alice");
        assert_eq!(db_rows[1].get::<String>("name").unwrap(), "Bob Modified");
        assert_eq!(db_rows[2].get::<String>("name").unwrap(), "Charlie");
    }

    #[tokio::test]
    async fn test_dataset_cancel_restores() {
        let engine = setup_people_db("ds_cancel_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.move_to(0).unwrap();
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Alice".to_string())
        );

        dataset.edit().expect("edit");
        dataset
            .set_field("name", "Alice Modified")
            .expect("set field");
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Alice Modified".to_string())
        );

        dataset.cancel();
        assert_eq!(dataset.state(), DatasetState::Browse);
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Alice".to_string())
        );

        // Assert database untouched
        let db_rows = engine
            .query("SELECT name FROM ds_people WHERE id = 1", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Alice");
    }

    #[tokio::test]
    async fn test_dataset_insert_and_post_auto_id() {
        let engine = setup_people_db("ds_insert_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.insert().expect("insert mode");
        assert_eq!(dataset.state(), DatasetState::Insert);
        assert_eq!(dataset.field("id").unwrap(), Value::Null);

        dataset.set_field("name", "Dave").expect("set name");
        dataset.set_field("age", 28).expect("set age");
        dataset.set_field("active", true).expect("set active");

        dataset.post().await.expect("post insert");
        assert_eq!(dataset.state(), DatasetState::Browse);

        // Database assigned an id (4), new row is in the view with that id, cursor on it
        assert_eq!(dataset.len(), 4);
        assert_eq!(dataset.position(), Some(3));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(4));
        assert_eq!(
            dataset.field("name").unwrap(),
            Value::Text("Dave".to_string())
        );

        // Assert database with fresh query
        let db_rows = engine
            .query(
                "SELECT id, name, age FROM ds_people WHERE id = 4",
                &params![],
            )
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 1);
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Dave");
        assert_eq!(db_rows[0].get::<i64>("age").unwrap(), 28);
    }

    #[tokio::test]
    async fn test_dataset_delete_and_cursor_rule() {
        let engine = setup_people_db("ds_delete_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        // Move to middle row (Bob, id 2, index 1)
        dataset.move_to(1).unwrap();
        dataset.delete().await.expect("delete row");

        // Removed from view; cursor stays at index 1 (which is now Charlie, id 3)
        assert_eq!(dataset.len(), 2);
        assert_eq!(dataset.position(), Some(1));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(3));

        // Assert database
        let db_rows = engine
            .query("SELECT id, name FROM ds_people ORDER BY id", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 2);
        assert_eq!(db_rows[0].get::<i64>("id").unwrap(), 1);
        assert_eq!(db_rows[1].get::<i64>("id").unwrap(), 3);

        // Delete last row (index 1, Charlie)
        dataset.delete().await.expect("delete last row");
        assert_eq!(dataset.len(), 1);
        assert_eq!(dataset.position(), Some(0));
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(1));

        // Delete remaining row (index 0, Alice)
        dataset.delete().await.expect("delete only row");
        assert_eq!(dataset.len(), 0);
        assert_eq!(dataset.position(), None);

        // Assert database is empty
        let db_count = engine
            .query("SELECT COUNT(*) AS c FROM ds_people", &params![])
            .await
            .expect("count");
        assert_eq!(db_count[0].get::<i64>("c").unwrap(), 0);
    }

    #[tokio::test]
    async fn test_dataset_typed_set_field() {
        let engine = setup_people_db("ds_typed_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.edit().expect("edit");

        // 1. Text into Integer -> TypeMismatch
        let err1 = dataset.set_field("age", "abc").unwrap_err();
        assert!(
            matches!(&err1, Error::TypeMismatch { column, expected, .. } if column == "age" && *expected == "Integer"),
            "expected TypeMismatch for age with string value, got {err1:?}"
        );

        // 2. Null into NOT NULL Text -> NullValue
        let err2 = dataset.set_field("name", Value::Null).unwrap_err();
        assert!(
            matches!(&err2, Error::NullValue { column } if column == "name"),
            "expected NullValue for non-nullable name, got {err2:?}"
        );

        // 3. Real into Integer -> TypeMismatch
        let err3 = dataset.set_field("age", 3.5).unwrap_err();
        assert!(
            matches!(&err3, Error::TypeMismatch { column, expected, .. } if column == "age" && *expected == "Integer"),
            "expected TypeMismatch for age with float value, got {err3:?}"
        );

        dataset.cancel();

        // Assert database is unchanged
        let db_rows = engine
            .query("SELECT age, name FROM ds_people WHERE id = 1", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows[0].get::<i64>("age").unwrap(), 30);
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Alice");
    }

    #[tokio::test]
    async fn test_dataset_refusals() {
        let engine = setup_people_db("ds_refusals_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        // Navigation refused while editing
        dataset.edit().expect("edit");
        assert!(dataset.next().is_err());
        assert!(dataset.prior().is_err());
        assert!(dataset.first().is_err());
        assert!(dataset.last().is_err());
        assert!(dataset.move_to(2).is_err());
        dataset.cancel();

        // Edit refused on Sql dataset
        let sql_ds = Dataset::open(
            &engine,
            DatasetSource::Sql("SELECT id, name FROM ds_people".to_string()),
        )
        .await
        .expect("open sql dataset");
        assert!(sql_ds.edit().is_err());
        assert!(sql_ds.insert().is_err());

        // Post refused on table without primary key
        engine
            .execute("CREATE TABLE ds_no_pk (info TEXT);", &params![])
            .await
            .expect("create table without pk");
        let no_pk_ds = Dataset::open(&engine, DatasetSource::Table("ds_no_pk".to_string()))
            .await
            .expect("open no pk dataset");
        no_pk_ds.insert().expect("insert on no pk");
        no_pk_ds.set_field("info", "test").expect("set info");
        let post_err = no_pk_ds.post().await.unwrap_err();
        assert!(
            matches!(&post_err, Error::ConfigurationError(_)),
            "expected ConfigurationError naming the table, got {post_err:?}"
        );
        assert!(
            post_err.to_string().contains("ds_no_pk"),
            "error should name table: {post_err}"
        );

        // Assert database for ds_no_pk is empty
        let db_rows = engine
            .query("SELECT COUNT(*) AS c FROM ds_no_pk", &params![])
            .await
            .expect("query no pk");
        assert_eq!(db_rows[0].get::<i64>("c").unwrap(), 0);
    }

    #[tokio::test]
    async fn test_dataset_stale_edit() {
        let engine = setup_people_db("ds_stale_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.move_to(0).unwrap(); // id 1, Alice
        dataset.edit().expect("edit");

        // Change the row's key behind the dataset's back
        engine
            .execute("UPDATE ds_people SET id = 99 WHERE id = 1;", &params![])
            .await
            .expect("update behind dataset's back");

        dataset
            .set_field("name", "Alice Stale")
            .expect("set name field");

        // Post fails because 0 rows affected
        let post_err = dataset.post().await.unwrap_err();
        assert!(
            matches!(post_err, Error::QueryError(_)),
            "expected QueryError, got {post_err:?}"
        );

        // Assert database: row 99 still has original name 'Alice'
        let db_rows = engine
            .query("SELECT name FROM ds_people WHERE id = 99", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 1);
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Alice");
    }

    #[tokio::test]
    async fn test_dataset_state_changed_events() {
        let engine = setup_people_db("ds_events_test").await;

        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        let recorded_state_transitions = Arc::new(Mutex::new(Vec::new()));
        let record_clone = Arc::clone(&recorded_state_transitions);

        dataset.subscribe(move |event| {
            if let DatasetEvent::StateChanged { previous, current } = event {
                record_clone.lock().unwrap().push((previous, current));
            }
        });

        // Browse -> Edit -> Browse
        dataset.edit().expect("edit");
        dataset.cancel();

        let transitions = recorded_state_transitions.lock().unwrap().clone();
        assert_eq!(
            transitions,
            vec![
                (DatasetState::Browse, DatasetState::Edit),
                (DatasetState::Edit, DatasetState::Browse)
            ]
        );

        // Assert database
        let db_count = engine
            .query("SELECT COUNT(*) AS c FROM ds_people", &params![])
            .await
            .expect("count");
        assert_eq!(db_count[0].get::<i64>("c").unwrap(), 3);
    }

    /// The UPDATE writes only what the user changed: a column changed by someone
    /// else since the dataset read the row survives the post (no lost update).
    #[tokio::test]
    async fn test_dataset_post_writes_only_changed_columns() {
        let engine = setup_people_db("ds_changed_only_test").await;
        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.edit().expect("enter edit");
        dataset.set_field("name", "Alicia").expect("set name");
        engine
            .execute("UPDATE ds_people SET age = 77 WHERE id = 1", &params![])
            .await
            .expect("change age behind the dataset's back");
        dataset.post().await.expect("post update");

        let db_rows = engine
            .query("SELECT name, age FROM ds_people WHERE id = 1", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows[0].get::<String>("name").unwrap(), "Alicia");
        assert_eq!(db_rows[0].get::<i64>("age").unwrap(), 77);
    }

    /// Editing the key itself: the row is found by its ORIGINAL key and moves
    /// to the new one.
    #[tokio::test]
    async fn test_dataset_post_finds_the_row_by_its_original_key() {
        let engine = setup_people_db("ds_key_edit_test").await;
        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_people".to_string()))
            .await
            .expect("open dataset");

        dataset.edit().expect("enter edit");
        dataset.set_field("id", 50).expect("set id");
        dataset.post().await.expect("post key change");

        let db_rows = engine
            .query("SELECT id FROM ds_people WHERE name = 'Alice'", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows.len(), 1);
        assert_eq!(db_rows[0].get::<i64>("id").unwrap(), 50);
        assert_eq!(dataset.field("id").unwrap(), Value::Integer(50));
    }

    /// An integer widens into a `REAL` column, and what reaches the database is
    /// the real number.
    #[tokio::test]
    async fn test_dataset_integer_widens_into_real() {
        let options = ConnectOptions::new("sqlite:file:ds_widen_test?mode=memory&cache=shared");
        let engine = Engine::connect(&options).await.expect("connect to sqlite");
        engine
            .execute(
                "CREATE TABLE ds_prices (id INTEGER PRIMARY KEY, price REAL)",
                &params![],
            )
            .await
            .expect("create ds_prices");
        engine
            .execute(
                "INSERT INTO ds_prices (id, price) VALUES (1, 2.5)",
                &params![],
            )
            .await
            .expect("seed ds_prices");
        let dataset = Dataset::open(&engine, DatasetSource::Table("ds_prices".to_string()))
            .await
            .expect("open dataset");

        dataset.edit().expect("enter edit");
        dataset.set_field("price", 7).expect("an integer into REAL");
        assert_eq!(dataset.field("price").unwrap(), Value::Real(7.0));
        dataset.post().await.expect("post");

        let db_rows = engine
            .query("SELECT price FROM ds_prices WHERE id = 1", &params![])
            .await
            .expect("query db");
        assert_eq!(db_rows[0].get::<f64>("price").unwrap(), 7.0);
    }
}
