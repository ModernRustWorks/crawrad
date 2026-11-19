//! The diagnostics as the editor writes them after their lines.

use super::{
    Diagnostic, DiagnosticSeverity, Location, Position, Range, line_messages, locations_from, marks,
};

fn diag(line: u32, severity: Option<DiagnosticSeverity>, message: &str) -> Diagnostic {
    let at = Position { line, character: 0 };
    Diagnostic {
        range: Range { start: at, end: at },
        severity,
        code: None,
        source: None,
        message: message.to_string(),
        tags: Vec::new(),
    }
}

/// **By line, the worst first; no severity counts as "the rest".**
#[test]
fn messages_come_by_line_worst_first() {
    let found = line_messages(&[
        diag(4, Some(DiagnosticSeverity::Warning), "unused variable"),
        diag(4, Some(DiagnosticSeverity::Error), "mismatched types"),
        diag(1, None, "a note"),
    ]);
    assert_eq!(
        found,
        vec![
            (1, "a note".to_string(), 2),
            (4, "mismatched types".to_string(), 0),
            (4, "unused variable".to_string(), 1),
        ]
    );
}

fn span(line: u32, start: u32, end: u32) -> Range {
    Range {
        start: Position {
            line,
            character: start,
        },
        end: Position {
            line,
            character: end,
        },
    }
}

/// **rust-analyzer answers Go to Definition with `LocationLink[]`, because
/// the client announces `linkSupport`. The link lands on the NAME
/// (`targetSelectionRange`), not on the whole item.** Before, only
/// `Location` was read and the IDE got an empty list (measured live).
#[test]
fn a_location_link_lands_on_the_name() {
    let value = serde_json::json!([{
        "originSelectionRange": span(5, 12, 18),
        "targetUri": "file:///c:/p/src/main.rs",
        "targetRange": span(0, 0, 25),
        "targetSelectionRange": span(0, 3, 9)
    }]);
    assert_eq!(
        locations_from(value),
        vec![Location {
            uri: "file:///c:/p/src/main.rs".to_string(),
            range: span(0, 3, 9)
        }]
    );
}

/// **The other shapes the LSP allows: one `Location`, `Location[]`, `null`.**
#[test]
fn one_location_a_list_and_null() {
    let one = serde_json::json!({ "uri": "file:///a.rs", "range": span(1, 2, 3) });
    let expected = vec![Location {
        uri: "file:///a.rs".to_string(),
        range: span(1, 2, 3),
    }];
    assert_eq!(locations_from(one.clone()), expected);
    assert_eq!(locations_from(serde_json::json!([one])), expected);
    assert!(locations_from(serde_json::Value::Null).is_empty());
}

/// **rust-analyzer's item: what goes into the code is the edit's text, not
/// the label; the grey detail is the label's description.** Before, the
/// label (with `(as Clone)` glued on) was inserted as is.
#[test]
fn a_completion_item_inserts_its_edit_and_shows_its_details() {
    let item: super::CompletionItem = serde_json::from_value(serde_json::json!({
        "label": "clone",
        "kind": 2,
        "labelDetails": { "detail": "(as Clone)", "description": "fn(&self) -> String" },
        "sortText": "7fffffff",
        "textEdit": { "range": span(0, 4, 4), "newText": "clone()" }
    }))
    .unwrap();
    assert_eq!(item.text_to_insert(), "clone()");
    assert_eq!(item.signature().as_deref(), Some("fn(&self) -> String"));
    assert_eq!(item.sort_key(), "7fffffff");
    let plain: super::CompletionItem =
        serde_json::from_value(serde_json::json!({ "label": "len" })).unwrap();
    assert_eq!(plain.text_to_insert(), "len");
    assert_eq!(plain.sort_key(), "len");
}

/// **One mark per range, merged.** What rust-analyzer published for
/// `let unused_zz = 1;` (captured 2026-10-04): rustc's Warning with tag 1 and
/// a Hint without tags, same range — the fade must survive the Hint. A
/// deprecated call is its own range, struck; an error wins the squiggle.
#[test]
fn marks_merge_a_range_and_keep_its_tags() {
    let mut unused = diag(4, Some(DiagnosticSeverity::Warning), "unused variable");
    unused.tags = vec![1];
    let hint = diag(
        4,
        Some(DiagnosticSeverity::Hint),
        "prefix it with an underscore",
    );
    let mut old = diag(
        5,
        Some(DiagnosticSeverity::Warning),
        "use of deprecated function",
    );
    old.tags = vec![2];
    let error = diag(5, Some(DiagnosticSeverity::Error), "mismatched types");
    let found = marks(&[unused, hint, old, error]);
    assert_eq!(found.len(), 2, "one per range");
    assert!(found[0].faded && !found[0].struck && !found[0].error);
    assert_eq!(
        found[0].level(),
        1,
        "the Warning keeps its squiggle under the Hint"
    );
    assert!(
        found[1].struck && found[1].error,
        "the error wins the squiggle, the strike stays"
    );
    assert_eq!(found[1].level(), 2);
    let raw = serde_json::json!({ "range": { "start": { "line": 0, "character": 0 },
        "end": { "line": 0, "character": 1 } }, "message": "m", "tags": [2] });
    let read: Diagnostic = serde_json::from_value(raw).expect("tags read");
    assert_eq!(read.tags, vec![2]);
}

/// **rust-analyzer's `inactive_code`** — a Hint tagged Unnecessary
/// (pulled, measured 2026-10-26) — fades with no squiggle and writes nothing
/// after its line; a Hint with no tag still writes its help.
#[test]
fn an_inactive_item_only_fades() {
    let mut inactive = diag(14, Some(DiagnosticSeverity::Hint), "code is inactive");
    inactive.tags = vec![1];
    let help = diag(20, Some(DiagnosticSeverity::Hint), "required by a bound");
    let found = marks(&[inactive.clone(), help.clone()]);
    assert!(found[0].faded && found[0].hint);
    assert_eq!(found[0].level(), 0, "no squiggle");
    let lines: Vec<usize> = line_messages(&[inactive, help])
        .iter()
        .map(|m| m.0)
        .collect();
    assert_eq!(lines, vec![20]);
}

/// **A deprecated completion item**, as rust-analyzer 0.3.3065 sent
/// `stale`: tag 1 and the older `deprecated` field; either one is enough.
#[test]
fn a_deprecated_completion_item_is_known() {
    let read = |v| serde_json::from_value::<super::CompletionItem>(v).unwrap();
    let stale = read(serde_json::json!({ "label": "stale", "tags": [1], "deprecated": true }));
    assert!(stale.is_deprecated());
    assert!(read(serde_json::json!({ "label": "old", "deprecated": true })).is_deprecated());
    assert!(read(serde_json::json!({ "label": "old", "tags": [1] })).is_deprecated());
    assert!(!read(serde_json::json!({ "label": "fresh" })).is_deprecated());
}
