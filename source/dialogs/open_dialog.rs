//! # `OpenDialog` — Canonical Reference Implementation
//!
//! Standard system file open dialog component.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual component displaying the operating system's standard file open dialog.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Dialogs",
    prefix = "dlg_open",
    size = (28.0, 28.0),
    targets = ["*"]
)]
pub struct OpenDialog {
    /// Dialog window title caption.
    #[prop(display = "Title", category = "appearance", default = "Open File")]
    pub title: String,

    /// File type filter string (e.g., "All Files (*.*)|*.*|Text Files (*.txt)|*.txt").
    #[prop(
        display = "Filter",
        category = "behavior",
        default = "All Files (*.*)|*.*"
    )]
    pub filter: String,

    /// Selected or default file path.
    #[prop(display = "FileName", category = "data", default = "")]
    pub file_name: String,

    /// Initial directory shown when the dialog opens.
    #[prop(display = "InitialDir", category = "behavior", default = "")]
    pub initial_dir: String,

    /// Default file extension appended when none is specified.
    #[prop(display = "DefaultExt", category = "behavior", default = "")]
    pub default_ext: String,
}

impl Default for OpenDialog {
    fn default() -> Self {
        Self {
            title: String::from("Open File"),
            filter: String::from("All Files (*.*)|*.*"),
            file_name: String::new(),
            initial_dir: String::new(),
            default_ext: String::new(),
        }
    }
}

impl Paint for OpenDialog {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    ///
    /// Renders the classic RAD OpenDialog visual metaphor: an open folder with an extraction arrow.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Open folder back tab (y=6)
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_round_rect(4.0, 6.0, 10.0, 4.0, 1.0);

        // 3. Open folder main body
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(4.0, 9.0, 20.0, 13.0, 2.0);

        // 4. Document sheet peeking out of folder
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(8.0, 4.0, 8.0, 8.0);
        canvas.set_stroke_style(ThemeRole::Text);
        canvas.stroke_rect(8.0, 4.0, 8.0, 8.0);
    }
}
