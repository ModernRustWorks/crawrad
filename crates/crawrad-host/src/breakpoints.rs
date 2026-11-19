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
mod tests {
    use super::*;

    #[test]
    fn toggling_adds_and_removes_and_an_empty_file_leaves_no_entry() {
        let mut bp = Breakpoints::default();
        let file = Path::new("src/form_main.rs");
        assert!(bp.toggle(file, 12));
        assert!(bp.toggle(file, 3));
        assert_eq!(bp.lines(file), vec![3, 12]);
        assert_eq!(bp.count(), 2);
        assert!(!bp.toggle(file, 12));
        assert!(!bp.toggle(file, 3));
        assert!(bp.is_empty());
        assert_eq!(bp.files().count(), 0);
    }

    #[test]
    fn set_lines_replaces_one_file_and_leaves_the_others() {
        let mut bp = Breakpoints::default();
        let a = Path::new("a.rs");
        let b = Path::new("b.rs");
        bp.set_lines(a, [5, 1]);
        bp.set_lines(b, [7]);
        bp.set_lines(a, [2, 0, 2]);
        assert_eq!(bp.lines(a), vec![2]);
        assert_eq!(bp.lines(b), vec![7]);
        bp.set_lines(a, []);
        assert_eq!(
            bp.files().map(|(f, _)| f.to_path_buf()).collect::<Vec<_>>(),
            vec![PathBuf::from("b.rs")]
        );
        // There is no line zero.
        assert!(!bp.toggle(a, 0));
        assert!(bp.lines(a).is_empty());
    }

    #[test]
    fn breakpoints_set_from_outside_are_pushed_to_the_view_not_overwritten() {
        let mut bp = Breakpoints::default();
        let form = Path::new("src/form_main.rs");
        bp.set_lines(form, [3]);
        assert_eq!(bp.sync_view(form, Some(form), &[2]), ViewSync::Same);
        // The MCP adds line 12 while the view still shows only line 3.
        bp.set_lines_from_outside(form, [3, 12]);
        assert_eq!(
            bp.sync_view(form, Some(form), &[2]),
            ViewSync::Push(vec![2, 11])
        );
        assert_eq!(bp.lines(form), vec![3, 12]);
        // Once pushed, the view is the editor's again.
        assert_eq!(bp.sync_view(form, Some(form), &[2, 11]), ViewSync::Same);
    }

    #[test]
    fn one_file_spelled_two_ways_is_one_file() {
        let dir = std::env::temp_dir().join(format!("crawrad-bp-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("src")).unwrap();
        let file = dir.join("src").join("main.rs");
        std::fs::write(
            &file,
            "fn main() {}
",
        )
        .unwrap();
        let other_spelling = dir.join("src").join("..").join("src").join("main.rs");
        let mut bp = Breakpoints::default();
        bp.toggle(&file, 1);
        assert_eq!(bp.lines(&other_spelling), vec![1]);
        let key = file_key(&file);
        assert!(!key.to_string_lossy().starts_with(r"\\?\"), "{key:?}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_view_of_another_file_is_given_this_file_s_lines_zero_based() {
        let mut bp = Breakpoints::default();
        let form = Path::new("src/form_main.rs");
        bp.set_lines(form, [12, 3]);
        assert_eq!(bp.sync_view(form, None, &[]), ViewSync::Push(vec![2, 11]));
        assert_eq!(
            bp.sync_view(form, Some(Path::new("src/main.rs")), &[0]),
            ViewSync::Push(vec![2, 11])
        );
        // Pushing does not touch the session.
        assert_eq!(bp.lines(form), vec![3, 12]);
    }

    #[test]
    fn a_view_that_was_clicked_or_edited_wins_and_one_that_agrees_changes_nothing() {
        let mut bp = Breakpoints::default();
        let form = Path::new("src/form_main.rs");
        bp.set_lines(form, [3, 12]);
        assert_eq!(bp.sync_view(form, Some(form), &[2, 11]), ViewSync::Same);
        // An Enter above line 12 moved it to 13; a click added line 20.
        assert_eq!(
            bp.sync_view(form, Some(form), &[2, 12, 19]),
            ViewSync::Pulled
        );
        assert_eq!(bp.lines(form), vec![3, 13, 20]);
        // Every dot clicked away empties the file.
        assert_eq!(bp.sync_view(form, Some(form), &[]), ViewSync::Pulled);
        assert!(bp.is_empty());
    }

    #[test]
    fn a_logpoint_is_a_breakpoint_line_with_a_message() {
        let mut bp = Breakpoints::default();
        let f = Path::new("src/form_main.rs");
        bp.set_lines(f, [3]);
        bp.set_logpoint(f, 12, "total = {total}");
        assert_eq!(bp.lines(f), vec![3, 12]);
        assert_eq!(bp.log_message(f, 12), Some("total = {total}"));
        assert_eq!(bp.log_message(f, 3), None);
        let sources = bp.sources(f);
        assert_eq!(sources[0].log_message, None);
        assert_eq!(sources[1].log_message.as_deref(), Some("total = {total}"));
        // An empty message makes it a plain breakpoint again.
        bp.set_logpoint(f, 12, "  ");
        assert_eq!(bp.log_message(f, 12), None);
        assert_eq!(bp.lines(f), vec![3, 12]);
    }

    #[test]
    fn the_message_follows_its_line_and_goes_with_it() {
        let mut bp = Breakpoints::default();
        let f = Path::new("src/form_main.rs");
        bp.set_lines(f, [3]);
        bp.set_logpoint(f, 12, "x = {x}");
        // An Enter above both moves both down by one.
        bp.set_lines(f, [4, 13]);
        assert_eq!(bp.log_message(f, 13), Some("x = {x}"));
        assert_eq!(bp.log_message(f, 12), None);
        // A click on the dot removes the line and its message.
        assert!(!bp.toggle(f, 13));
        assert_eq!(bp.log_message(f, 13), None);
        assert!(!bp.toggle(f, 4));
        assert!(bp.is_empty());
    }
}
