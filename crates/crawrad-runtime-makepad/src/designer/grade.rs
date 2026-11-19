//! Grid coordinate calculation for Makepad designer.

use crawrad_ir::Rect;
use crawrad_note::note;

#[note("designer-grade-calculate-grid-points")]
pub fn calculate_grid_points(form_rect: Rect, step: f32) -> Vec<(f32, f32)> {
    if step < 2.0 {
        return Vec::new();
    }
    let mut points = Vec::new();
    let mut x = form_rect.x + step;
    let max_x = form_rect.x + form_rect.w;
    let max_y = form_rect.y + form_rect.h;
    while x < max_x - 1.0 {
        let mut y = form_rect.y + step;
        while y < max_y - 1.0 {
            points.push((x, y));
            y += step;
        }
        x += step;
    }
    points
}
