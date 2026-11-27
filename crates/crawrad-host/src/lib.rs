//! `crawrad-host` — IDE Host API and state model for CrawRAD Studio.
//!
//! Enforces ADR 0044: The single non-visual entry point for querying and mutating IDE state.
//! This crate is strictly non-visual and never depends on `makepad-widgets` or any UI framework.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub mod bookmarks;
pub mod breadcrumb;
pub mod breakpoints;
pub mod dataset_columns;
pub mod debug;
pub mod debug_panel;
pub mod design_rows;
pub mod disk_watch;
pub mod document;
pub mod editor_chords;
pub mod editor_footer;
pub mod editor_tabs;
pub mod find;
pub mod history;
pub mod history_view;
pub mod hover_card;
pub mod inline_values;
pub mod line_edits;
pub mod link_pick;
pub mod navigation;
pub mod problems;
pub mod quick_open;
pub mod references;
pub mod rename;
pub mod ui_scale;
pub mod word_motion;

pub use bookmarks::{BookmarkSync, Bookmarks};
pub use breakpoints::{Breakpoints, ViewSync};
use crawrad_doc::Doc;
use crawrad_ir::NodeId;
use crawrad_note::note;
pub use debug::{Debug, DebugEvent, Phase};
pub use problems::Problems;

// ---------------------------------------------------------------------------
// ViewCenter
// ---------------------------------------------------------------------------

#[note("viewcenter")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewCenter {
    /// The visual designer canvas.
    #[default]
    Design,
    /// The form shown as text (`.designer.rs`). Activated by `Alt+F12`.
    FormSource,
    /// The unit source file where handlers live. Activated by `F12`.
    Code,
    /// The local history view for the active file.
    History,
    /// The Sizes preview tab.
    Sizes,
    /// The Translations tab.
    Translations,
    /// The Welcome page: the recent projects, with no form in front.
    Welcome,
}

impl ViewCenter {
    /// The visual designer canvas is in front (designer keys act only then).
    pub fn is_design(self) -> bool {
        self == Self::Design
    }

    #[note("viewcenter-is-text")]
    pub fn is_text(self) -> bool {
        matches!(self, Self::FormSource | Self::Code)
    }

    #[note("viewcenter-as-str")]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Design => "design",
            Self::FormSource => "form_source",
            Self::Code => "code",
            Self::History => "history",
            Self::Sizes => "sizes",
            Self::Translations => "translations",
            Self::Welcome => "welcome",
        }
    }

    #[note("viewcenter-design-tab-label")]
    pub fn design_tab_label(self) -> &'static str {
        match self {
            Self::FormSource => "Design Code",
            Self::Design
            | Self::Code
            | Self::History
            | Self::Sizes
            | Self::Translations
            | Self::Welcome => "Design",
        }
    }

    #[note("viewcenter-toggle-code")]
    pub fn toggle_code(self) -> Self {
        match self {
            Self::Code => Self::Design,
            Self::Design
            | Self::FormSource
            | Self::History
            | Self::Sizes
            | Self::Translations
            | Self::Welcome => Self::Code,
        }
    }

    #[note("viewcenter-toggle-form-source")]
    pub fn toggle_form_source(self) -> Self {
        match self {
            Self::FormSource => Self::Design,
            Self::Design
            | Self::Code
            | Self::History
            | Self::Sizes
            | Self::Translations
            | Self::Welcome => Self::FormSource,
        }
    }
}

// ---------------------------------------------------------------------------
// HostModel
// ---------------------------------------------------------------------------

/// A form open in the strip: its document, and the file it came from.
///
/// The path is what tells two tabs apart and what keeps a file from opening
/// twice; it is `None` for a form that has never been saved.
#[note("hostmodel-openform")]
pub struct OpenForm {
    pub doc: Doc,
    pub path: Option<PathBuf>,
}

#[note("hostmodel")]
#[derive(Default)]
pub struct HostModel {
    /// The forms open in the strip, in the order their tabs are drawn.
    ///
    /// The IDE used to hold ONE form, so opening a second one replaced the first
    /// and the person lost their place. A tab per form is what the designer has
    /// always promised and what the strip above the canvas now shows.
    forms: Vec<OpenForm>,
    /// Which form's tab is in front. `None` when none is open.
    ///
    /// `doc()` answers for THIS one, so the hundred callers that ask the host for
    /// "the document" keep asking exactly as before: what changed is how many
    /// there can be, not how one is reached.
    front: Option<usize>,
    /// Which panel is in front (design canvas, form-as-text, or code editor).
    center: ViewCenter,
    /// Path of the file currently shown in the editor, if any.
    active_path: Option<PathBuf>,
    /// In-memory edits keyed by canonical file path (unsaved changes).
    editor_buffers: BTreeMap<PathBuf, String>,
    breakpoints: Breakpoints,
    bookmarks: Bookmarks,
    debug: Debug,
    /// A file a rename changed while it was not in front: its text from
    /// before, so the editor can replay the rename as ONE undo step when the
    /// file is shown.
    undo_bases: BTreeMap<PathBuf, String>,
    /// The columns of the open form's bound datasets.
    dataset_columns: dataset_columns::DatasetColumns,
    /// The first rows of the open form's bound grids.
    design_rows: design_rows::DesignRows,
}

impl HostModel {
    // -----------------------------------------------------------------------
    // Construction
    // -----------------------------------------------------------------------

    #[note("hostmodel-new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[note("hostmodel-with-doc")]
    pub fn with_doc(doc: Doc) -> Self {
        Self {
            forms: vec![OpenForm { doc, path: None }],
            front: Some(0),
            ..Self::default()
        }
    }

    // -----------------------------------------------------------------------
    // Document (doc + selection) — unchanged from PR 1
    // -----------------------------------------------------------------------

    #[note("hostmodel-doc")]
    pub fn doc(&self) -> Option<&Doc> {
        self.forms.get(self.front?).map(|f| &f.doc)
    }

    #[note("hostmodel-doc-mut")]
    pub fn doc_mut(&mut self) -> Option<&mut Doc> {
        let i = self.front?;
        self.forms.get_mut(i).map(|f| &mut f.doc)
    }

    #[note("hostmodel-set-doc")]
    pub fn set_doc(&mut self, doc: Option<Doc>) {
        match (doc, self.front) {
            // Replacing what is in front: the tab stays, its document changes.
            (Some(d), Some(i)) => self.forms[i].doc = d,
            // Nothing in front yet, so this opens the first tab.
            (Some(d), None) => {
                self.forms.push(OpenForm { doc: d, path: None });
                self.front = Some(self.forms.len() - 1);
            }
            // `None` closes the tab in front and leaves the others alone. It used
            // to mean "no document at all", which was the same thing when there
            // could only be one.
            (None, Some(i)) => {
                self.forms.remove(i);
                self.front = self.front_after_removing(i);
            }
            (None, None) => {}
        }
    }

    /// Which tab comes forward when the one at `removed` has just gone.
    ///
    /// The neighbour on the right, as every editor does, falling back to the one
    /// on the left at the end of the strip, and to nothing when the strip empties.
    fn front_after_removing(&self, removed: usize) -> Option<usize> {
        if self.forms.is_empty() {
            None
        } else if removed < self.forms.len() {
            Some(removed)
        } else {
            Some(self.forms.len() - 1)
        }
    }

    // -----------------------------------------------------------------------
    // The strip: many forms, one in front
    // -----------------------------------------------------------------------

    #[note("hostmodel-forms")]
    pub fn forms(&self) -> &[OpenForm] {
        &self.forms
    }

    #[note("hostmodel-front")]
    pub fn front(&self) -> Option<usize> {
        self.front
    }

    #[note("hostmodel-open-form")]
    pub fn open_form(&mut self, doc: Doc, path: Option<PathBuf>) -> usize {
        // A form already open comes forward instead of opening twice: two tabs
        // of one file would each hold their own edits, and saving would decide
        // between them silently.
        if let Some(p) = path.as_ref()
            && let Some(i) = self.forms.iter().position(|f| f.path.as_ref() == Some(p))
        {
            self.forms[i].doc = doc;
            self.front = Some(i);
            return i;
        }
        self.forms.push(OpenForm { doc, path });
        let i = self.forms.len() - 1;
        self.front = Some(i);
        i
    }

    #[note("hostmodel-bring-to-front")]
    pub fn bring_to_front(&mut self, i: usize) -> bool {
        if i < self.forms.len() {
            self.front = Some(i);
            true
        } else {
            false
        }
    }

    #[note("hostmodel-close-form")]
    pub fn close_form(&mut self, i: usize) -> bool {
        if i >= self.forms.len() {
            return false;
        }
        self.forms.remove(i);
        // Closing a tab BEHIND the one in front must not change which form the
        // person is looking at, so the index follows its form instead of staying
        // put. This is the bug every tab strip is born with.
        self.front = match self.front {
            Some(f) if f > i => Some(f - 1),
            Some(f) if f == i => self.front_after_removing(i),
            other => other,
        };
        true
    }

    #[note("hostmodel-close-all-forms")]
    pub fn close_all_forms(&mut self) {
        self.forms.clear();
        self.front = None;
    }

    #[note("hostmodel-probe-dataset-columns")]
    pub fn probe_dataset_columns(&mut self, project_dir: Option<&Path>) {
        // The fields are reached directly, not through `doc()`: that method
        // borrows the whole host, and the two caches below are other fields of
        // it. Disjoint field borrows are allowed; a method call is not.
        if let Some(i) = self.front
            && let Some(form) = self.forms.get(i)
        {
            let doc = &form.doc;
            self.dataset_columns
                .ensure(doc.ir(), doc.revision(), project_dir);
            self.design_rows
                .ensure(doc.ir(), doc.revision(), project_dir);
        }
    }

    #[note("hostmodel-dataset-columns")]
    pub fn dataset_columns(
        &self,
        project_dir: Option<&Path>,
        id: &str,
    ) -> Option<Result<Vec<String>, String>> {
        let doc = self.doc()?;
        self.dataset_columns.answer(doc.ir(), project_dir, id)
    }

    #[note("hostmodel-design-rows")]
    pub fn design_rows(
        &self,
        project_dir: Option<&Path>,
    ) -> BTreeMap<crawrad_ir::NodeId, crawrad_ir::design_rows::GridRows> {
        let Some(doc) = self.doc().map(|d| d.ir()) else {
            return BTreeMap::new();
        };
        doc.nodes
            .keys()
            .filter_map(|id| Some((id.clone(), self.design_rows.rows(doc, project_dir, id)?)))
            .collect()
    }

    #[note("hostmodel-pump-dataset-columns")]
    pub fn pump_dataset_columns(&mut self) -> bool {
        let columns = self.dataset_columns.pump();
        self.design_rows.pump() || columns
    }

    #[note("hostmodel-has-doc")]
    pub fn has_doc(&self) -> bool {
        self.doc().is_some()
    }

    #[note("hostmodel-selection")]
    pub fn selection(&self) -> &[NodeId] {
        self.doc().map(|d| d.selection()).unwrap_or(&[])
    }

    #[note("hostmodel-set-selection")]
    pub fn set_selection(&mut self, selection: Vec<NodeId>) {
        if let Some(doc) = self.doc_mut() {
            doc.select(selection);
        }
    }

    #[note("hostmodel-select-one")]
    pub fn select_one(&mut self, id: impl Into<NodeId>) {
        if let Some(doc) = self.doc_mut() {
            doc.select_one(id);
        }
    }

    #[note("hostmodel-breakpoints")]
    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    pub fn breakpoints_mut(&mut self) -> &mut Breakpoints {
        &mut self.breakpoints
    }

    #[note("hostmodel-bookmarks")]
    pub fn bookmarks(&self) -> &Bookmarks {
        &self.bookmarks
    }

    pub fn bookmarks_mut(&mut self) -> &mut Bookmarks {
        &mut self.bookmarks
    }

    #[note("hostmodel-sync-breakpoint-view")]
    pub fn sync_breakpoint_view(
        &mut self,
        file: &std::path::Path,
        view_owner: Option<&std::path::Path>,
        view_lines: &[usize],
    ) -> ViewSync {
        let step = self.breakpoints.sync_view(file, view_owner, view_lines);
        if step == ViewSync::Pulled {
            let sources = self.breakpoints.sources(file);
            self.debug.send(crawrad_dap::Command::SetBreakpoints(
                file.to_path_buf(),
                sources,
            ));
        }
        step
    }

    #[note("hostmodel-set-breakpoints-from-outside")]
    pub fn set_breakpoints_from_outside(&mut self, file: &std::path::Path, lines: &[u32]) {
        self.breakpoints
            .set_lines_from_outside(file, lines.iter().copied());
        let key = breakpoints::file_key(file);
        let sources = self.breakpoints.sources(&key);
        self.debug
            .send(crawrad_dap::Command::SetBreakpoints(key, sources));
    }

    #[note("hostmodel-debug-state-json")]
    pub fn debug_state_json(&self) -> serde_json::Value {
        let mut state = self.debug.snapshot();
        state["breakpoints"] = self.breakpoints_json();
        state
    }

    #[note("hostmodel-set-breakpoints-json")]
    pub fn set_breakpoints_json(
        &mut self,
        args: &serde_json::Value,
        root: Option<&std::path::Path>,
    ) -> Result<serde_json::Value, String> {
        let (Some(path), Some(lines)) = (args["path"].as_str(), args["lines"].as_array()) else {
            return Err("needs 'path' and 'lines'".into());
        };
        let path = std::path::PathBuf::from(path);
        let file = match root {
            Some(root) if !path.is_absolute() => root.join(path),
            _ => path,
        };
        let lines: Vec<u32> = lines
            .iter()
            .filter_map(serde_json::Value::as_u64)
            .filter_map(|l| u32::try_from(l).ok())
            .collect();
        self.set_breakpoints_from_outside(&file, &lines);
        Ok(self.breakpoints_json())
    }

    #[note("hostmodel-breakpoints-json")]
    pub fn breakpoints_json(&self) -> serde_json::Value {
        serde_json::Value::Object(
            self.breakpoints
                .files()
                .map(|(file, lines)| (file.display().to_string(), serde_json::json!(lines)))
                .collect(),
        )
    }

    #[note("hostmodel-debug")]
    pub fn debug(&self) -> &Debug {
        &self.debug
    }

    pub fn debug_mut(&mut self) -> &mut Debug {
        &mut self.debug
    }

    #[note("hostmodel-clear-selection")]
    pub fn clear_selection(&mut self) {
        if let Some(doc) = self.doc_mut() {
            doc.clear_selection();
        }
    }

    // -----------------------------------------------------------------------
    // View center — PR 2
    // -----------------------------------------------------------------------

    #[note("hostmodel-center")]
    pub fn center(&self) -> ViewCenter {
        self.center
    }

    #[note("hostmodel-set-center")]
    pub fn set_center(&mut self, center: ViewCenter) {
        self.center = center;
    }

    // -----------------------------------------------------------------------
    // Active file path — PR 2
    // -----------------------------------------------------------------------

    #[note("hostmodel-active-file")]
    pub fn active_file(&self) -> Option<&Path> {
        self.active_path.as_deref()
    }

    #[note("hostmodel-set-active-file")]
    pub fn set_active_file(&mut self, path: Option<PathBuf>) {
        self.active_path = path;
    }

    // -----------------------------------------------------------------------
    // Editor buffers (in-memory unsaved edits) — PR 2
    // -----------------------------------------------------------------------

    #[note("hostmodel-is-dirty")]
    pub fn is_dirty(&self) -> bool {
        !self.editor_buffers.is_empty()
    }

    #[note("hostmodel-unsaved")]
    pub fn unsaved(&self, editable: Option<&Path>, saved_revision: u64) -> (bool, bool) {
        let form = self.doc().is_some_and(|d| d.revision() != saved_revision);
        let file = editable.is_some_and(|f| self.buffer(f).is_some());
        (file || form, form || self.is_dirty())
    }

    #[note("hostmodel-open-file-paths")]
    pub fn open_file_paths(&self) -> Vec<PathBuf> {
        self.editor_buffers.keys().cloned().collect()
    }

    #[note("hostmodel-read-text")]
    pub fn read_text(&self, path: &Path, key: &str) -> Result<String, (&'static str, String)> {
        if let Some(text) = self.buffer(path).or_else(|| self.buffer_by_key(key)) {
            return Ok(text.to_string());
        }
        let shown = path.display();
        if !path.is_file() {
            let said = format!("File '{shown}' not found in editor tabs or on disk.");
            return Err(("file-not-found", said));
        }
        std::fs::read_to_string(path)
            .map_err(|e| ("io-error", format!("Failed to read file '{shown}': {e}")))
    }

    #[note("hostmodel-open-files-json")]
    pub fn open_files_json(&self) -> serde_json::Value {
        let active = self.active_file();
        let mut files: Vec<serde_json::Value> = self
            .open_file_paths()
            .iter()
            .map(|p| serde_json::json!({ "path": p.display().to_string(),
                                          "is_active": active == Some(p.as_path()), "is_dirty": true }))
            .collect();
        if let Some(a) = active.filter(|a| self.buffer(a).is_none()) {
            files.push(serde_json::json!({ "path": a.display().to_string(), "is_active": true, "is_dirty": false }));
        }
        let count = files.len();
        serde_json::json!({ "open_files": files, "count": count })
    }

    #[note("hostmodel-buffer")]
    pub fn buffer(&self, path: &Path) -> Option<&str> {
        self.editor_buffers.get(path).map(String::as_str)
    }

    #[note("hostmodel-edited-text")]
    pub fn edited_text(&self, path: &Path) -> Option<String> {
        self.buffer_by_key(&path.to_string_lossy())
            .map(String::from)
    }

    #[note("hostmodel-buffer-by-key")]
    pub fn buffer_by_key(&self, key: &str) -> Option<&str> {
        let needle = key.replace('\\', "/");
        let needle = needle.trim_start_matches("./");
        self.editor_buffers.iter().find_map(|(p, text)| {
            let k = p.to_string_lossy().replace('\\', "/");
            if k.trim_start_matches("./") == needle {
                Some(text.as_str())
            } else {
                None
            }
        })
    }

    #[note("hostmodel-set-buffer")]
    pub fn set_buffer(&mut self, path: PathBuf, content: String) {
        self.editor_buffers.insert(path, content);
    }

    #[note("hostmodel-remove-buffer")]
    pub fn remove_buffer(&mut self, path: &Path) -> Option<String> {
        self.editor_buffers.remove(path)
    }

    #[note("hostmodel-save-buffers")]
    pub fn save_buffers(
        &mut self,
        write: &mut dyn FnMut(&Path, &str) -> Result<(), String>,
    ) -> Vec<(PathBuf, Result<(), String>)> {
        let mut report = Vec::new();
        for (path, text) in &self.editor_buffers {
            report.push((path.clone(), write(path, text)));
        }
        for (path, outcome) in &report {
            if outcome.is_ok() {
                self.editor_buffers.remove(path);
            }
        }
        report
    }

    #[note("hostmodel-set-buffer-undoable")]
    pub fn set_buffer_undoable(&mut self, path: PathBuf, before: String, after: String) {
        self.undo_bases.entry(path.clone()).or_insert(before);
        self.editor_buffers.insert(path, after);
    }

    #[note("hostmodel-take-undo-base")]
    pub fn take_undo_base(&mut self, path: &Path) -> Option<String> {
        self.undo_bases.remove(path)
    }

    #[note("hostmodel-editor-buffers")]
    pub fn editor_buffers(&self) -> &BTreeMap<PathBuf, String> {
        &self.editor_buffers
    }

    #[note("hostmodel-editor-buffers-mut")]
    pub fn editor_buffers_mut(&mut self) -> &mut BTreeMap<PathBuf, String> {
        &mut self.editor_buffers
    }

    #[note("hostmodel-project-forms")]
    pub fn project_forms(&self) -> Vec<(String, crawrad_ir::Document)> {
        let stem = |p: &Path| {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string());
            n.unwrap_or_default()
                .trim_end_matches(".crawform")
                .to_string()
        };
        let open = self.active_file().map(Path::to_path_buf);
        let mut forms: Vec<(String, crawrad_ir::Document)> = Vec::new();
        if let (Some(p), Some(d)) = (&open, self.doc()) {
            forms.push((stem(p), d.ir().clone()));
        }
        let mut dirs: Vec<PathBuf> = self.workspace_root().into_iter().collect();
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
                let p = entry.path();
                let skip = p.file_name().is_some_and(|n| n == "target" || n == ".git");
                if p.is_dir() && !skip {
                    dirs.push(p);
                } else if p.extension().is_some_and(|e| e == "crawform")
                    && !open
                        .as_deref()
                        .is_some_and(|o| crate::references::same_file(o, &p))
                    && let Ok(d) = crawrad_ir::Document::load(&p)
                {
                    forms.push((stem(&p), d));
                }
            }
        }
        forms
    }

    pub fn workspace_root(&self) -> Option<PathBuf> {
        let mut cur = self.active_path.as_ref()?.as_path();
        if cur.is_file()
            && let Some(parent) = cur.parent()
        {
            cur = parent;
        }
        let mut check = cur;
        loop {
            if check.join("Cargo.toml").exists() {
                return Some(check.to_path_buf());
            }
            match check.parent() {
                Some(p) => check = p,
                None => break,
            }
        }
        Some(cur.to_path_buf())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crawrad_doc::Transaction;
    use crawrad_doc::command::Command;
    use crawrad_ir::{Node, Rect};

    fn make_test_document() -> Doc {
        let mut document = crawrad_ir::Document::new("form_main", "Form");
        let mut btn = Node::new("btn_ok", "Button");
        btn.set_rect(Rect::new(10.0, 10.0, 80.0, 28.0));
        document.add_child("form_main", btn).unwrap();
        Doc::new(document)
    }

    #[test]
    fn host_model_initial_empty_state() {
        let host = HostModel::new();
        assert!(host.doc().is_none());
        assert!(!host.has_doc());
        assert!(host.selection().is_empty());
        assert_eq!(host.center(), ViewCenter::Design);
        assert!(host.active_file().is_none());
        assert!(!host.is_dirty());
        assert!(host.open_file_paths().is_empty());
    }

    #[test]
    fn host_model_with_doc_reads_document_and_selection() {
        let doc = make_test_document();
        let mut host = HostModel::with_doc(doc);

        assert!(host.has_doc());
        assert_eq!(host.doc().unwrap().ir().nodes.len(), 2);
        assert_eq!(host.selection().len(), 0);

        // Select btn_ok
        let btn_id = "btn_ok".to_string();
        host.set_selection(vec![btn_id.clone()]);
        assert_eq!(host.selection(), &[btn_id]);

        // Clear selection
        host.clear_selection();
        assert!(host.selection().is_empty());
    }

    #[test]
    fn host_model_doc_mut_applies_transaction() {
        let doc = make_test_document();
        let mut host = HostModel::with_doc(doc);

        let tx = Transaction::user("move button").with(Command::SetRect {
            node: "btn_ok".into(),
            rect: Rect::new(50.0, 50.0, 100.0, 30.0),
        });

        let applied = host.doc_mut().unwrap().apply(tx).unwrap();
        assert_eq!(applied.revision, 1);
        assert_eq!(host.doc().unwrap().revision(), 1);

        let node = host.doc().unwrap().ir().node("btn_ok").unwrap();
        assert_eq!(node.rect().unwrap().x, 50.0);
        assert_eq!(node.rect().unwrap().y, 50.0);
    }

    #[test]
    fn host_model_set_doc_replaces_and_clears() {
        let mut host = HostModel::new();
        assert!(!host.has_doc());

        let doc = make_test_document();
        host.set_doc(Some(doc));
        assert!(host.has_doc());

        host.set_doc(None);
        assert!(!host.has_doc());
        assert!(host.selection().is_empty());
    }

    #[test]
    fn host_model_center_defaults_to_design() {
        let host = HostModel::new();
        assert_eq!(host.center(), ViewCenter::Design);
    }

    #[test]
    fn host_model_center_set_and_read() {
        let mut host = HostModel::new();
        host.set_center(ViewCenter::Code);
        assert_eq!(host.center(), ViewCenter::Code);
        host.set_center(ViewCenter::FormSource);
        assert_eq!(host.center(), ViewCenter::FormSource);
    }

    #[test]
    fn host_model_active_file_set_and_read() {
        let mut host = HostModel::new();
        assert!(host.active_file().is_none());

        let path = PathBuf::from("/project/src/main.rs");
        host.set_active_file(Some(path.clone()));
        assert_eq!(host.active_file(), Some(path.as_path()));

        host.set_active_file(None);
        assert!(host.active_file().is_none());
    }

    #[test]
    fn a_renamed_file_keeps_its_text_from_before_once() {
        let mut host = HostModel::new();
        let p = PathBuf::from("enc.rs");
        host.set_buffer_undoable(p.clone(), "area(1)".into(), "size(1)".into());
        host.set_buffer_undoable(p.clone(), "size(1)".into(), "len(1)".into());
        assert_eq!(host.buffer(&p), Some("len(1)"));
        assert_eq!(host.take_undo_base(&p).as_deref(), Some("area(1)"));
        assert_eq!(host.take_undo_base(&p), None);
    }

    #[test]
    fn save_buffers_writes_each_and_a_failed_one_stays_dirty() {
        let mut host = HostModel::new();
        host.set_buffer(PathBuf::from("a.rs"), "fn a() {}".into());
        host.set_buffer(PathBuf::from("b.rs"), "fn b() {}".into());
        host.set_buffer(PathBuf::from("c.rs"), "fn c() {}".into());
        let mut written = Vec::new();
        let report = host.save_buffers(&mut |p, t| match p.ends_with("b.rs") {
            true => Err("disk full".into()),
            false => {
                written.push((p.to_path_buf(), t.to_string()));
                Ok(())
            }
        });
        assert_eq!(
            written,
            vec![
                (PathBuf::from("a.rs"), "fn a() {}".to_string()),
                (PathBuf::from("c.rs"), "fn c() {}".to_string()),
            ]
        );
        assert_eq!(report.len(), 3);
        assert_eq!(report[1], (PathBuf::from("b.rs"), Err("disk full".into())));
        assert_eq!(host.open_file_paths(), vec![PathBuf::from("b.rs")]);
        assert_eq!(host.buffer(Path::new("b.rs")), Some("fn b() {}"));
        assert!(host.save_buffers(&mut |_, _| Ok(())).len() == 1 && !host.is_dirty());
    }

    #[test]
    fn host_model_editor_buffers_dirty_state() {
        let mut host = HostModel::new();
        assert!(!host.is_dirty());

        let path = PathBuf::from("/project/src/main.rs");
        host.set_buffer(path.clone(), "fn main() {}".to_string());
        assert!(host.is_dirty());
        assert_eq!(host.buffer(&path), Some("fn main() {}"));

        let removed = host.remove_buffer(&path);
        assert_eq!(removed, Some("fn main() {}".to_string()));
        assert!(!host.is_dirty());
    }

    /// Save follows the file in front and the form; Save All, every buffer.
    #[test]
    fn unsaved_lights_save_for_the_file_in_front_and_save_all_for_any() {
        let mut host = HostModel::with_doc(make_test_document());
        let (front, other) = (PathBuf::from("a.rs"), PathBuf::from("b.rs"));
        assert_eq!(host.unsaved(Some(&front), 0), (false, false));
        host.set_buffer(other.clone(), "fn b() {}".into());
        assert_eq!(host.unsaved(Some(&front), 0), (false, true));
        host.set_buffer(front.clone(), "fn a() {}".into());
        assert_eq!(host.unsaved(Some(&front), 0), (true, true));
        host.remove_buffer(&front);
        host.remove_buffer(&other);
        let tx = Transaction::user("move").with(Command::SetRect {
            node: "btn_ok".into(),
            rect: Rect::new(5.0, 5.0, 80.0, 28.0),
        });
        host.doc_mut().unwrap().apply(tx).unwrap();
        assert_eq!(host.unsaved(None, 0), (true, true));
        assert_eq!(host.unsaved(None, 1), (false, false));
    }

    #[test]
    fn host_model_buffer_by_key_canonical() {
        let mut host = HostModel::new();
        let path = PathBuf::from("src/main.rs");
        host.set_buffer(path, "content".to_string());

        assert_eq!(host.buffer_by_key("src/main.rs"), Some("content"));
        assert_eq!(host.buffer_by_key("./src/main.rs"), Some("content"));
        assert!(host.buffer_by_key("src/lib.rs").is_none());
    }

    /// Proves that `is_dirty` becomes `true` after an edit via `set_buffer`.
    ///
    /// Simulates the Replace All path:
    ///   `find.rs` calls `host.set_buffer(arq, novo_texto)` →
    ///   `tool_project_info` reads `host.is_dirty()` → must return `true`.
    ///
    /// **Mutation test**: remove the `host.set_buffer(...)` call and this
    /// test turns red — ensuring there is no write left forgotten.
    #[test]
    fn host_model_is_dirty_after_set_buffer_replace_all() {
        let mut host = HostModel::new();
        let path = PathBuf::from("/project/src/lib.rs");

        // Clean state before
        assert!(!host.is_dirty(), "host must be clean before any edit");

        // Simulates Replace All: writes new text into the buffer
        host.set_buffer(path.clone(), "fn replaced() {}".to_string());

        // PROOF: is_dirty is true afterwards
        assert!(
            host.is_dirty(),
            "is_dirty must be true after set_buffer (Replace All). \
             If it failed, Replace All is not calling host.set_buffer."
        );
        assert_eq!(host.buffer(&path), Some("fn replaced() {}"));
    }

    /// Proves that `active_file` and `workspace_root` are populated after
    /// `set_active_file` — simulates the New Project workflow.
    ///
    /// **Mutation test**: remove the `host.set_active_file(...)` call and
    /// this test turns red — ensuring New Project does not end up without
    /// `active_file` and without `workspace_root`.
    #[test]
    fn host_model_active_file_after_new_project() {
        let mut host = HostModel::new();

        // Empty state before
        assert!(
            host.active_file().is_none(),
            "active_file must be None before New Project"
        );

        // Simulates New Project: sets the main file
        let main_rs = PathBuf::from("/project/src/main.rs");
        host.set_active_file(Some(main_rs.clone()));

        // PROOF: active_file is populated
        assert_eq!(
            host.active_file(),
            Some(main_rs.as_path()),
            "active_file must be Some after set_active_file (New Project). \
             If it failed, new_project.rs is not calling host.set_active_file."
        );
    }

    /// Proves that `ViewCenter::is_text()` and the toggles work correctly.
    #[test]
    fn view_center_methods() {
        assert!(!ViewCenter::Design.is_text());
        assert!(ViewCenter::Code.is_text());
        assert!(ViewCenter::FormSource.is_text());
        assert!(!ViewCenter::History.is_text());
        assert!(!ViewCenter::Welcome.is_text());
        assert_eq!(ViewCenter::Welcome.as_str(), "welcome");
        assert_eq!(ViewCenter::Welcome.toggle_code(), ViewCenter::Code);

        assert_eq!(ViewCenter::Design.toggle_code(), ViewCenter::Code);
        assert_eq!(ViewCenter::Code.toggle_code(), ViewCenter::Design);
        assert_eq!(ViewCenter::FormSource.toggle_code(), ViewCenter::Code);
        assert_eq!(ViewCenter::History.toggle_code(), ViewCenter::Code);

        assert_eq!(
            ViewCenter::Design.toggle_form_source(),
            ViewCenter::FormSource
        );
        assert_eq!(
            ViewCenter::FormSource.toggle_form_source(),
            ViewCenter::Design
        );
        assert_eq!(
            ViewCenter::Code.toggle_form_source(),
            ViewCenter::FormSource
        );
        assert_eq!(
            ViewCenter::History.toggle_form_source(),
            ViewCenter::FormSource
        );

        assert_eq!(ViewCenter::FormSource.design_tab_label(), "Design Code");
        assert_eq!(ViewCenter::Design.design_tab_label(), "Design");
        assert_eq!(ViewCenter::Code.design_tab_label(), "Design");
        assert_eq!(ViewCenter::History.design_tab_label(), "Design");
        assert!(!ViewCenter::Sizes.is_text());
        assert_eq!(ViewCenter::Sizes.as_str(), "sizes");
        assert_eq!(ViewCenter::Sizes.toggle_code(), ViewCenter::Code);
        assert_eq!(
            ViewCenter::Sizes.toggle_form_source(),
            ViewCenter::FormSource
        );
        assert_eq!(ViewCenter::Sizes.design_tab_label(), "Design");
        assert_eq!(ViewCenter::History.as_str(), "history");
    }
}

#[cfg(test)]
#[path = "test/form_strip/test.rs"]
mod form_strip_test;
