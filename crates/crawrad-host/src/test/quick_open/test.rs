//! Quick Open's filter, line:column and command-mode parsing.

use super::*;

#[test]
fn parse_linha_coluna_diversos_formatos() {
    assert_eq!(parse_line_column("42"), Some((42, 1)));
    assert_eq!(parse_line_column(":42"), Some((42, 1)));
    assert_eq!(parse_line_column("42:15"), Some((42, 15)));
    assert_eq!(parse_line_column(":42:15"), Some((42, 15)));
    assert_eq!(parse_line_column("  :100:20  "), Some((100, 20)));
    assert_eq!(parse_line_column("0"), Some((1, 1)));
    assert_eq!(parse_line_column(""), None);
    assert_eq!(parse_line_column("abc"), None);
}

#[test]
fn filter_files_by_name_and_path() {
    let lista = vec![
        PathBuf::from("src/main.rs"),
        PathBuf::from("src/ide_app/editor_keys.rs"),
        PathBuf::from("Cargo.toml"),
        PathBuf::from("README.md"),
    ];

    let filtrado_editor = filter_files(&lista, "editor");
    assert_eq!(filtrado_editor.len(), 1);
    assert_eq!(
        filtrado_editor[0],
        PathBuf::from("src/ide_app/editor_keys.rs")
    );

    let filtrado_rs = filter_files(&lista, ".rs");
    assert_eq!(filtrado_rs.len(), 2);

    let filtrado_tudo = filter_files(&lista, "");
    assert_eq!(filtrado_tudo.len(), 4);
}

#[test]
fn is_command_mode_sees_the_prefix() {
    assert!(is_command_mode(">"));
    assert!(is_command_mode("> build"));
    assert!(is_command_mode("  > save"));
    assert!(!is_command_mode("main.rs"));
    assert!(!is_command_mode("Cargo.toml"));
    assert!(!is_command_mode(""));
}
