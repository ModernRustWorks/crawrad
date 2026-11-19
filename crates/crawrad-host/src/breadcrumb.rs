//! **The symbol the caret is in**, for the editor's breadcrumb: the nearest
//! `fn` above the caret (`name()`), or `impl` / `pub struct` / `pub enum`
//! (`name`), each with its Codicons mark — `symbol-method` U+EA8C,
//! `symbol-class` U+EB5B. The shell draws the text in a family that falls
//! back to Codicons for these points.

use crawrad_note::note;
use std::path::{Component, Path};

#[note("breadcrumb-method")]
pub const METHOD: &str = "\u{ea8c}";
#[note("breadcrumb-type")]
pub const TYPE: &str = "\u{eb5b}";

#[note("breadcrumb-symbol-item")]
pub fn symbol_item(text: &str, line: usize) -> Option<String> {
    let lines: Vec<&str> = text.lines().take(line + 1).collect();
    for l in lines.into_iter().rev() {
        let t = l.trim();
        let is_fn = ["pub fn ", "pub(crate) fn ", "pub(super) fn ", "fn "]
            .iter()
            .any(|p| t.starts_with(p));
        if is_fn {
            let after = t.find("fn ").map(|i| &t[i + 3..]).unwrap_or("");
            let end = after.find(['(', '<']).unwrap_or(after.len());
            let name = after[..end].trim();
            if !name.is_empty() {
                return Some(format!("{METHOD} {name}()"));
            }
        } else if ["impl ", "pub struct ", "pub enum "]
            .iter()
            .any(|p| t.starts_with(p))
        {
            let name = t[..t.find('{').unwrap_or(t.len())].trim();
            if !name.is_empty() {
                return Some(format!("{TYPE} {name}"));
            }
        }
    }
    None
}

#[note("breadcrumb-path-items")]
pub fn path_items(file: &Path, root: Option<&Path>) -> Vec<String> {
    let rel = root.and_then(|r| file.strip_prefix(r).ok()).unwrap_or(file);
    let mut parts: Vec<String> = rel
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        // a drive (`C:`) is a plain component off Windows: never a segment
        .filter(|s| !s.contains(':'))
        .collect();
    if rel == file && parts.len() > 4 {
        parts.drain(..parts.len() - 4);
    }
    let mark = if file.extension().is_some_and(|e| e == "rs") {
        "🦀"
    } else {
        "📄"
    };
    if let Some(first) = parts.first_mut() {
        *first = format!("{mark} {first}");
    }
    parts
}

#[cfg(test)]
#[path = "test/breadcrumb/test.rs"]
mod tests;
