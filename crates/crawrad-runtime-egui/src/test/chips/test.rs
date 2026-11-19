use crate::chips_layout;

#[test]
fn test_chips_layout_empty() {
    let positions = chips_layout(&[], 100.0, 24.0, 6.0);
    assert!(positions.is_empty());
}

#[test]
fn test_chips_layout_single_line() {
    let widths = [40.0, 50.0, 30.0];
    let positions = chips_layout(&widths, 200.0, 24.0, 6.0);
    assert_eq!(positions, vec![(0.0, 0.0), (46.0, 0.0), (102.0, 0.0),]);
}

#[test]
fn test_chips_layout_wrapping() {
    let widths = [60.0, 60.0, 60.0];
    // area_width = 100.0: first chip fits at 0, second (60+6+60 = 126 > 100) wraps to (0, 30.0)
    let positions = chips_layout(&widths, 100.0, 24.0, 6.0);
    assert_eq!(positions, vec![(0.0, 0.0), (0.0, 30.0), (0.0, 60.0),]);
}

#[test]
fn test_chips_layout_exact_boundary() {
    let widths = [50.0, 44.0];
    // x = 0.0, fits 50.0. next x = 56.0. 56.0 + 44.0 = 100.0 <= 100.0, fits on first row!
    let positions = chips_layout(&widths, 100.0, 24.0, 6.0);
    assert_eq!(positions, vec![(0.0, 0.0), (56.0, 0.0),]);
}
