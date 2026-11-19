//! The gate against writing `border_size` at run time.
//!
//! A plain `<View>`'s `draw_bg` has **no** `border_size` at all — only
//! `RectView`/`RoundedView` declare it (`makepad-widgets/src/view_ui.rs`). An
//! `apply_over` that writes it on a `View` compiles, logs `Apply without file …
//! no matching field: border_size` on every redraw of the running app, and draws
//! nothing: the ListView cells were plain `View`s, and their grid lines
//! never drew. On a `RoundedView` the same write works (its border is a uniform
//! of that widget's own draw call) — but which widget a `live!` block lands on is
//! not visible from the source, so the gate keeps the safer rule: declare the
//! border in the DSL and switch an **instance** colour at run time.
//!
//! The static DSL lives in `live_design! { … }`; a `live! { … }` block is what
//! `apply_over` applies at run time. So the gate counts `border_size` inside
//! `live!` blocks outside tests. The ratchet reached **0** when the New Project
//! template cards became the palette's `CrawradCard`: none is left.

use std::path::{Path, PathBuf};

/// The runtime writes of `border_size` still in the tree — only goes down.
pub const MAX_RUNTIME_BORDER_SIZE_WRITES: usize = 0;

/// Every `path:line` under `crates_dir` that writes `border_size` at run time.
pub(super) fn scan(crates_dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    walk(crates_dir, &mut files);
    files.sort();
    let mut found = Vec::new();
    for file in files {
        let Ok(source) = std::fs::read_to_string(&file) else {
            continue;
        };
        let short = file
            .strip_prefix(crates_dir)
            .unwrap_or(&file)
            .display()
            .to_string();
        for line in runtime_writes(&source) {
            found.push(format!("{short}:{line}"));
        }
    }
    found
}

/// The one-based lines of `source` where a `live! { … }` block names
/// `border_size` (line comments ignored).
pub(super) fn runtime_writes(source: &str) -> Vec<usize> {
    let code: String = source
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    let mut lines = Vec::new();
    let mut from = 0;
    while let Some(at) = code[from..].find("live!") {
        let start = from + at + "live!".len();
        from = start;
        let rest = &code[start..];
        let Some(open) = rest.find(|c: char| !c.is_whitespace()) else {
            break;
        };
        if !rest[open..].starts_with('{') {
            continue;
        }
        let body = start + open;
        let mut depth = 0usize;
        let mut end = code.len();
        for (i, c) in code[body..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = body + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(k) = code[body..end].find("border_size") {
            lines.push(code[..body + k].matches('\n').count() + 1);
        }
        from = end;
    }
    lines
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if path.is_dir() {
            if matches!(name, "target" | "test" | "tests") {
                continue;
            }
            walk(&path, out);
        } else if name.ends_with(".rs") && name != "tests.rs" && name != "uniform_write_gate.rs" {
            out.push(path);
        }
    }
}

#[cfg(test)]
#[path = "test/uniform_write_gate/test.rs"]
mod tests;
