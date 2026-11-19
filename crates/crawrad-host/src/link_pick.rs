//! **Link item…** of the note dialog: the list that finds an item by
//! name and gives back the intra-doc link `` [`Type::item`] `` to write at
//! the note's caret.
//!
//! The names come from the language server (`workspace/symbol`); this keeps
//! where the link goes, the newest question asked and its answer. An answer
//! to an older question is dropped: the user typed on. Only the project's
//! own items are listed: rust-analyzer also knows the path dependencies,
//! whose items a note cannot link by name.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("link-pick-linkable")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Linkable {
    pub name: String,
    pub container: Option<String>,
    pub file: PathBuf,
    pub line: usize,
}

/// `(name, container, file, line)`, as the language server gives them.
impl From<(String, Option<String>, PathBuf, usize)> for Linkable {
    fn from((name, container, file, line): (String, Option<String>, PathBuf, usize)) -> Self {
        Self {
            name,
            container,
            file,
            line,
        }
    }
}

impl Linkable {
    #[note("link-pick-linkable-path")]
    pub fn path(&self) -> String {
        match &self.container {
            Some(c) => format!("{c}::{}", self.name),
            None => self.name.clone(),
        }
    }

    #[note("link-pick-linkable-link")]
    pub fn link(&self) -> String {
        format!("[`{}`]", self.path())
    }

    #[note("link-pick-linkable-label")]
    pub fn label(&self) -> String {
        let file = self.file.file_name().map(Path::new).unwrap_or(&self.file);
        format!("{}  {}:{}", self.path(), file.display(), self.line + 1)
    }
}

#[note("link-pick-linkpick")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LinkPick {
    caret: Option<usize>,
    root: String,
    tickets: u64,
    pending: Option<u64>,
    pub found: Vec<Linkable>,
}

impl LinkPick {
    #[note("link-pick-linkpick-open")]
    pub fn open(&mut self, caret: usize, root: &Path) {
        *self = Self {
            caret: Some(caret),
            root: comparable(root),
            tickets: self.tickets,
            ..Self::default()
        };
    }

    pub fn is_open(&self) -> bool {
        self.caret.is_some()
    }

    pub fn close(&mut self) {
        (self.caret, self.pending) = (None, None);
        self.found.clear();
    }

    #[note("link-pick-linkpick-ask")]
    pub fn ask(&mut self, typed: &str) -> Option<(u64, String)> {
        let query = typed.trim();
        if !self.is_open() || query.is_empty() {
            self.pending = None;
            self.found.clear();
            return None;
        }
        self.tickets += 1;
        self.pending = Some(self.tickets);
        Some((self.tickets, query.to_string()))
    }

    #[note("link-pick-linkpick-pending")]
    pub fn pending(&self) -> Option<u64> {
        self.pending
    }

    #[note("link-pick-linkpick-answered")]
    pub fn answered(&mut self, ticket: u64, found: Vec<Linkable>) -> bool {
        if self.pending != Some(ticket) {
            return false;
        }
        self.pending = None;
        let under = |f: &Linkable| comparable(&f.file).starts_with(&self.root);
        self.found = found.into_iter().filter(under).collect();
        true
    }

    #[note("link-pick-linkpick-labels")]
    pub fn labels(&self) -> Vec<String> {
        self.found.iter().map(Linkable::label).collect()
    }

    #[note("link-pick-linkpick-choose")]
    pub fn choose(&mut self, i: usize) -> Option<(usize, String)> {
        let link = self.found.get(i)?.link();
        let caret = self.caret?;
        self.close();
        Some((caret, link))
    }

    #[note("link-pick-linkpick-to-json")]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({ "open": self.is_open(), "items": self.labels() })
    }
}

#[note("link-pick-comparable")]
fn comparable(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/").to_lowercase();
    format!("{}/", s.trim_end_matches('/'))
}

#[cfg(test)]
#[path = "test/link_pick/test.rs"]
mod tests;
