//! # What `crawrad` does, and the binary only wraps it
//!
//! The assembly lives here, and not in `main.rs`, for one reason: **the gate
//! has to reach it.** `the_cli_writes_the_same_as_the_macro` runs this function
//! over the real packages of the tree and compares it with the file the
//! macro's test blessed. If the two writers ever diverge, it goes red, instead
//! of someone finding out on the machine of whoever installed it.
//!
//! See [ADR 0039] for why there are two writers, and what each one guarantees
//! that the other does not.
//!

// No `unsafe`, and the compiler guarantees it, as in every crate of this house.
#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

pub mod note;
use crawrad_note::note;
pub use note::{
    BrokenLink, DanglingAnchor, ExtractReport, ExtractedNote, InlineReport, InlinedNote,
    KindMismatch, MissingDoc, NoteEntry, NoteReport, SkippedDoc, SkippedNote, StaleSignature,
    broken_links, broken_links_from_cargo_json, extract_notes, inline_notes, missing_docs,
    scan as scan_notes,
};

#[cfg(test)]
#[path = "test/contract/test.rs"]
mod contract_test;

#[cfg(test)]
#[path = "test/note/test.rs"]
mod note_test;

#[note("manifest-file")]
pub const MANIFEST_FILE: &str = "manifest.json";

#[note("assemble")]
pub fn assemble(root: &Path) -> Result<String, String> {
    let (name, version, description) = package_identity(root)?;

    let mut blocks = Vec::new();
    for file in source_files(&root.join("src"))? {
        let text = std::fs::read_to_string(&file)
            .map_err(|e| format!("cannot read {}: {e}", file.display()))?;
        let found = crawrad_manifest_gen::from_source(&text)
            .map_err(|e| format!("{}: {e}", file.display()))?;
        for spec in found {
            // Indentation 4: the block goes inside the `components` array,
            // which the box's assembler opens two levels deep.
            blocks.push(spec.to_json(4));
        }
    }
    if blocks.is_empty() {
        return Err(format!(
            "no `#[derive(Component)]` found under {}",
            root.join("src").display()
        ));
    }

    let borrowed: Vec<&str> = blocks.iter().map(String::as_str).collect();
    // **THE ASSEMBLER IS THE BOX'S**, the same one the macro calls. Writing the
    // header here would be a second copy of the format, and it would agree with
    // the first only until someone touched one of the two.
    Ok(crawrad_component::manifest::assemble(
        &name,
        &version,
        &description,
        &borrowed,
    ))
}

#[note("package-identity")]
fn package_identity(root: &Path) -> Result<(String, String, String), String> {
    let path = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let v: toml::Value = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let package = v
        .get("package")
        .ok_or_else(|| format!("{}: no [package] section", path.display()))?;
    let field = |key: &str| -> Result<String, String> {
        package
            .get(key)
            .and_then(|x| x.as_str())
            .map(str::to_string)
            .ok_or_else(|| format!("{}: [package] has no `{key}`", path.display()))
    };
    Ok((field("name")?, field("version")?, field("description")?))
}

#[note("source-files")]
fn source_files(src: &Path) -> Result<Vec<PathBuf>, String> {
    let mut found = Vec::new();
    collect_rs_files(src, &mut found)?;
    found.sort();
    let root = src.join("lib.rs");
    if let Some(i) = found.iter().position(|p| *p == root) {
        found[..=i].rotate_right(1);
    }
    Ok(found)
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        std::fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for e in entries {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = e.path();
        if p.is_dir() {
            collect_rs_files(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    Ok(())
}

#[note("first-difference")]
pub fn first_difference(found: &str, expected: &str) -> Option<String> {
    let a = found.replace("\r\n", "\n");
    let b = expected.replace("\r\n", "\n");
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return Some(format!(
                "  line {}:\n    on disk: {x}\n    source:  {y}",
                i + 1
            ));
        }
    }
    (a.lines().count() != b.lines().count()).then(|| {
        format!(
            "  the file has {} lines and the source says {}",
            a.lines().count(),
            b.lines().count()
        )
    })
}

#[note("same-file")]
pub fn same_file(a: &str, b: &str) -> bool {
    a.replace("\r\n", "\n") == b.replace("\r\n", "\n")
}
