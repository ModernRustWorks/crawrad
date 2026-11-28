//! File and message dialog helpers for Slint generated code.

pub use crawrad_runtime::file_dialog::*;

/// Represents the result of a message dialog prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogChoice {
    Ok,
    Cancel,
    Yes,
    No,
    Abort,
    Retry,
    Ignore,
}

/// Dispatches an open file dialog and returns selected path.
pub fn show_open_dialog(title: &str, filter: &str) -> Option<std::path::PathBuf> {
    let req = FileDialogRequest {
        title: title.to_string(),
        filter: filter.to_string(),
        ..Default::default()
    };
    crawrad_runtime::file_dialog::pick_open(&req).and_then(|mut paths| paths.pop())
}

/// Dispatches a save file dialog and returns selected path.
pub fn show_save_dialog(title: &str, filter: &str) -> Option<std::path::PathBuf> {
    let req = FileDialogRequest {
        title: title.to_string(),
        filter: filter.to_string(),
        ..Default::default()
    };
    crawrad_runtime::file_dialog::pick_save(&req)
}
