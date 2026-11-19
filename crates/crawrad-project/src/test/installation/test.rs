//! **The INSTALLATION gate** — what enters, what is refused, and where it stops.
//!
//! What is measured here is the entire decision: verification uses the same gate as
//! loading, refusal happens **before** any copying, and what is copied is what the IDE
//! will find upon reopening.

use super::*;

/// A temporary folder for this test round.
fn tmp(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("crawrad_inst_{}_{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).expect("temp folder");
    p
}

/// The `gauge-crawrad` of this repository — the real package.
fn real_package() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("components")
        .join("gauge-crawrad")
}

// ---------------------------------------------------------------------------
// 1 — THE HAPPY PATH
// ---------------------------------------------------------------------------

/// **The real package enters, and what was copied is what the IDE reads.**
///
/// The second half is what counts: installing and having the IDE not find it would be
/// the worst possible outcome — the user would do everything right and still not have the component.
#[test]
fn real_package_enters_and_catalogue_finds_it_afterwards() {
    let base = tmp("happy");
    let v = install_package(&real_package(), &base, &Registry::builtin());
    let Verdict::Installed {
        package,
        classes,
        destination,
    } = &v
    else {
        panic!("repo package must enter: {v:?}");
    };
    assert_eq!(package, "gauge-crawrad");
    assert_eq!(classes, &["Gauge".to_string()]);
    assert!(
        destination.join("manifest.json").is_file(),
        "manifest copied"
    );
    assert!(destination.join("Cargo.toml").is_file(), "crate copied");
    assert!(
        destination.join("src").join("lib.rs").is_file(),
        "the SOURCE went along — without it `cargo add --path` has nothing to compile"
    );

    // And the loading gate finds what the installation gate left.
    let i = crate::installed::from_folder(&base);
    assert!(i.refusals.is_empty(), "refusals: {:?}", i.refusals);
    assert!(
        i.catalog.get("Gauge").is_some(),
        "installed and the IDE doesn't find it — it is the worst outcome this gate has"
    );

    let _ = std::fs::remove_dir_all(&base);
}

/// **`target/` is NOT copied.**
///
/// It is an artifact of the publisher's machine, can have gigabytes, and the user's
/// `cargo` rebuilds it.
#[test]
fn component_target_is_not_copied() {
    let origin = tmp("with_target");
    write_mock_package(&origin, "my-comp", "my-comp");
    std::fs::create_dir_all(origin.join("target").join("debug")).expect("target");
    std::fs::write(
        origin.join("target").join("debug").join("fat.bin"),
        [0u8; 64],
    )
    .expect("artifact");

    let base = tmp("dest_target");
    let v = install_package(&origin, &base, &Registry::builtin());
    assert!(matches!(v, Verdict::Installed { .. }), "{v:?}");
    assert!(
        !base.join("my-comp").join("target").exists(),
        "`target/` was copied"
    );

    let _ = std::fs::remove_dir_all(&origin);
    let _ = std::fs::remove_dir_all(&base);
}

/// **EVERY PACKAGE THIS REPOSITORY DELIVERS PASSES THROUGH THE GATE.**
///
/// It is the most obvious ruler and the easiest not to have: if what comes in the box
/// does not install, the gate is wrong — or the package is.
#[test]
fn every_package_in_repo_passes_verification() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("components");
    let mut seen = 0;
    for e in std::fs::read_dir(&folder).expect("repo `components/`") {
        let p = e.expect("entry").path();
        if !p.join("manifest.json").is_file() {
            continue;
        }
        seen += 1;
        // Against the BUILTIN catalog: the repository's packages cannot
        // depend on each other to pass.
        if let Err(why) = verify_package(&p, &Registry::builtin()) {
            panic!(
                "package `{}` we deliver failed our own installation gate: {why}",
                p.display()
            );
        }
    }
    assert!(
        seen >= 2,
        "expected at least two packages in `components/`, found {seen}"
    );
}

// ---------------------------------------------------------------------------
// 2 — THE REFUSALS, AND NONE OF THEM COPIES ANYTHING
// ---------------------------------------------------------------------------

/// **Folder without manifest is not a package.**
#[test]
fn folder_without_manifest_is_refused_and_nothing_is_copied() {
    let origin = tmp("no_manifest");
    std::fs::write(origin.join("readme.txt"), "nothing here").expect("file");
    let base = tmp("dest_no_manifest");

    let v = install_package(&origin, &base, &Registry::builtin());
    let Verdict::Refused(phrase) = &v else {
        panic!("folder without manifest is not a package: {v:?}");
    };
    assert!(
        phrase.contains("manifest.json"),
        "phrase states what is missing: {phrase}"
    );
    assert_eq!(
        std::fs::read_dir(&base).expect("base").count(),
        0,
        "refused but copied anyway"
    );

    let _ = std::fs::remove_dir_all(&origin);
    let _ = std::fs::remove_dir_all(&base);
}

/// **A class that already exists is refused — and the refusal is the SAME as during loading.**
///
/// Without this check, the package would be copied, and the refusal would only appear on the
/// next IDE startup — with the folder sitting there, half installed, and nobody to remove it.
#[test]
fn duplicate_class_is_refused_before_copying() {
    let origin = tmp("duplicate");
    write_package_with_class(&origin, "my-button", "Button");
    let base = tmp("dest_duplicate");

    let v = install_package(&origin, &base, &Registry::builtin());
    let Verdict::Refused(phrase) = &v else {
        panic!("`Button` is already in builtin catalogue: {v:?}");
    };
    assert!(
        phrase.to_lowercase().contains("button"),
        "phrase names class: {phrase}"
    );
    assert_eq!(
        std::fs::read_dir(&base).expect("base").count(),
        0,
        "refused AFTER copying — which is what this order exists to prevent"
    );

    let _ = std::fs::remove_dir_all(&origin);
    let _ = std::fs::remove_dir_all(&base);
}

/// **The crate name must match the manifest name.**
///
/// Generated code writes the type with the manifest name and `cargo add --path`
/// installs the one from `Cargo.toml`. If they diverge, the user's project will not
/// compile — and the message appears during `cargo build` for someone who merely designed a form.
#[test]
fn crate_with_different_name_is_refused() {
    let origin = tmp("wrong_name");
    write_mock_package(&origin, "my-comp", "other-thing");
    let base = tmp("dest_wrong_name");

    let v = install_package(&origin, &base, &Registry::builtin());
    let Verdict::Refused(phrase) = &v else {
        panic!("names diverge: {v:?}");
    };
    assert!(
        phrase.contains("my-comp") && phrase.contains("other-thing"),
        "the phrase displays BOTH names, otherwise one cannot fix it: {phrase}"
    );

    let _ = std::fs::remove_dir_all(&origin);
    let _ = std::fs::remove_dir_all(&base);
}

/// **Code-generating package without Cargo.toml is refused.**
#[test]
fn package_generating_code_without_cargo_toml_is_refused() {
    let origin = tmp("no_crate");
    write_mock_package(&origin, "my-comp", "my-comp");
    std::fs::remove_file(origin.join("Cargo.toml")).expect("removing crate");
    let base = tmp("dest_no_crate");

    let v = install_package(&origin, &base, &Registry::builtin());
    let Verdict::Refused(phrase) = &v else {
        panic!("generates code without crate: {v:?}");
    };
    assert!(phrase.contains("Cargo.toml"), "{phrase}");

    let _ = std::fs::remove_dir_all(&origin);
    let _ = std::fs::remove_dir_all(&base);
}

/// **Installing twice does not overwrite silently.**
///
/// Overwriting would erase the version in use, and the open IDE would continue with
/// the old one in memory — two truths at the same time, and neither on screen.
#[test]
fn installing_again_does_not_silently_overwrite() {
    let base = tmp("twice");
    let p = real_package();
    assert!(matches!(
        install_package(&p, &base, &Registry::builtin()),
        Verdict::Installed { .. }
    ));
    let v = install_package(&p, &base, &Registry::builtin());
    let Verdict::Refused(phrase) = &v else {
        panic!("second attempt must be refused: {v:?}");
    };
    assert!(
        phrase.contains("already installed"),
        "phrase says it is already there: {phrase}"
    );

    let _ = std::fs::remove_dir_all(&base);
}

// ---------------------------------------------------------------------------
// 3 — STATUS BAR PHRASE
// ---------------------------------------------------------------------------

/// **Both outcomes have a phrase, and it explains what to do next.**
///
/// Success says the package is in the palette now: the install reloads the
/// catalog (`installation::install_now`), so nothing waits for a restart.
#[test]
fn success_phrase_says_it_is_in_the_palette() {
    let base = tmp("phrase");
    let v = install_package(&real_package(), &base, &Registry::builtin());
    let phrase = v.phrase();
    assert!(
        phrase.contains("Gauge"),
        "phrase names what was installed: {phrase}"
    );
    assert!(
        phrase.contains("in the palette now"),
        "the phrase must say the components can be used at once: {phrase}"
    );

    let refusal = Verdict::Refused("no manifest.json".into());
    assert!(refusal.phrase().starts_with("Not installed:"));

    let _ = std::fs::remove_dir_all(&base);
}

// ---------------------------------------------------------------------------
// mock packages for testing
// ---------------------------------------------------------------------------

fn write_mock_package(folder: &Path, manifest_name: &str, crate_name: &str) {
    write_package_files(folder, manifest_name, crate_name, "MyClass");
}

fn write_package_with_class(folder: &Path, name: &str, class: &str) {
    write_package_files(folder, name, name, class);
}

fn write_package_files(folder: &Path, manifest_name: &str, crate_name: &str, class: &str) {
    std::fs::create_dir_all(folder.join("src")).expect("src directory");
    let manifest = format!(
        r#"{{
  "sdk": {sdk},
  "package": {{
    "name": "{manifest_name}",
    "version": "0.1.0",
    "description": "a mock package for testing"
  }},
  "components": [
    {{
      "class": "{class}",
      "display": "{class}",
      "palette": "Additional",
      "prefix": "mc",
      "default_size": [80.0, 24.0],
      "targets": ["*"],
      "props": [
        {{
          "key": "name",
          "display": "Name",
          "category": "identity",
          "kind": "text",
          "default": ""
        }}
      ]
    }}
  ]
}}"#,
        sdk = crawrad_package::manifest::SDK
    );
    std::fs::write(folder.join("manifest.json"), manifest).expect("manifest");
    std::fs::write(
        folder.join("Cargo.toml"),
        format!("[package]\nname = \"{crate_name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
    )
    .expect("crate");
    std::fs::write(folder.join("src").join("lib.rs"), "pub struct MyClass;\n").expect("source");
}
