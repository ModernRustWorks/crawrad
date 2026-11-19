//! Rename and format edits: read from the server's shapes, applied to text.

use super::{TextEdit, apply_text_edits, workspace_edit_from};
use crate::encoding::PositionEncoding::Utf16 as U16;
use crate::protocol::{Position, Range};

fn edit(line: u32, start: u32, end_line: u32, end: u32, new_text: &str) -> TextEdit {
    TextEdit {
        range: Range {
            start: Position {
                line,
                character: start,
            },
            end: Position {
                line: end_line,
                character: end,
            },
        },
        new_text: new_text.to_string(),
    }
}

/// **A rename's edits all land, even on the same line, from the last back.**
#[test]
fn edits_on_one_line_apply_from_the_last() {
    let text = "let total = total + 1;\nprintln!(\"{total}\");\n";
    let out = apply_text_edits(
        text,
        &[
            edit(0, 4, 0, 9, "sum"),
            edit(0, 12, 0, 17, "sum"),
            edit(1, 11, 1, 16, "sum"),
        ],
        U16,
    );
    assert_eq!(out, "let sum = sum + 1;\nprintln!(\"{sum}\");\n");
}

/// **Columns are UTF-16: a name after `naïve` and an emoji lands on the name.**
#[test]
fn columns_count_utf16_not_bytes() {
    // "ï" = 1 UTF-16 unit (2 bytes); "🦀" = 2 units (4 bytes).
    let text = "// naïve 🦀 x\nlet x = 1;\n";
    let out = apply_text_edits(text, &[edit(0, 12, 0, 13, "y"), edit(1, 4, 1, 5, "y")], U16);
    assert_eq!(out, "// naïve 🦀 y\nlet y = 1;\n");
}

/// **A format edit that spans lines, on a CRLF file, keeps the CRLF it did
/// not touch.**
#[test]
fn a_multi_line_edit_on_crlf_text() {
    let text = "fn a(){\r\nlet x=1;\r\n}\r\n";
    let out = apply_text_edits(text, &[edit(0, 6, 2, 0, " {\r\n    let x = 1;\r\n")], U16);
    assert_eq!(out, "fn a() {\r\n    let x = 1;\r\n}\r\n");
}

/// **Both shapes of `WorkspaceEdit` give the same edits by file.**
#[test]
fn changes_and_document_changes_read_the_same() {
    let changes = serde_json::json!({ "changes": { "file:///a.rs": [
        { "range": { "start": { "line": 0, "character": 4 }, "end": { "line": 0, "character": 9 } }, "newText": "sum" }
    ]}});
    let document_changes = serde_json::json!({ "documentChanges": [
        { "textDocument": { "uri": "file:///a.rs", "version": 3 }, "edits": [
            { "range": { "start": { "line": 0, "character": 4 }, "end": { "line": 0, "character": 9 } }, "newText": "sum" }
        ]}
    ]});
    let a = workspace_edit_from(changes).unwrap();
    let b = workspace_edit_from(document_changes).unwrap();
    assert_eq!(a, b);
    assert_eq!(a["file:///a.rs"], vec![edit(0, 4, 0, 9, "sum")]);
}

/// **A file operation is refused whole, never half-applied.**
#[test]
fn a_file_rename_in_the_edit_is_refused() {
    let value = serde_json::json!({ "documentChanges": [
        { "textDocument": { "uri": "file:///a.rs", "version": 1 }, "edits": [] },
        { "kind": "rename", "oldUri": "file:///a.rs", "newUri": "file:///b.rs" }
    ]});
    let err = workspace_edit_from(value).unwrap_err();
    assert!(err.contains("rename"), "{err}");
}
