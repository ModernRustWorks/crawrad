use crate::set_checked_item;

#[test]
fn test_set_checked_item_add_and_remove() {
    let items = vec![
        "Rust".to_string(),
        "egui".to_string(),
        "Makepad".to_string(),
    ];
    let mut checked = vec!["egui".to_string()];

    // Add "Makepad" (index 2)
    set_checked_item(&items, &mut checked, 2, true);
    assert_eq!(checked, vec!["egui".to_string(), "Makepad".to_string()]);

    // Add "Makepad" again (no duplicate)
    set_checked_item(&items, &mut checked, 2, true);
    assert_eq!(checked, vec!["egui".to_string(), "Makepad".to_string()]);

    // Remove "egui" (index 1)
    set_checked_item(&items, &mut checked, 1, false);
    assert_eq!(checked, vec!["Makepad".to_string()]);

    // Remove "Rust" (not present)
    set_checked_item(&items, &mut checked, 0, false);
    assert_eq!(checked, vec!["Makepad".to_string()]);

    // Out of bounds index does nothing
    set_checked_item(&items, &mut checked, 99, true);
    assert_eq!(checked, vec!["Makepad".to_string()]);

    set_checked_item(&items, &mut checked, 99, false);
    assert_eq!(checked, vec!["Makepad".to_string()]);
}
