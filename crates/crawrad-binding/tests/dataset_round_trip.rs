//! Integration tests verifying end-to-end round-trip synchronization between
//! [`DatasetAdapter`] and a real SQLite database via `portadb::Dataset`.
//!
//! Behind the crate's `rusqlite` feature: `cargo test -p crawrad-binding --features rusqlite`.
#![cfg(feature = "rusqlite")]

use crawrad_binding::adapter::{DatasetAdapter, DatasetState, PortaDbBridge};
use portadb::{ConnectOptions, Dataset, DatasetSource, Engine, NamedParams};

fn setup_test_dataset(db_name: &str) -> (Engine, DatasetAdapter) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime for test setup");

    let (engine, dataset) = rt.block_on(async {
        let options =
            ConnectOptions::new(format!("sqlite:file:{db_name}?mode=memory&cache=shared"));
        let engine = Engine::connect(&options).await.expect("connect to sqlite");

        engine
            .execute(
                "CREATE TABLE people (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER);",
                &NamedParams::new(),
            )
            .await
            .expect("create people table");

        engine
            .execute(
                "INSERT INTO people (id, name, age) VALUES (1, 'Alice', 30), (2, 'Bob', 25);",
                &NamedParams::new(),
            )
            .await
            .expect("seed people table");

        let ds = Dataset::open(&engine, DatasetSource::Table("people".to_string()))
            .await
            .expect("open dataset");

        (engine, ds)
    });

    let adapter = PortaDbBridge::from_dataset(&dataset);
    (engine, adapter)
}

fn query_people_db(engine: &Engine) -> Vec<(i64, String, Option<i64>)> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime for test query");

    rt.block_on(async {
        let rows = engine
            .query(
                "SELECT id, name, age FROM people ORDER BY id",
                &NamedParams::new(),
            )
            .await
            .expect("query people table");

        rows.into_iter()
            .map(|r| {
                let id: i64 = r.get("id").expect("get id");
                let name: String = r.get("name").expect("get name");
                let age: Option<i64> = r.get_opt("age").expect("get age");
                (id, name, age)
            })
            .collect()
    })
}

#[test]
fn test_round_trip_read() {
    let (_engine, adapter) = setup_test_dataset("rt_read");

    let name_field = adapter.field("name").expect("name field");
    assert_eq!(name_field.get(), "Alice");
    assert_eq!(adapter.collection().cursor(), Some(0));

    adapter.collection().next();
    assert_eq!(name_field.get(), "Bob");
    assert_eq!(adapter.collection().cursor(), Some(1));
}

#[test]
fn test_round_trip_write() {
    let (engine, adapter) = setup_test_dataset("rt_write");

    let name_field = adapter.field("name").expect("name field");
    name_field.set("Alice Cooper".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);

    adapter.post().expect("post edit");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let db_rows = query_people_db(&engine);
    assert_eq!(db_rows[0].1, "Alice Cooper");
    assert_eq!(
        adapter
            .collection()
            .current_item()
            .unwrap()
            .get("name")
            .unwrap(),
        "Alice Cooper"
    );
}

#[test]
fn test_round_trip_typed_validation() {
    let (engine, adapter) = setup_test_dataset("rt_typed");

    let age_field = adapter.field("age").expect("age field");
    age_field.set("abc".to_string());

    let err = adapter.last_error().get();
    assert!(err.is_some(), "expected error for non-integer age");
    let err_msg = err.unwrap();
    assert!(
        err_msg.contains("age"),
        "error message should mention column 'age': {err_msg}"
    );

    let db_rows_before = query_people_db(&engine);
    assert_eq!(db_rows_before[0].2, Some(30));

    age_field.set("41".to_string());
    assert_eq!(adapter.last_error().get(), None);

    adapter.post().expect("post valid age");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let db_rows_after = query_people_db(&engine);
    assert_eq!(db_rows_after[0].2, Some(41));
}

#[test]
fn test_round_trip_insert() {
    let (engine, adapter) = setup_test_dataset("rt_insert");

    adapter.insert().expect("insert mode");
    assert_eq!(adapter.state().get(), DatasetState::Insert);

    let name_field = adapter.field("name").expect("name field");
    let age_field = adapter.field("age").expect("age field");

    name_field.set("Charlie".to_string());
    age_field.set("50".to_string());

    adapter.post().expect("post insert");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let db_rows = query_people_db(&engine);
    assert_eq!(db_rows.len(), 3);
    assert_eq!(db_rows[2].0, 3);
    assert_eq!(db_rows[2].1, "Charlie");
    assert_eq!(db_rows[2].2, Some(50));
    assert_eq!(adapter.collection().len(), 3);
}

#[test]
fn test_round_trip_delete() {
    let (engine, adapter) = setup_test_dataset("rt_delete");

    assert_eq!(adapter.collection().len(), 2);
    adapter.delete().expect("delete active record");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let db_rows = query_people_db(&engine);
    assert_eq!(db_rows.len(), 1);
    assert_eq!(db_rows[0].1, "Bob");
    assert_eq!(adapter.collection().len(), 1);
}

#[test]
fn test_round_trip_cancel() {
    let (engine, adapter) = setup_test_dataset("rt_cancel");

    let name_field = adapter.field("name").expect("name field");
    name_field.set("Alicia".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);

    adapter.cancel().expect("cancel edit");
    assert_eq!(adapter.state().get(), DatasetState::Browse);
    assert_eq!(name_field.get(), "Alice");

    let db_rows = query_people_db(&engine);
    assert_eq!(db_rows[0].1, "Alice");
}

#[test]
fn test_round_trip_navigation_while_editing_refused() {
    let (engine, adapter) = setup_test_dataset("rt_nav_refused");

    let name_field = adapter.field("name").expect("name field");
    name_field.set("Alice Editing".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);
    assert_eq!(adapter.collection().cursor(), Some(0));

    adapter.collection().next();
    assert_eq!(adapter.collection().cursor(), Some(0));
    assert!(adapter.last_error().get().is_some());

    let db_rows = query_people_db(&engine);
    assert_eq!(db_rows[0].1, "Alice");
    assert_eq!(db_rows[1].1, "Bob");
}

#[test]
fn test_round_trip_empty_cell_writes_null() {
    let (engine, adapter) = setup_test_dataset("rt_null");

    let age_field = adapter.field("age").expect("age field");
    age_field.set(String::new());
    assert!(adapter.last_error().get().is_none());
    adapter.post().expect("post the cleared age");

    let db_rows = query_people_db(&engine);
    assert_eq!(
        db_rows[0].2, None,
        "an emptied cell is NULL in the database"
    );
}

#[test]
fn test_open_table_round_trip() {
    let temp_file = std::env::temp_dir().join(format!(
        "crawrad_test_open_table_{}_{}.db",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _guard = Cleanup(temp_file.clone());
    let url = format!("sqlite:{}", temp_file.display());

    // 1. Setup the table via a direct Engine connection
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime for test setup");
    let engine = rt.block_on(async {
        let options = ConnectOptions::new(&url);
        let engine = Engine::connect(&options)
            .await
            .expect("connect to sqlite file");
        engine
            .execute(
                "CREATE TABLE items (id INTEGER PRIMARY KEY, title TEXT NOT NULL);",
                &NamedParams::new(),
            )
            .await
            .expect("create items table");
        engine
            .execute(
                "INSERT INTO items (id, title) VALUES (1, 'Initial');",
                &NamedParams::new(),
            )
            .await
            .expect("insert initial item");
        engine
    });

    // 2. Open via PortaDbBridge::open_table without async
    let adapter = PortaDbBridge::open_table(&url, "items").expect("open_table succeeds");

    // 3. Opens and shows row 0
    assert_eq!(adapter.collection().cursor(), Some(0));
    let title_field = adapter.field("title").expect("title field exists");
    assert_eq!(title_field.get(), "Initial");

    // 4. An edit + post lands in the file
    title_field.set("Modified Title".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);
    adapter.post().expect("post edit");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    // Verify a second connection sees the data in the file
    let rows = rt.block_on(async {
        engine
            .query("SELECT title FROM items WHERE id = 1", &NamedParams::new())
            .await
            .expect("query file")
    });
    assert_eq!(rows.len(), 1);
    let title: String = rows[0].get("title").expect("get title");
    assert_eq!(title, "Modified Title");

    // 5. Empty table name is an Err
    let err_empty_table = PortaDbBridge::open_table(&url, "");
    assert!(err_empty_table.is_err());
    let msg = err_empty_table.err().unwrap();
    // refused BEFORE connecting, by this check -- not by the driver's SQL error
    assert_eq!(msg, "table name cannot be empty");

    // Empty URL is an Err
    let err_empty_url = PortaDbBridge::open_table("", "items");
    assert!(err_empty_url.is_err());
    let msg_url = err_empty_url.err().unwrap();
    assert_eq!(msg_url, "URL cannot be empty");
}

#[test]
fn test_round_trip_boolean_words_on_an_integer_column() {
    let (engine, adapter) = setup_test_dataset("rt_bool_words");

    let age_field = adapter.field("age").expect("age field");
    age_field.set("true".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);
    adapter.post().expect("post true");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let rows = query_people_db(&engine);
    assert_eq!(rows[0].2, Some(1));

    age_field.set("FALSE".to_string());
    assert_eq!(adapter.state().get(), DatasetState::Edit);
    adapter.post().expect("post FALSE");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let rows_after = query_people_db(&engine);
    assert_eq!(rows_after[0].2, Some(0));
}

#[test]
fn test_round_trip_post_refuses_a_refused_cell() {
    let (engine, adapter) = setup_test_dataset("rt_post_refused");

    let age_field = adapter.field("age").expect("age field");
    age_field.set("abc".to_string());

    let post_res = adapter.post();
    assert!(
        post_res.is_err(),
        "post should fail when a cell holds refused text"
    );
    let err_msg = post_res.unwrap_err();
    assert!(
        err_msg.contains("age"),
        "error message should contain column name 'age': {err_msg}"
    );
    assert!(
        err_msg.contains("cannot post"),
        "error message should contain 'cannot post': {err_msg}"
    );

    assert!(
        adapter.last_error().get().is_some(),
        "adapter.last_error() should remain Some after failed post"
    );

    let rows_before = query_people_db(&engine);
    assert_eq!(rows_before[0].2, Some(30));

    adapter.cancel().expect("cancel edit");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    age_field.set("31".to_string());
    adapter.post().expect("post valid age");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let rows_after = query_people_db(&engine);
    assert_eq!(rows_after[0].2, Some(31));
}

#[test]
fn test_round_trip_a_corrected_cell_can_post() {
    let (engine, adapter) = setup_test_dataset("rt_corrected_cell");

    let age_field = adapter.field("age").expect("age field");
    age_field.set("abc".to_string());
    assert!(adapter.last_error().get().is_some());

    age_field.set("32".to_string());
    assert_eq!(adapter.last_error().get(), None);

    adapter.post().expect("post corrected age");
    assert_eq!(adapter.state().get(), DatasetState::Browse);

    let rows = query_people_db(&engine);
    assert_eq!(rows[0].2, Some(32));
}

#[test]
fn test_round_trip_cancel_forgets_a_refused_cell() {
    // The refused cell is never typed again: only the store's own refresh
    // (after Cancel) can tell the adapter that it shows the record's value again.
    let (engine, adapter) = setup_test_dataset("rt_cancel_refused");

    adapter
        .field("age")
        .expect("age field")
        .set("abc".to_string());
    adapter.cancel().expect("cancel");
    assert_eq!(adapter.field("age").expect("age field").get(), "30");

    adapter
        .field("name")
        .expect("name field")
        .set("Zed".to_string());
    adapter
        .post()
        .expect("post the other field once the refused cell is gone");

    let rows = query_people_db(&engine);
    assert_eq!(rows[0].1, "Zed");
    assert_eq!(rows[0].2, Some(30));
}

/// A dataset over a SQLite file that is not there says which file, with its
/// full path, and does not create an empty database.
#[test]
fn a_missing_database_file_is_reported_not_created() {
    let path =
        std::env::temp_dir().join(format!("crawrad_binding_missing_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let url = format!("sqlite:{}", path.to_string_lossy());
    let err = PortaDbBridge::open_table(&url, "contacts")
        .err()
        .expect("a missing file must not open");
    assert!(err.starts_with("database file not found: "), "{err}");
    assert!(!path.exists(), "the read created the file");
}
