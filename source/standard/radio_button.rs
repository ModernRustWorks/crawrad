//! # `RadioButton` — Canonical Reference Implementation
//!
//! Standard mutually exclusive radio button control with circle indicator,
//! caption text, focus support, and selection click notification.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// An option button allowing the user to select one choice from a mutually exclusive set.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "rb",
    size = (120.0, 20.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct RadioButton {
    /// Caption text displayed next to the radio button.
    #[prop(display = "Caption", category = "appearance", default = "RadioButton")]
    pub text: String,

    /// Whether this radio button is currently selected.
    #[prop(display = "Checked", category = "behavior", default = false)]
    pub checked: bool,

    /// Controls visibility of the radio button.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Optional tooltip or help text.
    #[prop(display = "Hint", category = "behavior", default = "")]
    pub hint: String,

    /// Tab order index within the parent container.
    #[prop(display = "TabOrder", category = "behavior", default = 0)]
    pub tab_order: i64,
}

impl Default for RadioButton {
    fn default() -> Self {
        Self {
            text: String::from("RadioButton"),
            checked: false,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for RadioButton {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Radio button circle (14x14 px with 7px corner radius, at y=3)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 3.0, 14.0, 14.0, 7.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 3.0, 14.0, 14.0, 7.0);

        // 2. Selected inner dot indicator
        if self.checked {
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.fill_round_rect(4.0, 6.0, 8.0, 8.0, 4.0);
        }

        // 3. Caption text next to the circle
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 22.0, 0.0, 98.0, 20.0, TextAlign::Left);
    }
}
