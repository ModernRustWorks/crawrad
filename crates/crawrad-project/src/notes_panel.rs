//! The Notes panel model.
//!
//! Provides the headless model and indexing logic for the project notes panel,
//! listing documentation notes, internal notes, and line comments grouped by file
//! and ordered by source position.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::doc_note::{doc_target_at, item_facts_for_target};
use crate::sidecar_notes::{NoteKind, SidecarNote, extract_anchors, list_notes};
use crawrad_note::note;

#[note("notes-panel-badgetone")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    /// A condition that requires attention (stale signature, doc problem, orphan doc).
    Warn,
    /// A neutral or completed status (resolved comment, unanchored internal note).
    Done,
}

#[note("notes-panel-indexednote")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedNote {
    /// The unique identifier of the note.
    pub id: String,
    /// Note kind: documentation, internal note, or comment.
    pub kind: NoteKind,
    /// Note title; for a Doc note with an empty title, the item name from `doc_target_at`.
    pub title: String,
    /// Anchor file relative to root, forward slashes; None when unanchored.
    pub file: Option<String>,
    /// 1-based anchor line; None when unanchored.
    pub line: Option<usize>,
    /// Whether the note is marked as resolved (comments only).
    pub resolved: bool,
    /// Short alert reason; None when not in alert.
    pub alert: Option<String>,
}

#[note("notes-panel-kindfilter")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KindFilter {
    /// All notes regardless of kind.
    #[default]
    All,
    /// Documentation notes only.
    Doc,
    /// Internal notes only.
    Internal,
    /// Comment notes only.
    Comment,
}

#[note("notes-panel-counts")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    /// Total number of notes.
    pub all: usize,
    /// Number of documentation notes.
    pub doc: usize,
    /// Number of internal notes.
    pub internal: usize,
    /// Number of comment notes.
    pub comment: usize,
    /// Number of notes currently in alert.
    pub alert: usize,
}

#[note("notes-panel-panelrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelRow {
    /// A header row grouping notes by file (or the unanchored group).
    Group {
        /// Group identifier (relative file path or empty string for unanchored).
        key: String,
        /// Display title of the group.
        title: String,
        /// Number of notes in the group matching current filters.
        count: usize,
        /// Whether the group is currently collapsed.
        collapsed: bool,
    },
    /// A note entry inside a group.
    Note {
        /// Note identifier.
        id: String,
        /// Kind of note.
        kind: NoteKind,
        /// Display title of the note.
        title: String,
        /// Optional badge (label text and visual tone).
        badge: Option<(String, BadgeTone)>,
        /// 1-based line string (e.g. "12") or "—" when unanchored.
        line_text: String,
        /// Whether the row is rendered faded (resolved comment).
        faded: bool,
        /// Whether the title is rendered struck through (resolved comment).
        struck: bool,
        /// Anchor file path relative to root, if anchored.
        file: Option<String>,
        /// 1-based anchor line number, if anchored.
        line: Option<usize>,
    },
}

#[note("notes-panel-viewrowparts")]
pub type ViewRowParts = (bool, u8, String, String, bool, String, bool, bool, bool);

#[note("notes-panel-viewheadparts")]
pub type ViewHeadParts = ([String; 4], usize, Vec<String>, usize, String, bool, String);

#[note("notes-panel-rowtarget")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowTarget {
    /// A group header: its key, to `toggle_group`.
    Group(String),
    /// A note: its id, and its anchor file and 1-based line when anchored.
    Note {
        /// The note identifier.
        id: String,
        /// The anchor file relative to root, when anchored.
        file: Option<String>,
        /// The 1-based anchor line, when anchored.
        line: Option<usize>,
    },
}

#[note("notes-panel-notespanel")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotesPanel {
    /// The active kind filter.
    pub kind: KindFilter,
    /// The active file filter (`None` means all files).
    pub file: Option<String>,
    /// Whether only notes in alert should be shown.
    pub in_alert: bool,
    notes: Vec<IndexedNote>,
    collapsed: BTreeSet<String>,
}

impl NotesPanel {
    #[note("notes-panel-notespanel-new")]
    pub fn new(notes: Vec<IndexedNote>) -> Self {
        Self {
            kind: KindFilter::All,
            file: None,
            in_alert: false,
            notes,
            collapsed: BTreeSet::new(),
        }
    }

    #[note("notes-panel-notespanel-set-notes")]
    pub fn set_notes(&mut self, notes: Vec<IndexedNote>) {
        if let Some(ref selected_file) = self.file {
            let still_exists = notes
                .iter()
                .any(|n| n.file.as_deref() == Some(selected_file.as_str()));
            if !still_exists {
                self.file = None;
            }
        }
        self.notes = notes;
    }

    #[note("notes-panel-notespanel-counts")]
    pub fn counts(&self) -> Counts {
        let mut counts = Counts {
            all: self.notes.len(),
            ..Counts::default()
        };
        for note in &self.notes {
            match note.kind {
                NoteKind::Doc => counts.doc += 1,
                NoteKind::Internal => counts.internal += 1,
                NoteKind::Comment => counts.comment += 1,
            }
            if note.alert.is_some() {
                counts.alert += 1;
            }
        }
        counts
    }

    #[note("notes-panel-notespanel-files")]
    pub fn files(&self) -> Vec<String> {
        let mut set = BTreeSet::new();
        for note in &self.notes {
            if let Some(ref file) = note.file {
                set.insert(file.clone());
            }
        }
        set.into_iter().collect()
    }

    #[note("notes-panel-notespanel-toggle-group")]
    pub fn toggle_group(&mut self, key: &str) {
        if self.collapsed.contains(key) {
            self.collapsed.remove(key);
        } else {
            self.collapsed.insert(key.to_string());
        }
    }

    #[note("notes-panel-notespanel-rows")]
    pub fn rows(&self) -> Vec<PanelRow> {
        let filtered: Vec<&IndexedNote> = self
            .notes
            .iter()
            .filter(|note| {
                let kind_matches = match self.kind {
                    KindFilter::All => true,
                    KindFilter::Doc => note.kind == NoteKind::Doc,
                    KindFilter::Internal => note.kind == NoteKind::Internal,
                    KindFilter::Comment => note.kind == NoteKind::Comment,
                };
                // An unanchored note only shows with "All files".
                let file_matches = self.file.is_none() || note.file == self.file;
                kind_matches && file_matches && (!self.in_alert || note.alert.is_some())
            })
            .collect();

        let mut anchored_groups: BTreeMap<String, Vec<&IndexedNote>> = BTreeMap::new();
        let mut unanchored: Vec<&IndexedNote> = Vec::new();

        for note in filtered {
            if let Some(ref file) = note.file {
                anchored_groups.entry(file.clone()).or_default().push(note);
            } else {
                unanchored.push(note);
            }
        }

        let mut rows = Vec::new();

        for (file, mut notes) in anchored_groups {
            notes.sort_by(|a, b| (a.line, &a.id).cmp(&(b.line, &b.id)));
            let count = notes.len();
            let key = file.clone();
            let collapsed = self.collapsed.contains(&key);

            rows.push(PanelRow::Group {
                key,
                title: file,
                count,
                collapsed,
            });

            if !collapsed {
                for note in notes {
                    rows.push(Self::note_row(note));
                }
            }
        }

        if !unanchored.is_empty() {
            unanchored.sort_by(|a, b| (a.line, &a.id).cmp(&(b.line, &b.id)));
            let count = unanchored.len();
            let key = String::new();
            let collapsed = self.collapsed.contains(&key);

            rows.push(PanelRow::Group {
                key,
                title: "Without anchor".to_string(),
                count,
                collapsed,
            });

            if !collapsed {
                for note in unanchored {
                    rows.push(Self::note_row(note));
                }
            }
        }

        rows
    }

    #[note("notes-panel-notespanel-footer")]
    pub fn footer(&self) -> String {
        let n = self.notes.len();
        if n == 1 {
            ".crawrad/notes \u{00B7} 1 file".to_string()
        } else {
            format!(".crawrad/notes \u{00B7} {n} files")
        }
    }

    #[note("notes-panel-notespanel-chip-label")]
    pub fn chip_label(&self, kind: KindFilter) -> String {
        let counts = self.counts();
        match kind {
            KindFilter::All => format!("All {}", counts.all),
            KindFilter::Doc => format!("Documentation {}", counts.doc),
            KindFilter::Internal => format!("Notes {}", counts.internal),
            KindFilter::Comment => format!("Comments {}", counts.comment),
        }
    }

    #[note("notes-panel-notespanel-alert-label")]
    pub fn alert_label(&self) -> String {
        format!("In alert {}", self.counts().alert)
    }

    #[note("notes-panel-notespanel-file-label")]
    pub fn file_label(&self) -> String {
        match &self.file {
            Some(file) => file.clone(),
            None => "All files".to_string(),
        }
    }

    fn note_row(note: &IndexedNote) -> PanelRow {
        let badge = if let Some(ref reason) = note.alert {
            Some((reason.clone(), BadgeTone::Warn))
        } else if note.kind == NoteKind::Comment && note.resolved {
            Some(("Resolved".to_string(), BadgeTone::Done))
        } else if note.file.is_none() && note.kind != NoteKind::Doc {
            Some(("No anchor".to_string(), BadgeTone::Done))
        } else {
            None
        };

        let line_text = match note.line {
            Some(l) => l.to_string(),
            None => "\u{2014}".to_string(),
        };

        let is_resolved_comment = note.kind == NoteKind::Comment && note.resolved;

        PanelRow::Note {
            id: note.id.clone(),
            kind: note.kind,
            title: note.title.clone(),
            badge,
            line_text,
            faded: is_resolved_comment,
            struck: is_resolved_comment,
            file: note.file.clone(),
            line: note.line,
        }
    }

    #[note("notes-panel-notespanel-view-rows")]
    pub fn view_rows(&self) -> Vec<ViewRowParts> {
        self.rows()
            .into_iter()
            .map(|row| match row {
                PanelRow::Group {
                    title,
                    count,
                    collapsed,
                    ..
                } => (
                    true,
                    0,
                    title,
                    String::new(),
                    false,
                    count.to_string(),
                    false,
                    false,
                    collapsed,
                ),
                PanelRow::Note {
                    kind,
                    title,
                    badge,
                    line_text,
                    faded,
                    struck,
                    ..
                } => {
                    let kind_num = match kind {
                        NoteKind::Doc => 0,
                        NoteKind::Internal => 1,
                        NoteKind::Comment => 2,
                    };
                    let (badge_text, badge_warn) = match badge {
                        Some((text, tone)) => (text, tone == BadgeTone::Warn),
                        None => (String::new(), false),
                    };
                    (
                        false, kind_num, title, badge_text, badge_warn, line_text, faded, struck,
                        false,
                    )
                }
            })
            .collect()
    }

    #[note("notes-panel-notespanel-view-head")]
    pub fn view_head(&self) -> ViewHeadParts {
        let chip_labels = [
            self.chip_label(KindFilter::All),
            self.chip_label(KindFilter::Doc),
            self.chip_label(KindFilter::Internal),
            self.chip_label(KindFilter::Comment),
        ];
        let active_chip = match self.kind {
            KindFilter::All => 0,
            KindFilter::Doc => 1,
            KindFilter::Internal => 2,
            KindFilter::Comment => 3,
        };
        let mut file_choices = vec!["All files".to_string()];
        file_choices.extend(self.files());
        let selected_file = match &self.file {
            None => 0,
            Some(f) => file_choices.iter().position(|c| c == f).unwrap_or(0),
        };
        (
            chip_labels,
            active_chip,
            file_choices,
            selected_file,
            self.alert_label(),
            self.in_alert,
            self.footer(),
        )
    }

    #[note("notes-panel-notespanel-pick-chip")]
    pub fn pick_chip(&mut self, index: usize) {
        self.kind = match index {
            1 => KindFilter::Doc,
            2 => KindFilter::Internal,
            3 => KindFilter::Comment,
            _ => KindFilter::All,
        };
    }

    #[note("notes-panel-notespanel-pick-file")]
    pub fn pick_file(&mut self, choice: usize) {
        if choice == 0 {
            self.file = None;
        } else {
            let files = self.files();
            self.file = files.get(choice - 1).cloned();
        }
    }

    #[note("notes-panel-notespanel-toggle-alert")]
    pub fn toggle_alert(&mut self) {
        self.in_alert = !self.in_alert;
    }

    #[note("notes-panel-notespanel-row-target")]
    pub fn row_target(&self, index: usize) -> Option<RowTarget> {
        match self.rows().get(index) {
            Some(PanelRow::Group { key, .. }) => Some(RowTarget::Group(key.clone())),
            Some(PanelRow::Note { id, file, line, .. }) => Some(RowTarget::Note {
                id: id.clone(),
                file: file.clone(),
                line: *line,
            }),
            None => None,
        }
    }

    #[note("notes-panel-notespanel-toggle-row")]
    pub fn toggle_row(&mut self, index: usize) {
        if let Some(RowTarget::Group(key)) = self.row_target(index) {
            self.toggle_group(&key);
        }
    }

    #[note("notes-panel-notespanel-note-place")]
    pub fn note_place(&self, index: usize) -> Option<(String, usize)> {
        match self.row_target(index)? {
            RowTarget::Note {
                file: Some(file),
                line,
                ..
            } => Some((file, line.unwrap_or(1).saturating_sub(1))),
            _ => None,
        }
    }
}

impl Default for NotesPanel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[note("notes-panel-alert-reason")]
pub fn alert_reason(
    note: &SidecarNote,
    source: Option<&str>,
    line: Option<usize>,
) -> Option<String> {
    if note.kind != NoteKind::Doc {
        return None;
    }
    let found = source
        .zip(line)
        .and_then(|(source, line)| Some((source, doc_target_at(source, line.checked_sub(1)?)?)));
    let Some((source, target)) = found else {
        return Some("Orphan".to_string());
    };
    if crawrad_note_format::stale_signature(&note.id, &note.signature, &target.signature).is_some()
    {
        return Some("Signature changed".to_string());
    }
    let facts = item_facts_for_target(source, &target);
    let problems = crawrad_note_format::rustdoc::doc_problems(&note.content, &facts);
    let first = problems.first()?;
    // "missing `# Errors`: why..." reads "Missing # Errors" on a badge.
    let section = first
        .strip_prefix("missing `")
        .and_then(|rest| rest.split('`').next());
    Some(match section {
        Some(section) => format!("Missing {section}"),
        None => "Out of rustdoc pattern".to_string(),
    })
}

#[note("notes-panel-index-project")]
pub fn index_project(root: &Path) -> Vec<IndexedNote> {
    index_project_live(root, |_| None)
}

/// As [`index_project`], reading a source from `live` first: an editor with
/// unsaved edits already holds the anchor the note dialog just wrote. The notes
/// come from the workspace's `.crawrad/notes` AND each crate's, where a
/// documentation note lives (`cargo doc` reads it there).
pub fn index_project_live(root: &Path, live: impl Fn(&Path) -> Option<String>) -> Vec<IndexedNote> {
    let mut dirs = vec![root.to_path_buf()];
    collect_crate_dirs(root, &mut dirs);
    let mut notes: BTreeMap<String, SidecarNote> = dirs
        .iter()
        .flat_map(|d| list_notes(d))
        .map(|n| (n.id.clone(), n))
        .collect();
    let mut files = Vec::new();
    collect_rs_files(root, &mut files);
    files.sort();
    let mut indexed = Vec::new();
    for path in files {
        let Some(source) = live(&path).or_else(|| std::fs::read_to_string(&path).ok()) else {
            continue;
        };
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let mut anchors: Vec<(usize, String)> = extract_anchors(&source)
            .into_iter()
            .map(|m| (m.line_number, m.note_id))
            .collect();
        anchors.extend(doc_attribute_anchors(&source));
        anchors.sort();
        for (line, id) in anchors {
            // A note already placed keeps its first anchor.
            if let Some(note) = notes.remove(&id) {
                indexed.push(indexed_note(note, Some((&rel, &source, line))));
            }
        }
    }
    indexed.extend(notes.into_values().map(|n| indexed_note(n, None)));
    indexed
}

#[note("notes-panel-doc-attribute-anchors")]
fn doc_attribute_anchors(source: &str) -> Vec<(usize, String)> {
    source
        .lines()
        .enumerate()
        .filter(|(_, l)| l.trim_start().starts_with("#[") && l.contains("note(\""))
        .filter_map(|(i, _)| Some((i + 1, doc_target_at(source, i)?.existing_note?)))
        .collect()
}

#[note("notes-panel-indexed-note")]
fn indexed_note(note: SidecarNote, anchor: Option<(&str, &str, usize)>) -> IndexedNote {
    let target = anchor.and_then(|(_, source, line)| doc_target_at(source, line.checked_sub(1)?));
    let title = match target {
        Some(t) if note.kind == NoteKind::Doc && note.title.trim().is_empty() => t.name,
        _ => note.title.clone(),
    };
    let alert = alert_reason(&note, anchor.map(|a| a.1), anchor.map(|a| a.2));
    IndexedNote {
        id: note.id,
        kind: note.kind,
        title,
        file: anchor.map(|a| a.0.to_string()),
        line: anchor.map(|a| a.2),
        resolved: note.resolved,
        alert,
    }
}

/// Every crate folder (one with a `Cargo.toml`) under `dir`, but `dir` itself.
fn collect_crate_dirs(dir: &Path, dirs: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for path in entries.filter_map(Result::ok).map(|e| e.path()) {
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
        let skip = name.is_none_or(|n| n == "target" || n == "node_modules" || n.starts_with('.'));
        if path.is_dir() && !skip {
            if path.join("Cargo.toml").is_file() {
                dirs.push(path.clone());
            }
            collect_crate_dirs(&path, dirs);
        }
    }
}

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut sorted_entries: Vec<_> = entries.filter_map(Result::ok).collect();
    sorted_entries.sort_by_key(|e| e.file_name());

    for entry in sorted_entries {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        if path.is_dir() {
            if name == "target"
                || name == ".git"
                || name == ".crawrad"
                || name == "node_modules"
                || name.starts_with('.')
            {
                continue;
            }
            collect_rs_files(&path, files);
        } else if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }
}

#[cfg(test)]
#[path = "test/notes_panel/test.rs"]
mod test;
