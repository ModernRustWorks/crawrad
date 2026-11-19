//! # `TrackBar` — Canonical Reference Implementation
//!
//! Slider control allowing fine visual adjustment of numeric values along a track with ticks.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A slider control that moves a thumb along a track to select a numeric value.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "tb",
    size = (150.0, 32.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
pub struct TrackBar {
    /// Minimum value of the slider range.
    #[prop(display = "Min", category = "behavior", default = 0)]
    pub min: i64,

    /// Maximum value of the slider range.
    #[prop(display = "Max", category = "behavior", default = 10)]
    pub max: i64,

    /// Current numeric position of the slider thumb.
    #[prop(display = "Position", category = "behavior", default = 0)]
    pub position: i64,

    /// Orientation of the track ("Horizontal" or "Vertical").
    #[prop(
        display = "Orientation",
        category = "appearance",
        default = "Horizontal"
    )]
    pub orientation: String,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for TrackBar {
    fn default() -> Self {
        Self {
            min: 0,
            max: 10,
            position: 0,
            orientation: String::from("Horizontal"),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for TrackBar {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 150.0;
        let _h = 32.0;

        // 1. Horizontal track line (at y=12)
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(10.0, 12.0, w - 20.0, 4.0);

        // 2. Scale tick marks below the track (y=20 to y=24)
        canvas.set_fill_style(ThemeRole::Border);
        let num_ticks = 6;
        for i in 0..num_ticks {
            let x = 10.0 + (i as f32) * ((w - 20.0) / (num_ticks - 1) as f32);
            canvas.fill_rect(x, 20.0, 1.0, 4.0);
        }

        // 3. Slider Thumb indicator (positioned near center at design time)
        let thumb_x = 55.0;
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(thumb_x, 4.0, 12.0, 18.0, 2.0);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(thumb_x, 4.0, 12.0, 18.0, 2.0);

        // Thumb inner grip accent
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_rect(thumb_x + 5.0, 8.0, 2.0, 10.0);
    }
}
