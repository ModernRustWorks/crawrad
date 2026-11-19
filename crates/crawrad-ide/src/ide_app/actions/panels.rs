//! Dock panel actions: Activity rail, Explorer, Package Manager, and close buttons.

use super::super::*;
use crawrad_note::note;

/// Two clicks on one project line within this make a double click.
const DOUBLE_CLICK: std::time::Duration = std::time::Duration::from_millis(450);

impl IdeApp {
    #[note("ide-app-actions-panels-ideapp-handle-panels-actions")]
    pub(in super::super) fn handle_panels_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let trilho = self.ui.portal_list(id!(activity_rail.rail_list));
        if trilho.any_items_with_actions(actions) {
            let mut escolhido = None;
            let visible_rail = rail::visible(!self.no_form_in_project());
            for (i, item) in trilho.items_with_actions(actions) {
                if item.as_button().clicked(actions) {
                    escolhido = visible_rail.get(i).copied();
                }
            }
            if let Some(painel) = escolhido {
                self.execute_with(
                    cx,
                    "layout.toggle_panel",
                    crawrad_actions::Args::text("panel", painel.id()),
                );
                self.sincronizar(cx);
            }
        }

        let comp_item = self.dock().item(layout::tab_of(Panel::Components));
        if comp_item.button(id!(no_form.new_form)).clicked(actions) {
            self.execute_action(cx, crawrad_project::creation::NEW_FORM_ACTION);
            self.reabrir_no_canvas(cx);
            self.sincronizar(cx);
        }

        let explorador = self
            .dock()
            .item(layout::tab_of(Panel::Explorer))
            .portal_list(id!(explorer_body.explorer_list));
        if explorador.any_items_with_actions(actions) {
            let linhas = self.linhas_do_explorer();
            let mut escolhida = None;
            let mut nota = None;
            let mut activate = None;
            for (i, item) in explorador.items_with_actions(actions) {
                if item.button(id!(alvo)).clicked(actions) {
                    let now = std::time::Instant::now();
                    let double = self
                        .last_member_click
                        .is_some_and(|(row, at)| row == i && now.duration_since(at) < DOUBLE_CLICK);
                    let member = linhas
                        .get(i)
                        .is_some_and(|l| l.template().starts_with("Member"));
                    self.last_member_click = member.then_some((i, now));
                    if member && double {
                        // The double click activates; the first click already
                        // opened or closed the line, so no second fold.
                        self.last_member_click = None;
                        activate = self.explorer_row_targets.get(i).cloned().flatten();
                        continue;
                    }
                    escolhida = linhas.get(i).map(|l| l.activation());
                    nota = linhas.get(i).and_then(|l| l.note());
                }
            }
            if let Some((path, _)) = activate {
                self.set_active_project(cx, path);
            }
            match escolhida {
                Some(explorer::Activation::Run(acao, args)) => {
                    self.execute_with(cx, acao, args);
                    self.sincronizar(cx);
                }
                Some(explorer::Activation::OpenForm(caminho)) => {
                    let _ = self.abrir_formulario(cx, &caminho);
                }
                Some(explorer::Activation::Inert) | None => {}
            }
            if let Some(frase) = nota {
                self.status = Some(frase);
                self.sincronizar(cx);
            }
        }

        // THE PACKAGE MANAGER: the two tabs, the search, and the two buttons.
        let loja_aberta = !self.dock().item(layout::tab_of(Panel::Packages)).is_empty();
        if loja_aberta {
            self.acoes_dos_pacotes(cx, actions);
        }

        let mut fechar = None;
        for p in Self::FECHAVEIS {
            if self
                .dock()
                .item(layout::tab_of(*p))
                .button(id!(btn_panel_close))
                .clicked(actions)
            {
                fechar = Some(*p);
            }
        }
        if let Some(p) = fechar {
            self.execute_with(
                cx,
                "layout.hide_panel",
                crawrad_actions::Args::text("panel", p.id()),
            );
            self.sincronizar(cx);
        }

        self.debug_bar_clicks(cx, actions);
        self.debug_list_clicks(cx, actions);
        self.references_list_clicks(cx, actions);
        self.problems_list_clicks(cx, actions);
        self.notes_actions(cx, actions);
        self.tests_actions(cx, actions);
        self.form_check_actions(cx, actions);
        self.extension_views_actions(cx, actions);

        // THE PROJECT LINE'S ACTIONS: New File, New Form, New Folder land in
        // the project of the line clicked (the workspace line: its root).
        let exp_item = self.dock().item(layout::tab_of(Panel::Explorer));
        let list = exp_item.portal_list(id!(explorer_body.explorer_list));
        let mut create = None;
        let mut dots_at = DVec2::default();
        for (i, item) in list.items_with_actions(actions) {
            let target = || {
                self.explorer_row_targets
                    .get(i)
                    .cloned()
                    .flatten()
                    .map(|(p, _)| p)
            };
            if item.button(id!(btn_exp_new_file)).clicked(actions) {
                create = Some((0, target()));
            } else if item.button(id!(btn_exp_new_form)).clicked(actions) {
                create = Some((1, target()));
            } else if item.button(id!(btn_exp_new_folder)).clicked(actions) {
                create = Some((2, target()));
            } else if item.button(id!(btn_ws_more)).clicked(actions) {
                let r = item.button(id!(btn_ws_more)).area().rect(cx);
                dots_at = r.pos + dvec2(0.0, r.size.y);
                create = Some((5, target()));
            } else if item.button(id!(btn_exp_refresh)).clicked(actions) {
                create = Some((3, None));
            } else if item.button(id!(btn_exp_collapse)).clicked(actions) {
                create = Some((4, None));
            }
        }
        match create.clone() {
            Some((0, at)) => self.create_new_file(cx, at.as_deref()),
            Some((1, at)) => {
                self.criar_novo_formulario(cx, at.as_deref());
            }
            Some((2, at)) => self.create_new_folder(cx, at.as_deref()),
            // The workspace's "⋯": its right-click menu, under the dots.
            Some((5, Some(root))) => {
                let em = dots_at;
                self.open_menu = Some(Aberto::ExplorerContext { is_dir: true, em });
                self.explorer_context_target = Some((root, true));
                self.show_menu(cx);
            }
            _ => {}
        }
        if create.as_ref().is_some_and(|c| c.0 == 3) {
            self.status = Some("Explorer refreshed".into());
            exp_item
                .portal_list(id!(explorer_body.explorer_list))
                .redraw(cx);
            self.ui.redraw(cx);
        }
        if create.as_ref().is_some_and(|c| c.0 == 4) {
            let documento = self.host.active_file();
            let membro = self.membro();
            self.folds.collapse_all(documento, membro.as_deref());
            self.status = Some("Folders collapsed".into());
            exp_item
                .portal_list(id!(explorer_body.explorer_list))
                .redraw(cx);
            self.ui.redraw(cx);
        }
    }
}
