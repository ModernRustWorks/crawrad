//! Language Server Protocol (LSP) integration service for the CrawRAD code editor.
//!
//! Connects in the background to `rust-analyzer` to provide real-time diagnostics,
//! compiler squiggles, semantic autocomplete, and status notifications.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use makepad_widgets::makepad_platform::thread::SignalToUI;

use crawrad_actions::Panel;
use crawrad_host::navigation::Place;
use crawrad_insight::lsp_cell::{self, LspCell, LspPhase, SERVER};
use crawrad_lsp::LspClient;
use makepad_code_editor::code_view::CodeView;
use makepad_code_editor::decoration::{Decoration, DecorationType};
use makepad_code_editor::text::Position as MakepadPos;
use makepad_widgets::*;

use super::IdeApp;
use crawrad_note::note;

#[note("ide-app-lsp-service-room")]
const ROOM: f64 = 230.0 - 12.0;

impl IdeApp {
    #[note("ide-app-lsp-service-ideapp-lsp-encoding")]
    pub(super) fn lsp_encoding(&self) -> crawrad_lsp::encoding::PositionEncoding {
        self.lsp
            .as_ref()
            .map(|l| l.position_encoding())
            .unwrap_or_default()
    }

    #[note("ide-app-lsp-service-ideapp-form-name")]
    pub(super) fn form_name(&self) -> String {
        let doc = self.host.doc();
        doc.map(|d| crawrad_host::document::nome_de(d.ir(), &d.ir().root))
            .unwrap_or_default()
    }

    #[note("ide-app-lsp-service-ideapp-garantir-lsp")]
    pub fn garantir_lsp(&mut self) {
        if let Some(ref lsp) = self.lsp
            && lsp.is_alive()
        {
            return;
        }

        // Determine workspace root or parent of current file
        let raiz = self
            .raiz()
            .or_else(|| {
                self.host
                    .active_file()
                    .and_then(|p| p.parent().map(Path::to_path_buf))
            })
            .unwrap_or_else(|| PathBuf::from("."));

        if LspClient::find_rust_analyzer().is_none() {
            self.lsp_cell = LspCell::down(SERVER, "offline");
            return;
        }

        self.lsp_cell = LspCell::starting(SERVER);
        match LspClient::spawn(None, &raiz) {
            Ok(client) => {
                let arc = Arc::new(client);
                self.lsp = Some(Arc::clone(&arc));
                self.lsp_hints.set_waker(SignalToUI::set_ui_signal);
                self.inlay_hints_off =
                    !crawrad_project::workspace::read_preferences().editor_inlay_hints;
                arc.set_waker(SignalToUI::set_ui_signal);
                self.lsp_cell = LspCell::running(arc.status_name(), None, None);

                let texto_opt = self.texto_do_code_view();
                if let (Some(arq), Some(texto)) = (self.editavel.as_ref(), texto_opt.as_deref()) {
                    let _ = arc.did_open(arq, "rust", self.lsp_versao, texto);
                }
            }
            Err(e) => {
                self.lsp_cell = LspCell::down(SERVER, &e.to_string());
            }
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-notificar-abertura")]
    pub fn lsp_notificar_abertura(&mut self, arquivo: &Path, texto: &str) {
        if arquivo.extension().and_then(|e| e.to_str()) != Some("rs") {
            return;
        }
        self.garantir_lsp();
        if let Some(ref lsp) = self.lsp {
            self.lsp_versao = 1;
            let _ = lsp.did_open(arquivo, "rust", self.lsp_versao, texto);
            self.ask_semantic(arquivo);
            if !self.inlay_hints_off {
                self.lsp_hints
                    .hints
                    .request(Arc::clone(lsp), arquivo, self.lsp_versao, texto);
            }
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-notificar-mudanca")]
    pub fn lsp_notificar_mudanca(&mut self, arquivo: &Path, texto: &str) {
        if arquivo.extension().and_then(|e| e.to_str()) != Some("rs") {
            return;
        }
        if let Some(ref lsp) = self.lsp {
            self.lsp_versao += 1;
            let _ = lsp.did_change(arquivo, self.lsp_versao, texto);
            self.ask_semantic(arquivo);
            if !self.inlay_hints_off {
                self.lsp_hints
                    .hints
                    .request(Arc::clone(lsp), arquivo, self.lsp_versao, texto);
            }
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-notificar-salvamento")]
    pub fn lsp_notificar_salvamento(&mut self, arquivo: &Path) {
        if arquivo.extension().and_then(|e| e.to_str()) != Some("rs") {
            return;
        }
        if let Some(ref lsp) = self.lsp {
            let _ = lsp.did_save(arquivo);
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-reload-workspace")]
    pub(super) fn lsp_reload_workspace(&self) {
        if let Some(lsp) = &self.lsp {
            let _ = lsp.reload_workspace();
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-apply-hints")]
    pub fn lsp_apply_hints(&mut self, cx: &mut Cx) {
        self.receber_completar_do_lsp(cx);
        self.give_bookmarks_to_view(cx);
        self.serve_notes(cx);
        let said = self.status.clone();
        self.lsp_poll_definition(cx);
        self.poll_links(cx);
        self.serve_hover(cx);
        self.poll_signature(cx);
        self.follow_code_actions(cx);
        self.follow_item_doc(cx);
        self.poll_semantic(cx);
        self.follow_line_marks(cx);
        self.follow_unsaved(cx);
        self.follow_extension_views(cx);
        self.focus_quick_open(cx);
        self.focus_inherit_form(cx);
        self.poll_format(cx);
        self.poll_references(cx);
        self.follow_code_lenses(cx);
        self.poll_rename(cx);
        self.poll_component_rename(cx);
        self.focus_rename(cx);
        self.follow_agent_install(cx);
        self.follow_live_agent(cx);
        if self.status != said {
            self.sincronizar(cx); // an answer's sentence reaches the status bar now
        }
        let generated = self
            .membro()
            .is_none_or(|m| crawrad_project::generator::has_a_target(&m));
        let progress = self.lsp.as_ref().and_then(|l| l.progress());
        let client_info = self
            .lsp
            .as_ref()
            .map(|l| (l.status_name(), l.is_alive(), progress.as_deref()));
        let idle_note = (!generated).then_some(crawrad_project::generator::NOT_GENERATED);
        let cell = LspCell::for_client_status(&self.lsp_cell, client_info, idle_note);
        if cell != self.lsp_shown.0 {
            // Written now: an idle IDE has no other event to repaint the bar.
            self.draw_lsp_cell(cx, cell);
        }
        let (Some(lsp), Some(arq)) = (self.lsp.clone(), self.editavel.clone()) else {
            return;
        };
        self.lsp_apply_occurrences(cx, &lsp, &arq);
        if self.inlay_hints_off {
            return;
        }
        let Some(hints) =
            self.lsp_hints
                .hints
                .poll(lsp, &arq, self.lsp_versao, &self.texto_na_vista)
        else {
            return;
        };
        if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
            cv.set_inline_hints(cx, &hints);
        };
    }

    #[note("ide-app-lsp-service-ideapp-lsp-apply-occurrences")]
    fn lsp_apply_occurrences(
        &mut self,
        cx: &mut Cx,
        lsp: &Arc<crawrad_lsp::LspClient>,
        file: &Path,
    ) {
        let view = self.code_view();
        let Some(caret) = view.borrow::<CodeView>().and_then(|cv| cv.caret()) else {
            return;
        };
        let worker = &self.lsp_hints.occurrences;
        let text = &self.texto_na_vista;
        worker.request(Arc::clone(lsp), file, self.lsp_versao, text, caret);
        if let Some(found) = worker.poll(file, self.lsp_versao, text, caret) {
            let found: Vec<_> = found.iter().map(crawrad_lsp::Occurrence::parts).collect();
            if let Some(mut cv) = view.borrow_mut::<CodeView>() {
                cv.set_occurrences(cx, &found);
            }
        }
    }

    #[note("ide-app-lsp-service-ideapp-serve-notes")]
    fn serve_notes(&mut self, cx: &mut Cx) {
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        let root = self.raiz_do_projeto();
        for (id, line) in cv.take_missing_notes() {
            let mark = crawrad_project::doc_note::note_mark(&root, &id, &self.texto_na_vista, line);
            cv.set_note_mark(&id, mark);
        }
        let clicked = cv.take_note_click();
        let jump = cv.take_ctrl_click();
        let popup_choice = cv.take_note_popup_choice();
        drop(cv);
        if let Some(id) = clicked {
            self.open_note_by_id(cx, &id);
        }
        if let Some(c) = popup_choice {
            self.open_note_cursor(cx, Some(crawrad_actions::NoteKindChoice::ALL[c]));
        }
        if jump {
            // The same action as the menu, the keys and MCP.
            self.execute_action(cx, "editor.goto_definition");
        }
    }

    #[note("ide-app-lsp-service-ideapp-give-bookmarks-to-view")]
    fn give_bookmarks_to_view(&mut self, cx: &mut Cx) {
        let Some(file) = self.editavel.clone() else {
            return;
        };
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        if cv.bookmarks_owner() != Some(file.as_path()) {
            let marks = self.host.bookmarks().of(&file);
            cv.set_bookmarks(cx, &file, &marks);
        }
    }

    #[note("ide-app-lsp-service-ideapp-editor-command")]
    pub(super) fn editor_command(
        &mut self,
        cx: &mut Cx,
        command: Option<crawrad_actions::EditorCommand>,
    ) {
        use crawrad_actions::EditorCommand;
        let go = |this: &mut Self, cx: &mut Cx, line: usize, column: usize| {
            this.pousar_cursor(cx, &super::PousoCodigo::cursor(line, column));
            // A caret set from here moves no key: the footer is told.
            this.escrever_rodape_do_editor(cx);
        };
        match command {
            Some(EditorCommand::GoTo { line, column }) => {
                go(self, cx, line as usize, column as usize)
            }
            Some(EditorCommand::ToggleInlayHints) => self.set_inlay_hints(cx, self.inlay_hints_off),
            Some(EditorCommand::Complete) => self.disparar_autocomplete(cx),
            Some(EditorCommand::Templates) => self.open_templates(cx),
            Some(EditorCommand::GoToDefinition) => self.goto_definition(cx),
            Some(EditorCommand::NavigateBack) => self.navigate(cx, true),
            Some(EditorCommand::NavigateForward) => self.navigate(cx, false),
            Some(EditorCommand::ShowHover) => self.hover_at_caret(cx),
            Some(EditorCommand::ParameterHints) => self.signature_at_caret(cx),
            Some(EditorCommand::CodeActions) => self.code_actions_at_caret(cx),
            Some(EditorCommand::ApplyCodeAction(i)) => self.apply_code_action(cx, i),
            Some(EditorCommand::FormatDocument) => self.format_document(),
            Some(EditorCommand::FindReferences) => self.find_references(cx),
            Some(EditorCommand::RenameSymbol) => self.rename_symbol(cx),
            Some(EditorCommand::ContextMenu) => self.open_editor_menu_at_caret(cx),
            Some(EditorCommand::FixExplain) => {
                self.execute_ai_prompt(cx, "/fix-explain");
                self.show_panel(cx, Panel::Chat);
                let pr = self
                    .painel(Panel::Chat)
                    .widget(id!(ai_prompt_card.prompt_input));
                if pr.area() != Area::Empty {
                    cx.set_key_focus(pr.area());
                }
            }
            Some(EditorCommand::AddFileToChat) => self.add_file_to_chat(cx),
            Some(EditorCommand::AddSelectionToChat) => self.add_selection_to_chat(cx),
            Some(EditorCommand::ExplainSelection) => self.ask_about_selection(cx, "/explain"),
            Some(EditorCommand::ReviewSelection) => self.ask_about_selection(cx, "/review"),
            Some(EditorCommand::KeepMine) => self.keep_mine(cx),
            Some(EditorCommand::ReloadFromDisk) => self.reload_from_disk(cx),
            Some(EditorCommand::CompareWithDisk) => self.compare_with_disk(cx),
            Some(
                c @ (EditorCommand::Cut
                | EditorCommand::Copy
                | EditorCommand::Paste
                | EditorCommand::SelectAll),
            ) => self.editor_clipboard(cx, c),
            Some(EditorCommand::ToggleBookmark(n)) => {
                let Some(file) = self.editavel.clone() else {
                    return;
                };
                let view = self.code_view();
                let Some(mut cv) = view.borrow_mut::<CodeView>() else {
                    return;
                };
                if cv.bookmarks_owner() != Some(file.as_path()) {
                    let marks = self.host.bookmarks().of(&file);
                    cv.set_bookmarks(cx, &file, &marks);
                }
                let on = cv.toggle_bookmark_at_cursor(cx, n);
                self.host.bookmarks_mut().set(&file, &cv.bookmarks());
                drop(cv);
                self.status = Some(match on {
                    Some(true) => format!("bookmark {n} set"),
                    Some(false) => format!("bookmark {n} removed"),
                    None => "a note's line takes no other mark".to_string(),
                });
            }
            Some(EditorCommand::GoToBookmark(n)) => {
                let Some(file) = self.editavel.clone() else {
                    return;
                };
                let view = self.code_view();
                let in_view = view
                    .borrow::<CodeView>()
                    .filter(|cv| cv.bookmarks_owner() == Some(file.as_path()))
                    .and_then(|cv| cv.bookmark_line(n));
                match in_view.or_else(|| self.host.bookmarks().line_of(&file, n)) {
                    Some(line) => go(self, cx, line + 1, 1),
                    None => self.status = Some(format!("no bookmark {n} in this file")),
                }
            }
            None => {}
        }
    }

    #[note("ide-app-lsp-service-ideapp-here")]
    pub(super) fn here(&self, cx: &Cx) -> Option<Place> {
        let file = self.abas.active_file()?.to_path_buf();
        let (line, byte) = self.cursor_do_code_view(cx);
        Some(Place { file, line, byte })
    }

    #[note("ide-app-lsp-service-ideapp-goto-definition")]
    fn goto_definition(&mut self, cx: &mut Cx) {
        let (Some(lsp), Some(from)) = (self.lsp.clone(), self.here(cx)) else {
            self.status = Some("Go to Definition needs rust-analyzer and a file".into());
            return;
        };
        let version = self.lsp_versao;
        let ticket = self.nav.jumps.ask(from.clone());
        let (text, caret) = (&self.texto_na_vista, (from.line, from.byte));
        let defs = &self.lsp_hints.definition;
        defs.request(lsp, ticket, &from.file, version, text, caret);
        self.status = Some("Looking for the definition\u{2026}".into());
    }

    #[note("ide-app-lsp-service-ideapp-poll-links")]
    fn poll_links(&mut self, cx: &mut Cx) {
        let symbols = &self.lsp_hints.symbols;
        let Some(answer) = self.nav.links.pending().and_then(|t| symbols.take(t)) else {
            return;
        };
        let found = answer.outcome.unwrap_or_default().into_iter();
        let found = found.map(|s| (s.name, s.container, s.file, s.line).into());
        if self.nav.links.answered(answer.ticket, found.collect()) {
            self.ui.redraw(cx);
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-poll-definition")]
    fn lsp_poll_definition(&mut self, cx: &mut Cx) {
        use crawrad_lsp::definition::Outcome;
        let Some(ticket) = self.nav.jumps.pending_ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.definition.take(ticket) else {
            return;
        };
        let Some(from) = self.nav.jumps.answered(ticket) else {
            return;
        };
        let line = self.texto_na_vista.lines().nth(from.line).unwrap_or("");
        let name = crawrad_host::navigation::word_at(line, from.byte).to_string();
        self.status = Some(match answer.outcome {
            _ if answer.version != self.lsp_versao => {
                "The file changed while looking; Go to Definition again".into()
            }
            Outcome::NoAnswer(_) => "Language server did not answer".into(),
            Outcome::Nothing => format!("No definition found for '{name}'"),
            Outcome::Found(found) => {
                let n = found.len();
                let mut places = found.into_iter().map(|t| Place {
                    file: t.file,
                    line: t.line,
                    byte: t.byte,
                });
                if n == 1 {
                    self.nav.jumps.jumped_from(from);
                    self.go_to_place(cx, places.next().unwrap_or_default());
                    format!("Definition of '{name}'")
                } else {
                    self.offer_definitions(cx, places.collect(), from);
                    format!("{n} definitions of '{name}': choose one")
                }
            }
        });
        self.ui.redraw(cx);
    }

    #[note("ide-app-lsp-service-ideapp-serve-hover")]
    fn serve_hover(&mut self, cx: &mut Cx) {
        use crawrad_host::hover_card::Anchor;
        let view = self.code_view();
        let Some(mut cv) = view.borrow_mut::<CodeView>() else {
            return;
        };
        let (moved, rest) = (cv.take_hover_moved(), cv.take_hover_rest());
        drop(cv);
        if moved || self.complete.aberto {
            self.close_hover(cx);
        }
        if let Some(r) = rest.filter(|_| !self.complete.aberto) {
            let (x, y, line_height) = (r.at.x, r.at.y, r.line_height);
            self.ask_hover((r.line, r.byte), Anchor { x, y, line_height }, false);
        }
        self.poll_hover(cx);
    }

    #[note("ide-app-lsp-service-ideapp-hover-at-caret")]
    fn hover_at_caret(&mut self, cx: &mut Cx) {
        let view = self.code_view();
        let at = view.borrow::<CodeView>().and_then(|cv| cv.caret_anchor());
        let (Some(at), Some(here)) = (at, self.here(cx)) else {
            return;
        };
        let (x, y, line_height) = (at.at.x, at.at.y, at.line_height);
        let anchor = crawrad_host::hover_card::Anchor { x, y, line_height };
        self.ask_hover((here.line, here.byte), anchor, true);
    }

    fn ask_hover(
        &mut self,
        caret: (usize, usize),
        anchor: crawrad_host::hover_card::Anchor,
        by_key: bool,
    ) {
        let (Some(lsp), Some(file)) = (self.lsp.clone(), self.abas.active_file()) else {
            return;
        };
        let file = file.to_path_buf();
        let ticket = self.nav.hover.ask(anchor, by_key);
        let (text, version) = (&self.texto_na_vista, self.lsp_versao);
        self.lsp_hints
            .hover
            .request(lsp, ticket, &file, version, text, caret);
    }

    #[note("ide-app-lsp-service-ideapp-poll-hover")]
    fn poll_hover(&mut self, cx: &mut Cx) {
        use crawrad_lsp::hover::Outcome;
        let Some(ticket) = self.nav.hover.pending_ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.hover.take(ticket) else {
            return;
        };
        let Some((anchor, by_key)) = self.nav.hover.answered(ticket) else {
            return;
        };
        match answer.outcome {
            Outcome::Text(text) if answer.version == self.lsp_versao => {
                let card = self.hover_box();
                let text = crawrad_host::hover_card::clip(&text, 30);
                card.text_input(id!(hover_text)).set_text(cx, &text);
                let lh = anchor.line_height.max(16.0);
                let note = answer.defined_at.and_then(|t| self.note_defined_at(t));
                let height = match &note {
                    Some((id, _, _)) => self.paint_note_card(cx, &card, id, &answer.markdown, lh),
                    None => crawrad_host::hover_card::plain_height(&text, lh, 64),
                };
                card.view(id!(hover_plain)).set_visible(cx, note.is_none());
                card.view(id!(hover_rich)).set_visible(cx, note.is_some());
                card.view(id!(hover_note_row))
                    .set_visible(cx, note.is_some());
                self.place_card(cx, &card, anchor, (520.0, height));
                self.nav.hover.shown_at(anchor);
                self.nav.hover.set_note(note);
            }
            Outcome::NoAnswer(_) if by_key => {
                self.status = Some("Language server did not answer".into())
            }
            Outcome::Nothing if by_key => self.status = Some("No hover information here".into()),
            _ => {}
        }
        self.ui.redraw(cx);
    }

    #[note("ide-app-lsp-service-ideapp-place-card")]
    pub(super) fn place_card(
        &mut self,
        cx: &mut Cx,
        card: &ViewRef,
        anchor: crawrad_host::hover_card::Anchor,
        size: (f64, f64),
    ) {
        let container = self.painel(Panel::Design).view(id!(code_container));
        let origin = container.area().rect(cx).pos;
        let window = self.ui.area().rect(cx);
        let w = (window.pos.x, window.pos.y, window.size.x, window.size.y);
        let (x, y) = crawrad_host::hover_card::place(anchor, size, w);
        let (x, y) = (x - origin.x, y - origin.y);
        card.apply_over(cx, live! { margin: { left: (x), top: (y) } });
        card.set_visible(cx, true);
        self.ui.redraw(cx);
    }

    #[note("ide-app-lsp-service-ideapp-hover-box")]
    pub(super) fn hover_box(&self) -> ViewRef {
        self.painel(Panel::Design)
            .view(id!(code_container.hover_box))
    }

    #[note("ide-app-lsp-service-ideapp-close-hover")]
    pub(super) fn close_hover(&mut self, cx: &mut Cx) -> bool {
        let was = self.nav.hover.close();
        if was {
            let card = self.hover_box();
            card.set_visible(cx, false);
            self.ui.redraw(cx);
        }
        was
    }

    #[note("ide-app-lsp-service-ideapp-hover-json")]
    pub(super) fn hover_json(&self) -> serde_json::Value {
        let card = self.hover_box();
        let text = card.text_input(id!(hover_text)).text();
        let at = self.nav.hover.shown().map(|a| [a.x, a.y, a.line_height]);
        serde_json::json!({ "open": at.is_some(), "text": if at.is_some() { text } else { String::new() }, "anchor": at, "note": self.nav.hover.note().map(|n| &n.0) })
    }

    #[note("ide-app-lsp-service-ideapp-format-document")]
    fn format_document(&mut self) {
        let (Some(file), Some(lsp)) = (self.editavel.clone(), self.lsp.clone()) else {
            let front = self.abas.active_file();
            self.status =
                Some(crawrad_lsp::format::refusal(front, crawrad_codegen::SUFIXO_GERADO).into());
            return;
        };
        let ticket = self.nav.format.ask(file.clone(), self.lsp_versao);
        self.lsp_hints
            .format
            .request(lsp, ticket, &file, self.lsp_versao, 4);
        self.status = Some("Formatting\u{2026}".into());
    }

    #[note("ide-app-lsp-service-ideapp-poll-format")]
    fn poll_format(&mut self, cx: &mut Cx) {
        use crawrad_lsp::format::{Verdict, verdict};
        let Some(ticket) = self.nav.format.ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.format.take(ticket) else {
            return;
        };
        let Some((file, version)) = self.nav.format.answered(ticket) else {
            return;
        };
        let same = self.editavel.as_deref() == Some(file.as_path()) && self.lsp_versao == version;
        let enc = self.lsp_encoding();
        self.status = Some(
            match verdict(answer.outcome, same, &self.texto_na_vista, enc) {
                Verdict::Say(s) => s.into(),
                Verdict::Apply(new) => {
                    if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
                        cv.replace_all(cx, &new);
                    }
                    "Formatted".into()
                }
            },
        );
        self.ui.redraw(cx);
    }

    #[note("ide-app-lsp-service-ideapp-navigate")]
    fn navigate(&mut self, cx: &mut Cx, back: bool) {
        let Some(here) = self.here(cx) else {
            return;
        };
        let to = match back {
            true => self.nav.jumps.back(here),
            false => self.nav.jumps.forward(here),
        };
        match to {
            Some(place) => self.go_to_place(cx, place),
            None => self.status = Some("Nothing to go to".into()),
        }
    }

    #[note("ide-app-lsp-service-ideapp-go-to-place")]
    pub(super) fn go_to_place(&mut self, cx: &mut Cx, place: Place) {
        if self.abas.active_file() != Some(place.file.as_path()) {
            self.abrir_no_editor(cx, place.file);
        }
        let at = super::PousoCodigo::cursor(place.line + 1, place.byte + 1);
        self.pousar_cursor(cx, &at);
        self.escrever_rodape_do_editor(cx);
    }

    #[note("ide-app-lsp-service-ideapp-set-inlay-hints")]
    pub fn set_inlay_hints(&mut self, cx: &mut Cx, on: bool) {
        self.inlay_hints_off = !on;
        let mut pref = crawrad_project::workspace::read_preferences();
        pref.editor_inlay_hints = on;
        crawrad_project::workspace::save_preferences(pref);
        if let (true, Some(lsp), Some(arq)) = (on, self.lsp.clone(), self.editavel.clone()) {
            self.lsp_hints
                .hints
                .request(lsp, &arq, self.lsp_versao, &self.texto_na_vista);
        } else if let Some(mut cv) = self.code_view().borrow_mut::<CodeView>() {
            cv.set_inline_hints(cx, &[]);
        }
    }

    #[note("ide-app-lsp-service-ideapp-lsp-command")]
    pub fn lsp_command(&mut self, command: crawrad_actions::LspCommand) {
        use crawrad_actions::LspCommand;
        if command == LspCommand::ReloadWorkspace {
            if let Some(lsp) = &self.lsp {
                let _ = lsp.reload_workspace();
            }
            return;
        }
        if let Some(lsp) = self.lsp.take() {
            lsp.shutdown();
        }
        self.lsp_cell = LspCell::down(SERVER, "stopped");
        if command == LspCommand::Restart {
            match (self.editavel.clone(), self.texto_do_code_view()) {
                (Some(file), Some(text)) => self.lsp_notificar_abertura(&file, &text),
                _ => self.garantir_lsp(),
            }
        }
    }

    #[note("ide-app-lsp-service-ideapp-draw-lsp-cell")]
    fn draw_lsp_cell(&mut self, cx: &mut Cx, cell: LspCell) {
        let shown = crawrad_insight::item::fit(&cell.detail, ROOM, |s| {
            self.measure
                .layout(cx, 0., 0., None, false, Align::default(), s)
                .size_in_lpxs
                .width as f64
        });
        let (phase, asked) = (cell.phase.code(), !cell.name.is_empty());
        for pill in [
            id!(bottom_status_bar.cell_lsp),
            id!(bottom_status_bar.cell_lsp_progress),
        ] {
            self.ui.view(pill).set_visible(cx, asked);
        }
        let icon = self.ui.view(id!(bottom_status_bar.cell_lsp.lsp_icon));
        icon.apply_over(cx, live! { draw_bg: { phase: (phase) } });
        icon.redraw(cx);
        if cell.phase == LspPhase::Busy {
            self.lsp_spin = cx.new_next_frame();
        }
        self.lsp_shown = (cell, shown);
        self.refresh_footer(cx);
    }

    #[note("ide-app-lsp-service-ideapp-spin-lsp-icon")]
    pub(super) fn spin_lsp_icon(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(frame) = self.lsp_spin.is_event(event)
            && self.lsp_shown.0.phase == LspPhase::Busy
        {
            let turn = lsp_cell::turn(frame.time);
            let icon = self.ui.view(id!(bottom_status_bar.cell_lsp.lsp_icon));
            icon.apply_over(cx, live! { draw_bg: { turn: (turn) } });
            icon.redraw(cx);
            self.lsp_spin = cx.new_next_frame();
        }
    }

    #[note("ide-app-lsp-service-ideapp-squiggles-json")]
    pub fn squiggles_json(&self) -> serde_json::Value {
        let doc = |cv: &CodeView| Some(cv.session.as_ref()?.document().decorations_json());
        self.with_code_view(doc).flatten().unwrap_or_default()
    }

    #[note("ide-app-lsp-service-ideapp-lsp-process-diagnostics")]
    pub fn lsp_process_diagnostics(&mut self, cx: &mut Cx) {
        let Some(ref lsp) = self.lsp else {
            return;
        };

        let params_lista = lsp.poll_diagnostics();
        let enc = lsp.position_encoding();
        if params_lista.is_empty() {
            return;
        }

        let arquivo_ativo = self.editavel.clone();

        for params in params_lista {
            let caminho_diag = crawrad_lsp::uri_to_path(&params.uri);

            // Apply decorations to active CodeView
            if arquivo_ativo.is_some() && arquivo_ativo == caminho_diag {
                self.decorate(cx, &params.diagnostics, enc);
            }
        }
        // the Explorer's colours and the bar's count follow what was published
        self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        self.refresh_footer(cx);
    }

    #[note("ide-app-lsp-service-ideapp-decorate")]
    pub(super) fn decorate(
        &mut self,
        cx: &mut Cx,
        diagnostics: &[crawrad_lsp::Diagnostic],
        enc: crawrad_lsp::encoding::PositionEncoding,
    ) {
        let vista = self.code_view();
        if let Some(mut cv) = vista.borrow_mut::<CodeView>() {
            cv.lazy_init_session();
            if let Some(ref sessao) = cv.session {
                let mut doc = sessao.document().clone();
                doc.clear_decorations();
                let lines = doc.as_text().as_lines().to_vec();
                for (i, m) in crawrad_lsp::marks(diagnostics).iter().enumerate() {
                    let at = |p| MakepadPos::from(enc.place(&lines, p));
                    use DecorationType::{Error, Hint, Warning};
                    let ty = [Hint, Warning, Error][m.level()];
                    let mark = Decoration::new(i + 1, at(m.range.start), at(m.range.end), ty);
                    doc.add_decoration(mark.tagged(m.faded, m.struck));
                }
                let mut messages = self.mark_panics(&mut doc);
                messages.extend(crawrad_lsp::line_messages(diagnostics));
                doc.set_line_messages(&messages);
            }
        }
        vista.redraw(cx);
    }

    #[note("ide-app-lsp-service-ideapp-update-editor-breadcrumbs")]
    pub fn update_editor_breadcrumbs(&mut self, cx: &mut Cx, _arquivo: &Path) {
        self.escrever_breadcrumbs_do_editor(cx);
    }

    #[note("ide-app-lsp-service-ideapp-follow-live-agent")]
    pub(crate) fn follow_live_agent(&mut self, cx: &mut Cx) {
        if self.chat.live.poll() {
            // A reset to Idle (the catalog arrived after a failed first try) warms again.
            self.warm_live_agent(cx);
            self.sync_model_button(cx);
            self.renderizar_historico_chat(cx);
        }
    }
}
