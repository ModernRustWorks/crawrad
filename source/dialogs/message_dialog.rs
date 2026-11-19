//! # `MessageDialog` — Canonical Reference Implementation
//!
//! Native modal message dialog non-visual component.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual component displaying a modal message box.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Dialogs",
    prefix = "dlg_msg",
    size = (28.0, 28.0),
    targets = ["*"]
)]
pub struct MessageDialog {
    /// Dialog window title caption.
    #[prop(display = "Title", category = "appearance", default = "")]
    pub title: String,

    /// Message dialog text body.
    #[prop(display = "Text", category = "appearance", default = "")]
    pub text: String,

    /// Classification kind and icon.
    #[prop(display = "Kind", category = "appearance", default = "mtInformation")]
    pub kind: String,

    /// Buttons layout for the dialog.
    #[prop(display = "Buttons", category = "appearance", default = "mbOK")]
    pub buttons: String,
}

impl Default for MessageDialog {
    fn default() -> Self {
        Self {
            title: String::new(),
            text: String::new(),
            kind: String::from("mtInformation"),
            buttons: String::from("mbOK"),
        }
    }
}

impl Paint for MessageDialog {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Speech / message box body
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(4.0, 5.0, 20.0, 14.0, 2.0);

        // 3. Message speech bubble tail
        canvas.fill_rect(7.0, 19.0, 4.0, 4.0);

        // 4. Message prompt lines
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(7.0, 9.0, 14.0, 2.0);
        canvas.fill_rect(7.0, 13.0, 9.0, 2.0);
    }
}
