//! **The change bars** — which lines this session touched, saved or not.
//!
//! Beside the line numbers the editor draws a thin bar: green on a line new
//! or changed in this session and already saved, yellow on one changed and
//! not saved yet. Three texts answer it:
//!
//! | text | what it is |
//! |---|---|
//! | `session_base` | the disk text the first time this run showed the file |
//! | `saved` | what the disk holds now, as far as the editor knows |
//! | `current` | what the editor holds |
//!
//! A line of `current` that differs from `saved` is unsaved; any other line
//! that differs from `session_base` was saved in this session. Computed only
//! when one of the three moves — never per frame.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use similar::{DiffTag, TextDiff};

/// How a line differs from the text the file had when this session first
/// showed it: changed and written to disk, or changed and not saved yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineChange {
    Saved,
    Unsaved,
}

/// Remembers the baseline text of files when first opened in a session,
/// and computes change bars against their current disk and editor states.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChangeTracker {
    session_base: HashMap<PathBuf, String>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// The disk text of `file` the first time this run shows it; later
    /// showings keep the first.
    pub fn remember(&mut self, file: &Path, disk: &str) {
        self.session_base
            .entry(file.to_path_buf())
            .or_insert_with(|| disk.to_owned());
    }

    /// `file` was reloaded because it changed outside: its session starts
    /// again from the text now on disk.
    pub fn forget(&mut self, file: &Path) {
        self.session_base.remove(file);
    }

    /// Computes the change bars for `file` comparing its `session_base` (falling
    /// back to `saved` when unknown) against `saved` and `current`.
    pub fn changes(&self, file: &Path, saved: &str, current: &str) -> Vec<(usize, LineChange)> {
        let base = self.session_base.get(file).map_or(saved, String::as_str);
        line_changes(base, saved, current)
    }
}

/// The lines of `current` changed against `saved` (`Unsaved`) or, failing
/// that, against `session_base` (`Saved`), sorted by line.
pub fn line_changes(session_base: &str, saved: &str, current: &str) -> Vec<(usize, LineChange)> {
    let unsaved = changed_lines(saved, current);
    let in_session = changed_lines(session_base, current);
    let mut out: Vec<(usize, LineChange)> =
        unsaved.iter().map(|&l| (l, LineChange::Unsaved)).collect();
    out.extend(
        in_session
            .into_iter()
            .filter(|l| unsaved.binary_search(l).is_err())
            .map(|l| (l, LineChange::Saved)),
    );
    out.sort_by_key(|(l, _)| *l);
    out
}

/// The lines of `new` inserted or replaced against `old`, sorted.
///
/// Both texts get a closing `\n` first, so a last line without one does not
/// count as changed when a line is added after it: the editor shows the same
/// line either way.
fn changed_lines(old: &str, new: &str) -> Vec<usize> {
    if old == new {
        return Vec::new();
    }
    let (old, new) = (format!("{old}\n"), format!("{new}\n"));
    let diff = TextDiff::from_lines(&old, &new);
    let mut lines = Vec::new();
    for op in diff.ops() {
        let (tag, _, range) = op.as_tag_tuple();
        if matches!(tag, DiffTag::Insert | DiffTag::Replace) {
            lines.extend(range);
        }
    }
    lines
}

#[cfg(test)]
#[path = "test/line_change/test.rs"]
mod test;
