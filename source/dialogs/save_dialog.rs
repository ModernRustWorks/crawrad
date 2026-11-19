//! # `SaveDialog` — Canonical Reference Implementation
//!
//! Standard system file save dialog component.

use crawrad_component::canvas::{Canvas, Paint, ThemeRole};
use crawrad_component_derive::Component;

/// A non-visual component displaying the operating system's standard file save dialog.
#[derive(Component, Debug, Clone, PartialEq)]
#[crawrad(
    palette = "Dialogs",
    prefix = "dlg_save",
    size = (28.0, 28.0),
    targets = ["*"]
)]
pub struct SaveDialog {
    /// Dialog window title caption.
    #[prop(display = "Title", category = "appearance", default = "Save File As")]
    pub title: String,

    /// File type filter string (e.g., "All Files (*.*)|*.*|Text Files (*.txt)|*.txt").
    #[prop(
        display = "Filter",
        category = "behavior",
        default = "All Files (*.*)|*.*"
    )]
    pub filter: String,

    /// Default file name suggested in the dialog.
    #[prop(display = "FileName", category = "data", default = "")]
    pub file_name: String,

    /// Initial directory shown when the dialog opens.
    #[prop(display = "InitialDir", category = "behavior", default = "")]
    pub initial_dir: String,

    /// Default file extension appended when none is specified.
    #[prop(display = "DefaultExt", category = "behavior", default = "")]
    pub default_ext: String,
}

impl Default for SaveDialog {
    fn default() -> Self {
        Self {
            title: String::from("Save File As"),
            filter: String::from("All Files (*.*)|*.*"),
            file_name: String::new(),
            initial_dir: String::new(),
            default_ext: String::new(),
        }
    }
}

impl Paint for SaveDialog {
    /// Design-time visual rendering on the IDE form surface (28x28 icon).
    ///
    /// Renders the classic RAD SaveDialog visual metaphor: a storage diskette with write indicator.
    fn paint(&self, canvas: &mut Canvas) {
        // 1. Icon container box (28x28 px)
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.set_stroke_style(ThemeRole::Border);
        canvas.fill_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        canvas.set_line_width(1.0);
        canvas.stroke_round_rect(1.0, 1.0, 26.0, 26.0, 4.0);

        // 2. Floppy diskette body (18x18 px at 5, 5)
        canvas.set_fill_style(ThemeRole::Accent);
        canvas.fill_round_rect(5.0, 5.0, 18.0, 18.0, 2.0);

        // 3. Shutter slider at the top
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(8.0, 6.0, 10.0, 6.0);
        canvas.set_fill_style(ThemeRole::Border);
        canvas.fill_rect(10.0, 7.0, 3.0, 4.0);

        // 4. White paper label on the bottom
        canvas.set_fill_style(ThemeRole::Surface);
        canvas.fill_rect(8.0, 14.0, 12.0, 8.0);
    }
}
