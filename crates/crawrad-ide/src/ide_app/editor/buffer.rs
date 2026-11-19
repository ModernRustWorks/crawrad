//! **THE EDITOR IS AN EDITOR** — what is typed, what survives, what is saved.
//!
//! # The defect this module exists to close
//!
//! The owner, with `form_main.rs` on screen:
//!
//! > *"now it's worse, NOTHING works, Ctrl+Arrows, Enter, Typing, as if
//! > it were readonly, where is the working Editor?"*
//!
//! He was reading it exactly right, and the footer was agreeing with him in
//! writing: it said `Read only`. The Makepad shell draws the code in a
//! `CodeView`, which is the framework's editor with `read_only: true` wired in
//! its own DSL (`makepad-code-editor-1.0.0/src/code_view.rs:22`), and every
//! mutating arm of `CodeEditor` asks that flag before touching the text
//! (`code_editor.rs:845`, `:874`, `:884`, `:893`, `:902`, `:912`).
//!
//! So the view was never an editor. It was a syntax-coloured viewer that
//! happened to have a blinking cursor.
//!
//! # Turning it on is one line — and the other ninety are the reason
//!
//! `read_only` is `#[live(false)]` (`code_editor.rs:250`), so `apply_over` sets
//! it at runtime. That alone would give a text box that **loses what you
//! type**, because `CodeView::set_text` throws the session away
//! (`code_view.rs:82`) and this shell rewrites the view on every tab switch.
//!
//! Three things had to come with it, and none is optional:
//!
//! 1. **The buffer survives** — `IdeApp::edicoes` keeps the text of every
//!    file with unsaved work, so `Design` and back does not erase an hour.
//! 2. **`Ctrl+S` writes the FILE** — and writes it back the way it was found,
//!    mark and line endings included ([`crate::support::encoding::Encoding::to_bytes`]).
//! 3. **What cannot be edited says so** — see [`pode_editar`].
//!
//! # What is NOT editable, and why that is not a gap
//!
//! | in front | editable? | because |
//! |---|---|---|
//! | the canvas | — | there is no text |
//! | `Alt+F12`, the form source | **no** | it is PLANNED each time it is shown, from the document. Typing in it would be typing into something the next drag regenerates |
//! | a unit that was never generated | **no** | what is on screen is a skeleton, not a file. Creating it is `Generate`'s job, and two creators would race |
//! | any `.rs` on disk | **yes** | |

use std::path::{Path, PathBuf};

use super::*;
use {super::source::Center, crate::ide_app::tema::rgba_to_vec4};

/// **The file the editor may edit right now** — `None` is a read-only view.
///
/// Pure on purpose: the rule is the module header's table, worth a test; the
/// one fact asked of the world is whether the path is a file.
pub(super) fn pode_editar(centro: Center, aberto: Option<&Path>, existe: bool) -> Option<PathBuf> {
    if centro != Center::Code {
        return None;
    }
    let p = aberto?;
    // std, downloaded crates and the generated form source are not typed into.
    let locked = crawrad_host::navigation::is_locked(p, crawrad_codegen::SUFIXO_GERADO);
    (existe && !locked).then(|| p.to_path_buf())
}

impl IdeApp {
    /// [`pode_editar`], asking the disk.
    pub(crate) fn editable_file(&self) -> Option<PathBuf> {
        let aberto = self.abas.active_file();
        pode_editar(
            self.host.center(),
            aberto,
            aberto.is_some_and(Path::is_file),
        )
    }

    /// Hands the read-only flag to the framework's editor.
    ///
    /// `apply_over` and not a DSL constant because the answer CHANGES while the
    /// IDE runs: the same widget shows the generated form source — which must
    /// not be typed into — and the unit next to it, which must.
    pub(super) fn aplicar_somente_leitura(&mut self, cx: &mut Cx) {
        let so_leitura = self.editavel.is_none() || self.review_view().is_some();
        self.code_view()
            .apply_over(cx, live! { editor: { read_only: (so_leitura) } });
    }

    /// Applies active color scheme and visual settings to the Makepad `CodeEditor`.
    pub(crate) fn aplicar_tema_editor(&mut self, cx: &mut Cx) {
        let pref = crawrad_project::workspace::read_preferences();
        let c: crate::ide_app::tema::EditorTokenColorsVec = pref.editor_colors().into();
        let (show_gutter, current_line) = (pref.editor_line_numbers, pref.editor_current_line);
        let d = pref.editor_diagnostic_colors.rgba().map(rgba_to_vec4);
        self.bulb().set_dark(cx, pref.is_dark());

        let vista = self.code_view();
        vista.apply_over(
            cx,
            live! {
                editor: {
                    show_gutter: (show_gutter),
                    highlight_current_line: (current_line),
                    draw_bg: { color: (c.bg) },
                    token_colors: {
                        branch_keyword: (c.branch_keyword),
                        loop_keyword: (c.loop_keyword),
                        other_keyword: (c.other_keyword),
                        typename: (c.typename),
                        function: (c.function),
                        identifier: (c.identifier),
                        comment: (c.comment),
                        number: (c.number),
                        string: (c.string),
                        constant: (c.constant),
                        delimiter: (c.delimiter),
                        delimiter_highlight: (c.delimiter_highlight),
                        punctuator: (c.punctuator),
                    }
                    diagnostic_colors: {
                        error: (d[0]),
                        warning: (d[1]),
                        info: (d[2]),
                        error_band: (d[3]),
                        warning_band: (d[4]),
                        info_band: (d[5]),
                    }
                }
            },
        );
        vista.redraw(cx);
    }

    /// The text the editor is holding right now — its own, not our copy.
    ///
    /// `None` while there is no session: the view builds one lazily on the
    /// first draw (`code_view.rs:40-52`), so the frame right after a tab switch
    /// legitimately has none.
    pub(crate) fn texto_do_code_view(&self) -> Option<String> {
        self.with_code_view(|cv| cv.live_text()).flatten()
    }

    /// `f` on the code editor the dock holds now; `None` before it is made.
    pub(crate) fn with_code_view<R>(
        &self,
        f: impl FnOnce(&makepad_code_editor::code_view::CodeView) -> R,
    ) -> Option<R> {
        let view = self.code_view();
        let cv = view.borrow::<makepad_code_editor::code_view::CodeView>()?;
        Some(f(&cv))
    }

    /// **Reads back what was typed, and marks the tab.**
    ///
    /// # Why it POLLS instead of listening
    ///
    /// Because there is nothing to listen to. `CodeEditor` does emit
    /// `CodeEditorAction::TextDidChange`, and `CodeView` **drops every one of
    /// them**: `for _action in self.editor.handle_event(...)`
    /// (`code_view.rs:67`) — the loop variable is discarded and no widget
    /// action is posted. From outside the widget the edit is invisible.
    ///
    /// The cost is a `join` over the lines once per sync, and it is paid only
    /// while a writable file is in front. The honest limit: on a very large
    /// file that is a copy of the whole text per sync, and the fix is a widget
    /// of ours in place of `CodeView` — which is also what would give back
    /// per-file undo history.
    pub(super) fn colher_do_editor(&mut self, cx: &mut Cx) {
        let Some(arquivo) = self.editavel.clone().filter(|_| !self.remake_editor) else {
            return;
        };
        if self.reviewing(&arquivo) {
            return;
        }
        let Some(texto) = self.texto_do_code_view() else {
            return;
        };
        if texto == self.texto_na_vista {
            return self.signature_follows_caret(cx);
        }
        let era_sujo = self.host.buffer(&arquivo).is_some();
        // Back to the disk text is clean: undo to the start clears the tab's mark.
        if texto == self.base_na_vista {
            self.host.remove_buffer(&arquivo);
        } else {
            self.host.set_buffer(arquivo.clone(), texto.clone());
        }
        self.texto_na_vista = texto.clone();
        self.refresh_change_bars(cx);
        self.auto_save_typed(cx);
        self.lsp_notificar_mudanca(&arquivo, &texto);
        self.completar_acompanha_digitacao(cx);
        self.signature_follows_typing(cx);
        // The strip redraws only when the mark flips, not one frame per key.
        if era_sujo != self.host.buffer(&arquivo).is_some() {
            self.redraw_editor_tabs(cx);
        }
    }

    /// A whole new text for the open editor (replace, a note's attribute or
    /// anchor): the buffer of the editable file, the view and the tab mark.
    pub(in super::super) fn show_edited_source(&mut self, cx: &mut Cx, text: String) {
        if let Some(arq) = self.editavel.clone() {
            self.host.set_buffer(arq, text.clone());
        }
        self.code_view().set_text(cx, &text);
        self.texto_na_vista = text;
        self.refresh_change_bars(cx);
        self.redraw_editor_tabs(cx);
    }

    /// Anything typed and not saved, anywhere: `Ctrl+S` has work even with no
    /// form open (the Explorer opens any `.rs`; ADR 0017 allows no form at all).
    pub(super) fn editor_sujo(&self) -> bool {
        self.host.is_dirty()
    }

    pub fn is_editor_dirty(&self) -> bool {
        self.editor_sujo()
    }

    /// The code editor's selected text, as Copy takes it; `None` without one.
    pub fn editor_selection_text(&self) -> Option<String> {
        self.with_code_view(|cv| cv.copy_selection()).flatten()
    }

    /// The first selection's ends as `(line, byte)`, 0-based; `None` without one.
    pub fn editor_selection_bounds(&self) -> Option<[(usize, usize); 2]> {
        let view = self.code_view();
        let cv = view.borrow::<makepad_code_editor::code_view::CodeView>()?;
        let sel = *cv.session.as_ref()?.selections().first()?;
        let ends = [sel.start(), sel.end()].map(|p| (p.line_index, p.byte_index));
        (!sel.is_empty()).then_some(ends)
    }

    /// The editor's text side for the catalog. The clipboard is read
    /// only with the code in front, when a menu or an action asks.
    pub fn editor_text(&self) -> crawrad_actions::EditorText {
        let code_in_front = self.host.center().is_text();
        let form_unit = self.host.active_file().map(super::source::unit_path);
        crawrad_actions::EditorText::for_view(
            code_in_front,
            code_in_front && self.with_code_view(|c| c.has_selection()) == Some(true),
            code_in_front && crawrad_os::clipboard::read_text().is_some(),
            self.abas.active_file() == form_unit.as_deref(),
        )
    }

    /// Cut, Copy, Paste and Select All from the catalog (the menu): the same
    /// editor operations the system's clipboard events reach by key.
    pub(super) fn editor_clipboard(
        &mut self,
        cx: &mut Cx,
        command: crawrad_actions::EditorCommand,
    ) {
        use crawrad_actions::EditorCommand::{Copy, Cut, Paste};
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<makepad_code_editor::code_view::CodeView>() else {
            return;
        };
        match command {
            Copy => cv
                .copy_selection()
                .into_iter()
                .for_each(|t| cx.copy_to_clipboard(&t)),
            Cut => cv
                .cut_selection(cx)
                .into_iter()
                .for_each(|t| cx.copy_to_clipboard(&t)),
            Paste => match crawrad_os::clipboard::read_text() {
                Some(text) => cv.paste_text(cx, &text),
                None => self.status = Some("The clipboard has no text".into()),
            },
            _ => cv.select_all(cx),
        }
    }

    /// The editor's context menu at `at` (window pixels), when the code is
    /// in front and `at` is on the editor. The caret was already moved
    /// by the editor's own right-click (outside the selection only).
    pub(super) fn open_editor_menu(&mut self, cx: &mut Cx, at: DVec2) -> bool {
        let view = self.code_view();
        if !self.host.center().is_text() || !view.area().rect(cx).contains(at) {
            return false;
        }
        self.open_menu = Some(super::Aberto::EditorContext { em: at });
        self.show_menu(cx);
        true
    }

    /// `Shift+F10`: the same menu at the caret, under its line.
    pub(super) fn open_editor_menu_at_caret(&mut self, cx: &mut Cx) {
        if let Some(a) = self.with_code_view(|c| c.caret_anchor()).flatten() {
            let at = dvec2(a.at.x, a.at.y + a.line_height);
            self.open_menu = Some(super::Aberto::EditorContext { em: at });
            self.show_menu(cx);
        }
    }

    pub fn get_editor_buffer(&self, path: &std::path::Path) -> Option<String> {
        self.host.buffer(path).map(|s| s.to_owned())
    }

    pub fn get_editor_buffer_by_key(&self, ckey: &str) -> Option<String> {
        self.host.buffer_by_key(ckey).map(|s| s.to_owned())
    }

    pub(super) fn redraw_editor_tabs(&mut self, cx: &mut Cx) {
        self.painel(Panel::Design)
            .portal_list(id!(editor_tabs_bar.editor_tabs))
            .redraw(cx);
    }

    /// **`Ctrl+S` over the editor writes the file, and stops there.**
    ///
    /// Answers `true` when it took the save, so [`IdeApp::salvar`] leaves the
    /// form alone. It has to: saving a source file must not run the generator.
    /// `Generate` rewrites `*_designer.rs` and prunes handlers (ADR 0025), and
    /// a user who pressed `Ctrl+S` after typing a line asked for one file to be
    /// written — not for the project to be rebuilt under them.
    /// Only a review pending on THIS file stops it.
    pub(super) fn save_editor_file(&mut self, cx: &mut Cx) -> bool {
        let Some(arquivo) = self.editavel.clone() else {
            return false;
        };
        if self.has_pending_diff_review(&arquivo) {
            self.save_waits_for_review();
            return true;
        }
        let Some(texto) = self.host.buffer(&arquivo).map(|s| s.to_owned()) else {
            return false;
        };
        let bytes = self.encoding_na_vista.to_bytes(&texto);
        match std::fs::write(&arquivo, bytes) {
            Ok(()) => {
                self.host.remove_buffer(&arquivo);
                self.base_na_vista = texto.clone();
                self.disk.saved(&arquivo, texto);
                self.refresh_change_bars(cx);
                self.update_disk_change_bar(cx);
                self.status = Some(format!("saved {}", arquivo.display()));
                self.lsp_notificar_salvamento(&arquivo);
            }
            Err(e) => {
                // THE MARK STAYS. The file was not written, and erasing the tab's
                // dot would make the screen say it was.
                self.status = Some(format!("save failed: {e}"));
            }
        }
        self.redraw_editor_tabs(cx);
        true
    }
}

#[cfg(test)]
#[path = "../../test/ide_app/editor_buffer.rs"]
mod test;
