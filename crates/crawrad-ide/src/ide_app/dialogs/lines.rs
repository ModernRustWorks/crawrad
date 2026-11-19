use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-lines-ideapp-open-lines")]
    pub(in super::super) fn open_lines(&mut self, cx: &mut Cx, no: NodeId, key: &str) {
        let Some(registro) = self.designer.registry.as_deref() else {
            return;
        };
        let novo = self
            .host
            .doc()
            .and_then(|d| d.ir().node(&no))
            .and_then(|n| {
                let p = registro.get(&n.class)?.prop(key)?;
                Some(LinesEditorState::from_node(&no, n, p))
            });
        let Some(estado) = novo else { return };
        self.lines = Some(estado);
        self.lines_na_tela = false;
        self.refresh(cx);
    }

    #[note("ide-app-dialogs-lines-ideapp-draw-lines")]
    pub(in super::super) fn draw_lines(&mut self, cx: &mut Cx) {
        let aberto = self.lines.is_some();
        self.show_veil(cx, id!(lines_dialog), aberto);
        let Some(estado) = self.lines.as_ref() else {
            return;
        };
        let titulo = lines_editor::editor_title(&estado.node_class, &estado.prop_display);
        let raiz = self.form_name();
        let dono = format!("{raiz}.{}.{}", estado.node_name, estado.prop_display);
        let conta = lines_editor::summary(&estado.text);
        let texto = estado.text.clone();

        let dlg = self.ui.view(id!(lines_dialog.dlg));
        dlg.label(id!(lines_title)).set_text(cx, &titulo);
        dlg.label(id!(lines_owner)).set_text(cx, &dono);
        dlg.label(id!(lines_count)).set_text(cx, &conta);
        dlg.button(id!(lines_ok)).set_text(cx, ITEMS_OK);
        dlg.button(id!(lines_cancel)).set_text(cx, ITEMS_CANCEL);
        dlg.button(id!(lines_apply)).set_text(cx, ITEMS_APPLY);

        if self.lines_na_tela {
            return;
        }
        self.lines_na_tela = true;
        dlg.text_input(id!(lines_text)).set_text(cx, &texto);
    }

    #[note("ide-app-dialogs-lines-ideapp-atender-lines")]
    pub(in super::super) fn atender_lines(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.lines.is_none() {
            return;
        }
        let dlg = self.ui.view(id!(lines_dialog.dlg));
        let campo = dlg.text_input(id!(lines_text));
        if let Some(texto) = campo.changed(actions) {
            if let Some(e) = self.lines.as_mut() {
                e.text = texto;
            }
            self.refresh(cx);
        }
        let aplicar = dlg.button(id!(lines_apply)).clicked(actions);
        let ok = dlg.button(id!(lines_ok)).clicked(actions);
        let cancelar = campo.escaped(actions) || dlg.button(id!(lines_cancel)).clicked(actions);
        if aplicar || ok {
            self.gravar_lines(cx);
        }
        if ok || cancelar {
            self.fechar_pad(cx);
        }
    }

    #[note("ide-app-dialogs-lines-ideapp-gravar-lines")]
    pub(in super::super) fn gravar_lines(&mut self, cx: &mut Cx) {
        let Some(estado) = self.lines.as_ref() else {
            return;
        };
        let (no, key, texto) = (
            estado.node_id.clone(),
            estado.prop_key.clone(),
            estado.text.clone(),
        );
        let resultado = self
            .linhas_do_inspector()
            .into_iter()
            .find_map(|linha| match linha {
                inspector::Row::Prop(p) if p.key == key => Some(p.editor.window_value(&texto)),
                _ => None,
            });
        if let Some(resultado) = resultado {
            self.aplicar_resultado(cx, resultado, Some(&no), &key);
        }
    }

    // --- THE ITEMS EDITOR — the `…` dialog of a collection ---
}
