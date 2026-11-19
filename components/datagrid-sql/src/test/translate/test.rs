use super::*;
use portadb::dialect::SqliteDialect;

#[test]
fn test_injection_bound_params() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    let malicious = "x'); DROP TABLE t; --";
    let filter = Filter::Eq("name".into(), Owned::Text(malicious.into()));
    let sql = translate_filter(&filter, &dialect, &mut ctx).unwrap();
    let params = ctx.finish();

    assert_eq!(sql, "\"name\" = :p0");
    assert_eq!(params.get("p0"), Some(&Value::Text(malicious.to_string())));
    assert!(!sql.contains("DROP TABLE"));
}

#[test]
fn test_not_with_nulls() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    let filter = Filter::Not(Box::new(Filter::Eq("price".into(), Owned::Float(50.0))));
    let sql = translate_filter(&filter, &dialect, &mut ctx).unwrap();
    assert_eq!(sql, "NOT COALESCE((\"price\" = :p0), 0)");
}

#[test]
fn test_nulls_placement() {
    use datagrid::{ColumnKind, DisplayHint};
    let dialect = SqliteDialect;
    let column = |name: &str, nullable: bool| ColumnInfo {
        name: name.into(),
        kind: ColumnKind::Float,
        nullable,
        hint: DisplayHint::Plain,
    };
    let table = Table::sqlite(
        "t",
        vec![
            column("price", true),
            column("name", true),
            column("qty", false),
        ],
    );
    let key = |column: &str, order: Order, nulls: Nulls| SortKey {
        column: column.into(),
        order,
        nulls,
    };

    // Placement SQLite does not give by itself needs the term
    let keys = vec![
        key("price", Order::Asc, Nulls::Last),
        key("name", Order::Desc, Nulls::First),
    ];
    assert_eq!(
        translate_order_by(&table, &keys, &dialect),
        "(\"price\" IS NULL) ASC, \"price\" ASC, (\"name\" IS NULL) DESC, \"name\" DESC"
    );

    // SQLite's own placement, or a NOT NULL column, keeps the plain index-friendly key
    let keys = vec![
        key("price", Order::Asc, Nulls::First),
        key("name", Order::Desc, Nulls::Last),
        key("qty", Order::Asc, Nulls::Last),
    ];
    assert_eq!(
        translate_order_by(&table, &keys, &dialect),
        "\"price\" ASC, \"name\" DESC, \"qty\" ASC"
    );
}

#[test]
fn test_eq_ignore_case_translation() {
    let dialect = SqliteDialect;
    let mut ctx = ParamContext::new();

    let filter = Filter::eq_ignore_case("city", "Boston");
    let sql = translate_filter(&filter, &dialect, &mut ctx).unwrap();
    let params = ctx.finish();

    assert_eq!(sql, "lower(\"city\") = lower(:p0)");
    assert_eq!(params.get("p0"), Some(&Value::Text("Boston".to_string())));
}

#[test]
fn test_reversed_sort_keys() {
    let keys = vec![
        SortKey {
            column: "col1".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "col2".into(),
            order: Order::Desc,
            nulls: Nulls::First,
        },
    ];

    let rev = reversed(&keys);
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].column, "col1");
    assert_eq!(rev[0].order, Order::Desc);
    assert_eq!(rev[0].nulls, Nulls::First);

    assert_eq!(rev[1].column, "col2");
    assert_eq!(rev[1].order, Order::Asc);
    assert_eq!(rev[1].nulls, Nulls::Last);
}

#[test]
fn test_fetch_keyset_with_skip() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "users",
        vec![ColumnInfo {
            name: "score".into(),
            kind: datagrid::ColumnKind::Int,
            nullable: false,
            hint: datagrid::DisplayHint::Plain,
        }],
    );

    let query = Query::new().sort_asc("score");
    let bookmark = vec![Some(Owned::Int(100)), Some(Owned::Int(5))]; // score, rowid
    let (sql, params) = translate_fetch_keyset(&table, &query, &bookmark, 25, 10, &dialect);

    // read the parameter names the SQL actually uses for LIMIT and OFFSET
    let tail = sql.rsplit("LIMIT :").next().expect("LIMIT present");
    let (limit_name, offset_name) = tail.split_once(" OFFSET :").expect("OFFSET present");
    assert_eq!(params.get(limit_name), Some(&Value::Integer(25)));
    assert_eq!(params.get(offset_name.trim()), Some(&Value::Integer(10)));
}

#[test]
fn test_sort_tuple_of() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "items",
        vec![ColumnInfo {
            name: "price".into(),
            kind: datagrid::ColumnKind::Float,
            nullable: false,
            hint: datagrid::DisplayHint::Plain,
        }],
    );

    let mut query = Query::new().sort_desc("price");
    query.filter = Some(Filter::gt("price", Owned::Float(10.0)));

    let (sql, params) = translate_sort_tuple_of(&table, &query, 42, &dialect);
    assert_eq!(
        sql,
        "SELECT \"price\", \"rowid\" FROM \"items\" WHERE \"rowid\" = :p0 AND (\"price\" > :p1)"
    );
    assert_eq!(params.get("p0"), Some(&Value::Integer(42)));
    assert_eq!(params.get("p1"), Some(&Value::Real(10.0)));
}

#[test]
fn test_count_before() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "items",
        vec![ColumnInfo {
            name: "price".into(),
            kind: datagrid::ColumnKind::Float,
            nullable: false,
            hint: datagrid::DisplayHint::Plain,
        }],
    );

    let query = Query::new().sort_asc("price");
    let tuple = vec![Some(Owned::Float(50.0)), Some(Owned::Int(12))];

    let (sql, params) = translate_count_before(&table, &query, &tuple, &dialect);
    assert!(sql.starts_with("SELECT COUNT(*) FROM \"items\" WHERE "));
    assert!(sql.contains("\"price\" < :p0"));
    assert_eq!(params.get("p0"), Some(&Value::Real(50.0)));
}

#[test]
fn test_locate() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "items",
        vec![ColumnInfo {
            name: "title".into(),
            kind: datagrid::ColumnKind::Text,
            nullable: false,
            hint: datagrid::DisplayHint::Plain,
        }],
    );

    let query = Query::new().sort_asc("title");
    let extra = Filter::eq_ignore_case("title", "book");

    let (sql, params) = translate_locate(&table, &query, &extra, &dialect);
    assert!(sql.starts_with("SELECT \"rowid\", \"title\", \"rowid\" FROM \"items\" WHERE "));
    assert!(sql.contains("lower(\"title\") = lower(:p0)"));
    assert!(sql.ends_with("ORDER BY \"title\" ASC, \"rowid\" ASC LIMIT 1"));
    assert_eq!(params.get("p0"), Some(&Value::Text("book".to_string())));
}

#[test]
fn test_checkpoints() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "items",
        vec![ColumnInfo {
            name: "rank".into(),
            kind: datagrid::ColumnKind::Int,
            nullable: false,
            hint: datagrid::DisplayHint::Plain,
        }],
    );

    let mut query = Query::new().sort_asc("rank");
    query.filter = Some(Filter::gt("rank", Owned::Int(0)));

    let (sql, params) = translate_checkpoints(&table, &query, 1024, &dialect);
    assert_eq!(
        sql,
        "SELECT \"rank\", \"rowid\" FROM (SELECT \"rank\", \"rowid\", ROW_NUMBER() OVER (ORDER BY \"rank\" ASC, \"rowid\" ASC) AS crawrad_rn FROM \"items\" WHERE \"rank\" > :p0) WHERE crawrad_rn % :p1 = 0 ORDER BY crawrad_rn"
    );
    assert_eq!(params.get("p0"), Some(&Value::Integer(0)));
    assert_eq!(params.get("p1"), Some(&Value::Integer(1024)));
}

#[test]
fn test_translate_summary() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "items",
        vec![
            ColumnInfo {
                name: "price".into(),
                kind: datagrid::ColumnKind::Float,
                nullable: false,
                hint: datagrid::DisplayHint::Plain,
            },
            ColumnInfo {
                name: "qty".into(),
                kind: datagrid::ColumnKind::Int,
                nullable: false,
                hint: datagrid::DisplayHint::Plain,
            },
        ],
    );

    let mut query = Query::new();
    query.summary = vec![
        datagrid::group::Total {
            column: "price".into(),
            aggregate: Aggregate::Sum,
        },
        datagrid::group::Total {
            column: "qty".into(),
            aggregate: Aggregate::Count,
        },
        datagrid::group::Total {
            column: "price".into(),
            aggregate: Aggregate::Avg,
        },
    ];
    query.filter = Some(Filter::gt("qty", Owned::Int(5)));

    let (sql, params) = translate_summary(&table, &query, &dialect);
    assert_eq!(
        sql,
        "SELECT SUM(\"price\"), COUNT(*), AVG(\"price\") FROM \"items\" WHERE \"qty\" > :p0"
    );
    assert_eq!(params.get("p0"), Some(&Value::Integer(5)));
}

#[test]
fn test_translate_distinct() {
    let dialect = SqliteDialect;
    let table = Table::sqlite(
        "customers",
        vec![
            ColumnInfo {
                name: "city".into(),
                kind: datagrid::ColumnKind::Text,
                nullable: true,
                hint: datagrid::DisplayHint::Plain,
            },
            ColumnInfo {
                name: "active".into(),
                kind: datagrid::ColumnKind::Bool,
                nullable: false,
                hint: datagrid::DisplayHint::Plain,
            },
        ],
    );

    // 1. Without filter
    let q1 = Query::default();
    let (sql1, params1) = translate_distinct(&table, &q1, "city", 500, &dialect);
    assert_eq!(
        sql1,
        "SELECT DISTINCT \"city\" FROM \"customers\" ORDER BY \"city\" LIMIT :p0"
    );
    assert_eq!(params1.get("p0"), Some(&Value::Integer(500)));

    // 2. With filter
    let q2 = Query {
        filter: Some(Filter::Eq("active".into(), Owned::Bool(true))),
        ..Default::default()
    };
    let (sql2, params2) = translate_distinct(&table, &q2, "city", 500, &dialect);
    assert_eq!(
        sql2,
        "SELECT DISTINCT \"city\" FROM \"customers\" WHERE \"active\" = :p0 ORDER BY \"city\" LIMIT :p1"
    );
    assert_eq!(params2.get("p0"), Some(&Value::Integer(1)));
    assert_eq!(params2.get("p1"), Some(&Value::Integer(500)));
}
