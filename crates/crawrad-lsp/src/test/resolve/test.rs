//! The resolved documentation, in the spec's two shapes.

use super::documentation;
use serde_json::json;

/// What rust-analyzer resolved for `String::len` (captured 2026-10-04):
/// markdown with a fenced example, made plain for the panel.
#[test]
fn markdown_is_made_plain() {
    let doc = json!({ "kind": "markdown", "value":
        "Returns the length of this `String`, in bytes.\n\n# Examples\n\n```rust\nlet a = String::from(\"foo\");\nassert_eq!(a.len(), 3);\n```" });
    let plain = documentation(&doc).expect("a doc");
    assert!(plain.starts_with("Returns the length of this String, in bytes."));
    assert!(plain.contains("Examples"));
    assert!(
        plain.contains("assert_eq!(a.len(), 3);"),
        "the code is kept"
    );
    assert!(!plain.contains("```"), "the fences go");
}

/// A plain string is the other shape; empty is none.
#[test]
fn a_string_and_nothing() {
    assert_eq!(documentation(&json!("plain")).as_deref(), Some("plain"));
    assert_eq!(documentation(&json!("")), None);
    assert_eq!(
        documentation(&json!({ "kind": "markdown", "value": "  " })),
        None
    );
    assert_eq!(documentation(&json!(3)), None);
}
