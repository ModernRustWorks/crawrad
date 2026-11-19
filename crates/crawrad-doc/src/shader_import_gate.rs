//! **The gate that keeps `Sdf2d` in reach of every shader that calls it.**
//!
//! # The defect it closes
//!
//! Measured on 30/09/2026, running `meu_app` under the debugger: the program's
//! output said
//!
//! ```text
//! Error …/crawrad-runtime-makepad/src/dialogs.rs:12:407 - Call not found `Sdf2d::viewport`
//! ```
//!
//! and the message dialog of every generated app drew no box. The `pixel`
//! shader of `CrawradMessageDialog` called `Sdf2d::viewport`, but its
//! `live_design!` imported only `link::widgets` and `link::theme`; `Sdf2d`
//! lives in `link::shaders` (makepad-draw's `shader/std.rs`). Nothing failed at
//! build time: the DSL is compiled when the app starts, and the error goes to
//! a console a windowed app does not have.
//!
//! # What it checks
//!
//! Every `live_design!` block under `crates/` that calls `Sdf2d::` outside a
//! `//` comment also imports the shader library, by either of the two names
//! this workspace uses: `use link::shaders::*;` or
//! `use makepad_draw::shader::std::*;`.
//!
//! # What it does NOT check
//!
//! DSL the code generator writes into a string is text for the user's
//! project, not a block of ours; it is read only when it is spelled as a
//! `live_design!` block in the source.

use std::path::{Path, PathBuf};

/// The imports that bring `Sdf2d` into a `live_design!` block.
const SHADER_IMPORTS: &[&str] = &["use link::shaders::*;", "use makepad_draw::shader::std::*;"];

/// What the gate saw: every block that calls `Sdf2d`, and the ones among them
/// without the import, as `path:line` from `crates/`.
#[derive(Debug, Default)]
pub(super) struct Scan {
    pub calling_sdf: Vec<String>,
    pub missing_import: Vec<String>,
}

/// Walks every `.rs` under `crates_dir`, skipping build output.
pub(super) fn scan(crates_dir: &Path) -> Scan {
    let mut files = Vec::new();
    walk(crates_dir, &mut files);
    files.sort();
    let mut scan = Scan::default();
    for file in files {
        let Ok(source) = std::fs::read_to_string(&file) else {
            continue;
        };
        for (line, block) in blocks(&source) {
            let code = without_line_comments(block);
            if !code.contains("Sdf2d::") {
                continue;
            }
            let at = format!("{}:{line}", short_path(&file, crates_dir));
            if !SHADER_IMPORTS.iter().any(|i| code.contains(i)) {
                scan.missing_import.push(at.clone());
            }
            scan.calling_sdf.push(at);
        }
    }
    scan
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            walk(&path, out);
        } else if path.extension().is_some_and(|x| x == "rs") {
            out.push(path);
        }
    }
}

/// Every `live_design! { … }` in `source`, with the one-based line it opens on.
/// The block ends at the brace that closes the first one.
pub(super) fn blocks(source: &str) -> Vec<(usize, &str)> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(at) = source[from..].find("live_design!") {
        let start = from + at;
        from = start + "live_design!".len();
        // A mention in prose (`the live_design! DSL block`) opens no block.
        let rest = &source[from..];
        let Some(open) = rest.find(|c: char| !c.is_whitespace()) else {
            break;
        };
        if !rest[open..].starts_with('{') {
            continue;
        }
        let body_start = from + open;
        let mut depth = 0usize;
        let mut end = None;
        for (i, c) in source[body_start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(body_start + i + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else {
            break;
        };
        let line = source[..start].matches('\n').count() + 1;
        found.push((line, &source[body_start..end]));
        from = end;
    }
    found
}

/// `block` with each `//` comment cut off its line: a comment that names
/// `Sdf2d::box` calls nothing.
pub(super) fn without_line_comments(block: &str) -> String {
    block
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn short_path(file: &Path, crates_dir: &Path) -> String {
    file.strip_prefix(crates_dir)
        .unwrap_or(file)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
#[path = "test/shader_import_gate/test.rs"]
mod tests;
