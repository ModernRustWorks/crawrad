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
///
/// The row height is read from `RECENT_ROW_HEIGHT` rather than written here as
/// a number: it used to be the literal 50.0, which is the same number the
/// live_design declares on the row, repeated. `recent_row_guard` now holds the
/// two together, and this test keeps checking the relationship -- an empty list
/// still reserves one row, and past eight the list scrolls instead of growing.
#[test]
fn the_recent_list_is_as_tall_as_its_rows() {
    assert_eq!(list_height(0), RECENT_ROW_HEIGHT);
    assert_eq!(list_height(3), 3.0 * RECENT_ROW_HEIGHT);
    assert_eq!(list_height(8), 8.0 * RECENT_ROW_HEIGHT);
    assert_eq!(list_height(30), 8.0 * RECENT_ROW_HEIGHT);
}
