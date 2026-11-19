//! What `project.get_tree` answers: the project's folders and files as JSON,
//! hidden entries and `target` left out. Pure of the window, so it lives here.

use std::path::Path;

use crawrad_note::note;
use serde_json::{Value, json};

#[note("project-tree-walk")]
pub fn walk(path: &Path, depth: usize, max_depth: usize) -> Vec<Value> {
    if depth > max_depth {
        return Vec::new();
    }
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut paths: Vec<_> = read_dir.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();
    let mut entries = Vec::new();
    for p in paths {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if name.starts_with('.') || name == "target" {
            continue;
        }
        let shown = p.display().to_string();
        if p.is_dir() {
            let children = walk(&p, depth + 1, max_depth);
            entries
                .push(json!({ "name": name, "path": shown, "is_dir": true, "children": children }));
        } else {
            let size = p.metadata().map(|m| m.len()).unwrap_or(0);
            entries
                .push(json!({ "name": name, "path": shown, "is_dir": false, "size_bytes": size }));
        }
    }
    entries
}

#[note("project-tree-resolve")]
pub fn resolve(path: &str, root: Option<&Path>) -> std::path::PathBuf {
    match root {
        Some(r) if !Path::new(path).is_absolute() => r.join(path),
        _ => std::path::PathBuf::from(path),
    }
}

#[note("project-tree-canonical-key")]
pub fn canonical_key(path_str: &str) -> String {
    path_str
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_string()
}

#[cfg(test)]
#[path = "test/project_tree/test.rs"]
mod tests;
