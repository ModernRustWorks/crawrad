use super::*;

#[test]
fn format_changes_text_formats_blocks_and_current_correctly() {
    assert_eq!(format_changes_text(0, None), "No changes");
    assert_eq!(format_changes_text(0, Some(0)), "No changes");

    assert_eq!(format_changes_text(1, None), "1 change · block 1 of 1");
    assert_eq!(format_changes_text(1, Some(0)), "1 change · block 1 of 1");

    assert_eq!(format_changes_text(3, None), "3 changes · block 1 of 3");
    assert_eq!(format_changes_text(3, Some(0)), "3 changes · block 1 of 3");
    assert_eq!(format_changes_text(3, Some(1)), "3 changes · block 2 of 3");
    assert_eq!(format_changes_text(3, Some(2)), "3 changes · block 3 of 3");
}

#[test]
fn can_navigate_blocks_only_when_blocks_exist() {
    assert!(!can_navigate_blocks(0));
    assert!(can_navigate_blocks(1));
    assert!(can_navigate_blocks(5));
}

#[test]
fn origin_badge_colors_distinguish_save_and_external() {
    let (text_save, bg_save) = origin_badge_colors("Save");
    let (text_ext, bg_ext) = origin_badge_colors("External");

    assert_ne!(text_save, text_ext);
    assert_ne!(bg_save, bg_ext);

    // A save wears the IDE accent; an external change the IDE's info blue.
    assert_eq!(text_save[0], 1.0);
    assert_eq!(text_ext[0], 179.0 / 255.0);
}

#[test]
fn history_mode_as_str_returns_canonical_identifiers() {
    assert_eq!(HistoryMode::Information.as_str(), "information");
    assert_eq!(HistoryMode::Contents.as_str(), "contents");
    assert_eq!(HistoryMode::Differences.as_str(), "differences");
}

#[test]
fn unbound_history_panel_ref_is_safe_noop() {
    let panel_ref = CrawradHistoryPanelRef::default();
    assert!(panel_ref.borrow().is_none());
}
