//! # `SpeedButton` — Canonical Reference Implementation
//!
//! Compact, flat tool button designed for toolbars and utility panels,
//! supporting toggle states (`Down`), mutual exclusion (`GroupIndex`), and glyphs.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A flat command button typically placed in toolbars to execute commands or toggle options.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "sbtn",
    size = (28.0, 28.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
pub struct SpeedButton {
    /// Caption text displayed when no glyph is used or alongside glyph.
    #[prop(display = "Caption", category = "appearance", default = "")]
    pub text: String,

    /// Whether the button is currently in the pressed/toggled state.
    #[prop(display = "Down", category = "behavior", default = false)]
    pub down: bool,

    /// Group index for mutually exclusive radio-style button groups.
    #[prop(display = "GroupIndex", category = "behavior", default = 0)]
    pub group_index: i64,

    /// Whether all buttons in the group can be unselected at the same time.
    #[prop(display = "AllowAllUp", category = "behavior", default = false)]
    pub allow_all_up: bool,

    /// Flat appearance: border appears only when hovered or pressed.
    #[prop(display = "Flat", category = "appearance", default = true)]
    pub flat: bool,

    /// Controls visibility on screen.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,

    /// Optional tooltip or help text.
    #[prop(display = "Hint", category = "behavior", default = "")]
    pub hint: String,
}

impl Default for SpeedButton {
    fn default() -> Self {
        Self {
            text: String::new(),
            down: false,
            group_index: 0,
            allow_all_up: false,
            flat: true,
            visible: true,
            enabled: true,
            hint: String::new(),
        }
    }
}

impl Paint for SpeedButton {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Background surface (28x28 px)
        if self.down {
            canvas.set_fill_style(ThemeRole::Accent);
            canvas.set_stroke_style(ThemeRole::Accent);
            canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 3.0);
            canvas.set_line_width(1.0);
            canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 3.0);
        } else if !self.flat {
            canvas.set_fill_style(ThemeRole::Surface);
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 3.0);
            canvas.set_line_width(1.0);
            canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 3.0);
        } else {
            // Flat appearance in resting state: subtle border guide
            canvas.set_stroke_style(ThemeRole::Border);
            canvas.set_line_width(1.0);
            canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 3.0);
        }

        // 2. Icon glyph or caption text
        let text_role = if self.down {
            ThemeRole::Surface
        } else {
            ThemeRole::Text
        };
        canvas.set_fill_style(text_role);

        if self.text.is_empty() {
            // Default lightning / tool glyph
            canvas.fill_text_in_rect("⚡", 4.0, 4.0, 20.0, 20.0, TextAlign::Center);
        } else {
            canvas.fill_text_prop("text", 2.0, 4.0, 24.0, 20.0, TextAlign::Center);
        }
    }
}
