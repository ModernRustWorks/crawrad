//! The caption a menu item shows.

use super::{caption, entry, without_accelerator};

#[test]
fn the_accelerator_mark_is_not_shown() {
    assert_eq!(without_accelerator("&File"), "File");
    assert_eq!(without_accelerator("Save && Close"), "Save & Close");
    assert_eq!(caption("&Open", false, ""), "    Open");
    assert_eq!(entry("&Edit", false, ""), "Edit");
}

#[test]
fn the_caption_carries_the_mark_and_the_shortcut() {
    assert_eq!(caption("Save", false, ""), "    Save");
    assert_eq!(caption("Save", false, "Ctrl+S"), "    Save    Ctrl+S");
    assert_eq!(caption("Wrap", true, ""), "\u{2713} Wrap");
}

#[test]
fn a_bar_entry_takes_no_room_for_an_absent_mark() {
    assert_eq!(entry("File", false, ""), "File");
    assert_eq!(entry("Wrap", true, "Ctrl+W"), "\u{2713} Wrap    Ctrl+W");
}
