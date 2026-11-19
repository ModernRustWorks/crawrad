use super::super::*;
use crawrad_actions::capability::search_commands as buscar_comandos;
use crawrad_host::navigation::Place;
use crawrad_host::quick_open::{
    filter_files, is_command_mode, parse_line_column as parse_linha_coluna, scan_project_files,
};
use crawrad_note::note;
use makepad_widgets::makepad_draw::text::selection::Cursor;
use std::path::PathBuf;

impl IdeApp {
    // --- GOTO LINE (Ctrl+G) ---

    pub(in super::super) fn open_goto(&mut self, cx: &mut Cx) {
        self.goto_dialog_aberto = true;
        let (linha, _) = self.cursor_do_code_view(cx);
        self.goto_input_texto = format!(":{}", linha + 1);
        self.goto_status_msg = None;
        self.refresh(cx);
    }

    pub(in super::super) fn draw_goto(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(goto_dialog), self.goto_dialog_aberto);
        if !self.goto_dialog_aberto {
            return;
        }

        let dlg = self.ui.view(id!(goto_dialog.dlg));
        let help = "Line or line:column, then Enter";
        let help = self.goto_status_msg.as_deref().unwrap_or(help);
        dlg.label(id!(goto_help)).set_text(cx, help);

        let input = dlg.text_input(id!(goto_input));
        if input.text().is_empty() {
            input.set_text(cx, &self.goto_input_texto);
            cx.set_key_focus(input.area());
        }
    }

    pub(in super::super) fn atender_goto(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.goto_dialog_aberto {
            return;
        }
        let dlg = self.ui.view(id!(goto_dialog.dlg));

        if let Some((txt, _)) = dlg.text_input(id!(goto_input)).returned(actions) {
            self.goto_input_texto = txt;
            self.confirmar_goto(cx);
            return;
        }

        if dlg.button(id!(goto_cancel)).clicked(actions)
            || dlg.text_input(id!(goto_input)).escaped(actions)
        {
            self.goto_dialog_aberto = false;
            cx.set_key_focus(self.code_view().area());
            self.refresh(cx);
            return;
        }

        if dlg.button(id!(goto_confirm)).clicked(actions) {
            let txt = dlg.text_input(id!(goto_input)).text();
            self.goto_input_texto = txt;
            self.confirmar_goto(cx);
        }
    }

    fn confirmar_goto(&mut self, cx: &mut Cx) {
        if let Some((linha, col)) = parse_linha_coluna(&self.goto_input_texto) {
            let pouso = crate::ide_app::PousoCodigo::cursor(linha, col);
            self.pousar_cursor(cx, &pouso);
            self.goto_dialog_aberto = false;
            cx.set_key_focus(self.code_view().area());
            self.refresh(cx);
        } else {
            self.goto_status_msg =
                Some("Invalid format. Use a line number (e.g. 42 or 42:5)".to_string());
            self.refresh(cx);
        }
    }

    // --- QUICK OPEN (Ctrl+P) E COMMAND PALETTE (Ctrl+Shift+P) ---

    pub(in super::super) fn open_command_palette(&mut self, cx: &mut Cx) {
        self.nav.choices.close();
        self.quick_open_aberto = true;
        self.quick_open_filtro = ">".to_string();
        self.quick_open_focar = true;
        self.quick_open_comandos = buscar_comandos(&self.capabilities, &self.quick_open_filtro);
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-quick-open-ideapp-focus-quick-open")]
    pub(in super::super) fn focus_quick_open(&mut self, cx: &mut Cx) {
        let input = self
            .ui
            .view(id!(quick_open_dialog.dlg))
            .text_input(id!(quick_input));
        if !self.quick_open_focar || !self.quick_open_aberto {
            return;
        }
        if input.area().is_valid(cx) {
            cx.set_key_focus(input.area());
            self.quick_open_focar = false;
        } else {
            cx.new_next_frame();
        }
    }

    #[note("ide-app-dialogs-quick-open-ideapp-offer-definitions")]
    pub(in super::super) fn offer_definitions(
        &mut self,
        cx: &mut Cx,
        places: Vec<Place>,
        from: Place,
    ) {
        self.quick_open_filtrados = places.iter().map(|p| p.file.clone()).collect();
        self.nav.choices.offer(places, from);
        (self.quick_open_aberto, self.quick_open_focar) = (true, true);
        self.quick_open_filtro.clear();
        self.quick_open_comandos.clear();
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-quick-open-ideapp-open-link-pick")]
    pub(in super::super) fn open_link_pick(&mut self, cx: &mut Cx) {
        self.nav.choices.close();
        (self.quick_open_aberto, self.quick_open_focar) = (true, true);
        self.quick_open_filtro.clear();
        self.quick_open_comandos.clear();
        self.quick_open_filtrados.clear();
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-quick-open-ideapp-ask-links")]
    fn ask_links(&mut self, cx: &mut Cx, typed: &str) {
        match (self.nav.links.ask(typed), self.lsp.clone()) {
            (Some((ticket, q)), Some(lsp)) => _ = self.lsp_hints.symbols.request(lsp, ticket, &q),
            (Some(_), None) => self.status = Some("Link item needs rust-analyzer".into()),
            _ => {}
        }
        self.ui.redraw(cx);
    }

    #[note("ide-app-dialogs-quick-open-ideapp-pick-link")]
    fn pick_link(&mut self, cx: &mut Cx, i: usize) {
        let Some((at, link)) = self.nav.links.choose(i) else {
            return;
        };
        if let Some(m) = self.nota.as_mut() {
            m.insert_link(at, &link);
        }
        self.close_quick_open(cx);
        let input = self
            .ui
            .view(id!(note_dialog.dlg))
            .text_input(id!(note_content_input));
        let after = Cursor {
            index: at + link.len(),
            prefer_next_row: false,
        };
        input.set_cursor(cx, after, false);
        cx.set_key_focus(input.area());
    }

    #[note("ide-app-dialogs-quick-open-ideapp-close-quick-open")]
    fn close_quick_open(&mut self, cx: &mut Cx) {
        self.quick_open_aberto = false;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-quick-open-ideapp-pick-definition")]
    fn pick_definition(&mut self, cx: &mut Cx, i: usize) {
        if let Some((to, from)) = self.nav.choices.choose(i) {
            self.nav.jumps.jumped_from(from);
            self.go_to_place(cx, to);
            self.close_quick_open(cx);
        }
    }

    pub(in super::super) fn open_quick_open(&mut self, cx: &mut Cx) {
        self.close_caret_popups(cx); // the list behind would keep the editor's Enter
        self.nav.choices.close();
        self.quick_open_aberto = true;
        self.quick_open_filtro.clear();
        self.quick_open_focar = true;
        let raiz = self.raiz().unwrap_or_else(|| PathBuf::from("."));
        self.quick_open_arquivos = scan_project_files(&raiz);
        self.quick_open_filtrados = self.quick_open_arquivos.clone();
        self.quick_open_comandos.clear();
        self.refresh(cx);
    }

    pub(in super::super) fn draw_quick_open(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(quick_open_dialog), self.quick_open_aberto);
        if !self.quick_open_aberto {
            return;
        }

        let dlg = self.ui.view(id!(quick_open_dialog.dlg));
        let is_cmd = is_command_mode(&self.quick_open_filtro);
        dlg.label(id!(quick_title)).set_text(
            cx,
            if self.nav.links.is_open() {
                "Link item: type a name"
            } else if self.nav.choices.is_open() {
                "Go to Definition: choose one"
            } else if is_cmd {
                "Command Palette"
            } else {
                "Quick Open"
            },
        );

        if self.quick_open_focar {
            dlg.text_input(id!(quick_input))
                .set_text(cx, &self.quick_open_filtro);
            self.focus_quick_open(cx);
        }
    }

    pub(in super::super) fn atender_quick_open(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.quick_open_aberto {
            return;
        }
        let dlg = self.ui.view(id!(quick_open_dialog.dlg));

        if let Some(txt) = dlg.text_input(id!(quick_input)).changed(actions) {
            if self.nav.links.is_open() {
                return self.ask_links(cx, &txt);
            }
            if self.nav.choices.is_open() {
                return; // the places are few: no filter to keep in step
            }
            self.quick_open_filtro = txt;
            if is_command_mode(&self.quick_open_filtro) {
                self.quick_open_comandos =
                    buscar_comandos(&self.capabilities, &self.quick_open_filtro);
            } else {
                self.quick_open_filtrados =
                    filter_files(&self.quick_open_arquivos, &self.quick_open_filtro);
            }
            self.ui.redraw(cx);
            return;
        }

        // Enter no campo de busca: executa comando ou abre primeiro arquivo
        if let Some((_, _)) = dlg.text_input(id!(quick_input)).returned(actions) {
            if self.nav.links.is_open() {
                return self.pick_link(cx, 0);
            }
            if self.nav.choices.is_open() {
                return self.pick_definition(cx, 0);
            }
            if is_command_mode(&self.quick_open_filtro) {
                if let Some(primeiro) = self.quick_open_comandos.first().cloned() {
                    self.execute_with(cx, &primeiro.id, crawrad_actions::Args::none());
                    self.close_quick_open(cx);
                    return;
                }
            } else if let Some(primeiro) = self.quick_open_filtrados.first().cloned() {
                self.abrir_no_editor(cx, primeiro);
                self.close_quick_open(cx);
                return;
            }
        }

        // Esc closes it as Cancel does (the field has the focus: Esc comes as its `Escaped`)
        let escaped = dlg.text_input(id!(quick_input)).escaped(actions);
        if dlg.button(id!(quick_cancel)).clicked(actions) || escaped {
            self.nav.choices.close();
            self.nav.links.close();
            cx.set_key_focus(self.code_view().area());
            self.close_quick_open(cx);
            return;
        }

        // Clique num item da lista
        let list = dlg.portal_list(id!(quick_list));
        if list.any_items_with_actions(actions) {
            let clicked = |(_, item): &(usize, WidgetRef)| item.button(id!(alvo)).clicked(actions);
            let chosen = list.items_with_actions(actions).into_iter().find(clicked);
            if let Some((i, _)) = chosen.filter(|_| self.nav.links.is_open()) {
                return self.pick_link(cx, i);
            }
            if is_command_mode(&self.quick_open_filtro) {
                for (item_id, item) in list.items_with_actions(actions) {
                    if item_id < self.quick_open_comandos.len()
                        && item.button(id!(alvo)).clicked(actions)
                    {
                        let cmd = self.quick_open_comandos[item_id].clone();
                        self.execute_with(cx, &cmd.id, crawrad_actions::Args::none());
                        self.close_quick_open(cx);
                        break;
                    }
                }
            } else {
                for (item_id, item) in list.items_with_actions(actions) {
                    if item_id < self.quick_open_filtrados.len()
                        && item.button(id!(alvo)).clicked(actions)
                    {
                        if self.nav.choices.is_open() {
                            return self.pick_definition(cx, item_id);
                        }
                        let arq = self.quick_open_filtrados[item_id].clone();
                        self.abrir_no_editor(cx, arq);
                        self.close_quick_open(cx);
                        break;
                    }
                }
            }
        }
    }
}
