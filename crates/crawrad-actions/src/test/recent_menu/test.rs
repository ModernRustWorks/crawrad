//! The File menu's Recent submenu.
//!
//! What matters is not that a row exists but that CLICKING IT OPENS THAT PROJECT:
//! the row has to carry its own folder, because `project.open_folder` without a
//! `path` opens the folder picker instead — which is a different command wearing
//! the same name.
use crate::menu;
use std::path::PathBuf;

fn items() -> Vec<(String, PathBuf)> {
    vec![
        (
            "crawrad  —  E:/DeveloperRust".to_string(),
            PathBuf::from("E:/DeveloperRust/CrawRAD"),
        ),
        (
            "oxifisco  —  E:/DeveloperRust".to_string(),
            PathBuf::from("E:/DeveloperRust/Oxifisco"),
        ),
    ]
}

#[test]
fn file_offers_a_row_that_opens_the_recent_submenu() {
    let row = menu::recent_header_row();
    assert_eq!(row.submenu.as_deref(), Some(menu::RECENT));
    assert!(
        row.action.is_none(),
        "the header runs nothing; a row with an action would fire instead of opening"
    );
    assert!(row.enabled, "the row must be clickable, not drawn muted");
}

#[test]
fn each_recent_row_carries_its_own_folder() {
    let rows = menu::recent_rows(&items());
    assert_eq!(rows.len(), 2);
    for (row, (_, folder)) in rows.iter().zip(items()) {
        assert_eq!(
            row.action.as_deref(),
            Some("project.open_folder"),
            "a recent row must run the same command Open Project runs"
        );
        assert_eq!(
            row.args.as_text("path"),
            Some(folder.display().to_string().as_str()),
            "without its own path the row opens the folder PICKER, not the project"
        );
    }
}

#[test]
fn the_rows_keep_the_order_they_were_given() {
    let rows = menu::recent_rows(&items());
    assert!(
        rows[0].label.contains("crawrad") && rows[1].label.contains("oxifisco"),
        "newest first is the caller's order; the menu must not re-sort it"
    );
}

#[test]
fn an_empty_list_draws_a_notice_and_not_an_empty_submenu() {
    let rows = menu::recent_rows(&[]);
    assert_eq!(
        rows.len(),
        1,
        "a submenu that opens onto nothing reads as a bug"
    );
    assert!(!rows[0].enabled, "there is nothing to click");
    assert!(rows[0].action.is_none(), "and nothing to run");
    assert_eq!(rows[0].label, menu::RECENT_NONE);
}

fn file_menu_shaped_like_the_catalogue() -> Vec<menu::Row> {
    let row = |id: &'static str, label: &'static str| menu::Row {
        action: Some(std::borrow::Cow::Borrowed(id)),
        ..menu::Row::custom(label, None)
    };
    vec![
        row("project.new", "New Project..."),
        row("project.open_folder", "Open Project..."),
        row("project.forget_recent", "Remove from Recent"),
        row("file.save", "Save"),
        row("file.close", "Close"),
    ]
}

#[test]
fn the_recent_row_sits_with_the_project_list_rows_not_after_close() {
    let mut rows = file_menu_shaped_like_the_catalogue();
    menu::insert_recent_header(&mut rows);
    let at = rows
        .iter()
        .position(|r| r.submenu.as_deref() == Some(menu::RECENT))
        .expect("the row must be there");
    assert_eq!(
        rows[at - 1].action.as_deref(),
        Some("project.open_folder"),
        "Recent belongs right after Open Project..."
    );
    assert_eq!(
        rows[at + 1].action.as_deref(),
        Some("project.forget_recent"),
        "and right before Remove from Recent — the three are one thought"
    );
    assert!(
        at < rows.len() - 1,
        "appending would park it below Close, where nobody looks"
    );
}

/// THE FALLBACK IS THE PATH THE IDE ACTUALLY TAKES.
///
/// `project.forget_recent` REQUIRES a `path` argument, and `menu.rs` says an
/// action that requires one cannot be drawn in a menu — so the preferred anchor
/// is most likely absent from the real File menu, and `Open Project...` is what
/// positions the row in practice. Testing only the preferred anchor tests the
/// branch that never runs.
#[test]
fn without_remove_from_recent_the_row_still_follows_open_project() {
    let mut rows: Vec<menu::Row> = file_menu_shaped_like_the_catalogue()
        .into_iter()
        .filter(|r| r.action.as_deref() != Some("project.forget_recent"))
        .collect();
    menu::insert_recent_header(&mut rows);
    let at = rows
        .iter()
        .position(|r| r.submenu.as_deref() == Some(menu::RECENT))
        .expect("the row must be there");
    assert_eq!(
        rows[at - 1].action.as_deref(),
        Some("project.open_folder"),
        "AFTER Open Project, not before it"
    );
}

#[test]
fn a_file_menu_without_either_anchor_gets_the_row_at_the_end() {
    let mut rows = vec![menu::Row {
        action: Some(std::borrow::Cow::Borrowed("file.save")),
        ..menu::Row::custom("Save", None)
    }];
    menu::insert_recent_header(&mut rows);
    assert_eq!(
        rows.last().and_then(|r| r.submenu.as_deref()),
        Some(menu::RECENT),
        "with nothing to anchor to it goes last — out of place beats absent"
    );
}
