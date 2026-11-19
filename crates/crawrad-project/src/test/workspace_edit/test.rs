use super::*;

#[test]
fn a_member_joins_a_one_line_list_and_the_rest_of_the_file_is_kept() {
    let manifest = "# my group\n[workspace]\nresolver = \"2\"\nmembers = [\"app\"]\n\n[profile.release]\nlto = true\n";
    let out = with_member(manifest, "lib").expect("edit");
    assert_eq!(
        out,
        "# my group\n[workspace]\nresolver = \"2\"\nmembers = [\"app\", \"lib\"]\n\n[profile.release]\nlto = true\n"
    );
    assert_eq!(workspace_list(&out, "members"), vec!["app", "lib"]);
}

#[test]
fn a_member_joins_a_list_across_lines_with_its_indentation() {
    let manifest = "[workspace]\nmembers = [\n    \"app\",\n    # the core\n    \"core\"\n]\n";
    let out = with_member(manifest, "lib").expect("edit");
    assert_eq!(
        out,
        "[workspace]\nmembers = [\n    \"app\",\n    # the core\n    \"core\",\n    \"lib\",\n]\n"
    );
    assert_eq!(workspace_list(&out, "members"), vec!["app", "core", "lib"]);
}

#[test]
fn an_empty_list_a_missing_key_and_a_repeated_member() {
    assert_eq!(
        with_member("[workspace]\nmembers = []\n", "a").unwrap(),
        "[workspace]\nmembers = [\"a\"]\n"
    );
    assert_eq!(
        with_member("[workspace]\nresolver = \"2\"\n", "a").unwrap(),
        "[workspace]\nmembers = [\"a\"]\nresolver = \"2\"\n"
    );
    let once = "[workspace]\nmembers = [\"a\"]\n";
    assert_eq!(
        with_member(once, "a").unwrap(),
        once,
        "listed already: the file does not change"
    );
    assert!(
        with_member("[package]\nname = \"x\"\n", "a").is_err(),
        "no [workspace]: refused, nothing invented"
    );
}

#[test]
fn the_active_project_replaces_default_members_wherever_it_is() {
    let one_line = "[workspace]\nmembers = [\"a\", \"b\"]\ndefault-members = [\"a\"]\n";
    let out = with_active(one_line, "b").unwrap();
    assert_eq!(
        out,
        "[workspace]\nmembers = [\"a\", \"b\"]\ndefault-members = [\"b\"]\n"
    );
    let across = "[workspace]\ndefault-members = [\n  \"a\",\n]\nmembers = [\"a\", \"b\"]\n";
    assert_eq!(
        with_active(across, "b").unwrap(),
        "[workspace]\ndefault-members = [\"b\"]\nmembers = [\"a\", \"b\"]\n"
    );
    let missing = "[workspace]\nmembers = [\"a\", \"b\"]\n";
    assert_eq!(
        workspace_list(&with_active(missing, "b").unwrap(), "default-members"),
        vec!["b"]
    );
}

#[test]
fn a_member_is_named_relative_to_the_root_and_must_be_inside_it() {
    let root = Path::new("C:/ws");
    assert_eq!(member_name(root, Path::new("C:/ws/app")).unwrap(), "app");
    assert_eq!(
        member_name(root, Path::new("C:/ws/crates/core")).unwrap(),
        "crates/core"
    );
    assert!(member_name(root, Path::new("C:/elsewhere/app")).is_err());
    assert!(member_name(root, root).is_err());
}

/// The whole round trip on disk: a workspace, a project added, the second made
/// active — and `members()`, which the Explorer and `cargo` agree with, follows.
#[test]
fn adding_a_project_and_making_it_active_on_disk() {
    let root = std::env::temp_dir().join(format!("crawrad_ws_edit_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    for m in ["app", "lib"] {
        std::fs::create_dir_all(root.join(m)).unwrap();
        std::fs::write(
            root.join(m).join(MANIFEST),
            format!("[package]\nname = \"{m}\"\n"),
        )
        .unwrap();
    }
    std::fs::write(
        root.join(MANIFEST),
        crate::workspace::workspace_manifest("app"),
    )
    .unwrap();
    assert_eq!(role_of(&root), WorkspaceRole::Root);
    assert_eq!(
        role_of(&root.join("app")),
        WorkspaceRole::Member { active: true }
    );
    assert_eq!(
        role_of(&root.join("lib")),
        WorkspaceRole::Other,
        "not a member yet"
    );

    add_member(&root, &root.join("lib")).expect("add lib");
    assert_eq!(
        role_of(&root.join("lib")),
        WorkspaceRole::Member { active: false }
    );
    set_active(&root, &root.join("lib")).expect("lib active");
    assert_eq!(
        role_of(&root.join("lib")),
        WorkspaceRole::Member { active: true }
    );
    assert_eq!(
        role_of(&root.join("app")),
        WorkspaceRole::Member { active: false }
    );
    assert_eq!(
        crate::workspace::members(&root)[0],
        root.join("lib"),
        "the active one comes first"
    );

    std::fs::create_dir_all(root.join("loose")).unwrap();
    assert!(
        add_member(&root, &root.join("loose")).is_err(),
        "a folder without Cargo.toml is not a project"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// New Project, then Add Project: the second project lands inside the same
/// workspace as a member, and the first one stays the active one.
#[test]
fn add_project_creates_a_member_inside_the_new_workspace() {
    let base = std::env::temp_dir().join(format!("crawrad_add_project_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();
    let first = crate::project_template::create_project_template(
        &base,
        "suite",
        "console_app",
        false,
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .expect("New Project");
    let root = base.join("suite");
    let second = crate::project_template::add_project(
        &root,
        "tools",
        "library_crate",
        crawrad_codegen::AlvoGerador::Makepad,
    )
    .expect("Add Project");
    assert_eq!(second, root.join("tools"));
    assert!(second.join("src").join("lib.rs").exists());
    assert_eq!(
        crate::workspace::members(&root),
        vec![first.clone(), second.clone()],
        "the first stays active"
    );
    assert_eq!(role_of(&second), WorkspaceRole::Member { active: false });
    assert!(
        crate::project_template::add_project(
            &root,
            "tools",
            "console_app",
            crawrad_codegen::AlvoGerador::Makepad
        )
        .is_err(),
        "the same name twice: refused, nothing overwritten"
    );
    let _ = std::fs::remove_dir_all(&base);
}
