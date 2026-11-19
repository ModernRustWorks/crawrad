//! Unit tests for editor footer formatting.

use std::path::Path;

use super::{
    INSERCAO, MODIFIED, SOMENTE_LEITURA, caret_label, editor_footer_cells, language_label,
    mode_label, modified_label,
};

#[test]
fn mode_label_reflects_editability() {
    assert_eq!(mode_label(true), INSERCAO);
    assert_eq!(mode_label(false), SOMENTE_LEITURA);
}

#[test]
fn modified_label_reflects_dirty_state() {
    assert_eq!(modified_label(true), MODIFIED);
    assert_eq!(modified_label(false), "");
}

#[test]
fn language_label_detects_known_file_types() {
    assert_eq!(language_label(None), "Rust");
    assert_eq!(
        language_label(Some(Path::new("src/form_main.designer.rs"))),
        "CrawRAD Form"
    );
    assert_eq!(language_label(Some(Path::new("src/main.rs"))), "Rust");
    assert_eq!(language_label(Some(Path::new("Cargo.toml"))), "TOML");
    assert_eq!(language_label(Some(Path::new("README.md"))), "Markdown");
    assert_eq!(language_label(Some(Path::new("data.json"))), "JSON");
    assert_eq!(language_label(Some(Path::new("script.py"))), "Plain Text");
}

#[test]
fn caret_label_formats_line_and_col() {
    let line_text = "fn main() {";
    assert_eq!(caret_label(0, 0, line_text), "Ln 1, Col 1");
    assert_eq!(caret_label(4, 3, line_text), "Ln 5, Col 4");
}

#[test]
fn editor_footer_cells_structure() {
    let cells = editor_footer_cells(0, 0, "hello", true, true, "UTF-8  LF", "Rust");
    assert_eq!(cells.len(), 6);
    assert_eq!(cells[0], "Ln 1, Col 1");
    assert_eq!(cells[1], INSERCAO);
    assert_eq!(cells[2], MODIFIED);
    assert_eq!(cells[3], "Spaces: 4");
    assert_eq!(cells[4], "UTF-8  LF");
    assert_eq!(cells[5], "Rust");
}
