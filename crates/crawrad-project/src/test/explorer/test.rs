//! Unit tests for pure Explorer operations.

use super::{Folds, delete_path, resolve_path, target_folder};
use std::path::Path;

#[test]
fn resolve_path_returns_base_for_empty() {
    let base = Path::new("my_project");
    let resolved = resolve_path(Some(base), Path::new(""));
    assert_eq!(resolved, base);
}

#[test]
fn resolve_path_joins_relative_path() {
    let base = Path::new("my_project");
    let resolved = resolve_path(Some(base), Path::new("src/main.rs"));
    assert_eq!(resolved, base.join("src/main.rs"));
}

#[test]
fn target_folder_defaults_to_src_if_exists() {
    let tmp = std::env::temp_dir().join("crawrad_test_target_folder");
    let _ = std::fs::remove_dir_all(&tmp);
    let src = tmp.join("src");
    let _ = std::fs::create_dir_all(&src);

    let target = target_folder(Some(&tmp), None);
    assert_eq!(target, src);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn target_folder_resolves_file_parent() {
    let tmp = std::env::temp_dir().join("crawrad_test_parent_folder");
    let _ = std::fs::remove_dir_all(&tmp);
    let sub = tmp.join("sub");
    let _ = std::fs::create_dir_all(&sub);
    let file = sub.join("foo.rs");
    let _ = std::fs::write(&file, "");

    let target = target_folder(Some(&tmp), Some(Path::new("sub/foo.rs")));
    assert_eq!(target, sub);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn folds_sections_returns_consistent_state() {
    let mut folds = Folds::default();
    let doc = Path::new("src/form_main.crawform");
    let s = folds.sections(Some(doc), None);
    assert!(!s.legend);
    assert!(!s.open_editors);
    assert!(!s.outline);
    assert!(!s.timeline);
    assert!(!s.dependencies);
}

#[test]
fn delete_path_removes_file_and_dir() {
    let tmp = std::env::temp_dir().join("crawrad_test_delete_path");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(&tmp);
    let file = tmp.join("test.txt");
    let _ = std::fs::write(&file, "hello");
    assert!(file.exists());
    assert!(delete_path(&file).is_ok());
    assert!(!file.exists());
    assert!(delete_path(&tmp).is_ok());
    assert!(!tmp.exists());
}
