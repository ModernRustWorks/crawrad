use super::*;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn no_permissive_crate_depends_on_a_gpl_crate() {
    let crates = workspace_crates(&root());
    assert!(
        crates.contains_key("crawrad-runtime"),
        "the scan found the workspace"
    );
    let bad = violations(&crates);
    assert!(
        bad.is_empty(),
        "these MIT/Apache crates ship inside user apps but depend on GPL IDE crates: {bad:?}"
    );
}

#[test]
fn the_gate_catches_a_runtime_crate_pulling_a_gpl_one() {
    let mut crates = BTreeMap::new();
    crates.insert(
        "crawrad-runtime".to_string(),
        (
            "MIT OR Apache-2.0".to_string(),
            vec!["crawrad-ide".to_string()],
        ),
    );
    crates.insert(
        "crawrad-ide".to_string(),
        ("GPL-3.0-only".to_string(), vec![]),
    );
    assert_eq!(
        violations(&crates),
        vec![("crawrad-runtime".to_string(), "crawrad-ide".to_string())]
    );
}
