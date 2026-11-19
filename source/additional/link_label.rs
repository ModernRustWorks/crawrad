//! # `LinkLabel` — Canonical Reference Implementation
//!
//! A caption drawn as a link: a click opens `url` in the system's
//! browser — web and mail only — and then runs `OnClick`.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A link: `text` is what shows, `url` where it goes.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "lnk",
    size = (120.0, 20.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
pub struct LinkLabel {
    /// The link's caption.
    #[prop(display = "Caption", category = "appearance", default = "LinkLabel")]
    pub text: String,

    /// The address a click opens (`http`, `https` or `mailto` only).
    #[prop(display = "URL", category = "behavior", default = "")]
    pub url: String,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for LinkLabel {
    fn default() -> Self {
        Self {
            text: String::from("LinkLabel"),
            url: String::new(),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for LinkLabel {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The caption, in the link colour
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_text_prop("text", 0.0, 0.0, 120.0, 18.0, TextAlign::Left);

        // 2. The underline
        canvas.set_stroke_style(ThemeRole::Accent);
        canvas.set_line_width(1.0);
        canvas.begin_path();
        canvas.move_to(0.0, 18.0);
        canvas.line_to(120.0, 18.0);
        canvas.stroke();
    }
}
