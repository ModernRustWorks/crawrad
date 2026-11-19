//! # `Badge` — Canonical Reference Implementation
//!
//! A short label in a pill: "New", "3", "Beta", "Error".
//! Not a container, no focus.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A short status or categorical label in a pill.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "bdg",
    size = (56.0, 20.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
pub struct Badge {
    /// The badge label text.
    #[prop(display = "Caption", category = "appearance", default = "New")]
    pub text: String,

    /// Classification kind / tone (`bkDefault`, `bkPrimary`, `bkInfo`, `bkWarning`, `bkDanger`).
    #[prop(display = "Kind", category = "appearance", default = "bkPrimary")]
    pub kind: String,

    /// Whether drawn outlined rather than filled.
    #[prop(display = "Outline", category = "appearance", default = false)]
    pub outline: bool,

    /// Notification count; 0 displays caption instead.
    #[prop(display = "Count", category = "data", default = 0)]
    pub count: i64,

    /// Maximum count before showing "+".
    #[prop(display = "MaxCount", category = "data", default = 99)]
    pub max_count: i64,

    /// Associated control whose top-right corner anchors this badge.
    #[prop(display = "Associate", category = "layout", default = "")]
    pub associate: String,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Badge {
    fn default() -> Self {
        Self {
            text: String::from("New"),
            kind: String::from("bkPrimary"),
            outline: false,
            count: 0,
            max_count: 99,
            associate: String::new(),
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Badge {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        if self.count == 0 && self.text.is_empty() {
            return;
        }
        let (w, h) = (56.0, 20.0);
        let radius = h * 0.5;

        if self.outline {
            canvas.set_stroke_style(ThemeRole::Accent);
            canvas.set_line_width(1.0);
            canvas.stroke_rect(0.5, 0.5, w - 1.0, h - 1.0);
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.fill_text_prop("text", 0.0, 0.0, w, h, TextAlign::Center);
        } else {
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.fill_rect(0.0, 0.0, w, h);
            canvas.set_fill_style(ThemeRole::Text);
            canvas.fill_text_prop("text", 0.0, 0.0, w, h, TextAlign::Center);
        }
        let _ = radius;
    }
}
