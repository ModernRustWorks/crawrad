//! # Native File Dialogs
//!
//! Provides toolkit-agnostic native file dialog helpers (`pick_open`, `pick_save`)
//! backed by `rfd::FileDialog` and shared by both `egui` and `makepad` targets.
//!
//! ## Testing and Automation Hook
//!
//! To allow E2E, CLI and unit tests to run non-interactively without displaying
//! a blocking native GUI dialog window, set the environment variable:
//!
//! `CRAWRAD_FILE_DIALOG_ANSWER`
//!
//! - An empty value (`""`) simulates user cancellation (dialog closed with Cancel),
//!   returning `None`.
//! - For `pick_save`, the value is treated as the chosen file path string, applying
//!   `with_default_ext` if no extension was provided.
//! - For `pick_open`, the value contains one or more paths separated by the pipe
//!   character (`|`). If `FileDialogRequest::multi` is `false`, only the first path
//!   is returned.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("file-dialog-crawrad-file-dialog-answer")]
pub const CRAWRAD_FILE_DIALOG_ANSWER: &str = "CRAWRAD_FILE_DIALOG_ANSWER";

#[note("file-dialog-filefilter")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileFilter {
    /// Display name of the filter category (e.g. "Text files").
    pub name: String,
    /// Extension patterns without `*.` (e.g. `["txt", "text"]`).
    pub patterns: Vec<String>,
}

#[note("file-dialog-parse-filter")]
pub fn parse_filter(classic: &str) -> Vec<FileFilter> {
    if classic.trim().is_empty() {
        return Vec::new();
    }

    let parts: Vec<&str> = classic.split('|').collect();
    let mut filters = Vec::new();

    for chunk in parts.chunks_exact(2) {
        let name = chunk[0].trim();
        let raw_patterns = chunk[1].trim();

        let patterns: Vec<String> = raw_patterns
            .split(';')
            .map(|pat| {
                let trimmed = pat.trim();
                let stripped = if let Some(s) = trimmed.strip_prefix("*.") {
                    s
                } else if let Some(s) = trimmed.strip_prefix('.') {
                    s
                } else {
                    trimmed
                };
                stripped.to_string()
            })
            .filter(|pat| !pat.is_empty())
            .collect();

        if !name.is_empty() || !patterns.is_empty() {
            filters.push(FileFilter {
                name: name.to_string(),
                patterns,
            });
        }
    }

    filters
}

#[note("file-dialog-filedialogrequest")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileDialogRequest {
    /// Dialog window title.
    pub title: String,
    /// Classic filter string (e.g. `"Text files (*.txt)|*.txt|All files (*.*)|*.*"`).
    pub filter: String,
    /// Initial directory to browse.
    pub initial_dir: String,
    /// Initial or default file name.
    pub file_name: String,
    /// Default extension added when saving without an extension.
    pub default_ext: String,
    /// Whether multi-selection of files is enabled (for open dialogs).
    pub multi: bool,
}

#[note("file-dialog-with-default-ext")]
pub fn with_default_ext(path: PathBuf, default_ext: &str) -> PathBuf {
    let ext = default_ext.trim().trim_start_matches('.');
    if ext.is_empty() {
        return path;
    }
    match path.extension() {
        Some(existing) if !existing.is_empty() => path,
        _ => path.with_extension(ext),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[note("file-dialog-build-rfd")]
fn build_rfd(req: &FileDialogRequest) -> rfd::FileDialog {
    let mut dialog = rfd::FileDialog::new();
    if !req.title.is_empty() {
        dialog = dialog.set_title(&req.title);
    }
    if !req.initial_dir.is_empty() {
        dialog = dialog.set_directory(Path::new(&req.initial_dir));
    }
    if !req.file_name.is_empty() {
        dialog = dialog.set_file_name(&req.file_name);
    }
    let filters = parse_filter(&req.filter);
    for f in &filters {
        let pattern_refs: Vec<&str> = f.patterns.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(&f.name, &pattern_refs);
    }
    dialog
}

#[note("file-dialog-pick-open")]
pub fn pick_open(req: &FileDialogRequest) -> Option<Vec<PathBuf>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(answer) = std::env::var(CRAWRAD_FILE_DIALOG_ANSWER) {
            return open_answer(&answer, req);
        }

        let dialog = build_rfd(req);
        if req.multi {
            dialog.pick_files()
        } else {
            dialog.pick_file().map(|p| vec![p])
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = req;
        None
    }
}

#[note("file-dialog-pick-save")]
pub fn pick_save(req: &FileDialogRequest) -> Option<PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(answer) = std::env::var(CRAWRAD_FILE_DIALOG_ANSWER) {
            return save_answer(&answer, req);
        }

        let dialog = build_rfd(req);
        dialog
            .save_file()
            .map(|p| with_default_ext(p, &req.default_ext))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = req;
        None
    }
}

#[note("file-dialog-open-answer")]
pub fn open_answer(answer: &str, req: &FileDialogRequest) -> Option<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = answer
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect();
    match paths.first() {
        None => None,
        Some(first) if !req.multi => Some(vec![first.clone()]),
        Some(_) => Some(paths),
    }
}

#[note("file-dialog-save-answer")]
pub fn save_answer(answer: &str, req: &FileDialogRequest) -> Option<PathBuf> {
    (!answer.is_empty()).then(|| with_default_ext(PathBuf::from(answer.trim()), &req.default_ext))
}

#[note("file-dialog-picture-filter")]
pub const PICTURE_FILTER: &str = "Images|*.png;*.jpg;*.jpeg;*.bmp;*.svg;*.ico";

#[note("file-dialog-pick-picture")]
pub fn pick_picture(file: Option<&str>) -> Option<Result<crawrad_ir::Picture, String>> {
    let req = FileDialogRequest {
        title: "Pick a picture".to_string(),
        filter: PICTURE_FILTER.to_string(),
        ..FileDialogRequest::default()
    };
    let path = match file {
        Some(f) => PathBuf::from(f),
        None => pick_open(&req)?.into_iter().next()?,
    };
    Some(picture_from_file(&path))
}

#[note("file-dialog-picture-from-file")]
pub fn picture_from_file(path: &Path) -> Result<crawrad_ir::Picture, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let picture = crawrad_ir::Picture::from_bytes(bytes);
    if picture.format() == crawrad_ir::ImageFormat::Unknown {
        return Err(format!(
            "not a picture: {} (png, jpeg, bmp, svg or ico)",
            path.display()
        ));
    }
    Ok(match path.file_name() {
        Some(n) => picture.with_source(n.to_string_lossy()),
        None => picture,
    })
}

#[cfg(test)]
#[path = "test/file_dialog/test.rs"]
mod tests;
