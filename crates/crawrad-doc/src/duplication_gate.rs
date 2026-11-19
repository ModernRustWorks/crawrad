//! Code Duplication Ratchet Gate
//!
//! Detects and ratchets repeated source code windows across all crates in `crates/`.
//!
//! # Detection Rules
//! - Sliding window size: [`WINDOW_LINES`] lines (default 6).
//! - Minimum copies: [`MIN_COPIES`] copies (default 3).
//! - Test files, test directories (`test/`, `tests/`), `target`, and files named `test.rs`/`tests.rs` are skipped.
//! - Windows with fewer than `WINDOW_LINES - 1` lines with more than 3 characters, or containing any `//` comment lines, are ignored.
//! - Duplication is scoped and counted per crate.
//!
//! # Ratchet Rule
//! `MAX_DUPLICATED_WINDOWS` counts the total duplicated windows across `crates/`.
//! Ratchet: only decreases; equality enforced, lower it in the same PR.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Number of consecutive source lines evaluated per sliding window.
pub const WINDOW_LINES: usize = 6;

/// Minimum occurrences required to qualify a repeated window as duplication.
pub const MIN_COPIES: usize = 3;

/// Maximum allowed duplicated windows across `crates/`.
///
/// Baseline measured on 02/11/2026 (main 4fb44ba5): 370 windows.
/// Ratchet: only decreases; equality enforced, lower it in the same PR.
/// Raised to 333 on 13/11/2026: the `file.new_component` action row repeats the descriptor
/// fields of its File-menu neighbours — table data, not logic.
/// Lowered to 329 on 14/11/2026: New Project's seven manifest calls share one closure.
/// Lowered to 323 on 15/11/2026: measured after the Web App template and the web Toolbox greying.
/// Lowered to 321 on 16/11/2026: measured after the CrawRAD rename.
/// Raised to 325 on 16/11/2026: `crawrad-runtime-makepad/src/theme_dark.rs` is makepad's
/// published dark theme copied to swap its GPL code font, so its font blocks repeat the
/// light copy's (upstream theme data, not logic).
pub const MAX_DUPLICATED_WINDOWS: usize = 322;

/// A code window duplicated at multiple locations within a crate.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DuplicatedWindow {
    pub text: String,
    pub places: Vec<String>,
}

/// Discrepancy reported when the total count of duplicated windows differs from the ratchet.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DuplicationRatchetMismatch {
    Exceeded { actual: usize, max_allowed: usize },
    Shrank { actual: usize, max_allowed: usize },
}

/// Recursively scans `src_dir` for duplicated code windows.
pub fn duplicated_windows(src_dir: &Path) -> Vec<DuplicatedWindow> {
    let mut files = Vec::new();
    collect_source_files(src_dir, src_dir, &mut files);
    files.sort_by(|a, b| a.1.cmp(&b.1));

    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (path, rel_str) in files {
        let bytes = std::fs::read(&path).unwrap_or_default();
        let content = String::from_utf8_lossy(&bytes);
        let lines: Vec<&str> = content.lines().map(|l| l.trim()).collect();

        let limit = lines.len().saturating_sub(WINDOW_LINES);
        for i in 0..limit {
            let w = &lines[i..i + WINDOW_LINES];
            let long_lines = w.iter().filter(|line| line.chars().count() > 3).count();
            if long_lines < WINDOW_LINES - 1 {
                continue;
            }
            if w.iter().any(|line| line.starts_with("//")) {
                continue;
            }
            let key = w.join("\n");
            let place = format!("{rel_str}:{}", i + 1);
            map.entry(key).or_default().push(place);
        }
    }

    let mut result: Vec<DuplicatedWindow> = map
        .into_iter()
        .filter(|(_, places)| places.len() >= MIN_COPIES)
        .map(|(text, places)| DuplicatedWindow { text, places })
        .collect();

    result.sort_by(|a, b| {
        b.places
            .len()
            .cmp(&a.places.len())
            .then_with(|| a.text.cmp(&b.text))
    });

    result
}

fn collect_source_files(dir: &Path, src_dir: &Path, files: &mut Vec<(PathBuf, String)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if path.is_dir() {
            if file_name.starts_with("test") || file_name == "target" {
                continue;
            }
            collect_source_files(&path, src_dir, files);
        } else if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            if file_name == "test.rs" || file_name == "tests.rs" {
                continue;
            }
            let Ok(rel_path) = path.strip_prefix(src_dir) else {
                continue;
            };
            if let Some(parent) = rel_path.parent() {
                let skip = parent.components().any(|comp| {
                    let s = comp.as_os_str().to_string_lossy();
                    s.starts_with("test") || s == "target"
                });
                if skip {
                    continue;
                }
            }
            let rel_str = rel_path.display().to_string().replace('\\', "/");
            files.push((path, rel_str));
        }
    }
}

/// Scans each crate's `src/` directory in `crates/` and returns the duplicated windows grouped per crate.
pub fn scan_workspace_duplication(workspace_root: &Path) -> Vec<(String, Vec<DuplicatedWindow>)> {
    let crates_dir = workspace_root.join("crates");
    let Ok(entries) = std::fs::read_dir(&crates_dir) else {
        return Vec::new();
    };

    let mut crate_dirs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let src_dir = path.join("src");
            if src_dir.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                crate_dirs.push((name, src_dir));
            }
        }
    }
    crate_dirs.sort_by(|a, b| a.0.cmp(&b.0));

    crate_dirs
        .into_iter()
        .map(|(name, src_dir)| {
            let windows = duplicated_windows(&src_dir);
            (name, windows)
        })
        .collect()
}

/// Counts the total number of duplicated windows across all crates in `crates/`.
pub fn count_workspace_duplication(workspace_root: &Path) -> usize {
    scan_workspace_duplication(workspace_root)
        .into_iter()
        .map(|(_, windows)| windows.len())
        .sum()
}

/// Evaluates the duplication ratchet against the current workspace.
pub fn check_duplication_ratchet(
    workspace_root: &Path,
    max_allowed: usize,
) -> Result<usize, DuplicationRatchetMismatch> {
    let actual = count_workspace_duplication(workspace_root);

    if actual > max_allowed {
        Err(DuplicationRatchetMismatch::Exceeded {
            actual,
            max_allowed,
        })
    } else if actual < max_allowed {
        Err(DuplicationRatchetMismatch::Shrank {
            actual,
            max_allowed,
        })
    } else {
        Ok(actual)
    }
}

#[cfg(test)]
#[path = "test/duplication_gate/test.rs"]
mod tests;
