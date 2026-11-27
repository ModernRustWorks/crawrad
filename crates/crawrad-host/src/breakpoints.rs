//! The breakpoints of the session: by file, by line.
//!
//! They live here and not in the editor because more than the editor needs
//! them: the debug session sends them to the adapter (`setBreakpoints`), the
//! catalogue's actions toggle them, and the MCP reads them — all through the
//! host, the one entry point of ADR 0044. The editor shows them and moves them
//! while the text is edited; what it ends up with is written back here.
//!
//! Lines are **one-based**, as the gutter numbers them and as the Debug Adapter
//! Protocol expects them.

use crawrad_note::note;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Breakpoints {
    by_file: BTreeMap<PathBuf, BTreeSet<u32>>,
    /// The LOGPOINTS: a breakpoint line that carries a message is written,
    /// not stopped at. Kept apart so a plain breakpoint costs nothing more.
    log_messages: BTreeMap<PathBuf, BTreeMap<u32, String>>,
    /// Files whose breakpoints were set from outside the editor (the MCP):
    /// the view must be given them, not win over them.
    stale_in_view: BTreeSet<PathBuf>,
}

#[note("breakpoints-file-key")]
pub fn file_key(path: &Path) -> PathBuf {
    match std::fs::canonicalize(path) {
        Ok(full) => {
            let text = full.to_string_lossy();
            // `canonicalize` answers `\\?\C:\…` on Windows, which the debug
            // adapter does not match against the debug info.
            match text.strip_prefix(r"\\?\") {
                Some(plain) => PathBuf::from(plain),
                None => full,
            }
        }
        Err(_) => path.to_path_buf(),
    }
}

impl Breakpoints {
    #[note("breakpoints-breakpoints-lines")]
    pub fn lines(&self, file: &Path) -> Vec<u32> {
        self.by_file
            .get(&file_key(file))
            .map(|lines| lines.iter().copied().collect())
            .unwrap_or_default()
    }

    #[note("breakpoints-breakpoints-set-lines")]
    pub fn set_lines(&mut self, file: &Path, lines: impl IntoIterator<Item = u32>) {
        let file = file_key(file);
        let lines: BTreeSet<u32> = lines.into_iter().filter(|&l| l > 0).collect();
        if let Some(messages) = self.log_messages.remove(&file) {
            let old = self.by_file.get(&file).cloned().unwrap_or_default();
            let mut gone = old.difference(&lines);
            let mut new = lines.difference(&old);
            let mut kept = BTreeMap::new();
            let mut moved = BTreeMap::new();
            for (line, message) in messages {
                if lines.contains(&line) {
                    kept.insert(line, message);
                } else {
                    moved.insert(line, message);
                }
            }
            for (line, message) in moved {
                for g in gone.by_ref() {
                    if *g == line {
                        if let Some(n) = new.next() {
                            kept.insert(*n, message);
                        }
                        break;
                    }
                    new.next();
                }
            }
            if !kept.is_empty() {
                self.log_messages.insert(file.clone(), kept);
            }
        }
        if lines.is_empty() {
            self.by_file.remove(&file);
        } else {
            self.by_file.insert(file, lines);
        }
    }

    #[note("breakpoints-breakpoints-set-lines-from-outside")]
    pub fn set_lines_from_outside(&mut self, file: &Path, lines: impl IntoIterator<Item = u32>) {
        self.set_lines(file, lines);
        self.stale_in_view.insert(file_key(file));
    }

    #[note("breakpoints-breakpoints-toggle")]
    pub fn toggle(&mut self, file: &Path, line: u32) -> bool {
        if line == 0 {
            return false;
        }
        let file = &file_key(file);
        let lines = self.by_file.entry(file.clone()).or_default();
        let now_set = if lines.remove(&line) {
            if let Some(messages) = self.log_messages.get_mut(file) {
                messages.remove(&line);
            }
            false
        } else {
            lines.insert(line);
            true
        };
        if lines.is_empty() {
            self.by_file.remove(file);
        }
        now_set
    }

    #[note("breakpoints-breakpoints-set-logpoint")]
    pub fn set_logpoint(&mut self, file: &Path, line: u32, message: &str) {
        if line == 0 {
            return;
        }
        let file = file_key(file);
        self.by_file.entry(file.clone()).or_default().insert(line);
        let messages = self.log_messages.entry(file.clone()).or_default();
        if message.trim().is_empty() {
            messages.remove(&line);
        } else {
            messages.insert(line, message.to_string());
        }
        if messages.is_empty() {
            self.log_messages.remove(&file);
        }
    }

    #[note("breakpoints-breakpoints-log-message")]
    pub fn log_message(&self, file: &Path, line: u32) -> Option<&str> {
        self.log_messages
            .get(&file_key(file))
            .and_then(|m| m.get(&line))
            .map(String::as_str)
    }

    #[note("breakpoints-breakpoints-sources")]
    pub fn sources(&self, file: &Path) -> Vec<crawrad_dap::SourceBreakpoint> {
        let key = file_key(file);
        let messages = self.log_messages.get(&key);
        self.lines(&key)
            .into_iter()
            .map(|line| crawrad_dap::SourceBreakpoint {
                line: i64::from(line),
                log_message: messages.and_then(|m| m.get(&line)).cloned(),
            })
            .collect()
    }

    #[note("breakpoints-breakpoints-all-sources")]
    pub fn all_sources(&self) -> BTreeMap<PathBuf, Vec<crawrad_dap::SourceBreakpoint>> {
        self.by_file
            .keys()
            .map(|file| (file.clone(), self.sources(file)))
            .collect()
    }

    #[note("breakpoints-breakpoints-files")]
    pub fn files(&self) -> impl Iterator<Item = (&Path, Vec<u32>)> {
        self.by_file
            .iter()
            .map(|(file, lines)| (file.as_path(), lines.iter().copied().collect()))
    }

    #[note("breakpoints-breakpoints-count")]
    pub fn count(&self) -> usize {
        self.by_file.values().map(BTreeSet::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.by_file.is_empty()
    }

    #[note("breakpoints-breakpoints-sync-view")]
    pub fn sync_view(
        &mut self,
        file: &Path,
        view_owner: Option<&Path>,
        view_lines: &[usize],
    ) -> ViewSync {
        let key = file_key(file);
        let here: Vec<usize> = self.lines(&key).iter().map(|&l| l as usize - 1).collect();
        let stale = self.stale_in_view.remove(&key);
        if stale || view_owner.map(file_key).as_deref() != Some(key.as_path()) {
            return ViewSync::Push(here);
        }
        if here == view_lines {
            return ViewSync::Same;
        }
        self.set_lines(&key, view_lines.iter().map(|&l| l as u32 + 1));
        ViewSync::Pulled
    }
}

#[note("breakpoints-viewsync")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewSync {
    /// The view shows another file: give it these (zero-based) lines.
    Push(Vec<usize>),
    /// The view had changed, and the session now has its lines.
    Pulled,
    /// Nothing to do.
    Same,
}

#[cfg(test)]
#[path = "test/breakpoints/test.rs"]
mod tests;
