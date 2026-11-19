use super::*;

#[test]
fn hidden_tabs_are_skipped() {
    let shown = [true, false, true, false, true];
    let w = window(320.0, &shown, 2);
    assert!(!w.arrows);
    assert!(w.contains(0));
    assert!(!w.contains(1));
    assert!(w.contains(2));
    assert!(!w.contains(3));
    assert!(w.contains(4));
}

#[test]
fn window_holds_active_with_overflow() {
    let shown = [true; 6];

    // Active at the beginning
    let w0 = window(320.0, &shown, 0);
    assert!(w0.arrows);
    assert!(w0.contains(0));
    assert!(w0.contains(1));
    assert!(w0.contains(2));
    assert!(!w0.contains(3));

    // Active at the end
    let w5 = window(320.0, &shown, 5);
    assert!(w5.arrows);
    assert!(!w5.contains(2));
    assert!(w5.contains(3));
    assert!(w5.contains(4));
    assert!(w5.contains(5));

    // Active in the middle
    let w3 = window(320.0, &shown, 3);
    assert!(w3.arrows);
    assert!(w3.contains(1));
    assert!(w3.contains(2));
    assert!(w3.contains(3));
    assert!(!w3.contains(4));
}

#[test]
fn hidden_tabs_combined_with_overflow() {
    // 8 sheets, indices 1, 4, 7 are hidden; visible indices are [0, 2, 3, 5, 6] (5 tabs)
    let shown = [true, false, true, true, false, true, true, false];
    let w = window(320.0, &shown, 5);
    assert!(w.arrows);
    assert!(w.contains(5));
    assert!(w.contains(2));
    assert!(w.contains(3));
    assert!(!w.contains(0));
    assert!(!w.contains(1));
    assert!(!w.contains(4));
    assert!(!w.contains(6));
    assert!(!w.contains(7));
}

#[test]
fn empty_shown_returns_empty_window() {
    let w = window(320.0, &[], 0);
    assert!(!w.arrows);
    assert!(!w.contains(0));
}

#[test]
fn next_page_finds_next_eligible_page_without_wrapping() {
    let eligible = [true, false, true, true];
    assert_eq!(next_page(0, &eligible), Some(2));
    assert_eq!(next_page(2, &eligible), Some(3));
    assert_eq!(next_page(3, &eligible), None);
}

#[test]
fn prev_page_finds_previous_eligible_page_without_wrapping() {
    let eligible = [true, false, true, true];
    assert_eq!(prev_page(3, &eligible), Some(2));
    assert_eq!(prev_page(2, &eligible), Some(0));
    assert_eq!(prev_page(0, &eligible), None);
}

#[test]
fn page_navigation_handles_empty_or_out_of_bounds() {
    assert_eq!(next_page(0, &[]), None);
    assert_eq!(prev_page(0, &[]), None);
    assert_eq!(next_page(5, &[true, true]), None);
    assert_eq!(prev_page(5, &[true, true]), Some(1));
}
