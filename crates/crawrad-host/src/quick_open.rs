//! **Quick Open's pure half**: what a typed filter means and which project
//! files it lists. Moved out of the shell so it is tested without a window.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("quick-open-parse-line-column")]
pub fn parse_line_column(input: &str) -> Option<(usize, usize)> {
    let s = input.trim().trim_start_matches(':').trim();
    if s.is_empty() {
        return None;
    }
    if let Some((line_str, col_str)) = s.split_once(':') {
        let l = line_str.trim().parse::<usize>().ok()?;
        let c = col_str.trim().parse::<usize>().unwrap_or(1);
        Some((l.max(1), c.max(1)))
    } else {
        let l = s.parse::<usize>().ok()?;
        Some((l.max(1), 1))
    }
}

#[note("quick-open-filter-files")]
pub fn filter_files(files: &[PathBuf], filter: &str) -> Vec<PathBuf> {
    let f = filter.trim().to_lowercase();
    if f.is_empty() {
        return files.to_vec();
    }
    files
        .iter()
        .filter(|p| {
            let name = p
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let path_text = p.to_string_lossy().to_lowercase();
            name.contains(&f) || path_text.contains(&f)
        })
        .cloned()
        .collect()
}

#[note("quick-open-scan-project-files")]
pub fn scan_project_files(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for input in entries.flatten() {
            let path = input.path();
            let file_name = path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();

            // skip the folders that are not sources
            if path.is_dir() {
                if file_name == "target"
                    || file_name == ".git"
                    || file_name == ".crawrad"
                    || file_name == "node_modules"
                {
                    continue;
                }
                stack.push(path);
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                if matches!(
                    ext.as_str(),
                    "rs" | "crawform" | "toml" | "md" | "json" | "txt"
                ) {
                    found.push(path);
                }
            }
        }
    }

    found.sort();
    found
}

#[note("quick-open-is-command-mode")]
pub fn is_command_mode(filter: &str) -> bool {
    filter.trim_start().starts_with('>')
}

#[cfg(test)]
#[path = "test/quick_open/test.rs"]
mod tests;
