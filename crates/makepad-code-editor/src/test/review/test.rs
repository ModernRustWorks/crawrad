//! Unit tests for diff review view model computation.

use super::{
    ReviewRow,
    ReviewRowKind::{Added, Removed, Same},
    review_view,
};

#[test]
fn test_pure_insert() {
    let orig = "first\nthird\n";
    let prop = "first\nsecond\nthird\n";
    let view = review_view(orig, prop);
    assert_eq!(view.text, "first\nsecond\nthird\n");
    assert_eq!(view.changes, vec![1]);
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
            ReviewRow {
                kind: Added,
                number: Some(2),
            },
            ReviewRow {
                kind: Same,
                number: Some(3),
            },
        ]
    );
}

#[test]
fn test_pure_delete() {
    let orig = "first\nsecond\nthird\n";
    let prop = "first\nthird\n";
    let view = review_view(orig, prop);
    assert_eq!(view.text, "first\nsecond\nthird\n");
    assert_eq!(view.changes, vec![1]);
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Same,
                number: Some(2),
            },
        ]
    );
}

#[test]
fn test_one_replace() {
    let orig = "first\nold\nthird\n";
    let prop = "first\nnew\nthird\n";
    let view = review_view(orig, prop);
    assert_eq!(view.text, "first\nold\nnew\nthird\n");
    assert_eq!(view.changes, vec![1]);
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Added,
                number: Some(2),
            },
            ReviewRow {
                kind: Same,
                number: Some(3),
            },
        ]
    );
}

#[test]
fn test_two_separate_changes() {
    let orig = "a\nb\nc\nd\ne\n";
    let prop = "a\nB\nc\nD\ne\n";
    let view = review_view(orig, prop);
    assert_eq!(view.changes.len(), 2);
    assert_eq!(view.changes, vec![1, 4]);
    assert_eq!(view.text, "a\nb\nB\nc\nd\nD\ne\n");
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Added,
                number: Some(2),
            },
            ReviewRow {
                kind: Same,
                number: Some(3),
            },
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Added,
                number: Some(4),
            },
            ReviewRow {
                kind: Same,
                number: Some(5),
            },
        ]
    );
}

#[test]
fn test_identical_texts() {
    let orig = "a\nb\nc\n";
    let prop = "a\nb\nc\n";
    let view = review_view(orig, prop);
    assert!(view.changes.is_empty());
    assert_eq!(view.text, "a\nb\nc\n");
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
            ReviewRow {
                kind: Same,
                number: Some(2),
            },
            ReviewRow {
                kind: Same,
                number: Some(3),
            },
        ]
    );
}

#[test]
fn test_numbering_skips_removed_rows() {
    let orig = "del1\ndel2\nkeep\n";
    let prop = "keep\n";
    let view = review_view(orig, prop);
    assert_eq!(view.changes, vec![0]);
    assert_eq!(
        view.rows,
        vec![
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Removed,
                number: None,
            },
            ReviewRow {
                kind: Same,
                number: Some(1),
            },
        ]
    );
}

#[test]
fn test_trailing_newline_behaviour() {
    let orig = "a\nb";
    let prop = "a\nc";
    let view = review_view(orig, prop);
    assert_eq!(view.text, "a\nb\nc");
    assert!(!view.text.ends_with('\n'));

    let orig_nl = "a\nb\n";
    let prop_nl = "a\nc\n";
    let view_nl = review_view(orig_nl, prop_nl);
    assert_eq!(view_nl.text, "a\nb\nc\n");
    assert!(view_nl.text.ends_with('\n'));
}

#[test]
fn stepping_wraps_both_ways() {
    let view = review_view(
        "a
b
c
d
", "A
b
C
d
",
    );
    assert_eq!(view.changes.len(), 2);
    assert_eq!(view.step(0, true), 1);
    assert_eq!(view.step(1, true), 0);
    assert_eq!(view.step(0, false), 1);
    assert_eq!(view.position_label(1).as_deref(), Some("2 of 2"));
    assert_eq!(view.changes_label(), "· 2 changes");
}

#[test]
fn one_change_has_no_position() {
    let view = review_view(
        "a
", "b
",
    );
    assert_eq!(view.position_label(0), None);
    assert_eq!(view.changes_label(), "· 1 change");
    assert_eq!(view.step(0, true), 0);
}

#[test]
fn counts_follow_the_painted_rows() {
    let view = review_view(
        "a
b
c
", "a
B
X
c
",
    );
    assert_eq!(view.count(Added), 2);
    assert_eq!(view.count(Removed), 1);
    assert_eq!(view.count(Same), 2);
}
