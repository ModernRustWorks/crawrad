use super::*;

#[test]
fn test_progress_bar_width() {
    assert_eq!(progress_bar_width(100.0, None), None);
    assert_eq!(progress_bar_width(100.0, Some(0.0)), Some(2.0));
    assert_eq!(progress_bar_width(100.0, Some(0.5)), Some(50.0));
    assert_eq!(progress_bar_width(100.0, Some(1.0)), Some(100.0));
}
