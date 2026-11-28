//! Code editor actions, center tabs, and open file tabs.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-actions-editor-ideapp-handle-editor-actions")]
    pub(in super::super) fn handle_editor_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.bulb_clicked(cx, actions);
        // LSP status in the bottom bar
        if self
            .ui
            .view(id!(bottom_status_bar.cell_lsp))
            .finger_up(actions)
            .is_some()
        {
            self.toggle_menu(cx, Aberto::LspStatus);
        }

        let mut abas_do_centro = WidgetSet::empty();
        abas_do_centro.push(
            self.painel(Panel::Design)
                .widget(id!(center_tabs_bar.tab_design)),
        );
        abas_do_centro.push(
            self.painel(Panel::Design)
                .widget(id!(center_tabs_bar.tab_code)),
        );
        abas_do_centro.push(
            self.painel(Panel::Design)
                .widget(id!(center_tabs_bar.tab_history)),
        );
        abas_do_centro.push(
            self.painel(Panel::Design)
                .widget(id!(center_tabs_bar.tab_sizes)),
        );
        abas_do_centro.push(
            self.painel(Panel::Design)
                .widget(id!(center_tabs_bar.tab_translations)),
        );
        let welcome = self.host.center() == crawrad_host::ViewCenter::Welcome;
        let selected = abas_do_centro.as_radio_button_set().selected(cx, actions);
        if let Some(i) = selected.filter(|_| !welcome) {
            if i == 4 {
                self.show_center(cx, Center::Translations);
            } else if i == 3 {
                self.show_center(cx, Center::Sizes);
            } else if i == 2 {
                // lighting the tab (a Timeline click) selects it again: keep the copy
                if self.host.center() != crawrad_host::ViewCenter::History {
                    self.open_history(cx);
                }
            } else if i == 1 && self.host.center() == crawrad_host::ViewCenter::Code {
            } else if i == 1 {
                match self.host.active_file().map(|p| p.to_path_buf()) {
                    Some(form) => self.abrir_no_editor(cx, source::unit_path(&form)),
                    None => self.show_center(cx, Center::Code),
                }
            } else {
                self.abas.show_design();
                let volta = if self.host.center() == Center::FormSource {
                    Center::FormSource
                } else {
                    Center::Design
                };
                self.show_center(cx, volta);
            }
        }

        let tira = self
            .painel(Panel::Design)
            .portal_list(id!(editor_tabs_bar.editor_tabs));
        if tira.any_items_with_actions(actions) {
            let mut abrir = None;
            let mut fechar = None;
            // WHICH OF THE TWO THE CLICK LANDED ON IS A HOST FACT. The shell asks
            // `strip_target` instead of subtracting, so the boundary is read from
            // the same `forms()` the strip was drawn from and cannot drift.
            let mut form_to_front = None;
            let mut form_to_close = None;
            for (i, item) in tira.items_with_actions(actions) {
                let i = match self.host.strip_target(i) {
                    crawrad_host::StripClick::Form(f) => {
                        if item.button(id!(tab_close)).clicked(actions) {
                            form_to_close = Some(f);
                        } else if item.button(id!(tab_label)).clicked(actions) {
                            form_to_front = Some(f);
                        }
                        continue;
                    }
                    crawrad_host::StripClick::File(a) => a,
                };
                if item.button(id!(tab_close)).clicked(actions) {
                    fechar = Some(i);
                } else if item.button(id!(tab_label)).clicked(actions) {
                    abrir = Some(i);
                }
            }
            // A form click ends in Design, a file click in Code: the strip is the
            // one place a person switches between the two, so each tab takes the
            // centre its own kind belongs to.
            if let Some(i) = form_to_close {
                self.host.close_form(i);
                if self.host.front().is_some() {
                    self.show_center(cx, Center::Design);
                } else {
                    self.show_center(cx, self.centro_das_abas());
                }
                self.sincronizar(cx);
            } else if let Some(i) = form_to_front {
                if self.host.bring_to_front(i) {
                    self.show_center(cx, Center::Design);
                    self.sincronizar(cx);
                }
            } else if let Some(i) = fechar {
                self.abas.close(i);
                self.show_center(cx, self.centro_das_abas());
            } else if let Some(i) = abrir {
                self.switch_tab(cx, |a| a.activate(i));
                self.show_center(cx, self.centro_das_abas());
            }
        }

        let p_des = self.painel(Panel::Design);

        // Action buttons of the editor tabs bar (AI, Split, More ...)
        if p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_ai))
            .clicked(actions)
        {
            self.execute_with(
                cx,
                "layout.toggle_panel",
                crawrad_actions::Args::text("panel", "chat"),
            );
            self.sincronizar(cx);
        } else if p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_split))
            .clicked(actions)
        {
            self.status = Some("Editor: Split view".into());
            self.sincronizar(cx);
        } else if p_des
            .button(id!(editor_tabs_bar.editor_tabs_actions.btn_editor_more))
            .clicked(actions)
        {
            self.open_menu = Some(Aberto::EditorMore);
            self.show_menu(cx);
        }

        // AI code diff review bar actions
        let painel_design = self.painel(Panel::Design);
        let steps = [
            (id!(diff_review_bar.btn_diff_prev), false),
            (id!(diff_review_bar.btn_diff_next), true),
        ];
        for (step, forward) in steps {
            if painel_design.button(step).clicked(actions) {
                self.step_diff_review(cx, forward);
            }
        }
        let btn_diff_acc = painel_design.button(id!(diff_review_bar.btn_diff_accept));
        if btn_diff_acc.clicked(actions)
            && let Some(active_file) = self.reviewed_file().or_else(|| self.active_file_path())
        {
            let _ = self.accept_diff_review(cx, &active_file);
        }
        let btn_diff_rej = painel_design.button(id!(diff_review_bar.btn_diff_reject));
        if btn_diff_rej.clicked(actions)
            && let Some(active_file) = self.reviewed_file().or_else(|| self.active_file_path())
        {
            let _ = self.reject_diff_review(cx, &active_file);
        }

        // The bar runs the same handlers as the catalog's three disk actions.
        if painel_design
            .button(id!(disk_change_bar.btn_disk_keep))
            .clicked(actions)
        {
            self.keep_mine(cx);
        } else if painel_design
            .button(id!(disk_change_bar.btn_disk_reload))
            .clicked(actions)
        {
            self.reload_from_disk(cx);
        } else if painel_design
            .button(id!(disk_change_bar.btn_disk_compare))
            .clicked(actions)
        {
            self.compare_with_disk(cx);
        }

        self.sizes_actions(cx, actions);
        self.translations_actions(cx, actions);
    }
}
