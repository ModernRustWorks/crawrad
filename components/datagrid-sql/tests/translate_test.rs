use datagrid::filter::{Filter, Owned};
use datagrid::query::Query;
use datagrid::sort::{Nulls, Order, SortKey};
use datagrid::{ColumnInfo, ColumnKind, DisplayHint};
use datagrid_sql::translate::{
    ParamContext, Table, translate_count, translate_fetch_keyset, translate_fetch_offset,
    translate_filter, translate_order_by,
};
use portadb::dialect::SqliteDialect;
use portadb::value::Value;

fn sample_table() -> Table {
    Table::sqlite(
        "users",
        vec![
            ColumnInfo {
                name: "id".into(),
                kind: ColumnKind::Int,
                nullable: false,
                hint: DisplayHint::Plain,
            },
            ColumnInfo {
                name: "name".into(),
                kind: ColumnKind::Text,
                nullable: true,
                hint: DisplayHint::Plain,
            },
            ColumnInfo {
                name: "score".into(),
                kind: ColumnKind::Float,
                nullable: true,
                hint: DisplayHint::Plain,
            },
            ColumnInfo {
                name: "active".into(),
                kind: ColumnKind::Bool,
                nullable: true,
                hint: DisplayHint::Plain,
            },
        ],
    )
}

#[test]
fn test_sql_injection_prevention() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    let malicious_literal = "x'); DROP TABLE users; --";
    let filter = Filter::Eq("name".into(), Owned::Text(malicious_literal.into()));
    let sql = translate_filter(&filter, &dialect, &mut ctx).expect("translated");
    let params = ctx.finish();

    // SQL must only contain the parameter placeholder, never the literal
    assert_eq!(sql, "\"name\" = :p0");
    assert!(!sql.contains("DROP TABLE"));
    assert_eq!(
        params.get("p0"),
        Some(&Value::Text(malicious_literal.to_string()))
    );
}

#[test]
fn test_identifier_quoting() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    let filter = Filter::Eq("user group".into(), Owned::Int(42));
    let sql = translate_filter(&filter, &dialect, &mut ctx).expect("translated");
    assert_eq!(sql, "\"user group\" = :p0");
}

#[test]
fn test_not_with_nulls_translation() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    // NOT (x) must translate as NOT COALESCE((x), 0) so null rows match
    let filter = Filter::Not(Box::new(Filter::Eq("score".into(), Owned::Float(100.0))));
    let sql = translate_filter(&filter, &dialect, &mut ctx).expect("translated");
    assert_eq!(sql, "NOT COALESCE((\"score\" = :p0), 0)");
}

#[test]
fn test_nulls_placement_order_by() {
    let dialect = SqliteDialect;
    let keys = vec![
        SortKey {
            column: "score".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "name".into(),
            order: Order::Desc,
            nulls: Nulls::First,
        },
    ];

    let sql = translate_order_by(&sample_table(), &keys, &dialect);
    assert_eq!(
        sql,
        "(\"score\" IS NULL) ASC, \"score\" ASC, (\"name\" IS NULL) DESC, \"name\" DESC"
    );
}

#[test]
fn test_translate_count_and_fetch() {
    let dialect = SqliteDialect;
    let table = sample_table();
    let query = Query::new()
        .filter(Filter::Gt("score".into(), Owned::Float(50.0)))
        .sort_asc("name");

    let (count_sql, count_params) = translate_count(&table, &query, &dialect);
    assert_eq!(
        count_sql,
        "SELECT COUNT(*) FROM \"users\" WHERE \"score\" > :p0"
    );
    assert_eq!(count_params.get("p0"), Some(&Value::Real(50.0)));

    let (fetch_sql, fetch_params) = translate_fetch_offset(&table, &query, 10..20, &dialect);
    assert_eq!(
        fetch_sql,
        "SELECT \"id\", \"name\", \"score\", \"active\", \"rowid\" FROM \"users\" WHERE \"score\" > :p0 ORDER BY (\"name\" IS NULL) ASC, \"name\" ASC, \"rowid\" ASC LIMIT :p1 OFFSET :p2"
    );
    assert_eq!(fetch_params.get("p0"), Some(&Value::Real(50.0)));
    assert_eq!(fetch_params.get("p1"), Some(&Value::Integer(10)));
    assert_eq!(fetch_params.get("p2"), Some(&Value::Integer(10)));
}

#[test]
fn test_translate_keyset() {
    let dialect = SqliteDialect;
    let table = sample_table();
    let query = Query::new().sort_asc("score");

    // Keys are: score ASC (Nulls::Last), rowid ASC (Nulls::Last)
    let bookmark = vec![Some(Owned::Float(75.5)), Some(Owned::Int(123))];

    let (keyset_sql, params) = translate_fetch_keyset(&table, &query, &bookmark, 50, 0, &dialect);
    assert!(keyset_sql.contains("(\"score\" > :p0 OR \"score\" IS NULL)"));
    assert!(keyset_sql.contains("(\"score\" = :p1 AND (\"rowid\" > :p2 OR \"rowid\" IS NULL))"));
    assert_eq!(params.get("p0"), Some(&Value::Real(75.5)));
    assert_eq!(params.get("p1"), Some(&Value::Real(75.5)));
    assert_eq!(params.get("p2"), Some(&Value::Integer(123)));
}
