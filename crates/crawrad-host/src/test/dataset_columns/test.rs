//! The columns of a bound dataset, read at design time.

use super::*;
use crawrad_ir::Node;
use crawrad_ir::binding::{BindingDef, BindingMode};

/// A form with `conn1` (url), `dst1` (table) and an Edit bound to `dst1`.
fn form(url: Option<&str>, table: Option<&str>, connection: Option<&str>) -> Document {
    let mut doc = Document::new("form_main", "Form");
    let mut eng = Node::new("conn1", "Connection");
    if let Some(u) = url {
        eng.props.insert("url".into(), Value::Str(u.into()));
    }
    doc.add_child("form_main", eng).unwrap();
    let mut ds = Node::new("dst1", "Dataset");
    if let Some(t) = table {
        ds.props.insert("table".into(), Value::Str(t.into()));
    }
    if let Some(c) = connection {
        ds.props.insert("connection".into(), Value::Ref(c.into()));
    }
    doc.add_child("form_main", ds).unwrap();
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_binding(
        "text",
        BindingDef::new("dst1", BindingMode::TwoWay).with_field("name"),
    );
    doc.add_child("form_main", edit).unwrap();
    doc
}

#[test]
fn a_complete_chain_gives_the_url_and_the_trimmed_table() {
    let doc = form(Some("sqlite://app.db"), Some(" clients "), Some("conn1"));
    assert_eq!(
        dataset_source(&doc, "dst1"),
        Some(DatasetSource {
            url: "sqlite://app.db".into(),
            table: "clients".into(),
        })
    );
}

#[test]
fn any_missing_link_gives_none() {
    let cases = [
        (
            "empty table",
            form(Some("sqlite://app.db"), Some("  "), Some("conn1")),
        ),
        (
            "no table",
            form(Some("sqlite://app.db"), None, Some("conn1")),
        ),
        (
            "no connection",
            form(Some("sqlite://app.db"), Some("t"), None),
        ),
        (
            "empty connection",
            form(Some("sqlite://app.db"), Some("t"), Some("")),
        ),
        (
            "dangling connection",
            form(Some("sqlite://app.db"), Some("t"), Some("gone")),
        ),
        (
            "connection to a non-engine",
            form(Some("sqlite://app.db"), Some("t"), Some("Edit1")),
        ),
        ("empty url", form(Some(""), Some("t"), Some("conn1"))),
        ("no url", form(None, Some("t"), Some("conn1"))),
    ];
    for (why, doc) in cases {
        assert_eq!(dataset_source(&doc, "dst1"), None, "{why}");
    }
    let mut doc = form(Some("sqlite://app.db"), Some("t"), Some("conn1"));
    // the same props on nodes of the wrong classes
    let mut other = Node::new("tmr1", "Timer");
    other.props.insert("table".into(), Value::Str("t".into()));
    other
        .props
        .insert("connection".into(), Value::Ref("conn1".into()));
    other
        .props
        .insert("url".into(), Value::Str("sqlite://app.db".into()));
    doc.add_child("form_main", other).unwrap();
    assert_eq!(dataset_source(&doc, "tmr1"), None, "not a Dataset");
    doc.node_mut("dst1")
        .unwrap()
        .props
        .insert("connection".into(), Value::Ref("tmr1".into()));
    assert_eq!(dataset_source(&doc, "dst1"), None, "not a Connection");
    assert_eq!(dataset_source(&doc, ""), None, "no id");
}

#[test]
fn a_relative_sqlite_file_is_resolved_against_the_project() {
    let proj = Path::new("C:/projects/myapp");
    assert_eq!(
        make_url_absolute("sqlite://app.db", Some(proj)),
        "sqlite://C:/projects/myapp/app.db"
    );
    assert_eq!(
        make_url_absolute("sqlite:data/app.db", Some(proj)),
        "sqlite://C:/projects/myapp/data/app.db"
    );
    for kept in [
        "sqlite://:memory:",
        "sqlite::memory:",
        "sqlite:file:shared?mode=memory",
        "postgres://localhost/shop",
    ] {
        assert_eq!(make_url_absolute(kept, Some(proj)), kept);
    }
    assert_eq!(
        make_url_absolute("sqlite://app.db", None),
        "sqlite://app.db"
    );
}

#[test]
fn the_probe_answer_is_read_both_ways() {
    assert_eq!(
        parse_answer(r#"{"columns":["id","name"]}"#),
        Ok(vec!["id".to_string(), "name".to_string()])
    );
    assert_eq!(
        parse_answer(r#"{"error":"no table named 'x'"}"#),
        Err("no table named 'x'".to_string())
    );
    assert!(parse_answer("boom").is_err());
}

#[test]
fn without_the_probe_the_dataset_fails_once_and_is_retried_on_a_new_revision() {
    let doc = form(Some("sqlite://app.db"), Some("clients"), Some("conn1"));
    let proj = Path::new("C:/p");
    static LOOKS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let mut cache = DatasetColumns::with_probe(
        || {
            LOOKS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            None
        },
        probe,
    );
    let looks = || LOOKS.load(std::sync::atomic::Ordering::SeqCst);
    cache.ensure(&doc, 1, Some(proj));
    assert_eq!(
        cache.state("sqlite://C:/p/app.db", "clients"),
        Some(&ColumnState::Failed(PROBE_NOT_FOUND.to_string()))
    );
    assert_eq!(cache.columns(&doc, Some(proj), "dst1"), None);
    assert_eq!(looks(), 1);

    // the same revision keeps the failure; a new one forgets it and asks again
    cache.ensure(&doc, 1, Some(proj));
    assert_eq!(looks(), 1, "a redraw must not look for the probe again");
    cache.ensure(&doc, 2, Some(proj));
    assert_eq!(looks(), 2, "an edit retries");
}

#[test]
fn ready_columns_are_what_the_inspector_gets() {
    let doc = form(Some("sqlite://app.db"), Some("clients"), Some("conn1"));
    let proj = Path::new("C:/p");
    let mut cache = DatasetColumns::with_probe(|| None, probe);
    cache.cache.insert(
        ("sqlite://C:/p/app.db".into(), "clients".into()),
        ColumnState::Ready(vec!["id".into(), "name".into()]),
    );
    assert_eq!(
        cache.columns(&doc, Some(proj), "dst1"),
        Some(vec!["id".to_string(), "name".to_string()])
    );
    // a ready answer survives a new revision
    cache.ensure(&doc, 7, Some(proj));
    assert!(cache.columns(&doc, Some(proj), "dst1").is_some());
}

#[test]
fn the_probe_answer_lands_in_the_cache_on_pump() {
    let doc = form(Some("sqlite://app.db"), Some("clients"), Some("conn1"));
    let proj = Path::new("C:/p");
    let mut cache = DatasetColumns::with_probe(
        || Some(PathBuf::from("probe")),
        |_, url, table| Ok(vec![url.to_string(), table.to_string()]),
    );
    cache.ensure(&doc, 1, Some(proj));
    assert_eq!(
        cache.state("sqlite://C:/p/app.db", "clients"),
        Some(&ColumnState::Loading)
    );
    let start = Instant::now();
    while !cache.pump() {
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "the probe never answered"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(
        cache.columns(&doc, Some(proj), "dst1"),
        Some(vec![
            "sqlite://C:/p/app.db".to_string(),
            "clients".to_string()
        ])
    );
}

#[test]
fn a_dataset_a_property_references_is_probed_too() {
    // No binding at all: only a grid column's `LookupSource` points at `dst1`.
    let mut doc = form(Some("sqlite://app.db"), Some("cities"), Some("conn1"));
    doc.node_mut("Edit1").unwrap().bindings.clear();
    let mut col = Node::new("gdc1", "GridColumn");
    col.props
        .insert("lookup_source".into(), Value::Ref("dst1".into()));
    doc.add_child("form_main", col).unwrap();
    let proj = Path::new("C:/p");
    let mut cache =
        DatasetColumns::with_probe(|| Some(PathBuf::from("probe")), |_, _, _| Ok(vec![]));
    cache.ensure(&doc, 1, Some(proj));
    assert_eq!(
        cache.state("sqlite://C:/p/app.db", "cities"),
        Some(&ColumnState::Loading)
    );
}
