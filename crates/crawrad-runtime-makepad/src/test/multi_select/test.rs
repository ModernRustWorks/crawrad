use crate::multi_select::{multi_select_summary, toggle_selected};

#[test]
fn makepad_summary_empty_selected_shows_hint() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
    ];
    let selected = vec![];
    let summary = multi_select_summary(&items, &selected, "Select…", |_| true);
    assert_eq!(summary, "Select…");
}

#[test]
fn makepad_summary_fits_shows_joined_in_items_order() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
        "Slint".to_string(),
    ];
    let selected = vec!["Slint".to_string(), "Rust".to_string()];
    let summary = multi_select_summary(&items, &selected, "Select…", |_| true);
    assert_eq!(summary, "Rust, Slint");
}

#[test]
fn makepad_summary_does_not_fit_shows_count() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
    ];
    let selected = vec!["Rust".to_string(), "Makepad".to_string()];
    let summary = multi_select_summary(&items, &selected, "Select…", |_| false);
    assert_eq!(summary, "2 selected");
}

#[test]
fn makepad_toggle_selected_adds_and_removes_keeping_order() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
    ];
    let mut selected = vec![];

    toggle_selected(&items, &mut selected, 1);
    assert_eq!(selected, vec!["egui"]);

    toggle_selected(&items, &mut selected, 0);
    assert_eq!(selected, vec!["Rust", "egui"]);

    toggle_selected(&items, &mut selected, 1);
    assert_eq!(selected, vec!["Rust"]);
}
