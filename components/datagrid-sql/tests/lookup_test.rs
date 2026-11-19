//! Integration tests for SqlLookup.

use std::sync::atomic::{AtomicU64, Ordering};

use datagrid::LookupSource;
use datagrid::filter::Owned;
use datagrid_sql::SqlLookup;
use portadb::params::NamedParams;
use portadb::value::Value;

static DB_COUNTER: AtomicU64 = AtomicU64::new(1);

struct TempDb {
    path: std::path::PathBuf,
    url: String,
}

impl TempDb {
    fn new() -> Self {
        let n = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
        let path =
            std::env::temp_dir().join(format!("test_lookup_{}_{}.db", std::process::id(), n));
        let url = path.to_string_lossy().into_owned();
        Self { path, url }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_file(format!("{}-wal", self.path.display()));
        let _ = std::fs::remove_file(format!("{}-shm", self.path.display()));
    }
}

fn seed_customers_table(lookup: &SqlLookup) {
    let mut params = NamedParams::new();
    lookup
        .execute(
            "CREATE TABLE customers (id INTEGER PRIMARY KEY, name TEXT NOT NULL, city TEXT NOT NULL)",
            &params,
        )
        .expect("table creation");

    let rows = [
        (1, "Alice", "New York"),
        (2, "Bob", "London"),
        (3, "Charlie", "Paris"),
        (4, "David", "Tokyo"),
        (5, "O'Connor", "Dublin"),
    ];

    for (id, name, city) in rows {
        params = NamedParams::new();
        params.insert("id", Value::Integer(id));
        params.insert("name", Value::Text(name.to_string()));
        params.insert("city", Value::Text(city.to_string()));
        lookup
            .execute(
                "INSERT INTO customers (id, name, city) VALUES (:id, :name, :city)",
                &params,
            )
            .expect("insert row");
    }
}

#[test]
fn test_lookup_labels() {
    let db = TempDb::new();
    let lookup = SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name", "city"])
        .unwrap_or_else(|_| {
            // Create dummy file or initial open
            SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name", "city"]).unwrap()
        });
    seed_customers_table(&lookup);

    let keys = vec![Owned::Int(1), Owned::Int(3), Owned::Int(99)];
    let labels = lookup.labels(&keys).expect("labels query");

    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].0, Owned::Int(1));
    assert_eq!(
        labels[0].1,
        vec!["Alice".to_string(), "New York".to_string()]
    );
    assert_eq!(labels[1].0, Owned::Int(3));
    assert_eq!(
        labels[1].1,
        vec!["Charlie".to_string(), "Paris".to_string()]
    );
}

#[test]
fn test_lookup_search_empty_prefix_and_limit() {
    let db = TempDb::new();
    let lookup =
        SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name", "city"]).unwrap();
    seed_customers_table(&lookup);

    // Empty prefix returns first `limit` rows ordered by d1 ("name")
    let results = lookup.search("", 3).expect("search query");
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].1[0], "Alice");
    assert_eq!(results[1].1[0], "Bob");
    assert_eq!(results[2].1[0], "Charlie");

    let results_limit_2 = lookup.search("", 2).expect("search limit");
    assert_eq!(results_limit_2.len(), 2);
}

#[test]
fn test_lookup_search_like_prefix() {
    let db = TempDb::new();
    let lookup = SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name"]).unwrap();
    seed_customers_table(&lookup);

    let results = lookup.search("Ch", 10).expect("search query");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, Owned::Int(3));
    assert_eq!(results[0].1[0], "Charlie");
}

#[test]
fn test_lookup_value_with_quote() {
    let db = TempDb::new();
    let lookup =
        SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name", "city"]).unwrap();
    seed_customers_table(&lookup);

    // Search with single quote
    let results = lookup.search("O'C", 10).expect("search quote");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, Owned::Int(5));
    assert_eq!(results[0].1[0], "O'Connor");
    assert_eq!(results[0].1[1], "Dublin");

    // Labels with id of row containing quote
    let labels = lookup.labels(&[Owned::Int(5)]).expect("labels quote");
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0].1[0], "O'Connor");
}

#[test]
fn test_lookup_sql_source() {
    let db = TempDb::new();
    let temp_lookup = SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name"]).unwrap();
    seed_customers_table(&temp_lookup);

    // Read-only SQL sub-select
    let sql_lookup = SqlLookup::open_sqlite(
        &db.url,
        "",
        "SELECT id, name, city FROM customers WHERE id > 1",
        "id",
        vec!["name", "city"],
    )
    .unwrap();

    let results = sql_lookup.search("", 10).expect("sql search");
    assert_eq!(results.len(), 4); // IDs 2, 3, 4, 5
    assert!(!results.iter().any(|r| r.1[0] == "Alice"));
}

#[test]
fn test_lookup_search_treats_percent_and_underscore_as_text() {
    let db = TempDb::new();
    let lookup = SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name"]).unwrap();
    seed_customers_table(&lookup);

    assert!(lookup.search("%", 10).expect("search query").is_empty());
    assert!(lookup.search("_lice", 10).expect("search query").is_empty());
}

#[test]
fn a_lookup_connects_only_when_first_used() {
    let db = TempDb::new();
    let lookup = SqlLookup::open_sqlite(&db.url, "customers", "", "id", vec!["name"]).unwrap();
    assert!(
        !lookup.is_connected(),
        "opening the lookup must not touch the database"
    );
    seed_customers_table(&lookup);
    assert!(lookup.is_connected());
}
