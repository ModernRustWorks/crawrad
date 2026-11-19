use super::{clamp, typed_at};

#[test]
fn the_limit_counts_characters_not_bytes() {
    assert_eq!(clamp("ação!", 3), "açã");
    assert_eq!(clamp("abc", 0), "abc", "0 is no limit");
    assert_eq!(clamp("ab", 5), "ab");
}

#[test]
fn a_typed_key_replaces_the_selection() {
    assert_eq!(
        typed_at("ab|cd", 2, 3, "\n", 0),
        Some(("ab\ncd".to_string(), 3))
    );
    assert_eq!(
        typed_at("abcd", 4, 4, "\t", 0),
        Some(("abcd\t".to_string(), 5))
    );
}

#[test]
fn a_full_memo_takes_no_more() {
    assert_eq!(typed_at("abc", 3, 3, "\n", 3), None);
    // replacing a selection frees its room
    assert_eq!(
        typed_at("abc", 1, 2, "\n", 3),
        Some(("a\nc".to_string(), 2))
    );
}
