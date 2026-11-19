//! **A control a Splitter moved**: its box, at run time.
//!
//! The DSL writes each control's margin and size once; a Splitter's drag
//! changes them, so the sync hands the new box to the widget.

use crawrad_note::note;
use makepad_widgets::*;

#[note("split-place")]
pub fn place(cx: &mut Cx, widget: &WidgetRef, x: f32, y: f32, w: f32, h: f32) {
    let (x, y, w, h) = (x as f64, y as f64, w.max(0.0) as f64, h.max(0.0) as f64);
    widget.apply_over(
        cx,
        live! { margin: { left: (x), top: (y) }, width: (w), height: (h) },
    );
}
