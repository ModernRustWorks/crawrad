//! The ACTION LIST EDITOR and the standard classes window.
//!
//! It is a part of `impl IdeApp`: the shell remains ONE, and the child
//! module reaches its private members. The `desenhar`/`atender` pair of each dialog
//! lives together, because whoever changes one changes the other.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-actions-ideapp-open-actions")]
    pub(in super::super) fn open_actions(&mut self, cx: &mut Cx, lista: NodeId) {
        if let Some(d) = self.host.doc() {
            self.actions_stack =
                action_list::owners_after_open(d.ir(), &self.actions_stack, &lista);
        }
        self.actions = Some(lista);
        self.categoria_de_acao = action_list::CAT_ALL.to_string();
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-actions-ideapp-linhas-das-acoes")]
    pub(in super::super) fn linhas_das_acoes(&self) -> (Vec<String>, Vec<action_list::ActionRow>) {
        let (Some(lista), Some(d)) = (self.actions.as_ref(), self.host.doc()) else {
            return (Vec::new(), Vec::new());
        };
        action_list::rows(d.ir(), lista, &self.categoria_de_acao)
    }

    #[note("ide-app-dialogs-actions-ideapp-linhas-padrao")]
    pub(in super::super) fn linhas_padrao(&self) -> Vec<action_list::StandardRow> {
        action_list::standard_rows(&|cat| self.grupos_padrao_fechados.contains(cat), &|nome| {
            self.classes_marcadas.contains(nome)
        })
    }

    #[note("ide-app-dialogs-actions-ideapp-open-standard")]
    pub(in super::super) fn open_standard(&mut self, cx: &mut Cx) {
        self.standard = true;
        self.classes_marcadas.clear();
        self.grupos_padrao_fechados = crawrad_registry::standard_actions_by_category()
            .into_iter()
            .map(|(cat, _)| cat.to_string())
            .filter(|cat| !action_list::group_starts_open(cat))
            .collect();
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-actions-ideapp-draw-actions")]
    pub(in super::super) fn draw_actions(&mut self, cx: &mut Cx) {
        let aberto = self.actions.is_some();
        self.show_veil(cx, id!(actions_dialog), aberto);
        let Some(lista) = self.actions.clone() else {
            return;
        };
        let grid = self
            .host
            .design_rows(self.membro().as_deref())
            .remove(&lista);
        let (Some(reg), Some(d)) = (self.designer.registry.as_deref(), self.host.doc()) else {
            return;
        };
        let columns = grid.as_ref().map(|g| g.columns());
        let owners = action_list::owners_after_open(d.ir(), &self.actions_stack, &lista);
        let st =
            action_list::state_with_owners(d.ir(), reg, &owners, self.host.selection(), columns);
        let dlg = self.ui.view(id!(actions_dialog.dlg));
        dlg.button(id!(act_back)).set_visible(cx, st.back);
        dlg.label(id!(crumb_prev)).set_text(cx, &st.crumb_prev);
        dlg.label(id!(actions_title)).set_text(cx, &st.title);
        dlg.button(id!(act_close)).set_text(cx, ACT_CLOSE);
        for (caminho, rotulo) in ACT_BOTOES.iter().zip(st.buttons) {
            dlg.button(caminho()).set_text(cx, rotulo);
            dlg.button(caminho()).set_visible(cx, !rotulo.is_empty());
        }
        dlg.view(id!(cat_column)).set_visible(cx, st.categories);
        dlg.label(id!(act_label)).set_text(cx, st.list_label);
        dlg.label(id!(act_footer_note)).set_text(cx, st.footer);
        // WHAT CANNOT BE DONE REMAINS, AND REMAINS DIMMED.
        Self::habilitar(cx, &dlg.widget(id!(act_fields)), st.can_add_fields);
        Self::habilitar(cx, &dlg.widget(id!(act_delete)), st.can_delete);
        Self::habilitar(cx, &dlg.widget(id!(act_up)), st.can_up);
        Self::habilitar(cx, &dlg.widget(id!(act_down)), st.can_down);
    }

    #[note("ide-app-dialogs-actions-ideapp-draw-standard")]
    pub(in super::super) fn draw_standard(&mut self, cx: &mut Cx) {
        let aberto = self.standard;
        self.show_veil(cx, id!(standard_dialog), aberto);
        if !aberto {
            return;
        }
        let dlg = self.ui.view(id!(standard_dialog.dlg));
        dlg.button(id!(std_ok)).set_text(cx, ITEMS_OK);
        dlg.button(id!(std_cancel)).set_text(cx, ITEMS_CANCEL);
        Self::habilitar(
            cx,
            &dlg.widget(id!(std_ok)),
            !self.classes_marcadas.is_empty(),
        );
    }

    #[note("ide-app-dialogs-actions-ideapp-atender-actions")]
    pub(in super::super) fn atender_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let Some(lista) = self.actions.clone() else {
            return;
        };
        let dlg = self.ui.view(id!(actions_dialog.dlg));
        let (categorias, acoes) = self.linhas_das_acoes();

        // CLICKING a category changes the filter of the right column.
        let cats = dlg.portal_list(id!(cat_list));
        if let Some(cat) = clicked_row(&cats, actions).and_then(|i| categorias.get(i)) {
            self.categoria_de_acao = cat.clone();
            self.redraw_actions(cx);
        }
        let lst = dlg.portal_list(id!(act_list));
        if let Some(a) = clicked_row(&lst, actions).and_then(|i| acoes.get(i)) {
            if let Some(d) = self.host.doc_mut() {
                d.select(vec![a.id.clone()]);
            }
            self.redraw_actions(cx);
        }

        if dlg.button(id!(act_new)).clicked(actions) {
            let reg = self.designer.registry.as_deref();
            let add = reg
                .zip(self.host.doc())
                .and_then(|(r, d)| action_list::add_args(d.ir(), r, &lista));
            if let Some((acao, args)) = add {
                self.execute_with(cx, acao, args);
            }
            self.redraw_actions(cx);
        }
        if dlg.button(id!(act_standard)).clicked(actions) {
            self.open_standard(cx);
        }
        if dlg.button(id!(act_delete)).clicked(actions) {
            self.execute_with(
                cx,
                action_list::ACTION_DELETE,
                crawrad_actions::Args::none(),
            );
            self.redraw_actions(cx);
        }
        let subir = dlg.button(id!(act_up)).clicked(actions);
        let descer = dlg.button(id!(act_down)).clicked(actions);
        if subir || descer {
            self.mover_acao(cx, &lista, if subir { -1 } else { 1 });
        }
        if dlg.button(id!(act_fields)).clicked(actions) {
            let dt = self.host.design_rows(self.membro().as_deref());
            if let Some(gr) = dt.get(&lista) {
                let args = action_list::add_all_fields_args(&lista, gr.columns(), gr.kinds());
                self.execute_with(cx, action_list::ACTION_ADD_ALL_FIELDS, args);
                self.redraw_actions(cx);
            }
        }
        if dlg.button(id!(act_back)).clicked(actions)
            && let Some((up, came_from)) = action_list::owners_after_back(&self.actions_stack)
        {
            (self.actions, self.actions_stack) = (up.last().cloned(), up);
            self.categoria_de_acao = action_list::CAT_ALL.to_string();
            if let Some(d) = self.host.doc_mut() {
                d.select(vec![came_from]);
            }
            self.refresh(cx);
        }
        if dlg.button(id!(act_close)).clicked(actions) {
            self.actions = None;
            self.actions_stack.clear();
            self.refresh(cx);
        }
    }

    #[note("ide-app-dialogs-actions-ideapp-mover-acao")]
    pub(in super::super) fn mover_acao(&mut self, cx: &mut Cx, lista: &NodeId, passo: i32) {
        let Some(d) = self.host.doc() else { return };
        let Some(acao) = action_list::selected(d.ir(), lista, self.host.selection()) else {
            return;
        };
        let args = crawrad_actions::Args::text("node", acao).and("step", Value::Int(passo.into()));
        self.execute_with(cx, action_list::ACTION_REORDER, args);
        self.redraw_actions(cx);
    }

    #[note("ide-app-dialogs-actions-ideapp-atender-standard")]
    pub(in super::super) fn atender_standard(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.standard {
            return;
        }
        let dlg = self.ui.view(id!(standard_dialog.dlg));
        let linhas = self.linhas_padrao();
        let lista = dlg.portal_list(id!(std_list));
        if lista.any_items_with_actions(actions) {
            let mut marcar: Option<(&'static str, bool)> = None;
            let mut dobrar: Option<String> = None;
            for (i, item) in lista.items_with_actions(actions) {
                match linhas.get(i) {
                    Some(action_list::StandardRow::Group { name, .. })
                        if item.button(id!(alvo)).clicked(actions) =>
                    {
                        dobrar = Some((*name).to_string());
                    }
                    Some(action_list::StandardRow::Class { name, .. }) => {
                        if let Some(v) = item.check_box(id!(marca)).changed(actions) {
                            marcar = Some((name, v));
                        }
                    }
                    _ => {}
                }
            }
            if let Some((nome, ligada)) = marcar {
                if ligada {
                    self.classes_marcadas.insert(nome.to_string());
                } else {
                    self.classes_marcadas.remove(nome);
                }
                self.redraw_standard(cx);
            }
            if let Some(cat) = dobrar {
                if !self.grupos_padrao_fechados.remove(&cat) {
                    self.grupos_padrao_fechados.insert(cat);
                }
                self.redraw_standard(cx);
            }
        }

        if dlg.button(id!(std_ok)).clicked(actions) {
            let args =
                crawrad_actions::Args::texts("classes", self.classes_marcadas.iter().cloned());
            self.execute_with(cx, action_list::ACTION_STANDARD, args);
            self.close_standard(cx);
        }
        if dlg.button(id!(std_cancel)).clicked(actions) {
            self.close_standard(cx);
        }
    }

    pub(in super::super) fn close_standard(&mut self, cx: &mut Cx) {
        self.standard = false;
        self.classes_marcadas.clear();
        self.redraw_actions(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-dialogs-actions-ideapp-habilitar")]
    pub(in super::super) fn habilitar(cx: &mut Cx, botao: &WidgetRef, ligado: bool) {
        botao.as_button().set_enabled(cx, ligado);
        botao.clone().set_disabled(cx, !ligado);
    }

    pub(in super::super) fn redraw_actions(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        let dlg = self.ui.view(id!(actions_dialog.dlg));
        dlg.portal_list(id!(cat_list)).redraw(cx);
        dlg.portal_list(id!(act_list)).redraw(cx);
        self.ui.redraw(cx);
    }

    pub(in super::super) fn redraw_standard(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        self.ui
            .view(id!(standard_dialog.dlg))
            .portal_list(id!(std_list))
            .redraw(cx);
        self.ui.redraw(cx);
    }

    // --- The LINES EDITOR — the classic `String List Editor` ---
}
