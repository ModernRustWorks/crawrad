//! # `Toggle` — Canonical Reference Implementation
//!
//! An on/off switch with its caption: the `CheckBox`'s state drawn as
//! a knob that slides along a track — right is on.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A switch: `checked` is on.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "tgl",
    size = (120.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_change")]
pub struct Toggle {
    /// Caption beside the switch.
    #[prop(display = "Caption", category = "appearance", default = "Toggle")]
    pub text: String,

    /// Whether the switch is on.
    #[prop(display = "Checked", category = "behavior", default = false)]
    pub checked: bool,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for Toggle {
    fn default() -> Self {
        Self {
            text: String::from("Toggle"),
            checked: false,
            visible: true,
            enabled: true,
        }
    }
}

impl Paint for Toggle {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The track, filled with the accent while on
        canvas.set_fill_style(if self.checked {
            ThemeRole::Accent
        } else {
            ThemeRole::Surface
        });
        canvas.fill_round_rect(2.0, 4.0, 32.0, 16.0, 8.0);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(2.0, 4.0, 32.0, 16.0, 8.0);

        // 2. The knob, at the right when on
        let x = if self.checked { 22.0 } else { 6.0 };
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_round_rect(x, 8.0, 8.0, 8.0, 4.0);

        // 3. The caption
        canvas.fill_text_prop("text", 40.0, 0.0, 80.0, 24.0, TextAlign::Left);
    }
}
