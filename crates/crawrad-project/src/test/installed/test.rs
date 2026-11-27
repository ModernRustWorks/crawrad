//! **The IDE loads what is installed** — and reports when it fails.

use super::*;

/// The IDE catalogue includes `Connection` from repo's `components/`.
///
/// This test runs with the working directory in the crate (`crawrad-ide`), and therefore
/// it does **not** find the folder — what it checks is the function over a given
/// folder, which is the same one the scan uses.
#[test]
fn repo_component_folder_loads() {
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("components");
    assert!(
        folder.is_dir(),
        "repo `components/` missing: {}",
        folder.display()
    );

    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    load_folder(&folder, ComponentScope::Installed, &mut i);
    let (catalog, refusals) = (&i.catalog, &i.refusals);

    assert!(
        refusals.is_empty(),
        "repo packages must load cleanly, but were refused:\n  {}",
        refusals.join("\n  ")
    );
    assert!(
        catalog.get("Connection").is_some(),
        "`Connection` not added to IDE catalogue"
    );
    assert!(
        catalog.get("Button").is_some(),
        "builtin Button is still present"
    );
    assert_eq!(
        i.scope.get("Connection"),
        Some(&ComponentScope::Installed),
        "`Connection` must have Installed scope"
    );
}

/// **What `Connection` brings arrives completely** — including the type's crate address.
///
/// Without `crate_module`, it would enter the palette and emerge as `// TODO` in
/// generated code, which was the state until 18/09/2026.
#[test]
fn installed_component_knows_its_originating_crate() {
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("components");
    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    load_folder(&folder, ComponentScope::Installed, &mut i);
    let catalog = &i.catalog;

    let d = catalog.get("Connection").expect("`Connection` loaded");
    assert_eq!(
        d.crate_module, "portadb_crawrad",
        "type module comes from `[package] name` with hyphen turned into underscore"
    );
    assert!(!d.props.is_empty(), "properties came along");
}

/// **A bad manifest does not crash the IDE, and does not vanish.**
///
/// It turns into a sentence. A component that disappears without explanation is the
/// worst thing this gate could do, as `registration.rs` already stated — here is
/// where that sentence reaches whoever installed it.
#[test]
fn malformed_manifest_becomes_message_not_panic() {
    let tmp = std::env::temp_dir().join(format!("crawrad_inst_{}", std::process::id()));
    let pkg = tmp.join("broken");
    std::fs::create_dir_all(&pkg).expect("temp folder");
    std::fs::write(pkg.join(FILE_NAME), "this is not valid TOML {{{").expect("writing");

    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    load_folder(&tmp, ComponentScope::Installed, &mut i);
    let (catalog, refusals) = (&i.catalog, &i.refusals);

    assert_eq!(refusals.len(), 1, "one refusal, from broken package");
    assert!(
        refusals[0].starts_with("broken:"),
        "phrase must name package: {:?}",
        refusals[0]
    );
    assert!(
        catalog.get("Button").is_some(),
        "builtin catalogue remains functional"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

/// A folder that does not exist is not an error — it is the normal case of whoever installed nothing.
#[test]
fn absent_folder_is_not_refusal() {
    let mut i = InstalledPackages {
        catalog: Registry::builtin(),
        refusals: Vec::new(),
        origin: std::collections::BTreeMap::new(),
        packages: Vec::new(),
        scope: std::collections::BTreeMap::new(),
    };
    load_folder(
        std::path::Path::new("non_existent_folder_here"),
        ComponentScope::Installed,
        &mut i,
    );
    let refusals = &i.refusals;
    assert!(refusals.is_empty(), "absence is not refusal: {refusals:?}");
}

static TEST_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn unique_temp_dir(tag: &str) -> PathBuf {
    let id = std::process::id();
    let count = TEST_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let p = std::env::temp_dir().join(format!("crawrad_inst_{tag}_{id}_{count}"));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).expect("create temp dir");
    p
}

fn write_manifest(pkg_dir: &Path, pkg_name: &str, class: &str, prefix: &str) {
    std::fs::create_dir_all(pkg_dir).expect("create pkg dir");
    let json = format!(
        r#"{{
  "sdk": 2,
  "package": {{
    "name": "{pkg_name}",
    "version": "0.1.0",
    "description": "Test component package"
  }},
  "components": [
    {{
      "class": "{class}",
      "display": "{class}",
      "palette": "Additional",
      "prefix": "{prefix}",
      "default_size": [120.0, 40.0],
      "private_state": false,
      "targets": ["*"],
      "doc": "A component for tests.",
      "props": [
        {{
          "key": "name",
          "display": "Name",
          "category": "identity",
          "kind": "text",
          "default": "",
          "doc": "The instance name."
        }}
      ]
    }}
  ]
}}"#
    );
    std::fs::write(pkg_dir.join(FILE_NAME), json).expect("writing manifest.json");
}

#[test]
fn from_folders_assigns_correct_scope_and_badges() {
    let tmp = unique_temp_dir("scope");
    let installed_dir = tmp.join("installed");
    let mine_dir = tmp.join("mine");
    let project_dir = tmp.join("project");

    write_manifest(
        &installed_dir.join("pkg_inst"),
        "pkg-inst",
        "InstComp",
        "ins",
    );
    write_manifest(&mine_dir.join("pkg_mine"), "pkg-mine", "MineComp", "min");
    write_manifest(&project_dir.join("pkg_proj"), "pkg-proj", "ProjComp", "prj");

    let cat = from_folders(&[
        (installed_dir.clone(), ComponentScope::Installed),
        (mine_dir.clone(), ComponentScope::Mine),
        (project_dir.clone(), ComponentScope::Project),
    ]);

    assert!(cat.refusals.is_empty(), "refusals: {:?}", cat.refusals);
    assert_eq!(cat.scope.get("InstComp"), Some(&ComponentScope::Installed));
    assert_eq!(cat.scope.get("MineComp"), Some(&ComponentScope::Mine));
    assert_eq!(cat.scope.get("ProjComp"), Some(&ComponentScope::Project));

    assert_eq!(ComponentScope::Installed.badge(), None);
    assert_eq!(ComponentScope::Project.badge(), Some("project"));
    assert_eq!(ComponentScope::Mine.badge(), Some("mine"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn duplicate_class_registered_once_with_installed_scope_and_refuses_project() {
    let tmp = unique_temp_dir("dup");
    let installed_dir = tmp.join("installed");
    let project_dir = tmp.join("project");

    write_manifest(
        &installed_dir.join("pkg_inst"),
        "pkg-inst",
        "SharedWidget",
        "sh1",
    );
    write_manifest(
        &project_dir.join("pkg_proj"),
        "pkg-proj",
        "SharedWidget",
        "sh2",
    );

    let cat = from_folders(&[
        (installed_dir.clone(), ComponentScope::Installed),
        (project_dir.clone(), ComponentScope::Project),
    ]);

    assert_eq!(
        cat.scope.get("SharedWidget"),
        Some(&ComponentScope::Installed)
    );
    assert_eq!(
        cat.refusals.len(),
        1,
        "expected 1 refusal: {:?}",
        cat.refusals
    );
    assert!(
        cat.refusals[0].contains("pkg_proj"),
        "the refusal names the project package's folder: {}",
        cat.refusals[0]
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn manifest_only_package_gets_scope_without_origin() {
    let tmp = unique_temp_dir("manifest_only");
    let pkg_dir = tmp.join("pkg_manifest_only");
    write_manifest(&pkg_dir, "pkg-manifest-only", "ManifestOnlyWidget", "mow");

    let cat = from_folders(&[(tmp.clone(), ComponentScope::Project)]);

    assert!(cat.refusals.is_empty(), "refusals: {:?}", cat.refusals);
    assert_eq!(
        cat.scope.get("ManifestOnlyWidget"),
        Some(&ComponentScope::Project)
    );
    assert!(
        !cat.origin.contains_key("ManifestOnlyWidget"),
        "manifest-only package without Cargo.toml must have no origin"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn component_folders_pure_function_orders_and_filters_scopes() {
    let tmp = unique_temp_dir("folders");
    let exe_dir = tmp.join("exe");
    let cwd_dir = tmp.join("cwd");
    let config_dir = tmp.join("config");
    let proj_dir = tmp.join("project");

    std::fs::create_dir_all(exe_dir.join("components")).expect("exe components");
    std::fs::create_dir_all(cwd_dir.join("components")).expect("cwd components");
    std::fs::create_dir_all(config_dir.join("components")).expect("config components");
    std::fs::create_dir_all(proj_dir.join("components")).expect("proj components");

    let folders = component_folders(
        Some(&exe_dir),
        Some(&cwd_dir),
        Some(&config_dir),
        Some(&proj_dir),
    );

    assert_eq!(folders.len(), 4);
    assert_eq!(
        folders[0],
        (exe_dir.join("components"), ComponentScope::Installed)
    );
    assert_eq!(
        folders[1],
        (cwd_dir.join("components"), ComponentScope::Installed)
    );
    assert_eq!(
        folders[2],
        (config_dir.join("components"), ComponentScope::Mine)
    );
    assert_eq!(
        folders[3],
        (proj_dir.join("components"), ComponentScope::Project)
    );
    assert_eq!(
        folders.last().map(|(_, s)| *s),
        Some(ComponentScope::Project)
    );

    // When project root is absent, project components do not appear
    let folders_no_proj =
        component_folders(Some(&exe_dir), Some(&cwd_dir), Some(&config_dir), None);
    assert_eq!(folders_no_proj.len(), 3);
    assert!(
        !folders_no_proj
            .iter()
            .any(|(_, s)| *s == ComponentScope::Project)
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn component_folders_scans_config_folder_components() {
    let tmp = unique_temp_dir("config_scan");
    let config_dir = tmp.join("config");
    let comp_dir = config_dir.join("components");
    std::fs::create_dir_all(&comp_dir).expect("create config components dir");

    let folders = component_folders(None, None, Some(&config_dir), None);
    assert!(
        folders.contains(&(comp_dir, ComponentScope::Mine)),
        "components installed under config folder must be found with Mine scope"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn an_exe_built_in_a_checkout_finds_the_suite_that_comes_with_it() {
    let tmp = unique_temp_dir("checkout");
    let exe_dir = tmp.join("target").join("release");
    let elsewhere = tmp.join("elsewhere");
    std::fs::create_dir_all(&exe_dir).expect("exe dir");
    std::fs::create_dir_all(tmp.join("components")).expect("suite");
    std::fs::create_dir_all(&elsewhere).expect("cwd");

    let folders = component_folders(Some(&exe_dir), Some(&elsewhere), None, None);
    assert_eq!(
        folders,
        vec![(tmp.join("components"), ComponentScope::Installed)]
    );

    // An exe that is NOT under `target/` does not walk up.
    let loose = tmp.join("bin");
    std::fs::create_dir_all(&loose).expect("bin");
    assert!(component_folders(Some(&loose), Some(&elsewhere), None, None).is_empty());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn set_project_root_pointer_stability_and_reload() {
    let tmp = unique_temp_dir("root_stat");
    let root_a = tmp.join("root_a");
    let root_b = tmp.join("root_b");
    std::fs::create_dir_all(&root_a).expect("root_a");
    std::fs::create_dir_all(&root_b).expect("root_b");

    let first = set_project_root(Some(&root_a));
    let second = set_project_root(Some(&root_a));
    assert!(
        std::ptr::eq(first, second),
        "same root twice must return the same static catalogue reference"
    );

    let third = set_project_root(Some(&root_b));
    assert!(
        !std::ptr::eq(second, third),
        "different root must rebuild and return a new catalogue reference"
    );

    // Resetting root to None
    let fourth = set_project_root(None);
    assert!(
        !std::ptr::eq(third, fourth),
        "clearing root to None must rebuild and return a new catalogue"
    );

    let fifth = set_project_root(None);
    assert!(
        std::ptr::eq(fourth, fifth),
        "calling None again must return the same catalogue reference"
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn with_notice_joins_the_status_and_the_refusal_notice() {
    use super::with_notice;
    let s = |t: &str| Some(t.to_string());
    assert_eq!(
        with_notice(s("ready"), s("1 refused")),
        s("ready — 1 refused")
    );
    assert_eq!(with_notice(s("ready"), None), s("ready"));
    assert_eq!(with_notice(None, s("1 refused")), s("1 refused"));
    assert_eq!(with_notice(None, None), None);
}
