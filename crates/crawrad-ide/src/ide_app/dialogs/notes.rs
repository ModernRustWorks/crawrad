//! The note dialog wiring (Sidecar Code Notes & Documentation Notes).
//!
//! What the dialog shows is decided by `NoteDialogModel::view`; this file
//! only paints it and turns clicks into model calls.

use super::super::*;
use crawrad_actions::NoteKindChoice;
use crawrad_note::note;
use crawrad_project::doc_note::Section;
use crawrad_project::note_dialog::{NoteDialogModel, NoteOpen, Tone};
use std::path::{Path, PathBuf};

const SECTIONS: &[(LiveId, Section)] = &[
    (live_id!(note_sec_summary), Section::Summary),
    (live_id!(note_sec_examples), Section::Examples),
    (live_id!(note_sec_errors), Section::Errors),
    (live_id!(note_sec_panics), Section::Panics),
    (live_id!(note_sec_safety), Section::Safety),
];

fn tone(t: Tone) -> Vec4 {
    Vec4::from_u32(t.rgba())
}

impl IdeApp {
    #[note("ide-app-dialogs-notes-ideapp-open-note-by-id")]
    pub(in super::super) fn open_note_by_id(&mut self, cx: &mut Cx, id: &str) {
        let file = self.active_file_path_str().unwrap_or_default();
        self.nota = Some(NoteDialogModel::from_id(id, &file, &self.raiz_do_projeto()));
        self.nota_dialog_aberta = true;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-notes-ideapp-raiz-do-projeto")]
    pub(in super::super) fn raiz_do_projeto(&self) -> PathBuf {
        self.alvo_do_cargo()
            .map(|(r, _)| r)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    #[note("ide-app-dialogs-notes-ideapp-active-file-path-str")]
    pub(in super::super) fn active_file_path_str(&self) -> Option<String> {
        self.abas
            .active_file()
            .map(|p| p.to_string_lossy().to_string())
    }

    #[note("ide-app-dialogs-notes-ideapp-draw-note")]
    pub(in super::super) fn draw_note(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(note_dialog), self.nota_dialog_aberta);
        if !self.nota_dialog_aberta {
            return;
        }
        let Some(ref model) = self.nota else {
            return;
        };
        let v = model.view(&self.texto_na_vista, self.doctest.is_running());
        let dlg = self.ui.view(id!(note_dialog.dlg));

        dlg.widget(id!(note_icon))
            .apply_over(cx, live! { kind: (v.icon()) });
        for (name, text, visible, t) in v.labels() {
            let label = dlg.label(&[LiveId::from_str(name)]);
            if let Some(t) = t {
                label.apply_over(cx, live! { draw_text: { color: (tone(t)) } });
            }
            // A label has no visibility of its own: hidden is empty.
            label.set_text(cx, if visible { text } else { "" });
        }
        for (name, visible) in v.boxes() {
            dlg.view(&[LiveId::from_str(name)]).set_visible(cx, visible);
        }
        if v.is_doc {
            for (id, sec) in SECTIONS {
                let t = tone(v.section_tone(sec));
                dlg.button(&[*id])
                    .apply_over(cx, live! { draw_text: { color: (t) } });
            }
        }

        // Edit or Preview; the editor hides by its box (a TextInput draws anyway).
        let preview = v.show_tabs && v.preview.is_some();
        dlg.view(id!(note_editor)).set_visible(cx, !preview);
        dlg.view(id!(note_preview)).set_visible(cx, preview);
        dlg.widget(id!(note_preview_md))
            .set_text(cx, v.preview.as_deref().unwrap_or(""));
        for (id, active) in [
            (id!(note_tab_edit), !preview),
            (id!(note_tab_preview), preview),
        ] {
            let t = tone(if active { Tone::Plain } else { Tone::Muted });
            dlg.button(id)
                .apply_over(cx, live! { draw_text: { color: (t) } });
        }
        for (name, text) in v.buttons() {
            let button = dlg.button(&[LiveId::from_str(name)]);
            button.set_visible(cx, text.is_some());
            button.set_text(cx, text.unwrap_or(""));
        }

        for (id, val) in [
            (id!(note_title_input), &model.title),
            (id!(note_tag_input), &model.tags),
            (id!(note_content_input), &model.content),
        ] {
            let input = dlg.text_input(id);
            if input.text() != *val {
                input.set_text(cx, val);
            }
        }
    }

    #[note("ide-app-dialogs-notes-ideapp-atender-nota")]
    pub(in super::super) fn atender_nota(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.nota_dialog_aberta {
            return;
        }
        let dlg = self.ui.view(id!(note_dialog.dlg));

        if let Some(ref mut m) = self.nota {
            for (id, target, trim) in [
                (id!(note_title_input), &mut m.title, true),
                (id!(note_tag_input), &mut m.tags, true),
                (id!(note_content_input), &mut m.content, false),
            ] {
                if let Some((t, _)) = dlg.text_input(id).returned(actions) {
                    *target = if trim { t.trim().to_string() } else { t };
                }
            }
            let content = dlg.text_input(id!(note_content_input));
            let mut changed = content.changed(actions).map(|t| m.typed(t)).is_some();
            for (id, preview) in [(id!(note_tab_edit), false), (id!(note_tab_preview), true)] {
                if dlg.button(id).clicked(actions) {
                    // The preview shows what was typed, Enter or not.
                    m.typed(content.text());
                    m.preview_tab = preview;
                    changed = true;
                }
            }
            if dlg.button(id!(note_resolve)).clicked(actions) {
                m.toggle_resolved();
                changed = true;
            }
            if dlg.button(id!(note_sec_link)).clicked(actions) {
                m.typed(content.text());
                self.nav
                    .links
                    .open(content.cursor().index, &self.raiz_do_projeto());
                return self.open_link_pick(cx);
            }
            for (btn_id, sec) in SECTIONS {
                if dlg.button(&[*btn_id]).clicked(actions) {
                    m.typed(content.text());
                    m.insert(*sec);
                    changed = true;
                }
            }
            if changed {
                self.refresh(cx);
                return;
            }
        }

        if dlg.button(id!(note_cancel)).clicked(actions) {
            self.nota_dialog_aberta = false;
        } else if dlg.button(id!(note_save)).clicked(actions) {
            self.save_note(cx);
        } else if dlg.button(id!(note_test)).clicked(actions) {
            // rustdoc reads the note from disk, so the example runs as saved.
            self.save_note(cx);
            if let Some(m) = self.nota.as_mut() {
                self.doctest.start(m);
            }
        } else {
            return;
        }
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-notes-ideapp-save-note")]
    fn save_note(&mut self, cx: &mut Cx) {
        let dlg = self.ui.view(id!(note_dialog.dlg));
        let save_result = self.nota.as_mut().map(|m| {
            m.title = dlg.text_input(id!(note_title_input)).text();
            m.tags = dlg.text_input(id!(note_tag_input)).text();
            m.typed(dlg.text_input(id!(note_content_input)).text());
            m.save(&self.texto_na_vista)
        });
        if let Some(Ok(new_source)) = save_result {
            // The attribute already in place gives the same text: no edit.
            if let Some(text) = new_source.filter(|t| *t != self.texto_na_vista) {
                self.show_edited_source(cx, text);
            }
            self.status = Some("Note saved successfully".to_string());
        }
        self.refresh_notes(cx);
    }

    #[note("ide-app-dialogs-notes-ideapp-pump-doctest")]
    pub(in super::super) fn pump_doctest(&mut self, cx: &mut Cx) {
        if self.doctest.poll(self.nota.as_mut()) {
            self.draw_note(cx);
            self.ui.redraw(cx);
        }
    }

    #[note("ide-app-dialogs-notes-ideapp-source-of")]
    fn source_of(&self, file: &Path) -> Option<String> {
        let open = self.abas.active_file().and_then(|f| f.canonicalize().ok());
        match open == file.canonicalize().ok() {
            true => Some(self.texto_na_vista.clone()),
            false => std::fs::read_to_string(file).ok(),
        }
    }

    #[note("ide-app-dialogs-notes-ideapp-note-defined-at")]
    pub(in super::super) fn note_defined_at(
        &self,
        t: crawrad_lsp::definition::Target,
    ) -> Option<(String, PathBuf, usize)> {
        let text = self.source_of(&t.file)?;
        let id = crawrad_project::note_preview::note_at_definition(&text, t.line)?;
        Some((id, t.file, t.line))
    }

    #[note("ide-app-dialogs-notes-ideapp-paint-note-card")]
    pub(in super::super) fn paint_note_card(
        &mut self,
        cx: &mut Cx,
        card: &ViewRef,
        id: &str,
        markdown: &str,
        line_height: f64,
    ) -> f64 {
        let md = crawrad_project::note_preview::demote_headings(markdown);
        card.widget(id!(hover_md)).set_text(cx, &md);
        card.label(id!(hover_note_kind))
            .set_text(cx, &format!("Documentation note \u{b7} {id}"));
        card.button(id!(hover_open_note))
            .set_text(cx, "Open note \u{b7} Ctrl+Shift+N");
        let body = crawrad_host::hover_card::markdown_height(&md, line_height, 64, 300.0);
        card.view(id!(hover_rich))
            .apply_over(cx, live! { height: (body) });
        body + line_height + 24.0
    }

    #[note("ide-app-dialogs-notes-ideapp-hover-note-clicked")]
    pub(in super::super) fn hover_note_clicked(&self, actions: &Actions) -> bool {
        let card = self.hover_box();
        card.button(id!(hover_open_note)).clicked(actions)
    }

    #[note("ide-app-dialogs-notes-ideapp-open-hover-note")]
    pub(in super::super) fn open_hover_note(&mut self, cx: &mut Cx) -> bool {
        let Some((_, file, line)) = self.nav.hover.note().cloned() else {
            return false;
        };
        self.close_hover(cx);
        self.open_note_dialog_at(cx, &file, line)
    }

    #[note("ide-app-dialogs-notes-ideapp-open-note-dialog-at")]
    pub(in super::super) fn open_note_dialog_at(
        &mut self,
        cx: &mut Cx,
        file: &Path,
        line: usize,
    ) -> bool {
        let Some(text) = self.source_of(file) else {
            return false;
        };
        let (path, root) = (file.to_string_lossy(), self.raiz_do_projeto());
        self.nota = Some(NoteDialogModel::from_caret(&text, line, &path, &root).model);
        self.nota_dialog_aberta = true;
        self.refresh(cx);
        true
    }

    #[note("ide-app-dialogs-notes-ideapp-open-note-cursor")]
    pub(in super::super) fn open_note_cursor(&mut self, cx: &mut Cx, kind: Option<NoteKindChoice>) {
        let raiz = self.raiz_do_projeto();
        let file = self
            .active_file_path_str()
            .unwrap_or_else(|| "source.rs".to_string());
        let in_text = self.host.center().is_text();
        let caret = self.cursor_do_code_view(cx).0;
        let line = if in_text { caret } else { 0 };
        let open = match NoteDialogModel::at_caret(&self.texto_na_vista, line, &file, &raiz, kind) {
            NoteOpen::Dialog(open) => open,
            NoteOpen::Popup(header, rows) => {
                let view = self.code_view();
                if let Some(mut cv) = view.borrow_mut::<makepad_code_editor::code_view::CodeView>()
                {
                    cv.open_note_popup(header, rows);
                }
                return view.redraw(cx);
            }
        };
        if let Some((_, novo_texto)) = open.insert_anchor {
            self.show_edited_source(cx, novo_texto);
        }
        self.nota = Some(open.model);
        self.nota_dialog_aberta = true;
        self.refresh(cx);
    }
}
