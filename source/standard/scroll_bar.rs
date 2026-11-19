//! # `ScrollBar` — Canonical Reference Implementation
//!
//! Standalone horizontal or vertical scroll bar component with range limits,
//! position tracking, step increment buttons, and scroll event dispatch.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A scroll bar control allowing users to adjust a numeric position within a range.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "sb",
    size = (120.0, 16.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_scroll")]
pub struct ScrollBar {
    /// Orientation of the scroll bar ("Horizontal" or "Vertical").
    #[prop(display = "Kind", category = "appearance", default = "Horizontal")]
    pub kind: String,

    /// Minimum range value.
    #[prop(display = "Min", category = "behavior", default = 0)]
    pub min: i64,

    /// Maximum range value.
    #[prop(display = "Max", category = "behavior", default = 100)]
    pub max: i64,

    /// Current scroll position within the min..max range.
    #[prop(display = "Position", category = "behavior", default = 0)]
    pub position: i64,

    /// Amount to scroll on arrow click or keyboard step.
    #[prop(display = "SmallChange", category = "behavior", default = 1)]
    pub small_change: i64,

    /// Amount to scroll on track click or page step.
    #[prop(display = "LargeChange", category = "behavior", default = 10)]
    pub large_change: i64,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for ScrollBar {
    fn default() -> Self {
        Self {
            kind: String::from("Horizontal"),
            min: 0,
            max: 100,
            position: 0,
            small_change: 1,
            large_change: 10,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for ScrollBar {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 120.0;
        let h = 16.0;

        // 1. Scroll track background
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(0.0, 0.0, w, h);

        // 2. Left arrow button (16x16 px)
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(0.0, 0.0, 16.0, h);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.stroke_rect(0.0, 0.0, 16.0, h);

        // 3. Step increment arrows (Left and Right for Horizontal, Up and Down for Vertical)
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("◄", 0.0, 0.0, 16.0, h, TextAlign::Center);

        // 4. Thumb slider button
        let thumb_x = 36.0;
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(thumb_x, 1.0, 24.0, h - 2.0, 2.0);
        canvas.stroke_round_rect(thumb_x, 1.0, 24.0, h - 2.0, 2.0);

        // 5. Right step arrow
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_in_rect("►", w - 16.0, 0.0, 16.0, h, TextAlign::Center);
    }
}
