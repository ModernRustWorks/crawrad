//! Pull diagnostics: a file's list is the pushed plus the pulled, the
//! newest answer wins, a closed file is not asked again.

use super::{State, full_items};
use serde_json::{Value, json};

/// What rust-analyzer 0.3.3065 answered for a `#[cfg(feature = "extra")]`
/// item (`textDocument/diagnostic`, measured 2026-10-26).
fn inactive() -> Value {
    json!({ "range": { "start": { "line": 14, "character": 0 }, "end": { "line": 16, "character": 22 } },
            "severity": 4, "source": "rust-analyzer", "code": "inactive_code", "tags": [1],
            "message": "code is inactive due to #[cfg] directives: feature = \"extra\" is disabled" })
}

#[test]
fn a_file_lists_the_pushed_then_the_pulled() {
    let mut s = State::default();
    let rustc = json!({ "message": "function `never_called` is never used", "source": "rustc" });
    s.asking("c:/p/src/main.rs", "file:///C:/p/src/main.rs", 1);
    assert!(s.pulled("c:/p/src/main.rs", 1, vec![inactive()]));
    s.pushed("c:/p/src/main.rs", vec![rustc.clone()]);
    assert_eq!(
        s.merged("c:/p/src/main.rs"),
        vec![rustc.clone(), inactive()]
    );
    // `cargo check` again: the pushed list is replaced, the pulled stays.
    s.pushed("c:/p/src/main.rs", vec![]);
    assert_eq!(s.merged("c:/p/src/main.rs"), vec![inactive()]);
}

/// Answers can cross: one asked at an older version does not replace a
/// newer one.
#[test]
fn the_newest_answer_wins() {
    let mut s = State::default();
    assert!(s.pulled("k", 3, vec![inactive()]));
    assert!(!s.pulled("k", 2, vec![]), "version 2 came late");
    assert_eq!(s.merged("k"), vec![inactive()]);
    assert!(s.pulled("k", 4, vec![]));
    assert!(s.merged("k").is_empty(), "the item was turned on again");
}

#[test]
fn a_closed_file_is_not_asked_again() {
    let mut s = State::default();
    s.asking("a", "file:///a", 1);
    s.asking("b", "file:///b", 7);
    s.pulled("a", 1, vec![inactive()]);
    s.closed("a");
    assert_eq!(
        s.open(),
        vec![("b".to_string(), "file:///b".to_string(), 7)]
    );
    assert!(s.merged("a").is_empty());
}

/// `full` carries the items; `unchanged` (or anything else) keeps what is
/// there.
#[test]
fn only_a_full_report_replaces() {
    let full = json!({ "kind": "full", "resultId": "1", "items": [inactive()] });
    assert_eq!(full_items(&full), Some(vec![inactive()]));
    assert_eq!(
        full_items(&json!({ "kind": "unchanged", "resultId": "1" })),
        None
    );
    assert_eq!(full_items(&Value::Null), None);
}
