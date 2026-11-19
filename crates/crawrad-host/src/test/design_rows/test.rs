//! The first rows of a bound Grid's table, read at design time.

use super::*;
use crawrad_ir::Node;
use crawrad_ir::design_rows::DEFAULT_LIMIT;

/// A form with `conn1` (url) and `grid1` bound to it through `table_name`.
fn form(url: &str, table: &str, design_rows: Option<&str>) -> Document {
    let mut doc = Document::new("form_main", "Form");
    let mut eng = Node::new("conn1", "Connection");
    eng.props.insert("url".into(), Value::Str(url.into()));
    doc.add_child("form_main", eng).unwrap();
    let mut grid = Node::new("grid1", "Grid");
    grid.props
        .insert("data_source".into(), Value::Ref("conn1".into()));
    grid.props
        .insert("table_name".into(), Value::Str(table.into()));
    if let Some(n) = design_rows {
        grid.props
            .insert(DESIGN_ROWS_KEY.into(), Value::Enum(n.into()));
    }
    doc.add_child("form_main", grid).unwrap();
    doc
}

fn settle(cache: &mut DesignRows) {
    let start = Instant::now();
    while !cache.pump() {
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "the probe never answered"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn two_rows(_: &Path, url: &str, query: &RowQuery, limit: usize) -> Answer {
    let table = match query {
        RowQuery::Table(t) => format!("table {t}"),
        RowQuery::Sql(q) => format!("sql {q}"),
    };
    Ok(Sample {
        columns: vec!["url".into(), "table".into()],
        kinds: Vec::new(),
        rows: vec![
            vec![Some(url.into()), Some(table)],
            vec![Some(limit.to_string()), Some(String::new())],
        ],
        total: 1240,
    })
}

#[test]
fn a_bound_grid_has_a_source_and_off_or_unbound_has_none() {
    let doc = form("sqlite://app.db", " customers ", None);
    assert_eq!(
        grid_source(&doc, Some(Path::new("C:/p")), "grid1"),
        Some(GridSource {
            url: "sqlite://C:/p/app.db".into(),
            query: RowQuery::Table("customers".into()),
            table: "customers".into(),
            limit: DEFAULT_LIMIT,
        })
    );
    let off = form("sqlite://app.db", "customers", Some("Off"));
    assert_eq!(grid_source(&off, None, "grid1"), None);
    let no_table = form("sqlite://app.db", "  ", None);
    assert_eq!(grid_source(&no_table, None, "grid1"), None);
    assert_eq!(grid_source(&doc, None, "conn1"), None, "only a Grid");
}

#[test]
fn the_rows_are_read_then_ready_with_the_file_name() {
    let doc = form("sqlite://data/app.db", "customers", Some("20"));
    let proj = Path::new("C:/p");
    let mut cache = DesignRows::with_probe(|| Some(PathBuf::from("probe")), two_rows);
    cache.ensure(&doc, 1, Some(proj));
    assert!(matches!(
        cache.rows(&doc, Some(proj), "grid1"),
        Some(GridRows::Reading { limit: 20, .. })
    ));
    settle(&mut cache);
    let Some(GridRows::Ready {
        sample,
        table,
        file,
    }) = cache.rows(&doc, Some(proj), "grid1")
    else {
        panic!("the rows should be ready");
    };
    assert_eq!(sample.rows[0][0], Some("sqlite://C:/p/data/app.db".into()));
    assert_eq!(sample.rows[1][0], Some("20".into()));
    assert_eq!(table, "customers");
    assert_eq!(file.as_deref(), Some("app.db"));
}

#[test]
fn a_failed_read_is_offline_and_keeps_the_last_columns() {
    let doc = form("sqlite://app.db", "customers", None);
    let mut cache = DesignRows::with_probe(|| Some(PathBuf::from("probe")), two_rows);
    cache.ensure(&doc, 1, None);
    settle(&mut cache);
    // the same grid, now asking for another limit, and the probe fails
    let doc2 = form("sqlite://app.db", "customers", Some("50"));
    cache.run = |_, _, _, _| Err("database file not found (app.db)".into());
    cache.ensure(&doc2, 2, None);
    settle(&mut cache);
    assert_eq!(
        cache.rows(&doc2, None, "grid1"),
        Some(GridRows::Offline {
            columns: vec!["url".into(), "table".into()],
            reason: "database file not found (app.db)".into(),
            table: "customers".into(),
        })
    );
}

#[test]
fn without_the_probe_the_grid_says_so() {
    let doc = form("sqlite://app.db", "customers", None);
    let mut cache = DesignRows::with_probe(|| None, two_rows);
    cache.ensure(&doc, 1, None);
    assert!(matches!(
        cache.rows(&doc, None, "grid1"),
        Some(GridRows::Offline { reason, .. }) if reason == PROBE_NOT_FOUND
    ));
}

#[test]
fn the_probe_answer_is_parsed_or_its_error_kept() {
    let ok =
        parse_sample(r#"{"columns":["id","name"],"rows":[["1","Ana"],["2",""]],"total":1240}"#);
    assert_eq!(
        ok,
        Ok(Sample {
            columns: vec!["id".into(), "name".into()],
            kinds: Vec::new(),
            rows: vec![
                vec![Some("1".into()), Some("Ana".into())],
                vec![Some("2".into()), Some(String::new())]
            ],
            total: 1240,
        })
    );
    let ok_with_kinds = parse_sample(
        r#"{"columns":["id","name"],"kinds":["INTEGER","TEXT"],"rows":[["1","Ana"]],"total":1}"#,
    );
    assert_eq!(
        ok_with_kinds,
        Ok(Sample {
            columns: vec!["id".into(), "name".into()],
            kinds: vec!["INTEGER".into(), "TEXT".into()],
            rows: vec![vec![Some("1".into()), Some("Ana".into())]],
            total: 1,
        })
    );
    let ok_with_null = parse_sample(r#"{"columns":["id","name"],"rows":[["1",null]],"total":1}"#);
    assert_eq!(
        ok_with_null,
        Ok(Sample {
            columns: vec!["id".into(), "name".into()],
            kinds: Vec::new(),
            rows: vec![vec![Some("1".into()), None]],
            total: 1,
        })
    );
    assert_eq!(
        parse_sample(r#"{"error":"no table named 'x'"}"#),
        Err("no table named 'x'".to_string())
    );
    assert!(parse_sample("not json").is_err());
    assert_eq!(
        sqlite_path("sqlite://C:/p/app.db"),
        Some(PathBuf::from("C:/p/app.db"))
    );
    assert_eq!(sqlite_path("sqlite::memory:"), None);
    assert_eq!(sqlite_path("postgres://h/db"), None);
}

/// `conn1` (url), `dst1` (its `sql` over `conn1`) and `grid1` bound to the
/// dataset; the grid's own `table_name` is set too, and must be ignored.
fn dataset_form(sql: &str) -> Document {
    let mut doc = form("sqlite://data/app.db", "ignored_table", None);
    let mut dst = Node::new("dst1", "Dataset");
    dst.props
        .insert("connection".into(), Value::Ref("conn1".into()));
    dst.props.insert("sql".into(), Value::Str(sql.into()));
    doc.add_child("form_main", dst).unwrap();
    doc.node_mut("grid1")
        .unwrap()
        .props
        .insert("data_source".into(), Value::Ref("dst1".into()));
    doc
}

/// Bound to a Dataset, the Designer's rows come from the dataset's `sql`
/// over the dataset's connection, not from the grid's `table_name`.
#[test]
fn a_grid_on_a_dataset_reads_the_dataset_sql() {
    let sql = "SELECT name, city FROM contacts ORDER BY name";
    let doc = dataset_form(sql);
    let proj = Path::new("C:/p");
    assert_eq!(
        grid_source(&doc, Some(proj), "grid1"),
        Some(GridSource {
            url: "sqlite://C:/p/data/app.db".into(),
            query: RowQuery::Sql(sql.into()),
            table: "dst1".into(),
            limit: DEFAULT_LIMIT,
        })
    );
    let mut cache = DesignRows::with_probe(|| Some(PathBuf::from("probe")), two_rows);
    cache.ensure(&doc, 1, Some(proj));
    settle(&mut cache);
    let Some(GridRows::Ready { sample, table, .. }) = cache.rows(&doc, Some(proj), "grid1") else {
        panic!("the rows should be ready");
    };
    assert_eq!(sample.rows[0][1], Some(format!("sql {sql}")));
    assert_eq!(table, "dst1");
    // a dataset with no query yet shows nothing (it is not a table fallback)
    assert_eq!(grid_source(&dataset_form("  "), None, "grid1"), None);
}
