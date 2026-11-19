//! Unit tests for pure source and center content resolution.

use super::{CenterView, NOT_GENERATED_YET, center_content, package_name_in, without_cr};

#[test]
fn without_cr_strips_carriage_returns() {
    assert_eq!(without_cr("foo\r\nbar\r"), "foo\nbar");
}

#[test]
fn package_name_in_extracts_from_manifest() {
    let manifest = r#"
[package]
name = "my_app"
version = "0.1.0"
"#;
    assert_eq!(package_name_in(manifest), Some("my_app".to_string()));
}

#[test]
fn center_content_returns_not_generated_when_no_active_file() {
    let (text, status) = center_content(None, None, CenterView::Code, None);
    assert_eq!(text, NOT_GENERATED_YET);
    assert!(status.is_none());
}

#[test]
fn center_content_returns_file_content_for_opened_tab() {
    let tmp = std::env::temp_dir().join("crawrad_test_center_content");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let file = tmp.join("other.rs");
    std::fs::write(&file, "fn other() {}\r\n").unwrap();

    let (text, status) = center_content(Some(&file), None, CenterView::Code, None);
    assert_eq!(text, "fn other() {}\n");
    assert!(status.is_none());

    let _ = std::fs::remove_dir_all(&tmp);
}
