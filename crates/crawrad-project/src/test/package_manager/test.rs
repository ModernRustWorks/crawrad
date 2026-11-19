//! Tests for the package manager WITHOUT UI.
//!
//! Valid for BOTH shells because the file tested is the same one compiled by both binaries.
//! What stayed on the other side (`src/test/package_manager/test.rs`) is what requires an `egui::Ui`.

use super::*;

#[test]
fn parse_cargo_search_extracts_items_correctly() {
    let output = r#"
rfd = "0.17.2"                    # Rusty File Dialog
brisk-rfd-components = "0.8.1"    # Components for rfd.
serde = "1.0.219"                 # A generic serialization framework
... and 108 crates more (use --limit N to see more)
note: to learn more about a package, run `cargo info <name>`
"#;
    let items = parse_cargo_search(output);
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].name, "rfd");
    assert_eq!(items[0].version, "0.17.2");
    assert_eq!(items[0].description, "Rusty File Dialog");
    // Extra fields are None when parsed from text
    assert!(items[0].downloads.is_none());
    assert!(items[0].documentation.is_none());

    assert_eq!(items[1].name, "brisk-rfd-components");
    assert_eq!(items[1].version, "0.8.1");

    assert_eq!(items[2].name, "serde");
    assert_eq!(items[2].version, "1.0.219");
    assert_eq!(items[2].description, "A generic serialization framework");
}

#[test]
fn parse_dependencies_json_filters_normal_kind() {
    let json = r#"{
        "packages": [
            {
                "name": "my_app",
                "dependencies": [
                    { "name": "rfd", "req": "^0.17", "kind": null },
                    { "name": "serde", "req": "^1.0", "kind": null },
                    { "name": "tempfile", "req": "^3.0", "kind": "dev" },
                    { "name": "cc", "req": "^1.0", "kind": "build" }
                ]
            }
        ]
    }"#;

    let deps = parse_dependencies_json(json, Some("my_app"));
    assert_eq!(deps.len(), 2);
    assert_eq!(deps[0].name, "rfd");
    assert_eq!(deps[0].req, "^0.17");
    assert_eq!(deps[1].name, "serde");
    assert_eq!(deps[1].req, "^1.0");
}

#[test]
fn format_number_adds_commas() {
    assert_eq!(format_number(0), "0");
    assert_eq!(format_number(999), "999");
    assert_eq!(format_number(1_000), "1,000");
    assert_eq!(format_number(29_449_600), "29,449,600");
    assert_eq!(format_number(10_505_845), "10,505,845");
}

#[test]
fn humanize_date_converts_iso_to_relative_text() {
    // Very old dates must generate "about N years ago"
    let result = humanize_date("2020-01-15T10:00:00Z");
    assert!(result.contains("year"), "expected 'year' in '{result}'");

    // Invalid string returns the string itself
    assert_eq!(humanize_date("garbage"), "garbage");
}

#[test]
fn parse_crates_io_api_response() {
    let json = r#"{
        "crates": [
            {
                "name": "rfd",
                "max_version": "0.17.2",
                "description": "Rusty File Dialog",
                "downloads": 29449600,
                "recent_downloads": 10505845,
                "updated_at": "2026-01-12T20:57:19.255609Z",
                "documentation": "https://docs.rs/rfd",
                "repository": "https://github.com/PolyMeilex/rfd",
                "homepage": null
            }
        ]
    }"#;

    let resp: CratesIoResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.crates.len(), 1);
    let c = &resp.crates[0];
    assert_eq!(c.name, "rfd");
    assert_eq!(c.downloads, 29_449_600);
    assert_eq!(c.recent_downloads, Some(10_505_845));
    assert_eq!(c.documentation.as_deref(), Some("https://docs.rs/rfd"));
    assert_eq!(
        c.repository.as_deref(),
        Some("https://github.com/PolyMeilex/rfd")
    );
    assert!(c.homepage.is_none());
}

// ------------------------------------------- the installed list, off the click

/// A one-member crate with one runtime dependency, in its own temp folder.
fn crate_with_one_dependency(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("crawrad_pkg_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).expect("temp crate");
    std::fs::write(root.join("src/lib.rs"), "").expect("lib.rs");
    std::fs::write(
        root.join("Cargo.toml"),
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"1\"\n"),
    )
    .expect("Cargo.toml");
    root
}

/// Pumps the channels until the read is back, like the shell's pulse does.
fn wait_for_read(pm: &mut PackageManagerState, root: &Path) {
    let limit = std::time::Instant::now() + std::time::Duration::from_secs(60);
    while pm.loading_installed {
        assert!(
            std::time::Instant::now() < limit,
            "cargo metadata never answered"
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
        pm.update_channels(Some(root), None);
    }
}

#[test]
fn reading_installed_does_not_block_caller() {
    let root = crate_with_one_dependency("does_not_block");
    let mut pm = PackageManagerState::default();
    pm.open(Some(&root), None);
    // Back from `open` with the thread still out, and no false zero on the tab.
    assert!(
        pm.loading_installed,
        "the read must be off the calling thread"
    );
    assert_eq!(pm.installed_count(), None);
    wait_for_read(&mut pm, &root);
    assert_eq!(pm.installed_count(), Some(1));
    assert_eq!(pm.installed[0].name, "serde");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn unchanged_manifest_does_not_rerun_cargo() {
    let root = crate_with_one_dependency("unchanged");
    let mut pm = PackageManagerState::default();
    pm.open(Some(&root), None);
    wait_for_read(&mut pm, &root);
    // Showing the panel again with nothing changed: no thread goes out.
    pm.open(Some(&root), None);
    assert!(
        !pm.loading_installed,
        "an unchanged manifest was read again"
    );
    assert_eq!(pm.installed_count(), Some(1));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn changed_manifest_is_reread_and_old_list_stays_on_screen() {
    let root = crate_with_one_dependency("changed");
    let mut pm = PackageManagerState::default();
    pm.open(Some(&root), None);
    wait_for_read(&mut pm, &root);
    // A `cargo add` made outside the IDE: the manifest changes on disk.
    std::thread::sleep(std::time::Duration::from_millis(20));
    let manifest = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest).expect("read") + "anyhow = \"1\"\n";
    std::fs::write(&manifest, text).expect("write");
    pm.reload_installed(Some(&root), None);
    assert!(
        pm.loading_installed,
        "a changed manifest must be read again"
    );
    assert_eq!(
        pm.installed_count(),
        Some(1),
        "the old list stays while the new one is read"
    );
    wait_for_read(&mut pm, &root);
    let names: Vec<&str> = pm.installed.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["anyhow", "serde"]);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn any_thread_out_keeps_panel_busy() {
    let mut pm = PackageManagerState::default();
    assert!(!pm.is_busy());
    pm.loading_installed = true;
    assert!(
        pm.is_busy(),
        "the pulse must keep beating while cargo metadata is out"
    );
}
