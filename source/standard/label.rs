//! # `Label` — Canonical Reference Implementation
//!
//! Standard text label control demonstrating direct typographic rendering
//! using the `fill_text_prop` method of the 2D `Canvas`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A simple text label for displaying captions and descriptions.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "lbl",
    size = (100.0, 20.0),
    targets = ["*"]
)]
pub struct Label {
    /// Text caption displayed on the label.
    #[prop(display = "Caption", category = "appearance", default = "Label")]
    pub text: String,

    /// Controls label visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or dims the label.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::from("Label"),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Label {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // Render left-aligned text with semantic theme text color
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 0.0, 0.0, 100.0, 20.0, TextAlign::Left);
    }
}
