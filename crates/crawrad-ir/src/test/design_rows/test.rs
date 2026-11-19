//! The footer words and the `design_rows` choices.

use super::*;

fn sample(rows: usize, total: u64) -> Sample {
    Sample {
        columns: vec!["id".into(), "name".into()],
        kinds: vec!["INTEGER".into(), "TEXT".into()],
        rows: vec![vec![Some("1".into()), Some("Ana".into())]; rows],
        total,
    }
}

#[test]
fn the_footer_says_how_many_of_how_many_and_from_where() {
    let ready = GridRows::Ready {
        sample: sample(8, 1240),
        table: "customers".into(),
        file: Some("app.db".into()),
    };
    assert_eq!(
        ready.footer(),
        "design-time rows \u{b7} 8 of 1,240 \u{b7} customers (app.db)"
    );
    let no_file = GridRows::Ready {
        sample: sample(2, 2),
        table: "t".into(),
        file: None,
    };
    assert_eq!(no_file.footer(), "design-time rows \u{b7} 2 of 2 \u{b7} t");
}

#[test]
fn offline_and_reading_have_their_own_words() {
    let offline = GridRows::Offline {
        columns: vec!["id".into()],
        reason: "database file not found".into(),
        table: "customers".into(),
    };
    assert_eq!(
        offline.footer(),
        "design-time rows \u{b7} offline \u{b7} customers"
    );
    assert_eq!(offline.columns(), ["id".to_string()]);
    let reading = GridRows::Reading {
        columns: Vec::new(),
        limit: 8,
        table: "customers".into(),
    };
    assert_eq!(reading.footer(), "reading 8 rows\u{2026}");
}

#[test]
fn the_limit_defaults_to_eight_and_off_is_none() {
    assert_eq!(limit_of(None), Some(DEFAULT_LIMIT));
    assert_eq!(limit_of(Some("")), Some(DEFAULT_LIMIT));
    assert_eq!(limit_of(Some("Off")), None);
    assert_eq!(limit_of(Some("20")), Some(20));
    assert_eq!(limit_of(Some("garbage")), Some(DEFAULT_LIMIT));
    assert_eq!(limit_of(Some("0")), Some(DEFAULT_LIMIT));
}

#[test]
fn thousands_groups_by_three() {
    assert_eq!(thousands(0), "0");
    assert_eq!(thousands(999), "999");
    assert_eq!(thousands(1000), "1,000");
    assert_eq!(thousands(1_234_567), "1,234,567");
}

#[test]
fn the_kinds_come_only_with_the_rows() {
    let ready = GridRows::Ready {
        sample: sample(1, 1),
        table: "customers".into(),
        file: None,
    };
    assert_eq!(ready.kinds(), ["INTEGER".to_string(), "TEXT".to_string()]);
    let reading = GridRows::Reading {
        columns: vec!["id".into()],
        limit: 8,
        table: "customers".into(),
    };
    assert!(reading.kinds().is_empty());
}

/// A grid bound to a Dataset reaches the Connection through it; bound to a
/// Connection, there is no dataset in between; a dataset without a connection
/// is not bound.
#[test]
fn a_grid_binding_follows_a_dataset_to_its_connection() {
    use crate::{Document, Node, Value};
    let mut doc = Document::new("form_main", "Form");
    doc.add_child("form_main", Node::new("conn1", "Connection"))
        .unwrap();
    let mut dst = Node::new("dst1", "Dataset");
    dst.props
        .insert("connection".into(), Value::Ref("conn1".into()));
    doc.add_child("form_main", dst).unwrap();
    let mut lonely = Node::new("dst2", "Dataset");
    lonely
        .props
        .insert("connection".into(), Value::Ref(String::new()));
    doc.add_child("form_main", lonely).unwrap();
    let grid_on = |target: &str| {
        let mut g = Node::new("grid1", "Grid");
        g.props
            .insert("data_source".into(), Value::Ref(target.into()));
        g
    };
    let on_dataset = grid_on("dst1");
    let b = grid_binding(&doc, &on_dataset).expect("bound through the dataset");
    assert_eq!(b.connection.id, "conn1");
    assert_eq!(b.dataset.map(|d| d.id.as_str()), Some("dst1"));
    let on_conn = grid_on("conn1");
    let b = grid_binding(&doc, &on_conn).expect("bound to the connection");
    assert_eq!(b.connection.id, "conn1");
    assert!(b.dataset.is_none());
    assert!(grid_binding(&doc, &grid_on("dst2")).is_none());
    assert!(grid_binding(&doc, &grid_on("")).is_none());
}
