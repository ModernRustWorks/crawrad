use crate::badge::badge_text;

#[test]
fn test_badge_text_computations() {
    assert_eq!(badge_text("", 0, 99), None);
    assert_eq!(badge_text("New", 0, 99), Some("New".to_string()));
    assert_eq!(badge_text("", 5, 99), Some("5".to_string()));
    assert_eq!(badge_text("New", 5, 99), Some("5".to_string()));
    assert_eq!(badge_text("", 150, 99), Some("99+".to_string()));
    assert_eq!(badge_text("New", 150, 99), Some("99+".to_string()));
}
