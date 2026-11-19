use crate::creation::missing_dependencies;
use crawrad_codegen::Dependencia;

#[test]
fn missing_dependencies_skips_present_and_keeps_absent() {
    let manifest = r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
makepad-widgets = "1.0.0"
crawrad-runtime.workspace = true

[dev-dependencies]
crawrad-runtime-makepad = "0.1"
"#;
    let deps = vec![
        Dependencia::registro("makepad-widgets", "1.0.0"),
        Dependencia::registro("crawrad-runtime", "0.1.0"),
        Dependencia::registro("crawrad-runtime-makepad", "0.1.0"),
    ];

    let missing = missing_dependencies(manifest, deps);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].nome(), "crawrad-runtime-makepad");
}

#[test]
fn dependency_named_inside_another_line_is_not_present() {
    let manifest = r#"
[dependencies]
other-crate = { version = "1.0", features = ["makepad-widgets"] }
"#;
    let deps = vec![Dependencia::registro("makepad-widgets", "1.0.0")];

    let missing = missing_dependencies(manifest, deps);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].nome(), "makepad-widgets");
}

#[test]
fn table_header_dependency_is_detected_as_present() {
    let manifest = r#"
[dependencies.makepad-widgets]
version = "1.0"
"#;
    let deps = vec![Dependencia::registro("makepad-widgets", "1.0.0")];

    let missing = missing_dependencies(manifest, deps);
    assert!(missing.is_empty());
}

#[test]
fn the_first_form_is_form_main_then_the_first_free_number() {
    let dir = std::env::temp_dir().join("crawrad-test-new-form-name");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let name = || crate::creation::new_form_name(&dir);
    assert_eq!(name(), ("form_main".to_string(), "Form1".to_string()));
    std::fs::write(dir.join("form_main.crawform"), "{}").unwrap();
    assert_eq!(name(), ("form_1".to_string(), "Form1".to_string()));
    std::fs::write(dir.join("form_1.crawform"), "{}").unwrap();
    assert_eq!(name(), ("form_2".to_string(), "Form2".to_string()));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn create_unit_file_creates_sequential_units() {
    let dir = std::env::temp_dir().join("crawrad-test-create-unit");
    let _ = std::fs::remove_dir_all(&dir);
    let u1 = crate::creation::create_unit_file(&dir).unwrap();
    assert!(u1.ends_with("unit_1.rs"));
    assert!(u1.exists());
    let u2 = crate::creation::create_unit_file(&dir).unwrap();
    assert!(u2.ends_with("unit_2.rs"));
    assert!(u2.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn create_subfolder_creates_sequential_folders() {
    let dir = std::env::temp_dir().join("crawrad-test-create-subfolder");
    let _ = std::fs::remove_dir_all(&dir);
    let f1 = crate::creation::create_subfolder(&dir).unwrap();
    assert!(f1.ends_with("folder_1"));
    assert!(f1.is_dir());
    let f2 = crate::creation::create_subfolder(&dir).unwrap();
    assert!(f2.ends_with("folder_2"));
    assert!(f2.is_dir());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn create_form_file_creates_valid_form() {
    let dir = std::env::temp_dir().join("crawrad-test-create-form");
    let _ = std::fs::remove_dir_all(&dir);
    let reg = crawrad_registry::Registry::builtin();
    let form = crate::creation::create_form_file(&dir, &reg).unwrap();
    assert!(form.ends_with("form_main.crawform"));
    assert!(form.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

/// Opening or running a project must not rewrite a path dependency the user
/// already wrote: a present `crawrad-binding` is not asked for again.
#[test]
fn present_path_dependency_is_not_added_again() {
    let manifest = r#"
[dependencies]
crawrad-binding = { path = "../../../crates/crawrad-binding", features = ["rusqlite"] }
datagrid = { version = "0.1.0", path = "../../../components/datagrid" }
"#;
    let deps = vec![
        Dependencia::caminho("crawrad-binding", "/x/crawrad-binding").with_features(&["rusqlite"]),
        Dependencia::caminho("datagrid", "/x/datagrid"),
        Dependencia::caminho("datagrid-sql", "/x/datagrid-sql"),
    ];

    let missing = missing_dependencies(manifest, deps);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].nome(), "datagrid-sql");
}
