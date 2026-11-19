//! # `GroupBox` — Canonical Reference Implementation
//!
//! Standard grouped container control with framed border, caption notch,
//! and support for hosting child components (`children = "area"`).

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A labeled frame container that visually groups related controls.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "gbx",
    size = (240.0, 120.0),
    children = "area",
    targets = ["*"]
)]
pub struct GroupBox {
    /// Caption title text displayed at the top border notch.
    #[prop(display = "Caption", category = "appearance", default = "GroupBox")]
    pub text: String,

    /// Controls visibility of the group box and hosted children.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the group box and hosted children.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for GroupBox {
    fn default() -> Self {
        Self {
            text: String::from("GroupBox"),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for GroupBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Group box outline starting slightly below the top (y=8 to allow caption notch)
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 8.0, 238.0, 111.0, 4.0);

        // 2. Caption notch background to clear border line behind the title
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(10.0, 0.0, 80.0, 16.0);

        // 3. Caption text
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 12.0, 0.0, 76.0, 16.0, TextAlign::Left);
    }
}
