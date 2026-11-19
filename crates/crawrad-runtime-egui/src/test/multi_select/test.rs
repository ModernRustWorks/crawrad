use crate::{multi_select_summary, toggle_selected};

#[test]
fn summary_empty_selected_shows_hint() {
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
fn summary_fits_shows_joined_in_items_order() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
        "Slint".to_string(),
    ];
    // selected in different order or containing unknown item
    let selected = vec![
        "Slint".to_string(),
        "Unknown".to_string(),
        "Rust".to_string(),
    ];
    let summary = multi_select_summary(&items, &selected, "Select…", |_| true);
    assert_eq!(summary, "Rust, Slint");
}

#[test]
fn summary_does_not_fit_shows_count() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
    ];
    let selected = vec!["Rust".to_string(), "egui".to_string()];
    let summary = multi_select_summary(&items, &selected, "Select…", |_| false);
    assert_eq!(summary, "2 selected");
}

#[test]
fn toggle_selected_adds_and_removes_keeping_order() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
        "Slint".to_string(),
    ];
    let mut selected = vec!["egui".to_string()];

    // Add "Slint" (index 3)
    toggle_selected(&items, &mut selected, 3);
    assert_eq!(selected, vec!["egui", "Slint"]);

    // Add "Rust" (index 0) - should appear before "egui"
    toggle_selected(&items, &mut selected, 0);
    assert_eq!(selected, vec!["Rust", "egui", "Slint"]);

    // Remove "egui" (index 1)
    toggle_selected(&items, &mut selected, 1);
    assert_eq!(selected, vec!["Rust", "Slint"]);

    // Remove "Rust" (index 0)
    toggle_selected(&items, &mut selected, 0);
    assert_eq!(selected, vec!["Slint"]);

    // Out of bounds index does nothing
    toggle_selected(&items, &mut selected, 99);
    assert_eq!(selected, vec!["Slint"]);
}
