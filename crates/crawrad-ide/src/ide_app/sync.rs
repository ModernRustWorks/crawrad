//! The sync of the window with the state: `sincronizar`, the minimap, the centre's tabs and view.
//! Cut out of `ide_app.rs` as a pure move, as `chat.rs` and `build.rs` were.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-sync-ideapp-refresh")]
    pub(crate) fn refresh(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        self.ui.redraw(cx);
    }

    /// With nothing selected on a form, the form itself is the selection, as on the
    /// classic designer: the Object Inspector always shows something. Projects without a
    /// form have no document here and keep their empty Inspector.
    fn select_form_when_empty(&mut self) {
        let Some(d) = self.host.doc_mut() else {
            return;
        };
        if !d.selection().is_empty() {
            return;
        }
        let root = d.ir().root.clone();
        d.select_one(&root);
        self.selection_to_canvas();
    }

    /// The document's selection, handed to the canvas that draws the handles.
    pub(crate) fn selection_to_canvas(&self) {
        if let Some(mut c) = self.canvas().borrow_mut::<DesignCanvas>()
            && let Some(d) = self.host.doc()
        {
            c.runtime_mut().set_selection(d.selection());
        }
    }

    #[note("ide-app-sync-ideapp-sincronizar")]
    pub(crate) fn sincronizar(&mut self, cx: &mut Cx) {
        self.disk.ensure(self.raiz());
        self.select_form_when_empty();
        let sel: Vec<NodeId> = self
            .host
            .doc()
            .map(|d| d.selection().to_vec())
            .unwrap_or_default();
        let pode_add = self.collection_selection().is_some();
        let botao = self.painel(Panel::Components).button(id!(add_item));
        Self::habilitar(cx, &botao, pode_add);

        let alvo = inspector::header(self.host.doc().map(|d| d.ir()), &sel);
        if alvo != self.alvo_na_tela {
            let (name, class) = alvo.split_once("  ·  ").unwrap_or((alvo.as_str(), ""));
            let picker = self.painel(Panel::Inspector).view(id!(inspector_picker));
            picker.label(id!(picker_name)).set_text(cx, name);
            picker.label(id!(picker_class)).set_text(cx, class);
            (picker.label(id!(picker_arrow))).set_text(cx, icon::Chrome::ChevronDown.glyph());
            self.alvo_na_tela = alvo;
        }
        if sel != self.selecao_na_arvore {
            self.redraw_panel_list(cx, Panel::Components, id!(structure_tree.structure_list));
            self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
            self.selecao_na_arvore = sel.clone();
        }
        self.check_form(cx);
        self.sync_page_look(cx);
        self.check_sizes(cx);
        self.check_translations(cx);
        self.draw_pad(cx);
        self.draw_items(cx);
        self.draw_lines(cx);
        self.draw_color(cx);
        self.draw_actions(cx);
        self.draw_standard(cx);
        self.draw_imagelist(cx);
        self.draw_quit(cx);
        self.draw_recover(cx);
        self.draw_options(cx);
        self.draw_project_options(cx);
        self.draw_new_project(cx);
        self.draw_creation(cx);
        self.draw_icons(cx);
        self.draw_memory(cx);
        self.draw_environment(cx);
        self.draw_mcp(cx);
        self.draw_acp_diagnostic(cx);
        self.draw_note(cx);
        self.draw_goto(cx);
        self.draw_quick_open(cx);
        self.draw_inherit_form(cx);
        self.draw_new_component(cx);
        self.draw_build(cx);
        self.draw_shrink(cx);
        self.draw_permission(cx);
        self.draw_templates(cx);
        self.draw_find_replace(cx);
        self.draw_autocomplete(cx);
        self.draw_component_rename(cx);

        self.colher_do_editor(cx);
        self.sync_breakpoints(cx);
        self.sincronizar_minimap(cx);
        self.sincronizar_extensoes();
        self.follow_project_framework();
        let kind_key = (
            self.desktop_root.clone(),
            self.host.active_file().map(Into::into),
        );
        if kind_key != self.project_kind_key {
            let (kind, without_form) = self.project_kind_now();
            self.project_kind = kind.map(str::to_string);
            self.project_without_form = without_form;
            self.project_kind_key = kind_key;
        }
        self.follow_recovery(cx);
        self.escrever_rodape_do_editor(cx);
        self.refresh_footer(cx);

        let active_proj = self
            .raiz()
            .as_deref()
            .and_then(crawrad_project::active_project::active_project_name);
        let form_file = self
            .host
            .active_file()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());
        let title =
            crawrad_project::active_project::window_title(active_proj.as_deref(), form_file);
        self.ui
            .label(id!(top_titlebar.title_container.title_lbl))
            .set_text(cx, &title);

        // Save and Save All follow what is left to save.
        self.follow_save_buttons(cx);

        self.sync_model_button(cx);
        self.sincronizar_barra_do_designer(cx);
        self.sync_toolbox_empty_state(cx);

        let p_des = self.painel(Panel::Design);
        p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_ai))
            .set_text(cx, icon::Chrome::Sparkle.glyph());
        p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_split))
            .set_text(cx, icon::Chrome::SplitHorizontal.glyph());
        p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_more))
            .set_text(cx, icon::Chrome::More.glyph());
    }

    fn sync_toolbox_empty_state(&mut self, cx: &mut Cx) {
        let no_form = self.no_form_in_project();
        let p = self.painel(Panel::Components);
        for v in [
            id!(toolbox_filter),
            id!(toolbox_palette),
            id!(structure_tree),
        ] {
            p.view(v).set_visible(cx, !no_form);
        }
        p.button(id!(structure_header.add_item))
            .set_visible(cx, !no_form);
        let empty = p.view(id!(no_form));
        empty.set_visible(cx, no_form);
        empty
            .label(id!(icon))
            .set_text(cx, icon::Chrome::NewForm.glyph());
        let hint = crawrad_project::creation::new_form_hint(self.project_kind.is_some());
        empty.label(id!(hint)).set_text(cx, hint);
    }

    #[note("ide-app-sync-ideapp-sincronizar-minimap")]
    pub fn sincronizar_minimap(&mut self, cx: &mut Cx) {
        if !self.host.center().is_text() {
            return;
        }

        let minimap_widget = self.painel(Panel::Design).widget(id!(editor_minimap));
        if minimap_widget.is_empty() {
            return;
        }

        minimap_widget.set_visible(cx, self.minimap_visivel);
        if !self.minimap_visivel {
            return;
        }

        let vista = self.code_view();
        let (
            scroll_pos,
            scroll_view_total,
            scroll_view_visible,
            cell_size,
            line_start,
            line_end,
            code_text,
        ) = {
            let Some(mut cv) = vista.borrow_mut::<makepad_code_editor::code_view::CodeView>()
            else {
                return;
            };
            cv.lazy_init_session();

            let scroll_pos = cv.get_scroll_pos();
            let scroll_view_total = cv.get_scroll_view_total();
            let scroll_view_visible = cv.get_scroll_view_visible();
            let cell_size = cv.cell_size();
            let line_start = cv.line_start();
            let line_end = cv.line_end();
            let text = cv.live_text().unwrap_or_else(|| cv.text());
            (
                scroll_pos,
                scroll_view_total,
                scroll_view_visible,
                cell_size,
                line_start,
                line_end,
                text,
            )
        };
        if self.texto_na_vista.is_empty() && !code_text.is_empty() {
            self.texto_na_vista = code_text;
        }
        let total_lines = self.texto_na_vista.lines().count().max(1);

        let view_rect = vista.area().rect(cx);
        let fallback_visible_h = view_rect.size.y.max(100.0);
        let visible_h = if scroll_view_visible.y > 0.0 {
            scroll_view_visible.y
        } else {
            fallback_visible_h
        };

        let cell_h = if cell_size.y > 0.0 { cell_size.y } else { 19.0 };
        let total_h = if scroll_view_total.y > 0.0 {
            scroll_view_total.y
        } else {
            total_lines as f64 * cell_h
        };

        let Some(mut minimap) = minimap_widget.borrow_mut::<CrawradMinimap>() else {
            return;
        };

        let mudou_linhas = minimap.update_lines(&self.texto_na_vista);
        let mudou_scroll = minimap.update_scroll_state(
            scroll_pos.y,
            total_h,
            visible_h,
            cell_h,
            line_start,
            line_end,
            total_lines,
        );

        if mudou_linhas || mudou_scroll {
            minimap.redraw(cx);
        }

        // If the user interacted with the minimap to navigate:
        let target_scroll_y = minimap.target_scroll_y.take();
        let target_line = minimap.target_line.take();
        drop(minimap);

        if let Some(target_y) = target_scroll_y {
            if let Some(mut cv) = vista.borrow_mut::<makepad_code_editor::code_view::CodeView>() {
                cv.set_scroll_pos(cx, dvec2(0.0, target_y));
                cv.editor.redraw(cx);
            }
            self.ui.redraw(cx);
        } else if let Some(target_l) = target_line {
            if let Some(mut cv) = vista.borrow_mut::<makepad_code_editor::code_view::CodeView>() {
                cv.scroll_to_line(cx, target_l);
            }
            self.ui.redraw(cx);
        }
    }

    #[note("ide-app-sync-ideapp-write-center-tabs")]
    pub(super) fn write_center_tabs(&mut self, cx: &mut Cx) {
        let centro = self.host.center();
        let design = self.painel(Panel::Design);
        let labels = [
            (id!(center_tabs_bar.tab_design), centro.design_tab_label()),
            (id!(center_tabs_bar.tab_code), "Code"),
            (id!(center_tabs_bar.tab_history), "History"),
            (id!(center_tabs_bar.tab_sizes), "Sizes"),
            (id!(center_tabs_bar.tab_translations), "Translations"),
        ];
        for (tab, label) in labels {
            design.radio_button(tab).set_text(cx, label);
        }
        let acesa = match centro {
            Center::Code => id!(center_tabs_bar.tab_code),
            Center::History => id!(center_tabs_bar.tab_history),
            Center::Sizes => id!(center_tabs_bar.tab_sizes),
            Center::Translations => id!(center_tabs_bar.tab_translations),
            _ => id!(center_tabs_bar.tab_design),
        };
        for tab_id in [
            id!(center_tabs_bar.tab_design),
            id!(center_tabs_bar.tab_code),
            id!(center_tabs_bar.tab_history),
            id!(center_tabs_bar.tab_sizes),
            id!(center_tabs_bar.tab_translations),
        ] {
            if tab_id != acesa
                && let Some(mut r) = self.painel(Panel::Design).radio_button(tab_id).borrow_mut()
            {
                r.animator_play(cx, id!(active.off));
                r.animator_play(cx, id!(focus.off));
            }
        }
        if centro != Center::Welcome {
            self.painel(Panel::Design)
                .radio_button(acesa)
                .select(cx, &mut Scope::empty());
        }
        if let Some(mut r) = self.painel(Panel::Design).radio_button(acesa).borrow_mut() {
            r.animator_play(cx, id!(focus.off));
        }
        let design = self.painel(Panel::Design);
        design.view(id!(center_tabs_bar)).redraw(cx);
        design
            .portal_list(id!(editor_tabs_bar.editor_tabs))
            .redraw(cx);
    }

    #[note("ide-app-sync-ideapp-show-center")]
    pub(super) fn show_center(&mut self, cx: &mut Cx, nova: source::Center) {
        self.host.set_center(nova);
        let (design_on, text_on) = (matches!(nova, Center::Design), nova.is_text());
        let welcome = matches!(nova, Center::Welcome);
        let shown: [(&[LiveId], bool); 12] = [
            (id!(designer_row.designer_canvas), design_on),
            (id!(designer_row.code_container), text_on),
            (
                id!(designer_row.history_container),
                matches!(nova, Center::History),
            ),
            (
                id!(designer_row.sizes_container),
                matches!(nova, Center::Sizes),
            ),
            (
                id!(designer_row.translations_container),
                matches!(nova, Center::Translations),
            ),
            (id!(designer_row.welcome_container), welcome),
            (id!(center_tabs_bar), !welcome),
            (id!(sep_center_bottom), !welcome),
            (id!(editor_tabs_bar), text_on),
            (id!(sep_editor_tabs), text_on),
            (id!(designer_options_bar), design_on),
            (id!(sep_designer_options), design_on),
        ];
        let design = self.painel(Panel::Design);
        for (path, on) in shown {
            design.widget(path).set_visible(cx, on);
        }
        self.sincronizar_barra_do_designer(cx);
        self.write_center_tabs(cx);
        if nova.is_text() {
            self.editavel = self.editable_file();
            let do_disco = self.texto_do_centro();
            let texto = match self.editavel.as_ref().and_then(|p| self.host.buffer(p)) {
                Some(pendente) => pendente.to_owned(),
                None => do_disco.clone(),
            };
            let review = self.review_view();
            let in_review = review.is_some();
            let shown = review.as_ref().map_or(&texto, |r| &r.text);
            self.close_caret_popups(cx);
            self.show_text_with_undo(cx, self.editavel.clone().as_deref(), shown);
            self.aplicar_somente_leitura(cx);
            self.set_editor_review(cx, review.as_ref().map_or(&[][..], |r| &r.rows));
            self.encoding_na_vista = editor_footer::file_encoding(self.abas.active_file());
            self.base_na_vista = do_disco;
            self.texto_na_vista = texto.clone();
            if !in_review {
                self.apply_diagnostics_decorations(cx);
            }
            if let Some(ref arq) = self.editavel.clone() {
                self.disk.shown(arq, self.base_na_vista.clone());
                self.change_tracker.remember(arq, &self.base_na_vista);
                self.update_editor_breadcrumbs(cx, arq);
                if !in_review {
                    self.lsp_notificar_abertura(arq, &texto);
                }
            }
            if !in_review {
                self.refresh_change_bars(cx);
            }
            self.sincronizar_minimap(cx);
        }
        if nova == Center::Sizes {
            self.paint_sizes(cx);
        }
        if nova == Center::Translations {
            self.paint_translations(cx);
        }
        self.update_diff_review_bar(cx);
        self.update_disk_change_bar(cx);
        self.escrever_rodape_do_editor(cx);
        self.ui.redraw(cx);
    }

    fn set_bar_button_enabled(&self, cx: &mut Cx, cmd: toolbar::Command, enabled: bool) {
        Self::habilitar(cx, &self.bar_button(cmd), enabled);
    }

    #[note("ide-app-sync-ideapp-follow-save-buttons")]
    pub(super) fn follow_save_buttons(&mut self, cx: &mut Cx) {
        let now = self
            .host
            .unsaved(self.editavel.as_deref(), self.host.saved_revision());
        if self.save_buttons_drawn == Some(now) {
            return;
        }
        self.save_buttons_drawn = Some(now);
        self.set_bar_button_enabled(cx, toolbar::Command::Save, now.0);
        self.set_bar_button_enabled(cx, toolbar::Command::SaveAll, now.1);
    }
}
