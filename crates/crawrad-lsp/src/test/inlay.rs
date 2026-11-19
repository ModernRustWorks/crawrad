use super::{InlayHint, end_of, placed, utf16_to_byte};
use crate::encoding::PositionEncoding::Utf16 as U16;

#[test]
fn both_label_shapes_rust_analyzer_sends_are_read() {
    let as_text: InlayHint = serde_json::from_value(serde_json::json!({
        "position": { "line": 0, "character": 9 },
        "label": ": usize",
        "kind": 1
    }))
    .unwrap();
    assert_eq!(as_text.text(), ": usize");
    let as_parts: InlayHint = serde_json::from_value(serde_json::json!({
        "position": { "line": 1, "character": 12 },
        "label": [{ "value": "path" }, { "value": ":" }],
        "kind": 2,
        "paddingRight": true
    }))
    .unwrap();
    assert_eq!(
        as_parts.text(),
        "path: ",
        "a parameter name keeps its right padding"
    );
}

#[test]
fn a_utf16_column_lands_on_the_right_byte_past_accented_and_astral_text() {
    // `é` is 2 bytes and 1 UTF-16 unit; `🦀` is 4 bytes and 2 units.
    let line = "let é🦀x = 1;";
    assert_eq!(utf16_to_byte(line, 4), 4, "before the é");
    assert_eq!(utf16_to_byte(line, 5), 6, "after the é");
    assert_eq!(utf16_to_byte(line, 7), 10, "after the crab");
    assert_eq!(
        utf16_to_byte(line, 99),
        line.len(),
        "past the end is the end"
    );
}

#[test]
fn hints_are_placed_on_their_line_and_byte() {
    let hint: InlayHint = serde_json::from_value(serde_json::json!({
        "position": { "line": 1, "character": 13 },
        "label": ": usize"
    }))
    .unwrap();
    let text = "fn f() {\n    let total = 3;\n}";
    assert_eq!(
        placed(std::slice::from_ref(&hint), text, U16),
        vec![(1, 13, ": usize".to_string())]
    );
    let off_the_end = InlayHint {
        position: position_at(9, 0),
        ..hint
    };
    assert!(
        placed(&[off_the_end], text, U16).is_empty(),
        "a line that does not exist is dropped"
    );
}

fn position_at(line: u32, character: u32) -> crate::protocol::Position {
    crate::protocol::Position { line, character }
}

#[test]
fn the_range_ends_on_the_last_real_line_not_one_past_it() {
    // rust-analyzer refused `line: 31` for a file of 31 lines (indices 0..=30).
    let end = end_of("fn a() {}\nlet é = 1;", U16);
    assert_eq!((end.line, end.character), (1, 10));
    let trailing = end_of("a\nb\n", U16);
    assert_eq!(
        (trailing.line, trailing.character),
        (2, 0),
        "after the last newline is an empty line"
    );
}

#[test]
fn the_server_progress_is_followed_and_its_end_asks_for_the_hints_again() {
    use crate::client::track_progress;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};
    let work = Mutex::new(BTreeMap::new());
    let refresh = AtomicBool::new(false);
    let msg = |kind: &str, extra: &str| {
        format!(
            r#"{{"jsonrpc":"2.0","method":"$/progress","params":{{"token":"rustAnalyzer/Indexing","value":{{"kind":"{kind}"{extra}}}}}}}"#
        )
    };
    assert!(track_progress(
        &msg("begin", r#","title":"Indexing""#),
        &work,
        &refresh
    ));
    assert!(track_progress(
        &msg("report", r#","message":"120/340","percentage":35"#),
        &work,
        &refresh
    ));
    let shown = |w: &Mutex<BTreeMap<String, String>>| {
        let w = w.lock().unwrap();
        let line = w.values().next().cloned().unwrap_or_default();
        line.split_once('\u{1}')
            .map(|(_, s)| s.to_string())
            .unwrap_or(line)
    };
    assert_eq!(shown(&work), "Indexing 120/340 (35%)");
    assert!(track_progress(
        &msg("report", r#","message":"130/340","percentage":38"#),
        &work,
        &refresh
    ));
    assert_eq!(
        shown(&work),
        "Indexing 130/340 (38%)",
        "a report replaces, never appends"
    );
    assert!(track_progress(
        &msg("report", r#","message":"loading: C:\\x\\y.dll""#),
        &work,
        &refresh
    ));
    assert_eq!(shown(&work), "Indexing", "a path is not shown");
    assert!(!refresh.load(Ordering::SeqCst));
    assert!(track_progress(&msg("end", ""), &work, &refresh));
    assert!(work.lock().unwrap().is_empty());
    assert!(
        refresh.load(Ordering::SeqCst),
        "the end of the work asks for the hints again"
    );
    assert!(!track_progress(
        r#"{"jsonrpc":"2.0","method":"window/logMessage","params":{}}"#,
        &work,
        &refresh
    ));
}
