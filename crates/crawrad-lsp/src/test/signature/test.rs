//! Signature help: the answer read, the parameter found in the label.

use super::{Trigger, parse, triggers_from};
use serde_json::json;

/// What rust-analyzer sent (captured 2026-10-03) for `soma(1, |` with
/// `labelOffsetSupport`: offsets in UTF-16 units, not bytes.
#[test]
fn offsets_are_utf16_units_not_bytes() {
    let help = json!({
        "signatures": [{
            "label": "fn soma(ação: i32, b: &str) -> i32",
            "parameters": [{ "label": [8, 17] }, { "label": [19, 26] }],
            "activeParameter": 1
        }],
        "activeSignature": 0,
        "activeParameter": 1
    });
    let s = parse(&help).expect("a signature");
    assert_eq!(&s.label[s.params[0].clone()], "ação: i32");
    assert_eq!(&s.label[s.params[1].clone()], "b: &str");
    assert_eq!(s.active, Some(1));
    assert_eq!(s.parts(), ("fn soma(ação: i32, ", "b: &str", ") -> i32"));
}

/// The same answer without `labelOffsetSupport`: substrings, found in order
/// so a repeated text marks the right one.
#[test]
fn string_labels_are_found_in_order() {
    let help = json!({
        "signatures": [{
            "label": "fn f(a: i32, a: i32)",
            "parameters": [{ "label": "a: i32" }, { "label": "a: i32" }]
        }],
        "activeParameter": 1
    });
    let s = parse(&help).expect("a signature");
    assert_eq!(s.params, vec![5..11, 13..19]);
    assert_eq!(s.parts().1, "a: i32");
    assert_eq!(s.parts().0, "fn f(a: i32, ");
}

/// **The signature's own `activeParameter` wins** (LSP 3.16), and the chosen
/// signature is `activeSignature`.
#[test]
fn the_signature_says_its_own_active_parameter() {
    let help = json!({
        "signatures": [
            { "label": "fn a(x: u8)", "parameters": [{ "label": "x: u8" }] },
            { "label": "fn b(x: u8, y: u8)",
              "parameters": [{ "label": "x: u8" }, { "label": "y: u8" }],
              "activeParameter": 0 }
        ],
        "activeSignature": 1,
        "activeParameter": 1
    });
    let s = parse(&help).expect("a signature");
    assert_eq!(s.label, "fn b(x: u8, y: u8)");
    assert_eq!(s.active, Some(0));
    assert_eq!(s.count, 2);
}

/// **No call, no card:** `null`, no signatures. An active index past the
/// parameters marks nothing rather than panicking.
#[test]
fn nothing_to_show() {
    assert_eq!(parse(&json!(null)), None);
    assert_eq!(parse(&json!({ "signatures": [] })), None);
    let s = parse(&json!({
        "signatures": [{ "label": "fn f()", "parameters": [] }],
        "activeParameter": 3
    }))
    .expect("a signature");
    assert_eq!(s.active, None);
    assert_eq!(s.parts(), ("fn f()", "", ""));
}

/// The request's `context` (LSP `SignatureHelpContext`).
#[test]
fn the_context_says_why() {
    assert_eq!(
        Trigger::Character(',').context(true),
        json!({ "triggerKind": 2, "triggerCharacter": ",", "isRetrigger": true })
    );
    assert_eq!(
        Trigger::ContentChange.context(true),
        json!({ "triggerKind": 3, "isRetrigger": true })
    );
    assert_eq!(
        Trigger::Invoked.context(false),
        json!({ "triggerKind": 1, "isRetrigger": false })
    );
}

/// The server's characters, both lists.
#[test]
fn the_triggers_come_from_the_server() {
    let caps = json!({ "signatureHelpProvider": { "triggerCharacters": ["(", ",", "<"] } });
    assert_eq!(triggers_from(Some(&caps)), vec!["(", ",", "<"]);
    let both = json!({ "signatureHelpProvider": {
        "triggerCharacters": ["("], "retriggerCharacters": [","] } });
    assert_eq!(triggers_from(Some(&both)), vec!["(", ","]);
    assert!(triggers_from(None).is_empty());
}
