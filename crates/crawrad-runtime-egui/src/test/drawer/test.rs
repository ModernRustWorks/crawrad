use crate::drawer_close_rect;

#[test]
fn drawer_close_rect_left_and_right() {
    let r = egui::Rect::from_min_size(egui::pos2(100.0, 50.0), egui::vec2(240.0, 320.0));

    // dsLeft: close button is at the far right of header (32x32)
    let close_left = drawer_close_rect(r, "dsLeft");
    assert_eq!(close_left.min, egui::pos2(340.0 - 32.0, 50.0));
    assert_eq!(close_left.width(), 32.0);
    assert_eq!(close_left.height(), 32.0);

    // dsRight: close button is at the left of header (32x32)
    let close_right = drawer_close_rect(r, "dsRight");
    assert_eq!(close_right.min, egui::pos2(100.0, 50.0));
    assert_eq!(close_right.width(), 32.0);
    assert_eq!(close_right.height(), 32.0);
}
