use super::*;

#[test]
fn test_checked_flags_empty() {
    let items: Vec<String> = Vec::new();
    let checked: Vec<String> = Vec::new();
    let flags = checked_flags(&items, &checked);
    assert!(flags.is_empty());
}

#[test]
fn test_checked_flags_matching() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let checked = vec!["B".to_string(), "C".to_string()];
    let flags = checked_flags(&items, &checked);
    assert_eq!(flags, vec![false, true, true]);
}

#[test]
fn test_checked_flags_no_match() {
    let items = vec!["A".to_string(), "B".to_string()];
    let checked = vec!["X".to_string()];
    let flags = checked_flags(&items, &checked);
    assert_eq!(flags, vec![false, false]);
}

#[test]
fn test_set_checked_item_bounds() {
    let items = vec!["A".to_string(), "B".to_string()];
    let mut checked = vec!["A".to_string()];

    // Out of bounds index
    set_checked_item(&items, &mut checked, 2, true);
    assert_eq!(checked, vec!["A".to_string()]);

    set_checked_item(&items, &mut checked, 10, false);
    assert_eq!(checked, vec!["A".to_string()]);
}

#[test]
fn test_set_checked_item_toggle_on() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut checked = vec!["A".to_string()];

    // Toggle B on
    set_checked_item(&items, &mut checked, 1, true);
    assert_eq!(checked, vec!["A".to_string(), "B".to_string()]);

    // Toggle B on again (should not duplicate)
    set_checked_item(&items, &mut checked, 1, true);
    assert_eq!(checked, vec!["A".to_string(), "B".to_string()]);
}

#[test]
fn test_set_checked_item_toggle_off() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let mut checked = vec!["A".to_string(), "B".to_string()];

    // Toggle B off
    set_checked_item(&items, &mut checked, 1, false);
    assert_eq!(checked, vec!["A".to_string()]);

    // Toggle C off (was not present)
    set_checked_item(&items, &mut checked, 2, false);
    assert_eq!(checked, vec!["A".to_string()]);
}
