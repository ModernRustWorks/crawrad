use crate::FilePath;

#[test]
fn default_is_empty() {
    let fp = FilePath::default();
    assert!(fp.is_empty());
    assert_eq!(fp.as_str(), "");
    assert_eq!(fp.len(), 0);
    assert_eq!(fp.file_name(), None);
    assert_eq!(fp.extension(), None);
    assert!(!fp.exists());
}

#[test]
fn constructors_and_accessors() {
    let fp = FilePath::new("src/main.rs");
    assert!(!fp.is_empty());
    assert_eq!(fp.as_str(), "src/main.rs");
    assert_eq!(fp.file_name(), Some("main.rs"));
    assert_eq!(fp.extension(), Some("rs"));

    let empty = FilePath::empty();
    assert!(empty.is_empty());
}

#[test]
fn normalization() {
    let fp = FilePath::new("c:\\projects\\app\\main.rs");
    assert_eq!(fp.normalized(), "c:/projects/app/main.rs");
}

#[test]
fn conversions_and_traits() {
    let fp1: FilePath = "data/database.sqlite".into();
    assert_eq!(fp1.as_str(), "data/database.sqlite");
    assert_eq!(fp1.extension(), Some("sqlite"));

    let fp2: FilePath = String::from("assets/icon.png").into();
    assert_eq!(fp2.file_name(), Some("icon.png"));
    assert_eq!(fp2.extension(), Some("png"));

    assert_eq!(format!("{fp1}"), "data/database.sqlite");
    assert_eq!(fp1.into_string(), "data/database.sqlite");
}

#[test]
fn existence_check() {
    let cargo = FilePath::new("Cargo.toml");
    assert!(cargo.exists());

    let nonexistent = FilePath::new("this_file_does_not_exist_12345.xyz");
    assert!(!nonexistent.exists());
}
