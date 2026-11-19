//! The delta sent to the server must rebuild exactly the new text.

use super::{SyncKind, apply_change, incremental_change, position_of, sync_kind_from};
use crate::encoding::PositionEncoding::Utf16 as U16;
use crate::protocol::Position;

/// Every pair: the computed change, applied to the old text, gives the new.
fn round_trips(old: &str, new: &str) {
    match incremental_change(old, new, U16) {
        None => assert_eq!(old, new),
        Some(change) => assert_eq!(
            apply_change(old, &change, U16),
            new,
            "{old:?} -> {new:?}: {change:?}"
        ),
    }
}

/// **Typing, deleting and replacing, anywhere, round-trip.**
#[test]
fn edits_round_trip() {
    let base = "fn main() {\n    let s = String::new();\n}\n";
    for new in [
        "fn main() {\n    let s = String::new();\n    s.\n}\n", // typed a line
        "fn main() {\n    let s = String::neww();\n}\n",        // one char inside
        "fn main() {\n}\n",                                     // deleted a line
        "",                                                     // emptied
        "x",                                                    // all replaced
        "fn main() {\n    let s = String::new();\n}\n// end\n", // appended
        "// start\nfn main() {\n    let s = String::new();\n}\n", // prepended
    ] {
        round_trips(base, new);
        round_trips(new, base);
    }
}

/// **The change is small: one key is a one-char text, not the file.**
#[test]
fn one_key_is_one_char() {
    let old = "fn main() {\n    s\n}\n";
    let new = "fn main() {\n    s.\n}\n";
    let change = incremental_change(old, new, U16).unwrap();
    assert_eq!(change.text, ".");
    assert_eq!(
        change.range.start,
        Position {
            line: 1,
            character: 5
        }
    );
    assert_eq!(change.range.end, change.range.start);
}

/// **UTF-16 columns, no split character:** accents, emoji, an edit between
/// two different multi-byte characters.
#[test]
fn multi_byte_text_round_trips_with_utf16_columns() {
    round_trips("let a = \"ção\";", "let a = \"cão\";");
    round_trips("// 🦀 x", "// 🦀 y");
    round_trips("é", "è");
    let change = incremental_change("// 🦀 x", "// 🦀 y", U16).unwrap();
    assert_eq!(
        change.range.start,
        Position {
            line: 0,
            character: 6
        }
    );
    assert_eq!(
        position_of("🦀a", "🦀".len(), U16),
        Position {
            line: 0,
            character: 2
        }
    );
}

/// **CRLF is never split** — a change that starts or ends at a line break
/// keeps `\r\n` whole.
#[test]
fn crlf_is_never_split() {
    for (old, new) in [
        ("a\r\nb\r\n", "a\r\nX\r\nb\r\n"),
        ("a\r\nb", "a\nb"),
        ("a\nb", "a\r\nb"),
        ("a\r\n", "a\r\n\r\n"),
        ("a\r\n\r\nb", "a\r\nb"),
    ] {
        round_trips(old, new);
        let change = incremental_change(old, new, U16).unwrap();
        let start = super::offset_of(old, change.range.start, U16);
        let end = super::offset_of(old, change.range.end, U16);
        for cut in [start, end] {
            assert!(
                !(cut > 0 && old.as_bytes()[cut - 1] == b'\r' && old[cut..].starts_with('\n')),
                "{old:?} -> {new:?} cuts a CRLF at {cut}"
            );
        }
    }
}

/// **The server decides:** a number or `{ change }`; missing stays `Full`.
#[test]
fn the_server_announces_the_kind() {
    use serde_json::json;
    assert_eq!(
        sync_kind_from(&json!({ "textDocumentSync": 2 })),
        SyncKind::Incremental
    );
    assert_eq!(
        sync_kind_from(&json!({ "textDocumentSync": { "openClose": true, "change": 2 } })),
        SyncKind::Incremental
    );
    assert_eq!(
        sync_kind_from(&json!({ "textDocumentSync": 1 })),
        SyncKind::Full
    );
    assert_eq!(
        sync_kind_from(&json!({ "textDocumentSync": 0 })),
        SyncKind::None
    );
    assert_eq!(sync_kind_from(&json!({})), SyncKind::Full);
}

/// **Many small edits in a row stay in step** — the texts the server would
/// hold after each delta equal the editor's.
#[test]
fn a_typing_session_stays_in_step() {
    let mut editor = String::from("fn main() {\n}\n");
    let mut server = editor.clone();
    let typed = "    let total = 1 + 2;\n";
    for (i, c) in typed.char_indices() {
        let at = "fn main() {\n".len() + i;
        editor.insert(at, c);
        let change = incremental_change(&server, &editor, U16).unwrap();
        server = apply_change(&server, &change, U16);
        assert_eq!(server, editor);
    }
    while editor.contains("1 + 2") {
        let at = editor.find(" + 2").unwrap() + 4;
        editor.remove(at - 1);
        let change = incremental_change(&server, &editor, U16).unwrap();
        server = apply_change(&server, &change, U16);
        assert_eq!(server, editor);
    }
}
