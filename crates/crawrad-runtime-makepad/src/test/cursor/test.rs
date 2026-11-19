use super::*;

/// Every name the generated `Cursor` writes has a pointer, but `default`.
#[test]
fn every_cursor_name_but_default_has_a_pointer() {
    let names = [
        "arrow",
        "hand",
        "text",
        "crosshair",
        "move",
        "wait",
        "not_allowed",
        "resize_ns",
        "resize_ew",
        "resize_nesw",
        "resize_nwse",
    ];
    for n in names {
        assert!(mouse(n).is_some(), "{n}");
    }
    assert_eq!(mouse("default"), None);
    assert_eq!(mouse("hand"), Some(MouseCursor::Hand));
}
