//! **A ProgressBar shows its Position**.
//!
//! Makepad 1.0.0 has no progress widget, and the generated DSL drew the inner
//! `bar` with `width: Fill`: the bar was always full, whatever the form said.
//! The sync now sizes that `bar` to the fraction of the control the position
//! stands for — its width, or its height for a vertical one (the control lays
//! the bar out from the bottom-left, so a vertical bar grows upwards).

use crawrad_note::note;
use makepad_widgets::*;

#[note("progress-fraction")]
pub fn fraction(position: i64, min: i64, max: i64) -> f64 {
    if max <= min {
        return 0.0;
    }
    ((position - min) as f64 / (max - min) as f64).clamp(0.0, 1.0)
}

#[note("progress-apply")]
pub fn apply(
    cx: &mut Cx,
    ui: &WidgetRef,
    path: &[LiveId],
    fraction: f64,
    vertical: bool,
    width: f64,
    height: f64,
) {
    let bar = ui.widget(path).widget(id!(bar));
    if vertical {
        let h = height * fraction;
        bar.apply_over(cx, live! { width: Fill, height: (h) });
    } else {
        let w = width * fraction;
        bar.apply_over(cx, live! { width: (w), height: Fill });
    }
}

#[cfg(test)]
#[path = "test/progress/test.rs"]
mod tests;
