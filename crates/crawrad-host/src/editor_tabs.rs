//! **THE EDITOR TAB STRIP** — one tab per open file, without a window.
//!
//! # What the owner asked for
//!
//! > *"we need tabs for each .RS opened in the editor"*
//!
//! He pointed at VS Code, where the strip above the editor carries one tab per
//! open document and disambiguates two files with the same name by showing a
//! slice of their folder (`inspector.rs  …\ide_app`).
//!
//! # The DESIGN tab is not in this list, and that is the whole shape
//!
//! The first tab of this IDE is the canvas, and it is permanent: it cannot be
//! closed, it is not a file, and it has no path to disambiguate. Putting it in
//! the same `Vec` as the files would mean every index here is off by one and
//! every `close` would need a guard against zero.
//!
//! So the canvas is `EditorTabs::active` being `None`, and a file is
//! `Some(i)`. The type makes the invalid state — *"the Design tab was
//! closed"* — unrepresentable instead of guarded against.
//!
//! # Why it lives here and not in a shell
//!
//! Both shells need it and neither owns it, and the rule *what appears in one
//! must appear in the other* has already cost this house the seven divergences
//! seen earlier. This file draws nothing and knows no framework, so it is
//! tested with no window at all — which is how the label arithmetic below can
//! be measured instead of eyeballed.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("editor-tabs-label")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    /// The file name, always — `inspector.rs`.
    pub name: String,
    /// The slice of folder that tells two same-named files apart.
    ///
    /// `None` when the name is already unique among the open tabs, which is
    /// the common case and the one that has to stay quiet: a hint on every tab
    /// would double the width of the strip to solve a problem that is not
    /// there.
    pub hint: Option<String>,
}

impl Label {
    #[note("editor-tabs-label-one-line")]
    pub fn one_line(&self) -> String {
        match &self.hint {
            Some(h) => format!("{}  {h}", self.name),
            None => self.name.clone(),
        }
    }
}

#[note("editor-tabs-editortabs")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EditorTabs {
    files: Vec<PathBuf>,
    /// `None` is the Design tab — see the module header.
    active: Option<usize>,
    /// Each file's caret (0-based line, byte) when it last left the front.
    carets: std::collections::BTreeMap<PathBuf, (usize, usize)>,
}

impl EditorTabs {
    #[note("editor-tabs-editortabs-files")]
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    #[note("editor-tabs-editortabs-len")]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    #[note("editor-tabs-editortabs-is-empty")]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    #[note("editor-tabs-editortabs-active-index")]
    pub fn active_index(&self) -> Option<usize> {
        self.active
    }

    #[note("editor-tabs-editortabs-active-file")]
    pub fn active_file(&self) -> Option<&Path> {
        self.active
            .and_then(|i| self.files.get(i))
            .map(|p| p.as_path())
    }

    #[note("editor-tabs-editortabs-position")]
    pub fn position(&self, path: &Path) -> Option<usize> {
        self.files.iter().position(|p| p == path)
    }

    #[note("editor-tabs-editortabs-open")]
    pub fn open(&mut self, path: impl Into<PathBuf>) -> usize {
        let path = path.into();
        let i = match self.position(&path) {
            Some(i) => i,
            None => {
                self.files.push(path);
                self.files.len() - 1
            }
        };
        self.active = Some(i);
        i
    }

    #[note("editor-tabs-editortabs-activate")]
    pub fn activate(&mut self, i: usize) {
        if i < self.files.len() {
            self.active = Some(i);
        }
    }

    #[note("editor-tabs-editortabs-remember-caret")]
    pub fn remember_caret(&mut self, file: &Path, line: usize, byte: usize) {
        if self.position(file).is_some() {
            self.carets.insert(file.to_path_buf(), (line, byte));
        }
    }

    #[note("editor-tabs-editortabs-caret-of")]
    pub fn caret_of(&self, file: &Path) -> Option<(usize, usize)> {
        self.carets.get(file).copied()
    }

    #[note("editor-tabs-editortabs-show-design")]
    pub fn show_design(&mut self) {
        self.active = None;
    }

    #[note("editor-tabs-editortabs-close")]
    pub fn close(&mut self, i: usize) {
        if i >= self.files.len() {
            return;
        }
        let gone = self.files.remove(i);
        self.carets.remove(&gone);
        self.active = match self.active {
            None => None,
            Some(a) if a < i => Some(a),
            Some(a) if a > i => Some(a - 1),
            // The one in front went away.
            Some(_) if i < self.files.len() => Some(i),
            Some(_) if i > 0 => Some(i - 1),
            Some(_) => None,
        };
    }

    #[note("editor-tabs-editortabs-retain")]
    pub fn retain(&mut self, keep: impl Fn(&std::path::Path) -> bool) {
        for i in (0..self.files.len()).rev() {
            if !keep(&self.files[i]) {
                self.close(i);
            }
        }
    }

    #[note("editor-tabs-editortabs-close-all")]
    pub fn close_all(&mut self) {
        self.files.clear();
        self.active = None;
    }

    #[note("editor-tabs-editortabs-labels")]
    pub fn labels(&self) -> Vec<Label> {
        labels_for(&self.files)
    }
}

#[note("editor-tabs-labels-for")]
pub fn labels_for(paths: &[PathBuf]) -> Vec<Label> {
    let names: Vec<String> = paths.iter().map(|p| file_name_of(p)).collect();
    let mut out = Vec::with_capacity(paths.len());

    for (i, name) in names.iter().enumerate() {
        // Who else is called the same?
        let group: Vec<usize> = names
            .iter()
            .enumerate()
            .filter(|(j, n)| *j != i && *n == name)
            .map(|(j, _)| j)
            .collect();
        if group.is_empty() {
            out.push(Label {
                name: name.clone(),
                hint: None,
            });
            continue;
        }
        let deepest = paths.iter().map(|p| parts_of(p).len()).max().unwrap_or(1);
        let mut hint = None;
        for k in 1..=deepest {
            let mine = tail(&paths[i], k);
            if group.iter().all(|j| tail(&paths[*j], k) != mine) {
                hint = Some(format!("…/{mine}"));
                break;
            }
        }
        out.push(Label {
            name: name.clone(),
            // No number of folders separated them: the paths are the same.
            // Saying nothing beats saying a hint that hints at nothing.
            hint,
        });
    }
    out
}

#[note("editor-tabs-parts-of")]
fn parts_of(p: &Path) -> Vec<String> {
    p.to_string_lossy()
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

#[note("editor-tabs-file-name-of")]
fn file_name_of(p: &Path) -> String {
    parts_of(p)
        .pop()
        .unwrap_or_else(|| p.to_string_lossy().into_owned())
}

#[note("editor-tabs-tail")]
fn tail(p: &Path, k: usize) -> String {
    let mut parts = parts_of(p);
    parts.pop(); // the file itself
    let start = parts.len().saturating_sub(k);
    parts[start..].join("/")
}

#[cfg(test)]
#[path = "test/editor_tabs/test.rs"]
mod test;
