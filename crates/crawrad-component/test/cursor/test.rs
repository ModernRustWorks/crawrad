use crate::Cursor;
use std::str::FromStr;

#[test]
fn default_cursor_is_default() {
    assert_eq!(Cursor::default(), Cursor::Default);
    assert_eq!(Cursor::Default.as_str(), "default");
}

#[test]
fn cursor_as_str_and_display() {
    assert_eq!(Cursor::Hand.as_str(), "hand");
    assert_eq!(format!("{}", Cursor::Hand), "hand");
    assert_eq!(Cursor::Text.as_str(), "text");
    assert_eq!(Cursor::Move.as_str(), "move");
    assert_eq!(Cursor::Wait.as_str(), "wait");
    assert_eq!(Cursor::NotAllowed.as_str(), "not_allowed");
    assert_eq!(Cursor::ResizeNs.as_str(), "resize_ns");
    assert_eq!(Cursor::ResizeEw.as_str(), "resize_ew");
    assert_eq!(Cursor::ResizeNesw.as_str(), "resize_nesw");
    assert_eq!(Cursor::ResizeNwse.as_str(), "resize_nwse");
}

#[test]
fn cursor_from_str_and_aliases() {
    assert_eq!(Cursor::from_str("default"), Ok(Cursor::Default));
    assert_eq!(Cursor::from_str("pointer"), Ok(Cursor::Hand));
    assert_eq!(Cursor::from_str("hand"), Ok(Cursor::Hand));
    assert_eq!(Cursor::from_str("ibeam"), Ok(Cursor::Text));
    assert_eq!(Cursor::from_str("text"), Ok(Cursor::Text));
    assert_eq!(Cursor::from_str("hourglass"), Ok(Cursor::Wait));
    assert_eq!(Cursor::from_str("nodrop"), Ok(Cursor::NotAllowed));
    assert_eq!(Cursor::from_str("sizeall"), Ok(Cursor::Move));
    assert_eq!(Cursor::from_str("unknown_cursor"), Err(()));
}
