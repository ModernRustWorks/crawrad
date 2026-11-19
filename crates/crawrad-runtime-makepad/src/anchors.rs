//! **The form's controls follow the window by their `Anchors`**, as
//! in classic RAD and as the egui target does.
//!
//! The DSL places each control at its design rect (`margin` + `width` /
//! `height`), which is right at design size and nowhere else. When the
//! window's geometry changes, the generated `handle_event` hands each
//! anchored control to [`place`] with how much the form grew; the sum is
//! `crawrad_component::pieces::anchors::anchored`, the ONE the designer and the
//! egui target spend.
//!
//! As in the egui target, only the form's own controls follow the window: a
//! control inside a container is placed relative to the container, which
//! already took it along.

pub use crawrad_component::pieces::anchors::Anchors;
use crawrad_note::note;
use makepad_widgets::*;

#[note("anchors-caption-bar")]
pub const CAPTION_BAR: f64 = 27.0;

#[note("anchors-grown")]
pub fn grown(inner: DVec2, caption: bool, design: DVec2) -> DVec2 {
    let bar = if caption { CAPTION_BAR } else { 0.0 };
    dvec2(inner.x - design.x, inner.y - bar - design.y)
}

#[note("anchors-rect")]
pub fn rect(design: (f64, f64, f64, f64), a: Anchors, grown: DVec2) -> (f64, f64, f64, f64) {
    let (x, y, w, h) = design;
    let (x, y, w, h) = crawrad_component::pieces::anchors::anchored(
        x as f32,
        y as f32,
        w as f32,
        h as f32,
        a,
        grown.x as f32,
        grown.y as f32,
    );
    (x as f64, y as f64, (w as f64).max(1.0), (h as f64).max(1.0))
}

#[note("anchors-place")]
pub fn place(
    cx: &mut Cx,
    widget: &WidgetRef,
    design: (f64, f64, f64, f64),
    a: Anchors,
    grown: DVec2,
) {
    let (x, y, w, h) = rect(design, a, grown);
    widget.apply_over(
        cx,
        live! {
            margin: { left: (x), top: (y) },
            width: (w),
            height: (h),
        },
    );
}

#[cfg(test)]
#[path = "test/anchors/test.rs"]
mod tests;
