//! # `Timer` — Canonical Reference Implementation
//!
//! Standard non-visual interval timer component with enabled state,
//! millisecond interval, and periodic timer event dispatch.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual timer component that triggers periodic events at specified intervals.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "System",
    prefix = "tmr",
    size = (28.0, 28.0),
    targets = ["*"]
)]
#[event(name = "on_timer")]
pub struct Timer {
    /// Controls whether the timer actively ticks.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Interval between ticks in milliseconds.
    #[prop(display = "Interval", category = "behavior", default = 1000, min = 1)]
    pub interval: i64,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: 1000,
        }
    }
}

impl Paint for Timer {
    /// Design-time visual rendering on the IDE form surface.
    ///
    /// Non-visual components render a compact 28x28 icon during visual form design.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Clock face circle (radius 8px at center 14, 14)
        canvas.set_stroke_style(ThemeRole::Accent);
        canvas.set_line_width(1.5);
        canvas.stroke_round_rect(6.0, 6.0, 16.0, 16.0, 8.0);

        // 3. Clock hands (top hand and right hand)
        canvas.fill_rect(13.0, 9.0, 2.0, 6.0);
        canvas.fill_rect(13.0, 13.0, 5.0, 2.0);
    }
}
