//! The read budget: whole characters, and whether more is left.

use super::{read_answer, slice_utf8_budget};

/// **A page never splits a character, and says when text is left.**
#[test]
fn pages_keep_characters_whole() {
    let text = "ab\u{1F980}cd";
    assert_eq!(
        slice_utf8_budget(text, 0, 3),
        ("ab".to_string(), true),
        "the crab does not fit"
    );
    assert_eq!(
        slice_utf8_budget(text, 0, 6),
        ("ab\u{1F980}".to_string(), true)
    );
    assert_eq!(
        slice_utf8_budget(text, 3, 100),
        ("\u{1F980}cd".to_string(), false),
        "an offset inside a char goes back"
    );
    assert_eq!(slice_utf8_budget(text, 99, 5), (String::new(), false));
}

/// A paged read's answer: the slice asked for, the whole text's size, and
/// the pending review said in the sentence.
#[test]
fn a_read_answers_its_slice_and_the_whole() {
    let args = serde_json::json!({ "offset": 2, "limit": 3 });
    let (a, said) = read_answer("a.rs", "abcdef\ng", "h1", &args, Some("h2".into()));
    assert_eq!(a["content"], "cde");
    assert_eq!(a["has_more"], true);
    assert_eq!(a["total_bytes"], 8);
    assert_eq!(a["lines_count"], 2);
    assert_eq!(a["predicted_hash"], "h2");
    assert!(said.contains("pending diff review"));
    let (b, said) = read_answer("a.rs", "x", "h", &serde_json::json!({}), None);
    assert_eq!(
        (b["content"].as_str(), b["pending_diff_review"].as_bool()),
        (Some("x"), Some(false))
    );
    assert!(!said.contains("NOTE"));
}
