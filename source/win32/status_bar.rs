//! # `StatusBar` — Canonical Reference Implementation
//!
//! Window footer bar displaying application status information, divided into panels.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A status bar control placed along the bottom of a parent form to display status messages.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Win32",
    prefix = "stb",
    size = (320.0, 20.0),
    targets = ["*"]
)]
pub struct StatusBar {
    /// Status text displayed when simple_panel is true.
    #[prop(display = "SimpleText", category = "appearance", default = "Ready")]
    pub simple_text: String,

    /// Whether the status bar displays a single simple panel rather than multiple panels.
    #[prop(display = "SimplePanel", category = "behavior", default = true)]
    pub simple_panel: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables the status bar.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            simple_text: String::from("Ready"),
            simple_panel: true,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for StatusBar {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        let w = 320.0;
        let h = 20.0;

        // 1. Surface background & top border line
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(0.0, 0.0, w, h);

        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(0.0, 0.0, w, 1.0);

        // 2. Main panel sunken bevel frame
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_rect(2.0, 2.0, w - 80.0, h - 4.0);

        // 3. Status text inside main panel
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("simple_text", 6.0, 2.0, w - 90.0, h - 4.0, TextAlign::Left);

        // 4. Secondary info panel on the right
        canvas.stroke_rect(w - 76.0, 2.0, 72.0, h - 4.0);
        canvas.fill_text_in_rect("INS", w - 72.0, 2.0, 40.0, h - 4.0, TextAlign::Left);

        // 5. Window resize grip lines in the bottom-right corner
        canvas.fill_rect(w - 6.0, h - 6.0, 2.0, 2.0);
        canvas.fill_rect(w - 10.0, h - 6.0, 2.0, 2.0);
        canvas.fill_rect(w - 6.0, h - 10.0, 2.0, 2.0);
    }
}
