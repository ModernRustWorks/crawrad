//! Install and uninstall while the IDE runs: copied into user store, registered
//! in project list, running at once; taken back whole — process, actions, cards,
//! marks, views, list entry — keeping user store folder for other projects.

use std::fs;
use std::path::{Path, PathBuf};

use super::{
    ExtensionRef, ProjectExtensions, extensions_dir, installed_dir, legacy_installed_dir,
    project_extensions_file, read_manifest, read_project_extensions, write_project_extensions,
};
use crate::host::ShellExtensionHost;
use crate::tests::get_mock_extension_binary;
use crawrad_extension_api::marks::{LineMark, SetLineMarksParams};

/// A fresh folder for one test.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("crawrad_install_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch");
    dir
}

/// An extension folder as someone hands it over: a manifest, the binary
/// next to it under a relative `entrypoint`.
fn package(dir: &Path) {
    package_with_version(dir, "0.1.0");
}

fn package_with_version(dir: &Path, version: &str) {
    let bin = get_mock_extension_binary();
    let name = bin
        .file_name()
        .expect("name")
        .to_string_lossy()
        .into_owned();
    fs::copy(&bin, dir.join(&name)).expect("binary copied");
    let manifest = serde_json::json!({
        "id": "acme.todo", "name": "TODO", "version": version, "publisher": "Acme",
        "entrypoint": name,
        "contributions": {
            "actions": [{ "id": "mock.tree", "label": "Tree" }],
            "cards": [{ "id": "acme.card", "title": "Card", "area": "left" }],
            "views": [{ "id": "todos", "title": "TODOs" }]
        }
    });
    fs::write(dir.join("manifest.json"), manifest.to_string()).expect("manifest");
}

#[test]
fn a_folder_without_a_manifest_is_refused() {
    let (dir, user_dir) = (scratch("empty"), scratch("user_empty"));
    assert!(read_manifest(&dir).is_err());
    let mut host = ShellExtensionHost::new(None);
    assert!(host.install_into(&dir, &user_dir, &dir).is_err());
    let _ = (fs::remove_dir_all(&dir), fs::remove_dir_all(&user_dir));
}

#[test]
fn installed_live_then_uninstalled_without_a_trace() {
    let (source, root, user_dir) = (scratch("source"), scratch("root"), scratch("user"));
    package(&source);
    let mut host = ShellExtensionHost::new_with_user_dir(Some(root.clone()), user_dir.clone());

    let manifest = host
        .install_into(&root, &user_dir, &source)
        .expect("installed");
    assert_eq!(manifest.id, "acme.todo");
    assert!(
        installed_dir(&root, &user_dir, "acme.todo")
            .is_some_and(|d| d.join("manifest.json").is_file())
    );
    assert!(
        host.install_into(&root, &user_dir, &source).is_err(),
        "twice is refused"
    );

    // it runs at once: its command fills its view
    let args =
        serde_json::json!({ "view_id": "todos", "items": [{ "id": "1", "label": "TODO one" }] });
    host.process_host
        .lock()
        .expect("process host")
        .execute_command("mock.tree", Some(args))
        .expect("ran");
    let views = host.views().expect("views");
    assert_eq!(views.lock().expect("views").views()[0].items.len(), 1);
    let marks = host.line_marks().expect("marks");
    let mark = LineMark {
        id: "m".into(),
        line: 0,
        start: 0,
        end: 1,
        label: "TODO".into(),
        kind: "todo".into(),
    };
    let on = SetLineMarksParams {
        file: "src/a.rs".into(),
        marks: vec![mark],
    };
    marks.lock().expect("marks").set("acme.todo", on);
    assert!(
        host.card_manager
            .lock()
            .expect("cards")
            .get_card("acme.card")
            .is_some()
    );

    host.uninstall(&root, "acme.todo").expect("uninstalled");
    assert!(host.manifests.is_empty());
    assert!(installed_dir(&root, &user_dir, "acme.todo").is_none());
    assert!(
        read_project_extensions(&root)
            .expect("exts")
            .extensions
            .is_empty()
    );
    // user store folder is preserved for other projects
    assert!(user_dir.join("acme.todo@0.1.0").is_dir());
    assert!(views.lock().expect("views").views().is_empty());
    assert!(
        marks
            .lock()
            .expect("marks")
            .on(Path::new("src/a.rs"))
            .is_empty()
    );
    assert!(
        host.card_manager
            .lock()
            .expect("cards")
            .get_card("acme.card")
            .is_none()
    );
    let ph = host.process_host.lock().expect("process host");
    assert!(ph.extensions.is_empty() && ph.action_to_extension.is_empty());
    drop(ph);
    assert!(
        host.uninstall(&root, "acme.todo").is_err(),
        "nothing left to uninstall"
    );

    // and it can come back
    host.install_into(&root, &user_dir, &source)
        .expect("installed again");
    host.uninstall(&root, "acme.todo")
        .expect("uninstalled again");
    let _ = (
        fs::remove_dir_all(&source),
        fs::remove_dir_all(&root),
        fs::remove_dir_all(&user_dir),
    );
}

#[test]
fn install_into_empty_user_dir_creates_folder_and_list_entry() {
    let (source, root, user_dir) = (
        scratch("install_src"),
        scratch("install_root"),
        scratch("install_user"),
    );
    package_with_version(&source, "1.2.3");
    let mut host = ShellExtensionHost::new_with_user_dir(Some(root.clone()), user_dir.clone());

    let manifest = host
        .install_into(&root, &user_dir, &source)
        .expect("installed");
    assert_eq!(manifest.id, "acme.todo");
    assert_eq!(manifest.version, "1.2.3");

    let expected_folder = user_dir.join("acme.todo@1.2.3");
    assert!(expected_folder.is_dir());
    assert!(expected_folder.join("manifest.json").is_file());

    let proj_exts = read_project_extensions(&root).expect("read project extensions");
    assert_eq!(proj_exts.extensions.len(), 1);
    assert_eq!(
        proj_exts.extensions[0],
        ExtensionRef {
            id: "acme.todo".to_string(),
            version: "1.2.3".to_string(),
        }
    );

    let _ = (
        fs::remove_dir_all(&source),
        fs::remove_dir_all(&root),
        fs::remove_dir_all(&user_dir),
    );
}

#[test]
fn second_project_reuses_folder_without_second_copy() {
    let (source, root1, root2, user_dir) = (
        scratch("reuse_src"),
        scratch("reuse_root1"),
        scratch("reuse_root2"),
        scratch("reuse_user"),
    );
    package_with_version(&source, "0.1.0");

    let mut host1 = ShellExtensionHost::new_with_user_dir(Some(root1.clone()), user_dir.clone());
    host1
        .install_into(&root1, &user_dir, &source)
        .expect("host1 installed");

    let target_folder = user_dir.join("acme.todo@0.1.0");
    assert!(target_folder.is_dir());

    // Place a marker file in the installed user store folder
    let marker_file = target_folder.join("marker.txt");
    fs::write(&marker_file, "original").expect("write marker");

    // Second project installs the same extension
    let mut host2 = ShellExtensionHost::new_with_user_dir(Some(root2.clone()), user_dir.clone());
    host2
        .install_into(&root2, &user_dir, &source)
        .expect("host2 installed");

    // The marker file still exists, proving it was not overwritten or wiped
    assert!(marker_file.is_file());
    assert_eq!(
        fs::read_to_string(&marker_file).expect("read marker"),
        "original"
    );

    // Both projects have the extension in their extensions.json
    let p1 = read_project_extensions(&root1).expect("read p1");
    let p2 = read_project_extensions(&root2).expect("read p2");
    assert_eq!(p1.extensions.len(), 1);
    assert_eq!(p2.extensions.len(), 1);
    assert_eq!(p1.extensions[0].id, "acme.todo");
    assert_eq!(p2.extensions[0].id, "acme.todo");

    let _ = (
        fs::remove_dir_all(&source),
        fs::remove_dir_all(&root1),
        fs::remove_dir_all(&root2),
        fs::remove_dir_all(&user_dir),
    );
}

#[test]
fn uninstall_removes_list_entry_and_keeps_user_folder() {
    let (source, root, user_dir) = (
        scratch("uninst_src"),
        scratch("uninst_root"),
        scratch("uninst_user"),
    );
    package_with_version(&source, "0.1.0");
    let mut host = ShellExtensionHost::new_with_user_dir(Some(root.clone()), user_dir.clone());
    host.install_into(&root, &user_dir, &source)
        .expect("installed");

    let store_folder = user_dir.join("acme.todo@0.1.0");
    assert!(store_folder.is_dir());

    host.uninstall(&root, "acme.todo").expect("uninstalled");

    let proj_exts = read_project_extensions(&root).expect("read project extensions");
    assert!(
        proj_exts.extensions.is_empty(),
        "entry removed from project list"
    );
    assert!(store_folder.is_dir(), "user store folder is preserved");

    let _ = (
        fs::remove_dir_all(&source),
        fs::remove_dir_all(&root),
        fs::remove_dir_all(&user_dir),
    );
}

#[test]
fn scan_returns_list_entries_and_legacy_folders_no_duplicates_missing_skipped() {
    let (root, user_dir) = (scratch("scan_root"), scratch("scan_user"));

    // 1. Valid user store extension
    let ext_store_folder = user_dir.join("ext.list@1.0.0");
    fs::create_dir_all(&ext_store_folder).expect("create ext_store_folder");
    fs::write(
        ext_store_folder.join("manifest.json"),
        serde_json::json!({
            "id": "ext.list", "name": "List Ext", "version": "1.0.0", "publisher": "Acme",
            "contributions": {}
        })
        .to_string(),
    )
    .expect("write list manifest");

    // 2. Project list containing ext.list and ext.missing (missing folder in user store)
    let project_exts = ProjectExtensions {
        extensions: vec![
            ExtensionRef {
                id: "ext.list".to_string(),
                version: "1.0.0".to_string(),
            },
            ExtensionRef {
                id: "ext.missing".to_string(),
                version: "2.0.0".to_string(),
            },
        ],
    };
    write_project_extensions(&root, project_exts).expect("write project extensions");

    // 3. Legacy extensions in .crawrad/extensions
    // ext.list in legacy should NOT duplicate (project list wins)
    let legacy_duplicate = extensions_dir(&root).join("ext.list");
    fs::create_dir_all(&legacy_duplicate).expect("create legacy duplicate");
    fs::write(
        legacy_duplicate.join("manifest.json"),
        serde_json::json!({
            "id": "ext.list", "name": "Legacy List Ext", "version": "0.9.0", "publisher": "Acme",
            "contributions": {}
        })
        .to_string(),
    )
    .expect("write legacy duplicate");

    // ext.legacy is only in legacy folder
    let legacy_only = extensions_dir(&root).join("ext.legacy");
    fs::create_dir_all(&legacy_only).expect("create legacy only");
    fs::write(
        legacy_only.join("manifest.json"),
        serde_json::json!({
            "id": "ext.legacy", "name": "Legacy Ext", "version": "0.5.0", "publisher": "Acme",
            "contributions": {}
        })
        .to_string(),
    )
    .expect("write legacy only");

    let scanned = ShellExtensionHost::scan_extensions(&root, &user_dir);

    // Missing folder was skipped with no panic
    assert_eq!(scanned.len(), 2);
    let list_ext = scanned
        .iter()
        .find(|m| m.id == "ext.list")
        .expect("ext.list found");
    assert_eq!(
        list_ext.version, "1.0.0",
        "project list entry won over legacy"
    );

    let legacy_ext = scanned
        .iter()
        .find(|m| m.id == "ext.legacy")
        .expect("ext.legacy found");
    assert_eq!(legacy_ext.version, "0.5.0");

    assert!(scanned.iter().all(|m| m.id != "ext.missing"));

    let _ = (fs::remove_dir_all(&root), fs::remove_dir_all(&user_dir));
}

#[test]
fn extensions_json_round_trip_sorted_by_id() {
    let root = scratch("roundtrip_root");

    let exts = ProjectExtensions {
        extensions: vec![
            ExtensionRef {
                id: "z.tool".to_string(),
                version: "1.0.0".to_string(),
            },
            ExtensionRef {
                id: "a.tool".to_string(),
                version: "2.0.0".to_string(),
            },
            ExtensionRef {
                id: "m.tool".to_string(),
                version: "0.5.0".to_string(),
            },
        ],
    };

    write_project_extensions(&root, exts).expect("write");

    let read_back = read_project_extensions(&root).expect("read");
    let ids: Vec<&str> = read_back.extensions.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(ids, vec!["a.tool", "m.tool", "z.tool"]);

    // Verify it is pretty-printed JSON
    let content = fs::read_to_string(project_extensions_file(&root)).expect("read file");
    assert!(content.contains("{\n"));
    assert!(content.contains("  \"extensions\": [\n"));

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn uninstalling_legacy_extension_deletes_its_folder() {
    let (root, user_dir) = (scratch("legacy_uninst_root"), scratch("legacy_uninst_user"));
    let legacy_ext_dir = extensions_dir(&root).join("acme.legacy");
    fs::create_dir_all(&legacy_ext_dir).expect("create legacy folder");
    fs::write(
        legacy_ext_dir.join("manifest.json"),
        serde_json::json!({
            "id": "acme.legacy", "name": "Legacy", "version": "0.1.0", "publisher": "Acme",
            "contributions": {}
        })
        .to_string(),
    )
    .expect("write manifest");

    let mut host = ShellExtensionHost::new_with_user_dir(Some(root.clone()), user_dir.clone());
    assert!(legacy_installed_dir(&root, "acme.legacy").is_some());
    assert_eq!(host.manifests.len(), 1);

    host.uninstall(&root, "acme.legacy")
        .expect("uninstall legacy");
    assert!(host.manifests.is_empty());
    assert!(legacy_installed_dir(&root, "acme.legacy").is_none());
    assert!(
        !legacy_ext_dir.exists(),
        "legacy folder deleted on uninstall"
    );

    let _ = (fs::remove_dir_all(&root), fs::remove_dir_all(&user_dir));
}
