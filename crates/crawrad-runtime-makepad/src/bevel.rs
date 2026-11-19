//! **A Bevel's shape and style, at run time**.
//!
//! The generated `draw_bg` of a Bevel is one shader with the shape and the
//! style as instances; this hands it the form's values, so
//! `self.bevel1.bevel_style = "Raised".into()` repaints it.

use crawrad_note::note;
use makepad_widgets::*;

#[note("bevel-shape-index")]
pub fn shape_index(shape: &str) -> f64 {
    match shape {
        "Box" => 0.0,
        "Frame" => 1.0,
        "TopLine" => 2.0,
        "BottomLine" => 3.0,
        "LeftLine" => 4.0,
        "RightLine" => 5.0,
        _ => 6.0,
    }
}

#[note("bevel-apply")]
pub fn apply(cx: &mut Cx, widget: &WidgetRef, shape: &str, style: &str) {
    let kind = shape_index(shape);
    let raised = if style == "Raised" { 1.0 } else { 0.0 };
    widget.apply_over(cx, live! { draw_bg: { kind: (kind), raised: (raised) } });
}
