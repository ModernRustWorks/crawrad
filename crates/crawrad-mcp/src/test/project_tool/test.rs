//! The file-only `project.*` tools, on a scratch project.

use super::{create_args, create_form, created, delete_file, get_dependencies, get_tree};
use serde_json::json;
use std::path::PathBuf;

fn scratch(tag: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("crawrad_mcp_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

#[test]
fn a_new_form_writes_both_files_and_delete_removes_one() {
    let root = scratch("project_tool");
    let ((r, outcome), form) = create_form(Some(root.clone()), &json!({ "name": "form_two" }));
    assert_eq!((r.is_error, outcome), (false, "applied"));
    let form = form.expect("the shell opens the new form");
    assert!(form.is_file() && root.join("src/form_two.rs").is_file());
    let ((_, outcome), gone) = delete_file(Some(&root), &json!({ "path": "src/form_two.rs" }));
    assert_eq!(outcome, "applied");
    assert!(!gone.unwrap().exists() && form.is_file());
    let ((r, _), gone) = delete_file(Some(&root), &json!({ "path": "src/form_two.rs" }));
    assert_eq!(
        (r.data.unwrap()["code"].clone(), gone),
        (json!("file-not-found"), None)
    );
    let (_, outcome) = get_tree(Some(root.clone()), &json!({}));
    assert_eq!(outcome, "applied");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn missing_names_and_projects_refuse_without_touching_disk() {
    let ((r, _), path) = create_form(None, &json!({}));
    assert_eq!(
        (r.data.unwrap()["code"].clone(), path),
        (json!("invalid-params"), None)
    );
    let ((r, _), _) = create_form(None, &json!({ "name": "f" }));
    assert_eq!(r.data.unwrap()["code"], json!("no-project"));
    assert_eq!(get_tree(None, &json!({})).1, "error");
    assert_eq!(get_dependencies(None).1, "error");
}

#[test]
fn dependencies_come_from_the_manifest() {
    let root = scratch("project_deps");
    assert_eq!(
        get_dependencies(Some(root.clone())).0.data.unwrap()["code"],
        json!("io-error")
    );
    std::fs::write(root.join("Cargo.toml"), "[dependencies]\nserde = \"1\"\n").unwrap();
    let (r, outcome) = get_dependencies(Some(root.clone()));
    assert_eq!(
        (r.data.unwrap()["count"].clone(), outcome),
        (json!(1), "applied")
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **`project.create` reads `folder`, else its old name `path`; a missing
/// name or folder refuses; the answer says where, or why not.**
#[test]
fn a_new_project_takes_its_folder_and_answers_where_it_went() {
    let both = json!({ "name": "app", "folder": "C:/a", "path": "C:/b" });
    assert_eq!(
        create_args(&both).ok(),
        Some(("app", "C:/a", "gui_app", false, None))
    );
    let old = json!({ "name": "app", "path": "C:/b", "template": "cli", "git_init": true, "framework": "egui" });
    assert_eq!(
        create_args(&old).ok(),
        Some(("app", "C:/b", "cli", true, Some("egui")))
    );
    let refused = create_args(&json!({ "name": "app" })).unwrap_err();
    assert_eq!(refused.0.data.unwrap()["code"], json!("invalid-params"));
    let (made, outcome) = created(&Ok(PathBuf::from("C:/a/app")), "app", "cli");
    assert_eq!(
        (made.data.unwrap()["template"].clone(), outcome),
        (json!("cli"), "applied")
    );
    assert_eq!(created(&Err("taken".into()), "app", "cli").1, "error");
}
