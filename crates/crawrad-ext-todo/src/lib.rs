#![forbid(unsafe_code)]
//! **The TODO extension**: the first extension of the IDE's
//! UI, written against the same contract any third party gets. It reads the
//! project's Rust sources for `TODO`, `FIXME` and `HACK` comments and gives
//! the IDE three things: a mark on each line (`editor/setLineMarks`), a tree
//! grouped by file (`view/setTree`), and a list an agent reads (`todos.list`).
//!
//! This library is the pure half — reading a text, walking a project, making
//! the marks and the tree. The binary (`crawrad-todo`) only speaks the protocol.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crawrad_extension_api::marks::{LineMark, SetLineMarksParams};
use crawrad_extension_api::views::{ItemLocation, SetTreeParams, ViewItem};
use crawrad_note::note;

#[note("view-id")]
pub const VIEW_ID: &str = "todos";

#[note("tags")]
const TAGS: [(&str, &str); 3] = [("TODO", "todo"), ("FIXME", "fixme"), ("HACK", "hack")];

#[note("todo")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    /// Relative to the project root, `/` separators.
    pub file: String,
    /// 0-based line.
    pub line: u32,
    /// The tag's bytes in the line.
    pub start: u32,
    pub end: u32,
    /// `todo`, `fixme` or `hack`.
    pub kind: String,
    /// What the comment says after the tag (its `(who)` and `:` dropped).
    pub text: String,
}

#[note("comment-start")]
fn comment_start(line: &str) -> Option<usize> {
    let slash = [line.find("//"), line.find("/*")]
        .into_iter()
        .flatten()
        .min();
    match slash {
        Some(i) => Some(i + 2),
        None => line
            .trim_start()
            .starts_with('*')
            .then(|| line.len() - line.trim_start().len()),
    }
}

#[note("wordy")]
fn wordy(s: &str, i: usize) -> bool {
    s[i..]
        .chars()
        .next()
        .is_some_and(|c| c.is_alphanumeric() || c == '_')
}

#[note("scan-text")]
pub fn scan_text(text: &str) -> Vec<(u32, u32, u32, &'static str, String)> {
    let mut found = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let Some(from) = comment_start(line) else {
            continue;
        };
        let comment = &line[from..];
        let first = TAGS.iter().filter_map(|&(tag, kind)| {
            let mut at = 0;
            while let Some(i) = comment[at..].find(tag) {
                let (s, e) = (at + i, at + i + tag.len());
                let alone = (s == 0 || !wordy(comment, s - 1)) && !wordy(comment, e);
                if alone {
                    return Some((s, e, kind));
                }
                at = e;
            }
            None
        });
        let Some((s, e, kind)) = first.min_by_key(|&(s, _, _)| s) else {
            continue;
        };
        let rest = comment[e..].trim_start();
        let rest = match rest.strip_prefix('(') {
            Some(r) => r.split_once(')').map_or(r, |(_, after)| after),
            None => rest,
        };
        let said = rest
            .trim_start_matches([':', '-', ' '])
            .trim_end()
            .trim_end_matches("*/")
            .trim()
            .to_string();
        found.push((n as u32, (from + s) as u32, (from + e) as u32, kind, said));
    }
    found
}

#[note("sources")]
pub fn sources(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(dir) = dirs.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                if name != "target" && !name.starts_with('.') {
                    dirs.push(path);
                }
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

#[note("scan-project")]
pub fn scan_project(root: &Path) -> Vec<Todo> {
    let mut all = Vec::new();
    for path in sources(root) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let rel = path.strip_prefix(root).unwrap_or(&path);
        let file = rel.to_string_lossy().replace('\\', "/");
        for (line, start, end, kind, text) in scan_text(&text) {
            let (file, kind) = (file.clone(), kind.to_string());
            all.push(Todo {
                file,
                line,
                start,
                end,
                kind,
                text,
            });
        }
    }
    all
}

#[note("marks")]
pub fn marks(root: &Path, tagged: &[Todo], before: &[Todo]) -> Vec<SetLineMarksParams> {
    let mut by: BTreeMap<&str, Vec<LineMark>> = BTreeMap::new();
    for t in before {
        by.entry(t.file.as_str()).or_default();
    }
    for t in tagged {
        by.entry(t.file.as_str()).or_default().push(LineMark {
            id: format!("{}:{}", t.file, t.line),
            line: t.line,
            start: t.start,
            end: t.end,
            label: t.kind.to_uppercase(),
            kind: t.kind.clone(),
        });
    }
    let path = |f: &str| root.join(f).to_string_lossy().replace('\\', "/");
    by.into_iter()
        .map(|(f, marks)| SetLineMarksParams {
            file: path(f),
            marks,
        })
        .collect()
}

#[note("tree")]
pub fn tree(tagged: &[Todo]) -> SetTreeParams {
    let mut files: Vec<ViewItem> = Vec::new();
    for t in tagged {
        if files.last().is_none_or(|f| f.id != t.file) {
            files.push(ViewItem {
                id: t.file.clone(),
                label: t.file.clone(),
                detail: None,
                kind: None,
                location: None,
                children: Vec::new(),
            });
        }
        let Some(file) = files.last_mut() else {
            continue;
        };
        file.children.push(ViewItem {
            id: format!("{}:{}", t.file, t.line),
            label: format!("{}: {}", t.kind.to_uppercase(), t.text),
            detail: Some(format!("line {}", t.line + 1)),
            kind: Some(t.kind.clone()),
            location: Some(ItemLocation {
                file: t.file.clone(),
                line: t.line,
            }),
            children: Vec::new(),
        });
    }
    for f in &mut files {
        f.detail = Some(f.children.len().to_string());
    }
    SetTreeParams {
        view_id: VIEW_ID.to_string(),
        items: files,
    }
}

#[note("list")]
pub fn list(tagged: &[Todo]) -> serde_json::Value {
    let items = tagged.iter().map(|t| {
        serde_json::json!({ "file": t.file, "line": t.line + 1, "kind": t.kind, "text": t.text })
    });
    serde_json::json!({ "count": tagged.len(), "todos": items.collect::<Vec<_>>() })
}

#[cfg(test)]
#[path = "test/scan/test.rs"]
mod tests;
