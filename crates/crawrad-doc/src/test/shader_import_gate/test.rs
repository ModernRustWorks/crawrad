//! The gate itself, plus the proof it can go red and that it still sees.

use super::{blocks, scan, without_line_comments};
use std::path::{Path, PathBuf};

/// The macro's name, built so this file's own samples are not blocks the gate
/// walks over.
const MACRO: &str = concat!("live_", "design!");

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

/// **Every `live_design!` that calls `Sdf2d` imports the shader library.**
#[test]
fn every_block_that_calls_sdf2d_imports_the_shaders() {
    let found = scan(&crates_dir());
    assert!(
        found.missing_import.is_empty(),
        "these {MACRO} blocks call `Sdf2d::` without importing it:\n  {}\n\n\
         Add `use link::shaders::*;` to the block. Without it the shader fails \
         when the app starts (`Call not found Sdf2d::…`) and draws nothing — \
         and a windowed app has no console to say so.",
        found.missing_import.join("\n  ")
    );
}

/// **The gate still sees the blocks it guards.** A walk that loses its path
/// (a folder renamed, a pattern changed) finds nothing and stays green; these
/// two are known callers, and the gate must count them.
#[test]
fn the_gate_sees_the_known_callers() {
    let found = scan(&crates_dir());
    for known in [
        "crawrad-runtime-makepad/src/canvas.rs",
        "crawrad-runtime-makepad/src/dialogs.rs",
    ] {
        assert!(
            found.calling_sdf.iter().any(|at| at.starts_with(known)),
            "the gate no longer sees {known} — it went blind, it did not pass. Saw: {:?}",
            found.calling_sdf
        );
    }
}

/// **A block without the import is really caught**, on a temporary tree.
#[test]
fn a_block_without_the_import_is_caught() {
    let dir = std::env::temp_dir().join(format!("crawrad-shader-gate-{}", std::process::id()));
    let src = dir.join("app/src");
    std::fs::create_dir_all(&src).unwrap();
    let missing = format!(
        "{MACRO} {{\n    use link::theme::*;\n    Box = {{ fn pixel(self) -> vec4 {{\n        \
         let sdf = Sdf2d::viewport(self.pos);\n        return sdf.result;\n    }} }}\n}}\n"
    );
    let imported = format!(
        "{MACRO} {{\n    use link::shaders::*;\n    Box = {{ fn pixel(self) -> vec4 {{ \
         let sdf = Sdf2d::viewport(self.pos); return sdf.result; }} }}\n}}\n"
    );
    std::fs::write(src.join("lib.rs"), missing).unwrap();
    std::fs::write(src.join("ok.rs"), imported).unwrap();
    let found = scan(&dir);
    std::fs::remove_dir_all(&dir).ok();
    assert_eq!(found.missing_import, vec!["app/src/lib.rs:1".to_string()]);
    assert_eq!(found.calling_sdf.len(), 2);
}

#[test]
fn a_comment_that_names_sdf2d_calls_nothing() {
    let src = format!("{MACRO} {{\n    // `Sdf2d::box` doubles the radius\n    X = {{}}\n}}\n");
    let found = blocks(&src);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].0, 1);
    assert!(!without_line_comments(found[0].1).contains("Sdf2d::"));
}

#[test]
fn prose_that_names_the_macro_opens_no_block() {
    let src = format!("/// Emits the {MACRO} DSL block for a CheckBox.\nfn f() {{}}\n");
    assert!(blocks(&src).is_empty());
}
