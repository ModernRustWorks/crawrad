//! # `CheckBox` — Canonical Reference Implementation
//!
//! Standard two-state checkbox control with box indicator,
//! caption text, focus support, and toggle click notification.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A toggleable checkbox control displaying a check indicator alongside a label.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Standard",
    prefix = "chk",
    size = (120.0, 20.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct CheckBox {
    /// Caption text displayed next to the checkbox.
    #[prop(display = "Caption", category = "appearance", default = "CheckBox")]
    pub text: String,

    /// Whether the checkbox is currently checked.
    #[prop(display = "Checked", category = "behavior", default = false)]
    pub checked: bool,

    /// Controls visibility of the checkbox control.
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

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            text: String::from("CheckBox"),
            checked: false,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for CheckBox {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Checkbox square box (14x14 px, centered vertically at y=3)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 3.0, 14.0, 14.0, 2.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 3.0, 14.0, 14.0, 2.0);

        // 2. Checkmark indicator when checked
        if self.checked {
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.fill_round_rect(4.0, 6.0, 8.0, 8.0, 1.0);
        }

        // 3. Caption text next to the box
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 22.0, 0.0, 98.0, 20.0, TextAlign::Left);
    }
}
