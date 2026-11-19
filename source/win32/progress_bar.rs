//! # `ProgressBar` — Canonical Reference Implementation
//!
//! Visual indicator displaying the progress of an operation along a continuum.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A standard Windows common control bar displaying task progress.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "pb",
    size = (150.0, 16.0),
    targets = ["*"]
)]
pub struct ProgressBar {
    /// Minimum value of the progress range.
    #[prop(display = "Min", category = "behavior", default = 0)]
    pub min: i64,

    /// Maximum value of the progress range.
    #[prop(display = "Max", category = "behavior", default = 100)]
    pub max: i64,

    /// Current progress value within the min..max range.
    #[prop(display = "Position", category = "behavior", default = 0)]
    pub position: i64,

    /// Step increment used by step_it methods.
    #[prop(display = "Step", category = "behavior", default = 10)]
    pub step: i64,

    /// Orientation of the bar ("Horizontal" or "Vertical").
    #[prop(
        display = "Orientation",
        category = "appearance",
        default = "Horizontal"
    )]
    pub orientation: String,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the progress bar.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            min: 0,
            max: 100,
            position: 0,
            step: 10,
            orientation: String::from("Horizontal"),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ProgressBar {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 150.0;
        let h = 16.0;

        // 1. Progress track background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, w, h, 2.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, w, h, 2.0);

        // 2. Filled progress chunk
        let range = (self.max - self.min).max(1);
        let ratio = if self.position > self.min {
            ((self.position - self.min) as f32 / range as f32).clamp(0.0, 1.0)
        } else {
            // Design-time default visual preview: 35%
            0.35
        };

        let fill_w = (w - 4.0) * ratio;
        if fill_w > 0.0 {
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.fill_round_rect(2.0, 2.0, fill_w, h - 4.0, 1.0);
        }
    }
}
