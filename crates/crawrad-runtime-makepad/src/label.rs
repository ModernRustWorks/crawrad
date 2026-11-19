//! **The run time half of the `Label`**: where the text sits, whether
//! it breaks, and whether the box fits the text — all read at run time, so the
//! form's code can change them.
//!
//! The generated `Label` is a `View` holding a makepad `Label` (the view
//! positions it; see the designer's note on makepad 1.0.0 spending the walk
//! twice). The wrapping and the alignment across belong to the inner `Label`;
//! the alignment down and the size belong to the outer `View`.

use crawrad_note::note;
use makepad_widgets::*;

#[note("label-align-of")]
pub fn align_of(alignment: &str, layout: &str) -> (f64, f64) {
    let x = match alignment {
        "taCenter" => 0.5,
        "taRightJustify" => 1.0,
        _ => 0.0,
    };
    let y = match layout {
        "tlCenter" => 0.5,
        "tlBottom" => 1.0,
        _ => 0.0,
    };
    (x, y)
}

#[note("label-labellook")]
pub struct LabelLook<'a> {
    pub alignment: &'a str,
    pub layout: &'a str,
    pub word_wrap: bool,
    pub auto_size: bool,
    /// The size the designer gave it, for when `auto_size` is off.
    pub size: (f64, f64),
}

#[note("label-apply")]
pub fn apply(cx: &mut Cx, view: &WidgetRef, label: &LabelRef, look: &LabelLook) {
    let (ax, ay) = align_of(look.alignment, look.layout);
    // across: the text's own layout (`DrawText::draw_walk` uses `align` only
    // there, makepad-draw 1.0.0 `draw_text.rs:166`); down: the outer `View`
    // places the label, which is as tall as its text
    label.apply_over(cx, live! { align: { x: (ax), y: 0.0 } });
    view.apply_over(cx, live! { align: { x: 0.0, y: (ay) } });
    if look.word_wrap && !look.auto_size {
        label.apply_over(cx, live! { flow: RightWrap });
    } else {
        label.apply_over(cx, live! { flow: Right });
    }
    if look.auto_size {
        view.apply_over(cx, live! { width: Fit, height: Fit });
        label.apply_over(cx, live! { width: Fit, height: Fit });
    } else {
        let (w, h) = look.size;
        view.apply_over(cx, live! { width: (w), height: (h) });
        label.apply_over(cx, live! { width: Fill, height: Fit });
    }
}
#[cfg(test)]
#[path = "test/label/test.rs"]
mod tests;
