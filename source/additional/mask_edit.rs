//! # `MaskEdit` — Canonical Reference Implementation
//!
//! An `Edit` whose text is shaped by an `EditMask`: `0` a digit, `L` a
//! letter, `>`/`<` the case, any other character a literal — the classic
//! syntax `crawrad_runtime::mask` reads. An empty text shows the blank mask.

use crawrad_component::canvas::{Canvas, Paint, TextAlign, ThemeRole};
use crawrad_component_derive::Component;

/// A single-line text input whose text follows a mask.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Additional",
    prefix = "msk",
    size = (200.0, 24.0),
    targets = ["*"]
)]
#[event(name = "on_change")]
#[event(name = "on_enter")]
#[event(name = "on_exit")]
#[event(name = "on_key_down")]
#[event(name = "on_key_press")]
#[event(name = "on_key_up")]
pub struct MaskEdit {
    /// Text the box shows: the typed characters laid into the mask.
    #[prop(display = "Text", category = "text", default = "")]
    pub text: String,

    /// The mask the text is shaped by; empty, the box is a plain `Edit`.
    #[prop(
        display = "EditMask",
        category = "behavior",
        default = "!99/99/0000;1;_"
    )]
    pub edit_mask: String,

    /// When true, prevents the user from editing text.
    #[prop(display = "ReadOnly", category = "behavior", default = false)]
    pub readonly: bool,

    /// Controls visibility of the control.
    #[prop(display = "Visible", category = "behavior", default = true)]
    pub visible: bool,

    /// Enables or disables user interaction.
    #[prop(display = "Enabled", category = "behavior", default = true)]
    pub enabled: bool,
}

impl Default for MaskEdit {
    fn default() -> Self {
        Self {
            text: String::new(),
            edit_mask: String::from("!99/99/0000;1;_"),
            readonly: false,
            visible: true,
            enabled: true,
        }
    }
}

impl MaskEdit {
    /// The text as the box shows it: each input position of the mask holds
    /// the next typed character, or the blank `_`; literals stay in place.
    /// A reading aid only — the full rule (case, `;0`, the blank section)
    /// lives in `crawrad_runtime::mask`.
    pub fn shown(&self) -> String {
        if self.edit_mask.is_empty() {
            return self.text.clone();
        }
        let body = self.edit_mask.split(';').next().unwrap_or("");
        let body = body.strip_prefix('!').unwrap_or(body);
        let mut typed = self.text.chars().filter(|c| c.is_alphanumeric());
        body.chars()
            .filter(|c| !matches!(c, '>' | '<'))
            .map(|c| match c {
                '0' | '9' | '#' | 'L' | 'l' | 'A' | 'a' | 'C' | 'c' => typed.next().unwrap_or('_'),
                other => other,
            })
            .collect()
    }
}

impl Paint for MaskEdit {
    /// Visual rendering using the 2D Canvas from `crawrad-component`.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. The Edit's box
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);
        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(0.0, 0.0, 200.0, 24.0, 2.0);

        // 2. The masked text: the blank mask when nothing was typed
        canvas.set_fill_style(ThemeRole::Text);
        let shown = self.shown();
        if !shown.is_empty() {
            canvas.fill_text_in_rect(&shown, 6.0, 4.0, 188.0, 16.0, TextAlign::Left);
        }
    }
}
