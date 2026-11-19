//! The headless model behind the Note dialog.
//!
//! Handles note loading/saving, state management, documentation suggestion
//! generation, section templates insertion, and editor buffer source updates.

use std::path::{Path, PathBuf};

use crate::doc_note::{
    DocTarget, Section, Suggestion, apply_note_attribute, crate_dir_of, crate_name_of,
    depends_on_crawrad_note, doc_skeleton, doc_target_at, insert_section, missing_sections,
    suggested_note_id, suggested_sections, summary_warning, with_crawrad_note_dependency,
};
use crate::sidecar_notes::{self, NoteKind, SidecarNote};
use crawrad_note::note;

#[note("note-dialog-notedialogmodel")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteDialogModel {
    pub note: SidecarNote,
    pub title: String,
    pub tags: String,
    pub content: String,
    pub target: Option<DocTarget>,
    pub file: String,
    pub notes_root: PathBuf,
    pub crate_dir: Option<PathBuf>,
    pub status: Option<String>,
    /// The Preview tab is showing instead of the editor.
    pub preview_tab: bool,
    /// The last Test example answer: its text and whether it passed (`None`
    /// while the run is going).
    pub test_line: Option<(String, Option<bool>)>,
}

#[note("note-dialog-tone")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Plain,
    Muted,
    Warn,
    Ok,
    Failed,
}

impl Tone {
    #[note("note-dialog-tone-rgba")]
    pub fn rgba(self) -> u32 {
        match self {
            Tone::Plain => 0xE7E7EAFF,
            Tone::Muted => 0x98989BFF,
            Tone::Warn => 0xD9A441FF,
            Tone::Ok => 0x3FB950FF,
            Tone::Failed => 0xE2726BFF,
        }
    }
}

#[note("note-dialog-popuprow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopupRow {
    pub kind: NoteKind,
    pub title: String,
    pub description: String,
    pub shortcut: &'static str,
    pub enabled: bool,
}

#[note("note-dialog-notepopup")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotePopup {
    pub header: String,
    pub rows: [PopupRow; 3],
}

#[note("note-dialog-note-popup")]
pub fn note_popup(source: &str, line: usize) -> NotePopup {
    let target = doc_target_at(source, line);
    let header = match &target {
        Some(t) => format!("Line {} \u{b7} {}", line + 1, t.signature),
        None => format!("Line {}", line + 1),
    };
    let line_text = source.lines().nth(line).unwrap_or("");
    let (doc_title, doc_desc, doc_enabled) = match &target {
        Some(t) => {
            let is_edit = t.existing_note.is_some()
                || line_text.contains("#[note(")
                || line_text.contains("#[crawrad_note::note(");
            let title = if is_edit {
                "Edit documentation".to_string()
            } else {
                "Add documentation".to_string()
            };
            (
                title,
                "Public docs of this item, in the rustdoc format. Shows in cargo doc and docs.rs."
                    .to_string(),
                true,
            )
        }
        None => (
            "Add documentation".to_string(),
            "Put the caret on a fn, struct, enum, trait or const to document it.".to_string(),
            false,
        ),
    };
    let has_note = line_text.contains("[crawrad:note");
    let note_title = if has_note {
        "Edit note".to_string()
    } else {
        "Add note".to_string()
    };
    let note_desc =
        "Private context for developers: why, history, decisions. Never becomes docs.".to_string();

    let has_comment = line_text.contains("[crawrad:comment#");
    let comment_title = if has_comment {
        "Edit comment".to_string()
    } else {
        "Add comment".to_string()
    };
    let comment_desc =
        "A short remark on this line, like a review comment. Open or resolved.".to_string();

    let rows = [
        PopupRow {
            kind: NoteKind::Doc,
            title: doc_title,
            description: doc_desc,
            shortcut: "Ctrl+Alt+D",
            enabled: doc_enabled,
        },
        PopupRow {
            kind: NoteKind::Internal,
            title: note_title,
            description: note_desc,
            shortcut: "Ctrl+Alt+N",
            enabled: true,
        },
        PopupRow {
            kind: NoteKind::Comment,
            title: comment_title,
            description: comment_desc,
            shortcut: "Ctrl+Alt+M",
            enabled: true,
        },
    ];
    NotePopup { header, rows }
}

#[note("note-dialog-popuprowparts")]
pub type PopupRowParts = (u8, String, String, &'static str, bool);

#[note("note-dialog-item-title")]
fn item_title(target: &DocTarget) -> String {
    match &target.owner {
        Some(owner) if !owner.trim().is_empty() => format!("{owner}::{}", target.name),
        _ => target.name.clone(),
    }
}

#[note("note-dialog-noteopen")]
pub enum NoteOpen {
    /// No kind chosen yet: the popup's header and rows, for the code view.
    Popup(String, Vec<PopupRowParts>),
    /// The chosen kind's dialog.
    Dialog(Box<CursorOpen>),
}

impl NoteDialogModel {
    #[note("note-dialog-notedialogmodel-at-caret")]
    pub fn at_caret(
        source: &str,
        line: usize,
        file: &str,
        project_root: &Path,
        choice: Option<crawrad_actions::NoteKindChoice>,
    ) -> NoteOpen {
        let kind = match choice {
            Some(crawrad_actions::NoteKindChoice::Doc) if doc_target_at(source, line).is_some() => {
                NoteKind::Doc
            }
            Some(crawrad_actions::NoteKindChoice::Note) => NoteKind::Internal,
            Some(crawrad_actions::NoteKindChoice::Comment) => NoteKind::Comment,
            // No choice yet, or documentation where nothing can be documented:
            // the popup shows the rows and why `Add documentation` is off.
            _ => {
                let (header, rows) = note_popup_tuples(source, line);
                return NoteOpen::Popup(header, rows);
            }
        };
        NoteOpen::Dialog(Box::new(Self::from_caret_as(
            source,
            line,
            file,
            project_root,
            kind,
        )))
    }
}

#[note("note-dialog-note-popup-tuples")]
pub fn note_popup_tuples(source: &str, line: usize) -> (String, Vec<PopupRowParts>) {
    let p = note_popup(source, line);
    let rows = p
        .rows
        .into_iter()
        .map(|r| {
            let k = match r.kind {
                NoteKind::Doc => 0,
                NoteKind::Internal => 1,
                NoteKind::Comment => 2,
            };
            (k, r.title, r.description, r.shortcut, r.enabled)
        })
        .collect();
    (p.header, rows)
}

#[note("note-dialog-noteview")]
#[derive(Debug, Clone, PartialEq)]
pub struct NoteView {
    pub kind: NoteKind,
    pub title_text: &'static str,
    pub subtitle: String,
    pub badge: Option<(&'static str, Tone)>,
    pub location: String,
    pub code_line: String,
    pub byline: String,
    pub show_title_tags: bool,
    pub show_tabs: bool,
    pub tab_kind_hint: &'static str,
    pub resolve_label: Option<&'static str>,
    pub header: String,
    pub meta: String,
    /// Documentation note: the item line, the section row and the check line show.
    pub is_doc: bool,
    pub item: String,
    pub suggestion: String,
    /// The section buttons the signature asks for (tinted).
    pub suggested: Vec<Section>,
    /// The warning, or the check when nothing is missing.
    pub check: (String, Tone),
    /// `Some(markdown)` on the Preview tab.
    pub preview: Option<String>,
    pub tab_hint: String,
    /// Test example shows only when there is an example to run.
    pub can_test: bool,
    pub test_button: &'static str,
    pub test_line: (String, Tone),
    pub footer: String,
}

#[note("note-dialog-labelpaint")]
pub type LabelPaint<'a> = (&'static str, &'a str, bool, Option<Tone>);

impl NoteView {
    #[note("note-dialog-noteview-icon")]
    pub fn icon(&self) -> i64 {
        match self.kind {
            NoteKind::Doc => 0,
            NoteKind::Internal => 1,
            NoteKind::Comment => 2,
        }
    }

    #[note("note-dialog-noteview-labels")]
    pub fn labels(&self) -> Vec<LabelPaint<'_>> {
        let comment = self.kind == NoteKind::Comment;
        let (badge, badge_tone) = self.badge.unwrap_or(("", Tone::Muted));
        // The line above the text: the item documented, or where the note is.
        let above = if self.is_doc {
            &self.item
        } else {
            &self.location
        };
        // The line below it: the doc's check, or who wrote the comment.
        let (below, below_tone) = match comment {
            true => (&self.byline, Tone::Muted),
            false => (&self.check.0, self.check.1),
        };
        let subtitle = !self.subtitle.is_empty();
        let below_shown = self.kind != NoteKind::Internal;
        let (test, test_tone) = (&self.test_line.0, self.test_line.1);
        vec![
            ("note_title", self.title_text, true, None),
            ("note_subtitle", &self.subtitle, subtitle, None),
            ("note_badge", badge, self.badge.is_some(), Some(badge_tone)),
            ("note_location_right", &self.location, comment, None),
            ("note_item", above, !comment, None),
            ("note_suggest", &self.suggestion, self.is_doc, None),
            ("note_code", &self.code_line, comment, None),
            ("note_tab_hint", self.tab_kind_hint, self.show_tabs, None),
            ("note_warning", below, below_shown, Some(below_tone)),
            ("note_test_result", test, true, Some(test_tone)),
            ("note_status", &self.footer, true, None),
        ]
    }

    #[note("note-dialog-noteview-buttons")]
    pub fn buttons(&self) -> [(&'static str, Option<&str>); 5] {
        [
            ("note_tab_preview", Some("Preview")),
            ("note_test", self.can_test.then_some(self.test_button)),
            ("note_resolve", self.resolve_label),
            ("note_cancel", Some("Close")),
            ("note_save", Some("Save")),
        ]
    }

    #[note("note-dialog-noteview-boxes")]
    pub fn boxes(&self) -> [(&'static str, bool); 6] {
        [
            ("note_item_row", self.kind != NoteKind::Comment),
            ("note_warning_row", self.kind != NoteKind::Internal),
            ("note_code_box", self.kind == NoteKind::Comment),
            ("note_insert_row", self.is_doc),
            ("note_id_row", self.show_title_tags),
            ("note_tab_row", self.show_tabs),
        ]
    }

    #[note("note-dialog-noteview-section-tone")]
    pub fn section_tone(&self, section: &Section) -> Tone {
        if self.suggested.contains(section) {
            Tone::Warn
        } else {
            Tone::Plain
        }
    }
}

#[note("note-dialog-cursoropen")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorOpen {
    pub model: NoteDialogModel,
    pub insert_anchor: Option<(usize, String)>,
}

impl NoteDialogModel {
    #[note("note-dialog-notedialogmodel-from-caret")]
    pub fn from_caret(source: &str, line: usize, file: &str, project_root: &Path) -> CursorOpen {
        let kind = if doc_target_at(source, line).is_some() {
            NoteKind::Doc
        } else {
            let lines: Vec<&str> = source.lines().collect();
            let idx = line.min(lines.len().saturating_sub(1));
            let cur = lines.get(idx).copied().unwrap_or("");
            let anchors = sidecar_notes::extract_anchors(cur);
            if let Some(first) = anchors.first() {
                if first.is_comment {
                    NoteKind::Comment
                } else {
                    NoteKind::Internal
                }
            } else {
                NoteKind::Internal
            }
        };
        Self::from_caret_as(source, line, file, project_root, kind)
    }

    #[note("note-dialog-notedialogmodel-from-caret-as")]
    pub fn from_caret_as(
        source: &str,
        line: usize,
        file: &str,
        project_root: &Path,
        kind: NoteKind,
    ) -> CursorOpen {
        let file_path = if Path::new(file).is_absolute() {
            PathBuf::from(file)
        } else {
            project_root.join(file)
        };
        let crate_dir = crate_dir_of(&file_path).or_else(|| crate_dir_of(project_root));
        let crate_name = crate_dir.as_deref().and_then(crate_name_of);

        if kind == NoteKind::Doc {
            let target = doc_target_at(source, line);
            if let Some(target) = target {
                if let Some(ref note_id) = target.existing_note {
                    let loaded = crate_dir
                        .as_ref()
                        .and_then(|cdir| sidecar_notes::load_note(cdir, note_id).ok());
                    let (note, status) = match loaded {
                        Some(mut n) => {
                            n.kind = NoteKind::Doc;
                            (n, None)
                        }
                        None => {
                            let content = if target.existing_docs.trim().is_empty() {
                                doc_skeleton(source, &target, crate_name.as_deref())
                            } else {
                                target.existing_docs.clone()
                            };
                            let n = SidecarNote {
                                id: note_id.clone(),
                                kind: NoteKind::Doc,
                                title: item_title(&target),
                                target_file: file.to_string(),
                                line_hint: line + 1,
                                content,
                                ..Default::default()
                            };
                            (n, Some(format!("New note template ({note_id})")))
                        }
                    };
                    let model =
                        Self::opened(note, Some(target), file, project_root, crate_dir, status);
                    return CursorOpen {
                        model,
                        insert_anchor: None,
                    };
                }

                // Caret on an item without an existing note
                let id = suggested_note_id(&target);
                let content = if target.existing_docs.trim().is_empty() {
                    doc_skeleton(source, &target, crate_name.as_deref())
                } else {
                    target.existing_docs.clone()
                };
                let note = SidecarNote {
                    id,
                    kind: NoteKind::Doc,
                    title: item_title(&target),
                    target_file: file.to_string(),
                    line_hint: line + 1,
                    content,
                    ..Default::default()
                };
                let model = Self::opened(note, Some(target), file, project_root, crate_dir, None);
                return CursorOpen {
                    model,
                    insert_anchor: None,
                };
            }

            // Doc requested but caret not on a documentable item
            let id = sidecar_notes::generate_note_id(file);
            let mut note = SidecarNote::new(id, file, line + 1);
            note.kind = NoteKind::Doc;
            note.title = format!("Documentation for {file}:{}", line + 1);
            let model = Self::opened(note, None, file, project_root, crate_dir, None);
            return CursorOpen {
                model,
                insert_anchor: None,
            };
        }

        // Internal or Comment
        let is_comment = kind == NoteKind::Comment;
        let mut lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }
        let idx = line.min(lines.len() - 1);
        let current_line = &lines[idx];

        let anchors = sidecar_notes::extract_anchors(current_line);
        let existing = anchors.iter().find(|a| a.is_comment == is_comment);
        if let Some(first) = existing {
            let note_id = &first.note_id;
            let (mut note, status) = match sidecar_notes::load_note(project_root, note_id) {
                Ok(n) => (n, None),
                Err(e) => {
                    let fallback_title = if is_comment {
                        format!("Comment: {note_id}")
                    } else {
                        format!("Note: {note_id}")
                    };
                    let n = SidecarNote {
                        id: note_id.clone(),
                        title: fallback_title,
                        target_file: file.to_string(),
                        line_hint: idx + 1,
                        ..Default::default()
                    };
                    (n, Some(format!("New note template ({e})")))
                }
            };
            note.kind = if is_comment {
                NoteKind::Comment
            } else {
                NoteKind::Internal
            };
            let model = Self::opened(note, None, file, project_root, crate_dir, status);
            return CursorOpen {
                model,
                insert_anchor: None,
            };
        }

        // Otherwise -> NEW internal or comment note
        let id = sidecar_notes::generate_note_id(file);
        let space = if current_line.trim().is_empty() {
            ""
        } else {
            " "
        };
        let tag_name = if is_comment { "comment" } else { "note" };
        lines[idx] = format!("{current_line}{space}// [crawrad:{tag_name}#{id}]");
        let line_ending = if source.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let mut new_source = lines.join(line_ending);
        if source.ends_with('\n') {
            new_source.push_str(line_ending);
        }

        let mut note = SidecarNote::new(id, file, idx + 1);
        note.kind = if is_comment {
            NoteKind::Comment
        } else {
            NoteKind::Internal
        };
        // The title names the file as the project sees it, never the
        // absolute path the editor holds.
        let shown = Path::new(file)
            .strip_prefix(project_root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| file.to_string());
        note.title = if is_comment {
            format!("Comment for {shown}:{}", idx + 1)
        } else {
            format!("Architectural note for {shown}:{}", idx + 1)
        };
        let model = Self::opened(note, None, file, project_root, crate_dir, None);
        CursorOpen {
            model,
            insert_anchor: Some((idx, new_source)),
        }
    }

    #[note("note-dialog-notedialogmodel-opened")]
    fn opened(
        note: SidecarNote,
        target: Option<DocTarget>,
        file: &str,
        project_root: &Path,
        crate_dir: Option<PathBuf>,
        status: Option<String>,
    ) -> Self {
        Self {
            title: note.title.clone(),
            tags: note.tags.join(", "),
            content: note.content.clone(),
            note,
            target,
            file: file.to_string(),
            notes_root: project_root.to_path_buf(),
            crate_dir,
            status,
            preview_tab: false,
            test_line: None,
        }
    }

    #[note("note-dialog-notedialogmodel-from-note")]
    pub fn from_note(note: SidecarNote, project_root: &Path) -> Self {
        let file = note.target_file.clone();
        let file_path = if Path::new(&file).is_absolute() {
            PathBuf::from(&file)
        } else {
            project_root.join(&file)
        };
        let crate_dir = crate_dir_of(&file_path).or_else(|| crate_dir_of(project_root));
        let title = note.title.clone();
        let tags = note.tags.join(", ");
        let content = note.content.clone();
        Self {
            note,
            title,
            tags,
            content,
            target: None,
            file,
            notes_root: project_root.to_path_buf(),
            crate_dir,
            status: None,
            preview_tab: false,
            test_line: None,
        }
    }

    #[note("note-dialog-notedialogmodel-from-id")]
    pub fn from_id(id: &str, file: &str, project_root: &Path) -> Self {
        let (note, status) = match sidecar_notes::load_note(project_root, id) {
            Ok(n) => (n, None),
            Err(e) => (
                SidecarNote {
                    id: id.to_string(),
                    title: format!("Note: {id}"),
                    target_file: file.to_string(),
                    ..Default::default()
                },
                Some(format!("New note template ({e})")),
            ),
        };
        let mut model = Self::from_note(note, project_root);
        model.status = status;
        model
    }

    #[note("note-dialog-notedialogmodel-set-kind")]
    pub fn set_kind(&mut self, kind: NoteKind) {
        if kind == NoteKind::Doc && self.target.is_none() {
            self.status =
                Some("Documentation needs the caret on an item (fn, struct, enum…)".to_string());
            return;
        }
        self.note.kind = kind;
    }

    #[note("note-dialog-notedialogmodel-toggle-resolved")]
    pub fn toggle_resolved(&mut self) {
        if self.kind() == NoteKind::Comment {
            self.note.resolved = !self.note.resolved;
        }
    }

    #[note("note-dialog-notedialogmodel-kind")]
    pub fn kind(&self) -> NoteKind {
        self.note.kind
    }

    #[note("note-dialog-notedialogmodel-insert")]
    pub fn insert(&mut self, section: Section) {
        self.content = insert_section(&self.content, section);
    }

    #[note("note-dialog-notedialogmodel-insert-link")]
    pub fn insert_link(&mut self, at: usize, link: &str) {
        let mut at = at.min(self.content.len());
        while !self.content.is_char_boundary(at) {
            at -= 1;
        }
        self.content.insert_str(at, link);
    }

    #[note("note-dialog-notedialogmodel-header")]
    pub fn header(&self) -> String {
        format!("Note — {}", self.note.id)
    }

    #[note("note-dialog-notedialogmodel-item-label")]
    pub fn item_label(&self) -> String {
        if self.kind() == NoteKind::Doc {
            self.target
                .as_ref()
                .map(|t| t.signature.clone())
                .unwrap_or_default()
        } else {
            String::new()
        }
    }

    #[note("note-dialog-notedialogmodel-meta")]
    pub fn meta(&self) -> String {
        match self.kind() {
            NoteKind::Doc => {
                let line = self
                    .target
                    .as_ref()
                    .map_or(self.note.line_hint, |t| t.item_line + 1);
                format!("{}:{line}", self.file)
            }
            NoteKind::Internal | NoteKind::Comment => format!(
                "Target: {}:{}  ·  Author: {}  ·  Created: {}",
                self.file, self.note.line_hint, self.note.author, self.note.created
            ),
        }
    }

    #[note("note-dialog-notedialogmodel-suggestions")]
    pub fn suggestions(&self, source: &str) -> Vec<Suggestion> {
        if self.kind() == NoteKind::Doc
            && let Some(ref target) = self.target
        {
            return suggested_sections(source, target);
        }
        Vec::new()
    }

    #[note("note-dialog-notedialogmodel-suggestion-text")]
    pub fn suggestion_text(&self, source: &str) -> String {
        let suggestions = self.suggestions(source);
        if suggestions.is_empty() {
            return String::new();
        }
        let parts: Vec<String> = suggestions
            .iter()
            .map(|s| {
                let sec_name = match s.section {
                    Section::Summary => "Summary",
                    Section::Examples => "Examples",
                    Section::Errors => "Errors",
                    Section::Panics => "Panics",
                    Section::Safety => "Safety",
                };
                format!("{} → {}", s.reason, sec_name)
            })
            .collect();
        format!("From the signature: {}", parts.join(" · "))
    }

    #[note("note-dialog-notedialogmodel-warning")]
    pub fn warning(&self, source: &str) -> Option<String> {
        if self.kind() != NoteKind::Doc {
            return None;
        }
        if let Some(t) = &self.target {
            let saved = &self.note.signature;
            if crawrad_note_format::stale_signature(&self.note.id, saved, &t.signature).is_some() {
                return Some(format!(
                    "Written for `{saved}`; the signature changed. Review the note, then Save."
                ));
            }
        }
        let suggestions = self.suggestions(source);
        let missing = missing_sections(&self.content, &suggestions);
        if let Some(first) = missing.first() {
            let reason = first.reason.trim_end_matches('.');
            return Some(format!("Missing {}: {reason}.", first.section.label()));
        }
        if let Some(warning) = summary_warning(&self.content) {
            return Some(warning);
        }
        // Whatever else the build warns about (`# Examples` without a rust
        // block...): the dialog must never show green for a note that warns.
        let target = self.target.as_ref()?;
        let facts = crate::doc_note::item_facts_for_target(source, target);
        crawrad_note_format::rustdoc::doc_problems(&self.content, &facts)
            .into_iter()
            .next()
            .map(|problem| sentence(&problem))
    }

    #[note("note-dialog-notedialogmodel-footer")]
    pub fn footer(&self) -> String {
        match self.kind() {
            NoteKind::Doc => format!(
                "Writes #[crawrad_note::note(\"{}\")] above the item",
                self.note.id
            ),
            NoteKind::Internal | NoteKind::Comment => format!(".crawrad/notes/{}.md", self.note.id),
        }
    }

    #[note("note-dialog-notedialogmodel-preview")]
    pub fn preview(&self) -> String {
        crate::note_preview::preview_markdown(&self.item_label(), &self.content)
    }

    #[note("note-dialog-notedialogmodel-check")]
    pub fn check(&self, source: &str) -> Option<String> {
        if self.kind() != NoteKind::Doc || self.warning(source).is_some() {
            return None;
        }
        let summary = self.content.lines().next().unwrap_or("").trim();
        Some(format!(
            "Summary {} chars · every section the signature asks for is present",
            summary.chars().count()
        ))
    }

    #[note("note-dialog-notedialogmodel-doctest-run")]
    pub fn doctest_run(&self) -> Option<crate::note_preview::DoctestRun> {
        if self.kind() != NoteKind::Doc || !crate::note_preview::has_example(&self.content) {
            return None;
        }
        let target = self.target.as_ref()?;
        let cargo = std::fs::read_to_string(self.crate_dir.as_ref()?.join("Cargo.toml")).ok()?;
        let package = crate::note_preview::package_name(&cargo)?;
        Some(crate::note_preview::DoctestRun::new(
            &package,
            target.owner.as_deref(),
            &target.name,
        ))
    }

    #[note("note-dialog-notedialogmodel-view")]
    pub fn view(&self, source: &str, testing: bool) -> NoteView {
        let is_doc = self.kind() == NoteKind::Doc;
        let title_text = match self.kind() {
            NoteKind::Doc => "Documentation",
            NoteKind::Internal => "Note",
            NoteKind::Comment => "Comment",
        };
        let subtitle = match self.kind() {
            NoteKind::Doc => "Public \u{b7} shows in cargo doc and docs.rs".to_string(),
            NoteKind::Internal => "Private \u{b7} stays in the IDE, never becomes docs".to_string(),
            NoteKind::Comment => String::new(),
        };
        let badge = if self.kind() == NoteKind::Comment {
            if self.note.resolved {
                Some(("Resolved", Tone::Muted))
            } else {
                Some(("Open", Tone::Warn))
            }
        } else {
            None
        };
        let file_name = Path::new(&self.file)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.file);
        // The anchor in the code is where the note is; the hint is only kept.
        let tag = format!("#{}]", self.note.id);
        let line_num = source
            .lines()
            .position(|l| l.contains("[crawrad:") && l.contains(&tag))
            .map_or(self.note.line_hint, |i| i + 1);
        let location = match self.kind() {
            NoteKind::Doc => String::new(),
            NoteKind::Comment => format!("{file_name} \u{b7} line {line_num}"),
            NoteKind::Internal => {
                let code_line = anchor_code_line(source, line_num);
                let code_cut = if code_line.chars().count() > 40 {
                    format!("{}…", code_line.chars().take(40).collect::<String>())
                } else {
                    code_line
                };
                if code_cut.is_empty() {
                    format!("{file_name} \u{b7} line {line_num}")
                } else {
                    format!("{file_name} \u{b7} line {line_num} \u{b7} {code_cut}")
                }
            }
        };
        let code_line = if self.kind() == NoteKind::Comment {
            anchor_code_line(source, line_num)
        } else {
            String::new()
        };
        let byline = if self.kind() == NoteKind::Comment {
            format_byline(&self.note.author, &self.note.created)
        } else {
            String::new()
        };
        let show_title_tags = self.kind() == NoteKind::Internal;
        let show_tabs = matches!(self.kind(), NoteKind::Doc | NoteKind::Internal);
        let tab_kind_hint = match self.kind() {
            NoteKind::Doc => "rustdoc Markdown",
            NoteKind::Internal => "Markdown",
            NoteKind::Comment => "",
        };
        let resolve_label = if self.kind() == NoteKind::Comment {
            if self.note.resolved {
                Some("Reopen")
            } else {
                Some("Resolve")
            }
        } else {
            None
        };
        let check = match self.warning(source) {
            Some(w) => (w, Tone::Warn),
            None => (self.check(source).unwrap_or_default(), Tone::Ok),
        };
        let test_line = match &self.test_line {
            Some((text, Some(true))) => (text.clone(), Tone::Ok),
            Some((text, Some(false))) => (text.clone(), Tone::Failed),
            Some((text, None)) => (text.clone(), Tone::Muted),
            None => (String::new(), Tone::Muted),
        };
        NoteView {
            kind: self.kind(),
            title_text,
            subtitle,
            badge,
            location,
            code_line,
            byline,
            show_title_tags,
            show_tabs,
            tab_kind_hint,
            resolve_label,
            header: self.header(),
            meta: self.meta(),
            is_doc,
            item: self.item_label(),
            suggestion: self.suggestion_text(source),
            suggested: self.suggestions(source).iter().map(|s| s.section).collect(),
            check,
            preview: self.preview_tab.then(|| self.preview()),
            tab_hint: if self.preview_tab {
                "As cargo doc will show it".to_string()
            } else {
                format!("Markdown \u{b7} .crawrad/notes/{}.md", self.note.id)
            },
            can_test: self.doctest_run().is_some(),
            test_button: if testing {
                "Testing…"
            } else {
                "Test example"
            },
            test_line,
            footer: self.status.clone().unwrap_or_else(|| self.footer()),
        }
    }

    #[note("note-dialog-notedialogmodel-typed")]
    pub fn typed(&mut self, text: String) {
        if !self.preview_tab {
            self.content = text;
        }
    }

    #[note("note-dialog-notedialogmodel-testing")]
    pub fn testing(&mut self) {
        self.test_line = Some(("Testing the example…".to_string(), None));
    }

    #[note("note-dialog-notedialogmodel-tested")]
    pub fn tested(&mut self, outcome: &crate::note_preview::DoctestOutcome) {
        self.test_line = Some((outcome.label(), Some(outcome.passed())));
    }

    #[note("note-dialog-notedialogmodel-save")]
    pub fn save(&mut self, source: &str) -> Result<Option<String>, String> {
        self.note.title = if self.title.trim().is_empty() {
            "Untitled Note".to_string()
        } else {
            self.title.trim().to_string()
        };
        self.note.tags = self
            .tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        self.note.content = self.content.clone();

        match self.kind() {
            NoteKind::Doc => {
                self.note.kind = NoteKind::Doc;
                // Where the item is now, and its signature for the stale check.
                if let Some(t) = &self.target {
                    (self.note.target_file, self.note.line_hint) =
                        (self.file.clone(), t.item_line + 1);
                    self.note.signature = t.signature.clone();
                }
                let crate_dir = self.crate_dir.as_ref().ok_or_else(|| {
                    "Cannot determine crate directory for documentation note".to_string()
                })?;

                let path = match sidecar_notes::save_note(crate_dir, &self.note) {
                    Ok(p) => p,
                    Err(err) => {
                        self.status = Some(format!("Failed to save: {err}"));
                        return Err(err);
                    }
                };

                // The attribute needs the `crawrad-note` crate: without it the
                // project stops compiling, so Save adds the dependency itself.
                let cargo_toml_path = crate_dir.join("Cargo.toml");
                let manifest = std::fs::read_to_string(&cargo_toml_path).unwrap_or_default();
                let note_crate = crawrad_codegen::dependencies::caminho_local_crate("crawrad-note");
                let added = with_crawrad_note_dependency(&manifest, crate_dir, &note_crate)
                    .map(|updated| std::fs::write(&cargo_toml_path, updated));
                self.status = Some(match added {
                    Some(Err(err)) => format!(
                        "Saved: {} · Could not add crawrad-note to Cargo.toml ({err}): add it to [dependencies] by hand.",
                        path.display()
                    ),
                    Some(Ok(())) => format!(
                        "Saved: {} · Added crawrad-note to [dependencies] in Cargo.toml.",
                        path.display()
                    ),
                    None if depends_on_crawrad_note(&manifest) => {
                        format!("Saved: {}", path.display())
                    }
                    None => format!(
                        "Saved: {} · Add crawrad-note to [dependencies] so the build turns this note into docs.",
                        path.display()
                    ),
                });

                let new_source = if let Some(ref target) = self.target {
                    apply_note_attribute(source, target, &self.note.id)
                } else {
                    source.to_string()
                };
                Ok(Some(new_source))
            }
            NoteKind::Internal | NoteKind::Comment => {
                self.note.kind = self.kind();
                let path = match sidecar_notes::save_note(&self.notes_root, &self.note) {
                    Ok(p) => p,
                    Err(err) => {
                        self.status = Some(format!("Failed to save: {err}"));
                        return Err(err);
                    }
                };
                self.status = Some(format!("Saved: {}", path.display()));
                let (old_tag, new_tag) = match self.kind() {
                    NoteKind::Comment => (
                        format!("[crawrad:note#{}", self.note.id),
                        format!("[crawrad:comment#{}", self.note.id),
                    ),
                    _ => (
                        format!("[crawrad:comment#{}", self.note.id),
                        format!("[crawrad:note#{}", self.note.id),
                    ),
                };
                if source.contains(&old_tag) {
                    Ok(Some(source.replace(&old_tag, &new_tag)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

/// `problem` as a plain sentence: no code ticks, a capital first letter and a
/// closing full stop.
fn sentence(problem: &str) -> String {
    let problem = problem.replace('`', "");
    let mut chars = problem.chars();
    let mut out: String = chars
        .next()
        .map(|c| c.to_uppercase().collect())
        .unwrap_or_default();
    out.push_str(chars.as_str());
    if !out.ends_with('.') {
        out.push('.');
    }
    out
}

#[note("note-dialog-anchor-code-line")]
fn anchor_code_line(source: &str, line_1based: usize) -> String {
    let line = source
        .lines()
        .nth(line_1based.saturating_sub(1))
        .unwrap_or("");
    let without_anchor = if let Some(pos) = line.find("// [crawrad:") {
        &line[..pos]
    } else if let Some(pos) = line.find("/* [crawrad:") {
        &line[..pos]
    } else {
        line
    };
    let trimmed = without_anchor.trim();
    if trimmed.is_empty() {
        line.trim().to_string()
    } else {
        trimmed.to_string()
    }
}

#[note("note-dialog-format-byline")]
fn format_byline(author: &str, created: &str) -> String {
    let author_clean = author.trim();
    let date_clean = format_created_date(created);
    match (!author_clean.is_empty(), date_clean.as_deref()) {
        (true, Some(date)) if !date.is_empty() => format!("{author_clean} \u{b7} {date}"),
        (true, _) => author_clean.to_string(),
        (false, Some(date)) if !date.is_empty() => date.to_string(),
        _ => String::new(),
    }
}

fn format_created_date(created: &str) -> Option<String> {
    let date_part = created.split('T').next()?.trim();
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() >= 3 {
        let year = parts[0].parse::<u32>().ok()?;
        let month = parts[1].parse::<usize>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        if (1..=12).contains(&month) {
            return Some(format!("{day} {} {year}", months[month - 1]));
        }
    }
    None
}

#[cfg(test)]
#[path = "test/note_dialog/test.rs"]
mod test;
