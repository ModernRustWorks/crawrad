//! # `Panel` — Canonical Reference Implementation
//!
//! Standard container panel with configurable border width, surface styling,
//! and support for hosting child components (`children = "area"`).

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A rectangular container grouping other visual controls.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "pnl",
    size = (240.0, 120.0),
    children = "area",
    targets = ["*"]
)]
pub struct Panel {
    /// Optional title or text caption displayed on the panel.
    #[prop(display = "Caption", category = "appearance", default = "")]
    pub text: String,

    /// Outer border thickness in pixels.
    #[prop(
        display = "BorderWidth",
        category = "appearance",
        default = 1,
        min = 0,
        max = 32
    )]
    pub border_width: i64,

    /// Controls visibility of the panel and all hosted children.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the panel and hosted children.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            text: String::new(),
            border_width: 1,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Panel {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Background surface with rounded corners
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(self.border_width as f32);
        canvas.fill_round_rect(0.0, 0.0, 240.0, 120.0, 4.0);

        // 2. Outer border with configured line width
        canvas.stroke_round_rect(0.0, 0.0, 240.0, 120.0, 4.0);

        // 3. Optional centered caption text
        if !self.text.is_empty() {
            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_prop("text", 0.0, 0.0, 240.0, 120.0, TextAlign::Center);
        }
    }
}
