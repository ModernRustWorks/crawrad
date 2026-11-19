//! The per-key answer of `set_component_properties`.

use super::{Outcome, unknown_key};

#[test]
fn an_undeclared_key_is_refused_with_the_class_keys() {
    let keys = ["text", "width", "alignment"];
    let why = unknown_key("ListColumn", &keys, "caption").expect("refused");
    assert!(
        why.contains("ListColumn has no property `caption`"),
        "{why}"
    );
    assert!(why.contains("text, width, alignment"), "{why}");
    assert_eq!(unknown_key("ListColumn", &keys, "text"), None);
    assert_eq!(
        unknown_key("ListColumn", &keys, "font.size"),
        None,
        "a nested path is tried"
    );
    assert_eq!(
        unknown_key("Gadget", &[], "anything"),
        None,
        "no known keys: tried"
    );
}

#[test]
fn the_count_is_what_was_applied_not_what_was_sent() {
    let mut out = Outcome::default();
    out.apply("hint");
    out.refuse("caption", "no such key");
    let (data, said) = out.answer("button1", 2, 7);
    assert_eq!(data["properties_updated"], 1);
    assert_eq!(data["applied"][0], "hint");
    assert_eq!(data["refused"][0]["key"], "caption");
    assert_eq!(data["status"], "partial");
    assert!(said.starts_with("Updated 1 of 2 properties"), "{said}");
    assert!(!out.failed());

    let mut none = Outcome::default();
    none.refuse("caption", "no such key");
    assert!(none.failed(), "nothing applied is a failed call");
}
