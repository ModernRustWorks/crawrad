//! `SqlSource::open_sqlite_table`: a table opened by name, its columns read from the database.

use std::sync::atomic::{AtomicU64, Ordering};

use datagrid::query::Query;
use datagrid::source::{ColumnKind, RowSource, SourceError};
use datagrid_sql::{SqlSource, sqlite_kind};
use portadb::params::NamedParams;

static DB_COUNTER: AtomicU64 = AtomicU64::new(1);

/// A shared in-memory database holding `orders` with three rows; the returned
/// source keeps it alive for the test (shared cache lives while one connection does).
fn orders_db() -> (String, SqlSource) {
    let id = DB_COUNTER.fetch_add(1, Ordering::Relaxed);
    let url = format!("file:datagrid_sql_open_table_{id}?mode=memory&cache=shared");
    let keeper = SqlSource::open_sqlite(&url, datagrid_sql::Table::sqlite("none", Vec::new()))
        .expect("open the database");
    keeper
        .execute(
            "CREATE TABLE orders (id INTEGER NOT NULL, customer TEXT, total REAL, paid BOOLEAN);",
            &NamedParams::new(),
        )
        .expect("CREATE TABLE");
    for (id, customer, total) in [(1, "Acme", 10.5), (2, "Beta", 20.0), (3, "Gamma", 7.25)] {
        let mut p = NamedParams::new();
        p.insert("id", id as i64);
        p.insert("customer", customer.to_string());
        p.insert("total", total);
        keeper
            .execute(
                "INSERT INTO orders (id, customer, total, paid) VALUES (:id, :customer, :total, 1);",
                &p,
            )
            .expect("INSERT");
    }
    (url, keeper)
}

#[test]
fn the_columns_come_from_the_database() {
    let (url, _keeper) = orders_db();
    let source = SqlSource::open_sqlite_table(&url, "orders").expect("open orders");
    let schema = source.schema();
    let names: Vec<&str> = schema.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["id", "customer", "total", "paid"]);
    let kinds: Vec<ColumnKind> = schema.iter().map(|c| c.kind).collect();
    assert_eq!(
        kinds,
        [
            ColumnKind::Int,
            ColumnKind::Text,
            ColumnKind::Float,
            ColumnKind::Bool
        ]
    );
    assert!(!schema[0].nullable, "id is NOT NULL");
    assert!(schema[1].nullable, "customer may be NULL");

    let query = Query::default();
    assert_eq!(source.count(&query).expect("count"), 3);
    let rows = source.fetch(&query, 0..3).expect("fetch");
    assert_eq!(rows.ids.len(), 3);
}

#[test]
fn an_unknown_table_is_an_error() {
    let (url, _keeper) = orders_db();
    match SqlSource::open_sqlite_table(&url, "invoices") {
        Err(SourceError::Backend(m)) => assert!(m.contains("table not found"), "{m}"),
        Err(other) => panic!("unexpected error {other:?}"),
        Ok(_) => panic!("a missing table must not open"),
    }
}

#[test]
fn a_name_that_is_not_an_identifier_never_reaches_sql() {
    let (url, keeper) = orders_db();
    for bad in [
        "orders; DROP TABLE orders",
        "orders\")",
        "",
        "1orders",
        "or ders",
    ] {
        match SqlSource::open_sqlite_table(&url, bad) {
            Err(SourceError::Backend(m)) => assert!(m.contains("invalid table name"), "{m}"),
            Err(other) => panic!("unexpected error {other:?}"),
            Ok(_) => panic!("{bad:?} must be refused"),
        }
    }
    // The table is still there, with its rows.
    let source = SqlSource::open_sqlite_table(&url, "orders").expect("open orders");
    assert_eq!(source.count(&Query::default()).expect("count"), 3);
    drop(keeper);
}

#[test]
fn sqlite_declared_types_map_to_grid_kinds() {
    assert_eq!(sqlite_kind("INTEGER"), ColumnKind::Int);
    assert_eq!(sqlite_kind("bigint"), ColumnKind::Int);
    assert_eq!(sqlite_kind("VARCHAR(40)"), ColumnKind::Text);
    assert_eq!(sqlite_kind("TEXT"), ColumnKind::Text);
    assert_eq!(sqlite_kind("CLOB"), ColumnKind::Text);
    assert_eq!(sqlite_kind("REAL"), ColumnKind::Float);
    assert_eq!(sqlite_kind("FLOAT"), ColumnKind::Float);
    assert_eq!(sqlite_kind("DOUBLE PRECISION"), ColumnKind::Float);
    assert_eq!(sqlite_kind("BOOLEAN"), ColumnKind::Bool);
    assert_eq!(sqlite_kind("DECIMAL(10,2)"), ColumnKind::Text);
    assert_eq!(sqlite_kind(""), ColumnKind::Text);
}

/// A Dataset's `sql`: the source shows the query's columns and rows in the
/// query's order, counts them, and refuses to write (a query is read-only).
#[test]
fn a_query_opens_read_only_in_its_own_order() {
    let (url, _keeper) = orders_db();
    let source = SqlSource::open_sqlite_query(
        &url,
        " SELECT customer, total FROM orders ORDER BY total DESC; ",
    )
    .expect("open the query");
    let names: Vec<String> = source.schema().iter().map(|c| c.name.clone()).collect();
    assert_eq!(names, ["customer", "total"]);
    let query = Query::default();
    assert_eq!(source.count(&query).expect("count"), 3);
    let rows = source.fetch(&query, 0..3).expect("fetch");
    let customers: Vec<String> = (0..3)
        .map(|i| match rows.columns[0].get(i) {
            datagrid::column::Value::Text(t) => t.to_string(),
            other => panic!("not text: {other:?}"),
        })
        .collect();
    assert_eq!(customers, ["Beta", "Acme", "Gamma"]);
    assert!(!source.can_write(), "a query is read-only");
    assert!(source.update(rows.ids[0], &[]).is_err());
    assert!(SqlSource::open_sqlite_query(&url, "  ").is_err());
    // the database itself was not touched: still one table, three rows
    let table = SqlSource::open_sqlite_table(&url, "orders").expect("open orders");
    assert_eq!(table.count(&query).expect("count"), 3);
}

/// A SQLite file that is not there is reported with its full path, and the
/// read never creates it (an empty database would only say "no such table").
#[test]
fn a_missing_database_file_is_reported_and_never_created() {
    let path = std::env::temp_dir().join(format!("datagrid_sql_missing_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let url = format!("sqlite:{}", path.to_string_lossy());
    for opened in [
        SqlSource::open_sqlite_table(&url, "orders").map(|_| ()),
        SqlSource::open_sqlite_query(&url, "SELECT 1").map(|_| ()),
    ] {
        match opened {
            Err(SourceError::Backend(m)) => {
                assert!(m.starts_with("database file not found: "), "{m}");
                assert!(m.contains(&*path.to_string_lossy()), "{m}");
            }
            other => panic!("a missing file must not open: {:?}", other.is_ok()),
        }
    }
    assert!(!path.exists(), "the read created the file");
}
