//! The numbered bookmarks of the session: per file, number 1 to 9 → line.
//!
//! As classic RAD IDEs do it: `Ctrl+Shift+N` sets bookmark N on the caret's
//! line (or clears it there), `Ctrl+N` jumps to it. They live here, not in the
//! editor, because the editor's document is rebuilt every time another file is
//! put in the view; the editor shows them and moves them with the text, and
//! what it ends up with is written back here — the same reconciliation as the
//! breakpoints ([`crate::breakpoints::Breakpoints::sync_view`]).
//!
//! Lines are **zero-based** here: they never leave the IDE.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::breakpoints::file_key;
use crawrad_note::note;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bookmarks {
    by_file: BTreeMap<PathBuf, BTreeMap<u8, usize>>,
}

#[note("bookmarks-bookmarksync")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BookmarkSync {
    /// The view shows another file: give it these `(number, line)`.
    Push(Vec<(u8, usize)>),
    /// The view had changed (a key, an edit), and the session took it.
    Pulled,
    /// Nothing to do.
    Same,
}

impl Bookmarks {
    #[note("bookmarks-bookmarks-of")]
    pub fn of(&self, file: &Path) -> Vec<(u8, usize)> {
        self.by_file
            .get(&file_key(file))
            .map(|m| m.iter().map(|(&n, &line)| (n, line)).collect())
            .unwrap_or_default()
    }

    #[note("bookmarks-bookmarks-line-of")]
    pub fn line_of(&self, file: &Path, n: u8) -> Option<usize> {
        self.by_file.get(&file_key(file))?.get(&n).copied()
    }

    #[note("bookmarks-bookmarks-set")]
    pub fn set(&mut self, file: &Path, bookmarks: &[(u8, usize)]) {
        let key = file_key(file);
        let map: BTreeMap<u8, usize> = bookmarks
            .iter()
            .filter(|(n, _)| (1..=9).contains(n))
            .map(|&(n, line)| (n, line))
            .collect();
        if map.is_empty() {
            self.by_file.remove(&key);
        } else {
            self.by_file.insert(key, map);
        }
    }

    #[note("bookmarks-bookmarks-sync-view")]
    pub fn sync_view(
        &mut self,
        file: &Path,
        view_owner: Option<&Path>,
        view: &[(u8, usize)],
    ) -> BookmarkSync {
        let here = self.of(file);
        let same_file = view_owner.is_some_and(|o| file_key(o) == file_key(file));
        if !same_file {
            return BookmarkSync::Push(here);
        }
        let mut view = view.to_vec();
        view.sort_unstable();
        if view == here {
            return BookmarkSync::Same;
        }
        self.set(file, &view);
        BookmarkSync::Pulled
    }
}

#[cfg(test)]
#[path = "test/bookmarks/test.rs"]
mod tests;
