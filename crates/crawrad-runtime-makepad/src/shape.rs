//! **A Shape's kind, pen and brush, at run time**.
//!
//! The generated `draw_bg` of a Shape is one `Sdf2d` shader with the kind,
//! the pen width and both colours as instances; this hands it the form's
//! values, so `self.shape1.brush_color = ...` repaints it.

use crawrad_note::note;
use makepad_widgets::*;

#[note("shape-kind-index")]
pub fn kind_index(kind: &str) -> f64 {
    match kind {
        "Square" => 1.0,
        "RoundRect" => 2.0,
        "RoundSquare" => 3.0,
        "Ellipse" => 4.0,
        "Circle" => 5.0,
        _ => 0.0,
    }
}

#[note("shape-apply")]
pub fn apply(
    cx: &mut Cx,
    widget: &WidgetRef,
    kind: &str,
    pen_width: i64,
    brush: [f32; 4],
    pen: [f32; 4],
) {
    let k = kind_index(kind);
    let w = pen_width.max(0) as f64;
    let b = vec4(brush[0], brush[1], brush[2], brush[3]);
    let p = vec4(pen[0], pen[1], pen[2], pen[3]);
    widget.apply_over(
        cx,
        live! { draw_bg: { kind: (k), pen: (w), brush: (b), pen_color: (p) } },
    );
}
