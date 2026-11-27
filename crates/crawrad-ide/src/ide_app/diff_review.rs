//! The AI code diff review gate: propose, accept, reject and its bar. Cut out of `ide_app.rs`.

use super::*;
use crawrad_agent::diff_review::DiffReview;
use crawrad_agent::{FileWriteRequest, PermissionRequest};
use crawrad_host::references::same_file;
use crawrad_mcp::project_tree::resolve;
use crawrad_note::note;
use makepad_code_editor::code_view::CodeView;
use makepad_code_editor::review::ReviewRowKind::{Added, Removed};
use makepad_code_editor::review::{ReviewRow, ReviewView, review_view};
use std::sync::Arc;

#[note("ide-app-diff-review-review-pending")]
pub(super) const REVIEW_PENDING: &str = "diff review pending (accept or reject first)";

impl IdeApp {
    // --- AI Code Diff Review Gate ---

    pub fn has_pending_diff_review(&self, path: &std::path::Path) -> bool {
        self.diff_reviews.has_pending_review(path)
    }

    /// The file shows a merged diff now: looked at, never typed into.
    pub(super) fn reviewing(&self, path: &std::path::Path) -> bool {
        self.has_pending_diff_review(path)
    }

    #[note("ide-app-diff-review-ideapp-save-waits-for-review")]
    pub(super) fn save_waits_for_review(&mut self) {
        self.status = Some(format!("Cannot save: {REVIEW_PENDING}"));
    }

    pub fn has_any_pending_diff_review(&self) -> bool {
        self.diff_reviews.any_pending().is_some()
    }

    pub fn get_pending_diff_review(
        &self,
        path: &std::path::Path,
    ) -> Option<&crawrad_agent::diff_review::DiffReview> {
        self.diff_reviews.get_pending_review(path)
    }

    pub fn propose_diff_review(
        &mut self,
        path: std::path::PathBuf,
        original: String,
        proposed: String,
        author_tool: &str,
    ) -> Result<crawrad_agent::diff_review::DiffReview, String> {
        self.diff_reviews
            .propose_edit(path, original, proposed, author_tool)
    }

    pub fn accept_diff_review(
        &mut self,
        cx: &mut Cx,
        path: &std::path::Path,
    ) -> Result<String, String> {
        let review = self.get_pending_diff_review(path);
        let base = review.map(|r| r.base_content.clone());
        // An agent writes the file itself: the disk brings the text, no buffer.
        let agent = review.is_some_and(DiffReview::from_agent);
        let proposed = self.diff_reviews.accept(path)?;
        let undo = self.editor_undo_stacks.entry(path.to_path_buf());
        undo.or_default().extend(base);
        match agent {
            true => drop(self.host.remove_buffer(path)),
            false => self.set_editor_buffer(path.to_path_buf(), proposed.clone()),
        }
        self.chat.writes.decide(path, true);
        self.chat.edit_asks.decide(path, true);
        self.decided(cx, path, "accepted");
        Ok(proposed)
    }

    pub fn reject_diff_review(
        &mut self,
        cx: &mut Cx,
        path: &std::path::Path,
    ) -> Result<String, String> {
        let original = self.diff_reviews.reject(path)?;
        if std::fs::read_to_string(path).is_ok_and(|disk| disk == original) {
            self.host.remove_buffer(path);
        } else {
            self.set_editor_buffer(path.to_path_buf(), original.clone());
        }
        self.chat.writes.decide(path, false);
        self.chat.edit_asks.decide(path, false);
        self.decided(cx, path, "rejected");
        Ok(original)
    }

    #[note("ide-app-diff-review-ideapp-decided")]
    fn decided(&mut self, cx: &mut Cx, path: &std::path::Path, decision: &str) {
        let what = serde_json::json!({ "path": path.display().to_string(), "decision": decision });
        crawrad_mcp::AuditLogger::record("diff_review.decision", &what, decision, "ide-user");
        self.review_change_index = 0;
        self.update_diff_review_bar(cx);
        self.redraw_editor_tabs(cx);
        self.show_center(cx, self.host.center());
    }

    pub fn desfazer_editor(&mut self, cx: &mut Cx, path: &std::path::Path) -> bool {
        if let Some(prev) = self
            .editor_undo_stacks
            .get_mut(path)
            .and_then(|stack| stack.pop())
        {
            self.set_editor_buffer(path.to_path_buf(), prev);
            self.redraw_editor_tabs(cx);
            return true;
        }
        false
    }

    /// Reviewed file: the shown source first, since a form's code view keeps its `.crawform` tab.
    pub(super) fn reviewed_file(&self) -> Option<std::path::PathBuf> {
        [self.editavel.clone(), self.active_file_path()]
            .into_iter()
            .flatten()
            .find(|p| self.has_pending_diff_review(p))
    }

    /// Any file with a review waiting — the shown one first, otherwise whichever
    /// is pending. A caller outside the IDE has no tab in front of it.
    pub fn any_reviewed_path(&self) -> Option<std::path::PathBuf> {
        self.reviewed_file()
            .or_else(|| self.diff_reviews.any_pending().map(|r| r.file_path.clone()))
    }

    pub(super) fn pending_review(&self) -> Option<&DiffReview> {
        self.get_pending_diff_review(&self.reviewed_file()?)
    }

    /// The merged diff of the review pending on the shown file.
    pub(super) fn review_view(&self) -> Option<ReviewView> {
        let review = self.pending_review()?;
        Some(review_view(
            &review.original_content,
            &review.proposed_content,
        ))
    }

    pub(super) fn set_editor_review(&self, cx: &mut Cx, rows: &[ReviewRow]) {
        // Per-change buttons for the IDE's own reviews; an agent decides whole.
        let buttons = !self.pending_review().is_some_and(DiffReview::from_agent);
        if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
            cv.set_review_buttons(buttons);
            cv.set_review(cx, rows);
        }
    }

    /// A per-change ✓/✕ clicked in the editor; the last one decides the review.
    pub(super) fn follow_diff_review(&mut self, cx: &mut Cx) {
        let choice = self.with_code_view(|cv| cv.take_review_choice()).flatten();
        let (Some((k, accept)), Some(path)) = (choice, self.reviewed_file()) else {
            return;
        };
        match self.diff_reviews.decide_change(&path, k, accept) {
            Ok(true) => drop(self.accept_diff_review(cx, &path)),
            _ => {
                // The bar at the top counts what is still UNDECIDED. Deciding a
                // change inline and not refreshing it left "Accept all" offering
                // to accept changes the user had already dealt with, over counts
                // that no longer matched the editor.
                self.update_diff_review_bar(cx);
                self.show_center(cx, self.host.center());
            }
        }
    }

    pub(super) fn step_diff_review(&mut self, cx: &mut Cx, forward: bool) {
        let Some(view) = self.review_view() else {
            return;
        };
        self.review_change_index = view.step(self.review_change_index, forward);
        let line = view.changes.get(self.review_change_index);
        if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
            cv.reveal_line(cx, line.copied().unwrap_or(0));
        }
        self.update_diff_review_bar(cx);
    }

    pub fn update_diff_review_bar(&mut self, cx: &mut Cx) {
        let bar = self.painel(Panel::Design).view(id!(diff_review_bar));
        let author = self.pending_review().map(|r| r.author_tool.clone());
        bar.set_visible(cx, author.is_some());
        let (Some(author), Some(view)) = (author, self.review_view()) else {
            return;
        };
        let position = view.position_label(self.review_change_index);
        let mut set = |id: &[LiveId], text: &str| bar.label(id).set_text(cx, text);
        set(id!(lbl_diff_info), &author);
        set(id!(lbl_diff_added), &format!("+{}", view.count(Added)));
        set(id!(lbl_diff_removed), &format!("−{}", view.count(Removed)));
        set(id!(lbl_diff_changes), &view.changes_label());
        set(id!(lbl_diff_count), position.as_deref().unwrap_or_default());
        for id in [id!(btn_diff_prev), id!(lbl_diff_count), id!(btn_diff_next)] {
            bar.widget(id).set_visible(cx, position.is_some());
        }
    }

    /// An agent's file change: lands like an MCP edit, under the user's review mode.
    fn land_agent_change(
        &mut self,
        cx: &mut Cx,
        tool: &str,
        file: &std::path::Path,
        change: impl FnOnce(&str) -> Result<String, String>,
    ) -> Result<Option<std::path::PathBuf>, String> {
        let path = resolve(&file.to_string_lossy(), self.raiz_projeto().as_deref());
        // The agent spells paths its own way; land on the tab already open for the file.
        let mut open = self.edicoes_paths();
        open.extend(self.editavel.clone());
        let path = open
            .into_iter()
            .find(|p| same_file(p, &path))
            .unwrap_or(path);
        let before = self
            .get_editor_buffer(&path)
            .or_else(|| std::fs::read_to_string(&path).ok());
        let after = change(before.as_deref().unwrap_or_default())?;
        let held = self.land_edit(
            tool,
            path.clone(),
            (before.unwrap_or_default(), after),
            true,
        )?;
        if held.is_some() {
            self.abrir_no_editor(cx, path.clone());
            self.update_diff_review_bar(cx);
        }
        Ok(held.map(|_| path))
    }

    /// An agent's `fs/write_text_file`.
    pub fn agent_write(&mut self, cx: &mut Cx, req: Arc<FileWriteRequest>) {
        match self.land_agent_change(cx, "agent.write", &req.path, |_| Ok(req.content.clone())) {
            Ok(Some(path)) => self.chat.writes.hold(path, req),
            Ok(None) => req.respond(Ok(())),
            Err(e) => req.respond(Err(e)),
        }
    }

    /// An agent's edit permission request: shown as the diff when it carries the change.
    pub(super) fn agent_edit_ask(&mut self, cx: &mut Cx, req: &Arc<PermissionRequest>) -> bool {
        let Some(edit) = req.proposed_edit.clone() else {
            return false;
        };
        match self.land_agent_change(cx, "agent.edit", &edit.path, |text| edit.apply(text)) {
            Ok(Some(path)) => self.chat.edit_asks.hold(path, Arc::clone(req)),
            Ok(None) => self.chat.edit_asks.allow(req),
            Err(_) => return false,
        }
        true
    }
}
