//! Tests for the LSP client and protocol subsystems.

use crate::protocol::{
    CompletionItemKind, CompletionResponse, DiagnosticSeverity, Documentation, Hover, Position,
    PublishDiagnosticsParams, Range,
};
use crate::uri::{path_to_uri, uri_to_path};
use std::path::PathBuf;

#[test]
fn test_position_and_range_serialization() {
    let range = Range::new(Position::new(10, 4), Position::new(10, 15));
    let json = serde_json::to_string(&range).unwrap();
    let deserialized: Range = serde_json::from_str(&json).unwrap();
    assert_eq!(range, deserialized);
}

#[test]
fn test_diagnostic_severity_mapping() {
    assert_eq!(DiagnosticSeverity::from(1), DiagnosticSeverity::Error);
    assert_eq!(DiagnosticSeverity::from(2), DiagnosticSeverity::Warning);
    assert_eq!(DiagnosticSeverity::from(3), DiagnosticSeverity::Information);
    assert_eq!(DiagnosticSeverity::from(4), DiagnosticSeverity::Hint);
}

#[test]
fn test_publish_diagnostics_deserialization() {
    let json_data = r#"{
        "uri": "file:///workspace/src/lib.rs",
        "version": 1,
        "diagnostics": [
            {
                "range": {
                    "start": { "line": 4, "character": 8 },
                    "end": { "line": 4, "character": 12 }
                },
                "severity": 1,
                "code": "E0425",
                "source": "rustc",
                "message": "cannot find value `foo` in this scope"
            }
        ]
    }"#;

    let params: PublishDiagnosticsParams = serde_json::from_str(json_data).unwrap();
    assert_eq!(params.uri, "file:///workspace/src/lib.rs");
    assert_eq!(params.diagnostics.len(), 1);
    assert_eq!(
        params.diagnostics[0].severity,
        Some(DiagnosticSeverity::Error)
    );
    assert_eq!(params.diagnostics[0].code.as_deref(), Some("E0425"));
    assert_eq!(
        params.diagnostics[0].message,
        "cannot find value `foo` in this scope"
    );
}

#[test]
fn test_completion_response_list_and_array() {
    let list_json = r#"{
        "isIncomplete": false,
        "items": [
            {
                "label": "clone",
                "kind": 2,
                "detail": "fn clone(&self) -> Self",
                "documentation": "Returns a copy of the value."
            }
        ]
    }"#;

    let resp: CompletionResponse = serde_json::from_str(list_json).unwrap();
    let items = resp.into_items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].label, "clone");
    assert_eq!(items[0].kind, Some(CompletionItemKind::Method));
    assert_eq!(items[0].detail.as_deref(), Some("fn clone(&self) -> Self"));
    assert_eq!(
        items[0].documentation.as_ref().map(Documentation::as_str),
        Some("Returns a copy of the value.")
    );
}

#[test]
fn test_hover_deserialization() {
    let hover_json = r#"{
        "contents": {
            "kind": "markdown",
            "value": "```rust\npub fn println() -> ()\n```\nPrints to the standard output."
        },
        "range": {
            "start": { "line": 5, "character": 4 },
            "end": { "line": 5, "character": 11 }
        }
    }"#;

    let hover: Hover = serde_json::from_str(hover_json).unwrap();
    assert_eq!(
        hover.contents.as_str(),
        "```rust\npub fn println() -> ()\n```\nPrints to the standard output."
    );
    assert!(hover.range.is_some());
}

#[test]
fn test_path_uri_roundtrip() {
    let p = PathBuf::from(if cfg!(windows) {
        r"C:\CrawRAD\crates\crawrad-lsp\src\lib.rs"
    } else {
        "/home/user/CrawRAD/crates/crawrad-lsp/src/lib.rs"
    });

    let uri = path_to_uri(&p);
    assert!(uri.starts_with("file://"));

    let recovered = uri_to_path(&uri).expect("must parse back to path");
    assert_eq!(recovered, p);
}
