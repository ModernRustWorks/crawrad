//! Pure formatting and labels for the code editor footer.
//!
//! Provides the presentation logic for editor status bar cells: caret line/col,
//! edit mode ("Insert" / "Read only"), modified state ("Modified"), file encoding,
//! and language mode.

use crawrad_note::note;
use std::path::Path;

#[note("editor-footer-somente-leitura")]
pub const SOMENTE_LEITURA: &str = "Read only";

#[note("editor-footer-insercao")]
pub const INSERCAO: &str = "Insert";

#[note("editor-footer-modified")]
pub const MODIFIED: &str = "Modified";

#[note("editor-footer-language-label")]
pub fn language_label(path: Option<&Path>) -> &'static str {
    let Some(path) = path else {
        return "Rust";
    };
    let nome = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if nome.ends_with(".designer.rs") {
        "CrawRAD Form"
    } else if nome.ends_with(".rs") {
        "Rust"
    } else if nome.ends_with(".toml") {
        "TOML"
    } else if nome.ends_with(".md") {
        "Markdown"
    } else if nome.ends_with(".json") {
        "JSON"
    } else {
        "Plain Text"
    }
}

#[note("editor-footer-caret-label")]
pub fn caret_label(line: usize, byte: usize, line_text: &str) -> String {
    format!(
        "Ln {}, Col {}",
        line + 1,
        crate::word_motion::column_of_byte(line_text, byte)
    )
}

#[note("editor-footer-mode-label")]
pub fn mode_label(editable: bool) -> &'static str {
    if editable { INSERCAO } else { SOMENTE_LEITURA }
}

#[note("editor-footer-modified-label")]
pub fn modified_label(modified: bool) -> &'static str {
    if modified { MODIFIED } else { "" }
}

#[note("editor-footer-editor-footer-cells")]
pub fn editor_footer_cells(
    line: usize,
    byte: usize,
    line_text: &str,
    editable: bool,
    modified: bool,
    encoding: &str,
    language: &str,
) -> Vec<String> {
    vec![
        caret_label(line, byte, line_text),
        mode_label(editable).to_string(),
        modified_label(modified).to_string(),
        "Spaces: 4".to_string(),
        encoding.to_string(),
        language.to_string(),
    ]
}

#[cfg(test)]
#[path = "test/editor_footer/test.rs"]
mod test;
