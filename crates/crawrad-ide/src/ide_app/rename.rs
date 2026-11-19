//! Rename Symbol: the glue between the window and what is decided
//! elsewhere — the requests in `crawrad_lsp::rename`, the guard and the
//! dialog's state in `crawrad_host::rename`. This file asks, takes the
//! answers, draws the dialog, and applies a previewed rename in memory.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};

use crawrad_host::references::same_file;
use crawrad_host::rename::{FileChange, Stage};
use makepad_widgets::*;

use makepad_code_editor::code_view::CodeView;

use super::IdeApp;
use crawrad_note::note;

#[note("ide-app-rename-out-of-view-version")]
static OUT_OF_VIEW_VERSION: AtomicI32 = AtomicI32::new(1_000_000);

#[note("ide-app-rename-change-of")]
pub(super) fn change_of(c: crawrad_lsp::rename::Planned) -> FileChange {
    let (file, count, before, after) = (c.file, c.count, c.before, c.after);
    FileChange {
        file,
        count,
        before,
        after,
    }
}

impl IdeApp {
    #[note("ide-app-rename-ideapp-rename-symbol")]
    pub(super) fn rename_symbol(&mut self, cx: &mut Cx) {
        let (Some(lsp), Some(file), Some(here)) =
            (self.lsp.clone(), self.editavel.clone(), self.here(cx))
        else {
            self.status =
                Some("Rename Symbol needs rust-analyzer and a file that takes typing".into());
            return;
        };
        let line = self.texto_na_vista.lines().nth(here.line).unwrap_or("");
        let name = crawrad_host::navigation::word_at(line, here.byte).to_string();
        let forms = self.host.project_forms();
        let forms: Vec<(String, &crawrad_ir::Document)> =
            forms.iter().map(|(n, d)| (n.clone(), d)).collect();
        if let Some(msg) = crawrad_host::rename::check_form_owner_refusal(&name, &forms) {
            self.status = Some(msg);
            return;
        }
        let caret = (here.line, here.byte);
        let ticket = self
            .nav
            .rename
            .start(file.clone(), caret, self.lsp_versao, &name);
        let asked = self.asked(&file, caret);
        self.lsp_hints.rename.prepare(lsp, ticket, asked);
        self.status = Some("Rename Symbol\u{2026}".into());
    }

    #[note("ide-app-rename-ideapp-asked")]
    fn asked<'a>(
        &'a self,
        file: &'a Path,
        caret: (usize, usize),
    ) -> crawrad_lsp::rename::Asked<'a> {
        let (version, text) = (self.lsp_versao, self.texto_na_vista.as_str());
        crawrad_lsp::rename::Asked {
            file,
            version,
            text,
            caret,
        }
    }

    #[note("ide-app-rename-ideapp-poll-rename")]
    pub(super) fn poll_rename(&mut self, cx: &mut Cx) {
        use crawrad_lsp::rename::Outcome;
        let Some(ticket) = self.nav.rename.pending.ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.rename.take(ticket) else {
            return;
        };
        if self.nav.rename.pending.answered(ticket).is_none() {
            return;
        }
        if answer.version != self.lsp_versao {
            self.nav.rename.close();
            self.status = Some("The file changed while renaming; Rename Symbol again".into());
            return self.draw_rename(cx);
        }
        match answer.outcome {
            Outcome::Renamable {
                start,
                end,
                declared,
            } => {
                let line = self.texto_na_vista.lines().nth(self.nav.rename.caret.0);
                let old = line
                    .and_then(|l| l.get(start..end))
                    .unwrap_or("")
                    .to_string();
                let project = self.host.workspace_root().unwrap_or_default();
                let roots = crawrad_host::navigation::foreign_roots();
                let suffix = crawrad_codegen::SUFIXO_GERADO;
                let bad = crawrad_host::rename::forbidden(&declared, &project, &roots, suffix);
                if let Some(place) = bad.first() {
                    self.nav.rename.close();
                    let name = place.file_name().map(|n| n.to_string_lossy().to_string());
                    self.status = Some(format!(
                        "`{old}` is declared in {} (generated from the form, or not the \
                         project's): rename refused, nothing changed",
                        name.unwrap_or_default()
                    ));
                    return self.draw_rename(cx);
                }
                let r = &mut self.nav.rename;
                (r.old, r.new, r.stage, r.wants_focus) = (old.clone(), old, Stage::Naming, true);
                self.status = None;
            }
            Outcome::NotRenamable => {
                self.nav.rename.close();
                self.status = Some("Nothing to rename here".into());
            }
            Outcome::NoAnswer(why) if self.nav.rename.stage == Stage::Asking => {
                (self.nav.rename.stage, self.nav.rename.message) = (Stage::Naming, why);
            }
            Outcome::NoAnswer(why) => {
                self.nav.rename.close();
                self.status = Some(format!("Rename Symbol: {why}"));
            }
            Outcome::Edits(edits) => self.preview_rename(&edits),
        }
        self.draw_rename(cx);
    }

    #[note("ide-app-rename-ideapp-preview-rename")]
    fn preview_rename(&mut self, edits: &crawrad_lsp::WorkspaceEdit) {
        let files: Vec<PathBuf> = edits
            .keys()
            .filter_map(|u| crawrad_lsp::uri_to_path(u))
            .collect();
        let project = self.host.workspace_root().unwrap_or_default();
        let roots = crawrad_host::navigation::foreign_roots();
        if let Some(msg) = crawrad_host::rename::check_refusal(
            &files,
            &project,
            &roots,
            crawrad_codegen::SUFIXO_GERADO,
        ) {
            self.nav.rename.close();
            self.status = Some(msg);
            return;
        }
        let enc = self.lsp_encoding();
        let mut read = |f: &Path| self.text_of(f);
        let planned = crawrad_lsp::rename::plan(edits, enc, &mut read)
            .map(|p| p.into_iter().map(change_of).collect());
        self.nav.rename.apply_preview(planned);
    }

    #[note("ide-app-rename-ideapp-apply-rename")]
    fn apply_rename(&mut self, cx: &mut Cx) {
        let Stage::Preview(changes) = self.nav.rename.stage.clone() else {
            return;
        };
        let stale = changes
            .iter()
            .find(|c| self.text_of(&c.file).as_deref() != Some(&c.before));
        if let Some(c) = stale {
            let name = c.file.display().to_string();
            (self.nav.rename.stage, self.nav.rename.message) = (
                Stage::Naming,
                format!("{name} changed since the preview; press Enter again"),
            );
            return self.draw_rename(cx);
        }
        self.apply_file_changes(cx, &changes);
        let r = &self.nav.rename;
        self.status = Some(format!(
            "Renamed `{}` to `{}` in {} file(s), not saved \u{2014} Save All writes them",
            r.old,
            r.new.trim(),
            changes.len()
        ));
        self.nav.rename.close();
        self.draw_rename(cx);
        self.redraw_editor_tabs(cx);
    }

    #[note("ide-app-rename-ideapp-apply-file-changes")]
    pub(super) fn apply_file_changes(&mut self, cx: &mut Cx, changes: &[FileChange]) {
        for c in changes {
            if self
                .abas
                .active_file()
                .is_some_and(|f| same_file(f, &c.file))
            {
                if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
                    cv.replace_all(cx, &c.after);
                }
                continue;
            }
            self.host
                .set_buffer_undoable(c.file.clone(), c.before.clone(), c.after.clone());
            if let Some(lsp) = &self.lsp {
                let version = OUT_OF_VIEW_VERSION.fetch_add(1, Ordering::Relaxed);
                let _ = match lsp.is_open(&c.file) {
                    true => lsp.did_change(&c.file, version, &c.after),
                    false => lsp.did_open(&c.file, "rust", version, &c.after),
                };
            }
        }
    }

    #[note("ide-app-rename-ideapp-show-text-with-undo")]
    pub(super) fn show_text_with_undo(&mut self, cx: &mut Cx, file: Option<&Path>, text: &str) {
        let view = self.code_view();
        match file.and_then(|f| self.host.take_undo_base(f)) {
            Some(before) => {
                view.set_text(cx, &before);
                if let Some(mut cv) = view.borrow_mut::<CodeView>() {
                    cv.replace_all(cx, text);
                }
            }
            None => view.set_text(cx, text),
        }
    }

    #[note("ide-app-rename-ideapp-draw-rename")]
    pub(super) fn draw_rename(&mut self, cx: &mut Cx) {
        let r = &self.nav.rename;
        let open = matches!(r.stage, Stage::Naming | Stage::Asking | Stage::Preview(_));
        let dialog = self.ui.widget(id!(rename_dialog));
        if dialog.visible() && !open {
            let cv = self.code_view();
            cx.set_key_focus(cv.area());
        }
        dialog.set_visible(cx, open);
        let dlg = self.ui.view(id!(rename_dialog.dlg));
        dlg.label(id!(rename_title)).set_text(cx, "Rename Symbol");
        dlg.label(id!(rename_what))
            .set_text(cx, &format!("Rename `{}` to", r.old));
        let (primary, hint) = match &r.stage {
            Stage::Preview(_) => ("Apply", r.preview()),
            Stage::Asking => ("Preview", "Asking rust-analyzer\u{2026}".to_string()),
            _ => (
                "Preview",
                "Enter shows what changes; nothing is applied before Apply".into(),
            ),
        };
        let note = if r.message.is_empty() {
            hint
        } else {
            r.message.clone()
        };
        dlg.label(id!(rename_preview)).set_text(cx, &note);
        dlg.button(id!(rename_ok)).set_text(cx, primary);
        if self.nav.rename.wants_focus {
            dlg.text_input(id!(rename_input))
                .set_text(cx, &self.nav.rename.new.clone());
        }
        self.ui.redraw(cx);
    }

    #[note("ide-app-rename-ideapp-focus-rename")]
    pub(super) fn focus_rename(&mut self, cx: &mut Cx) {
        if !self.nav.rename.wants_focus {
            return;
        }
        let input = self
            .ui
            .view(id!(rename_dialog.dlg))
            .text_input(id!(rename_input));
        if input.area().is_valid(cx) {
            cx.set_key_focus(input.area());
            if let Some(mut field) = input.borrow_mut() {
                field.select_all(cx);
            }
            self.nav.rename.wants_focus = false;
        } else {
            cx.new_next_frame();
        }
    }

    #[note("ide-app-rename-ideapp-atender-rename")]
    pub(super) fn atender_rename(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.nav.rename.is_open() {
            return;
        }
        let dlg = self.ui.view(id!(rename_dialog.dlg));
        let input = dlg.text_input(id!(rename_input));
        if let Some(text) = input.changed(actions) {
            self.nav.rename.typed(&text);
            self.draw_rename(cx);
        }
        let go = input.returned(actions).is_some() || dlg.button(id!(rename_ok)).clicked(actions);
        if input.escaped(actions) || dlg.button(id!(rename_cancel)).clicked(actions) {
            self.nav.rename.close();
            return self.draw_rename(cx);
        }
        if !go {
            return;
        }
        if matches!(self.nav.rename.stage, Stage::Preview(_)) {
            return self.apply_rename(cx);
        }
        if let Some(why) = self.nav.rename.cannot_ask() {
            self.nav.rename.message = why.into();
            return self.draw_rename(cx);
        }
        let Some(lsp) = self.lsp.clone() else {
            return;
        };
        let r = &mut self.nav.rename;
        let (file, caret, new) = (r.file.clone(), r.caret, r.new.trim().to_string());
        (r.stage, r.message) = (Stage::Asking, String::new());
        let ticket = r.pending.ask(file.clone(), self.lsp_versao);
        let asked = self.asked(&file, caret);
        self.lsp_hints.rename.rename(lsp, ticket, asked, &new);
        self.draw_rename(cx);
    }
}
