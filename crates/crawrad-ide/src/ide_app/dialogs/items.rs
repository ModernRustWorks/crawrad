//! The ITEMS EDITOR — the `ListBox`, `ComboBox`, and `TreeView` collection.
//!
//! It is part of `impl IdeApp`: the shell remains ONE, and the child
//! module accesses its private members. The `desenhar`/`atender` pair of each dialog
//! lives together, because whoever changes one changes the other.

use super::super::*;
use crate::items_editor::ItemEdit;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-items-ideapp-open-items")]
    pub(in super::super) fn open_items(&mut self, cx: &mut Cx, no: NodeId) {
        let Some(registro) = self.designer.registry.as_deref() else {
            return;
        };
        let novo = self
            .host
            .doc()
            .and_then(|d| d.ir().node(&no))
            .map(|n| ItemsEditorState::from_node(&no, n, registro));
        let Some(estado) = novo else { return };
        self.items = Some(estado);
        self.items_na_tela = None;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-items-ideapp-draw-items")]
    pub(in super::super) fn draw_items(&mut self, cx: &mut Cx) {
        let aberto = self.items.is_some();
        self.show_veil(cx, id!(items_dialog), aberto);
        let Some(estado) = self.items.as_ref() else {
            return;
        };
        let titulo = items_editor::editor_title(&estado.node_class);
        let raiz = self.form_name();
        let dono = format!("{raiz}.{}", estado.node_name);
        let selecionado = estado.selected.clone();
        let fields = items_editor::selected_fields(estado);

        let dlg = self.ui.view(id!(items_dialog.dlg));
        dlg.label(id!(items_title)).set_text(cx, &titulo);
        dlg.label(id!(items_owner)).set_text(cx, &dono);
        dlg.label(id!(props_note)).set_text(cx, ITEMS_NOTA);
        for (caminho, rotulo, _) in ITEMS_BOTOES {
            dlg.button(caminho()).set_text(cx, rotulo);
        }
        dlg.button(id!(items_ok)).set_text(cx, ITEMS_OK);
        dlg.button(id!(items_cancel)).set_text(cx, ITEMS_CANCEL);
        dlg.button(id!(items_apply)).set_text(cx, ITEMS_APPLY);
        let tem_sel = fields.has_selection();
        for (caminho, enabled) in [
            (id!(btn_new_subitem), fields.can_add_child),
            (id!(btn_delete), tem_sel),
            (id!(btn_move_up), tem_sel),
            (id!(btn_move_down), tem_sel),
            (id!(item_expanded), fields.expanded.is_some()),
            (id!(image_label), fields.image_index.is_some()),
            (id!(item_image), fields.image_index.is_some()),
        ] {
            Self::habilitar(cx, &dlg.widget(caminho), enabled);
        }

        if self.items_na_tela == selecionado {
            return;
        }
        self.items_na_tela.clone_from(&selecionado);
        dlg.text_input(id!(item_text))
            .set_text(cx, fields.text.as_deref().unwrap_or_default());
        dlg.check_box(id!(item_expanded))
            .set_active(cx, fields.expanded.unwrap_or(true));
        dlg.text_input(id!(item_image))
            .set_text(cx, &fields.image_text());
    }

    #[note("ide-app-dialogs-items-ideapp-atender-items")]
    pub(in super::super) fn atender_items(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.items.is_none() {
            return;
        }
        let dlg = self.ui.view(id!(items_dialog.dlg));

        let typed = |caminho: &[LiveId]| dlg.text_input(caminho).changed(actions);
        let ticked = dlg.check_box(id!(item_expanded)).changed(actions);
        let edit = [
            typed(id!(item_text)).map(ItemEdit::Text),
            ticked.map(ItemEdit::Expanded),
            typed(id!(item_image)).map(ItemEdit::ImageIndex),
        ];
        if let Some(edit) = edit.into_iter().flatten().next() {
            if let Some(e) = self.items.as_mut() {
                items_editor::edit_selected(e, edit);
            }
            self.redraw_items(cx);
        }

        let lista = dlg.portal_list(id!(items_list));
        if let Some(i) = clicked_row(&lista, actions)
            && let Some(e) = self.items.as_mut()
            && let Some(path) = items_editor::rows(&e.items).get(i).map(|l| l.path.clone())
        {
            e.selected = Some(path);
            self.items_na_tela = None;
            self.redraw_items(cx);
        }

        let mut mexeu = false;
        for (caminho, _, cmd) in ITEMS_BOTOES {
            if dlg.button(caminho()).clicked(actions) {
                if let Some(e) = self.items.as_mut() {
                    items_editor::run(e, cmd);
                }
                mexeu = true;
                break;
            }
        }
        if mexeu {
            self.items_na_tela = None;
            self.redraw_items(cx);
        }

        let aplicar = dlg.button(id!(items_apply)).clicked(actions);
        let ok = dlg.button(id!(items_ok)).clicked(actions);
        let cancelar = dlg.button(id!(items_cancel)).clicked(actions);
        if aplicar || ok {
            self.gravar_items(cx);
        }
        if ok || cancelar {
            self.fechar_pad(cx);
        }
    }

    #[note("ide-app-dialogs-items-ideapp-gravar-items")]
    pub(in super::super) fn gravar_items(&mut self, cx: &mut Cx) {
        let Some(estado) = self.items.as_ref() else {
            return;
        };
        let (no, valor) = (
            estado.node_id.clone(),
            crawrad_ir::Item::list_to(&estado.items),
        );
        let alvos = self.alvos_da_edicao(&no);
        self.write_property(cx, alvos, ITEMS_KEY, valor);
    }

    #[note("ide-app-dialogs-items-ideapp-redraw-items")]
    pub(in super::super) fn redraw_items(&mut self, cx: &mut Cx) {
        self.sincronizar(cx);
        self.ui
            .view(id!(items_dialog.dlg))
            .portal_list(id!(items_list))
            .redraw(cx);
        self.ui.redraw(cx);
    }
}
