//! # `BitBtn` — Canonical Reference Implementation
//!
//! Push button control featuring integrated graphic glyph icons,
//! standard dialog button presets (`Kind`), and native `ModalResult` support.

use crawrad_component::ModalResult;
use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A command button with bitmap glyph display and modal result capability.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "bbtn",
    size = (96.0, 32.0),
    targets = ["*"]
)]
#[event(name = "on_click")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
pub struct BitBtn {
    /// Caption text displayed on the button surface alongside the glyph.
    #[prop(display = "Caption", category = "appearance", default = "BitBtn")]
    pub text: String,

    /// Standard preset button kind determining default glyph and behavior.
    #[prop(display = "Kind", category = "behavior", default = "bkCustom")]
    pub kind: String,

    /// Result returned when used on modal forms and dialogs.
    #[prop(display = "ModalResult", category = "behavior")]
    pub modal_result: ModalResult,

    /// Controls visibility on screen.
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

impl Default for BitBtn {
    fn default() -> Self {
        Self {
            text: String::from("BitBtn"),
            kind: String::from("bkCustom"),
            modal_result: ModalResult::None,
            visible: true,
            enabled: true,
            hint: String::new(),
            tab_order: 0,
        }
    }
}

impl Paint for BitBtn {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Button background container (96x32 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 96.0, 32.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 96.0, 32.0, 4.0);

        // 2. Icon glyph representation based on Kind preset
        let (glyph, glyph_color) = match self.kind.as_str() {
            "bkOK" => ("✔", ThemeRole::Accent),
            "bkCancel" => ("✖", ThemeRole::Accent),
            "bkHelp" => ("?", ThemeRole::Accent),
            "bkClose" => ("🚪", ThemeRole::Border),
            "bkYes" => ("✔", ThemeRole::Accent),
            "bkNo" => ("✖", ThemeRole::Accent),
            _ => ("✦", ThemeRole::Accent),
        };

        canvas.set_fill_style(glyph_color);
        canvas.fill_text_in_rect(glyph, 10.0, 7.0, 16.0, 18.0, TextAlign::Center);

        // 3. Caption text aligned next to glyph
        canvas.set_fill_style(ThemeRole::Text);
        canvas.fill_text_prop("text", 28.0, 0.0, 64.0, 32.0, TextAlign::Left);
    }
}
