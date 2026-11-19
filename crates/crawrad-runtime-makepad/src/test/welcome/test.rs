use super::*;

/// The note under the box says what the box does now, both ways, and points
/// at the same box in Options.
#[test]
fn the_reopen_note_follows_the_box() {
    assert!(reopen_note(true).starts_with("On: the last project opens"));
    assert!(reopen_note(false).starts_with("Off: this page opens"));
    for on in [true, false] {
        assert!(reopen_note(on).ends_with("Also in Options › IDE › Saving and Recovering."));
    }
}

/// The list grows with its rows and stops at eight, then scrolls.
#[test]
fn the_recent_list_is_as_tall_as_its_rows() {
    assert_eq!(list_height(0), 50.0);
    assert_eq!(list_height(3), 150.0);
    assert_eq!(list_height(8), 400.0);
    assert_eq!(list_height(30), 400.0);
}
