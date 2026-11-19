//! The keyboard: one key, two destinations (the catalog, then the designer). Cut out of `ide_app.rs`.

use super::*;
use crawrad_note::note;

impl IdeApp {
    // --- THE KEYBOARD — one key, two destinations ---

    #[note("ide-app-key-dispatch-ideapp-editor-com-foco")]
    pub(crate) fn editor_com_foco(&self, cx: &Cx) -> bool {
        let area = self.code_view().area();
        if area != Area::Empty && cx.has_key_focus(area) {
            return true;
        }
        if let Some(sb_area) = self.with_code_view(|cv| cv.editor.area())
            && sb_area != Area::Empty
            && cx.has_key_focus(sb_area)
        {
            return true;
        }
        let c_cont = self.painel(Panel::Design).widget(id!(code_container));
        if self.host.center().is_text() || c_cont.visible() {
            let f_in = c_cont.widget(id!(find_box.find_row.find_input));
            if f_in.area() != Area::Empty && cx.has_key_focus(f_in.area()) {
                return false;
            }
            let r_in = c_cont.widget(id!(find_box.replace_row.replace_input));
            if r_in.area() != Area::Empty && cx.has_key_focus(r_in.area()) {
                return false;
            }
            if self.chat_prompt_focused(cx) {
                return false;
            }
            if self.find.aberto
                || self.options
                || self.confirmando_saida.is_some()
                || self.novo_projeto.is_some()
                || self.memoria_aberta
                || self.environment_open
                || self.mcp_aberto
                || self.nota_dialog_aberta
                || self.shrink_dialog.is_some()
                || self.component_rename.is_some()
            {
                return false;
            }
            return true;
        }
        false
    }

    pub(super) fn chat_prompt_focused(&self, cx: &Cx) -> bool {
        let pr = self.get_chat_prompt_input();
        pr.area() != Area::Empty && cx.has_key_focus(pr.area())
    }

    #[note("ide-app-key-dispatch-ideapp-enter-no-editor-somente-leitura")]
    pub(super) fn enter_no_editor_somente_leitura(&self, cx: &Cx, event: &Event) -> bool {
        let Event::KeyDown(ke) = event else {
            return false;
        };
        if ke.key_code != KeyCode::ReturnKey || self.editavel.is_some() {
            return false;
        }
        self.editor_com_foco(cx)
    }

    #[note("ide-app-key-dispatch-ideapp-tecla")]
    pub(super) fn tecla(&mut self, cx: &mut Cx, ke: &KeyEvent, do_campo: bool) {
        if self.component_rename.is_some() {
            if ke.key_code == KeyCode::Escape {
                self.cancel_component_rename(cx);
                return;
            }
            if ke.key_code == KeyCode::ReturnKey {
                self.execute_component_rename_all(cx);
                return;
            }
        }
        // `Esc` with an open menu CLOSES THE MENU, and does not reach the designer. It is the
        if ke.key_code == KeyCode::Escape {
            if self.close_menu(cx) || self.close_history(cx) {
                return;
            }
            if self.shrink_dialog.is_some() {
                self.close_shrink(cx);
                return;
            }
            if self.environment_open {
                self.close_environment(cx);
                return;
            }
            let chat_drop = self.painel(Panel::Chat).view(id!(ai_model_dropdown));
            if chat_drop.visible() {
                chat_drop.set_visible(cx, false);
                self.chat.open_menu = None;
                self.painel(Panel::Chat).redraw(cx);
                return;
            }
        }
        if self.modal_open() {
            // A modal dialog owns the keyboard: Esc presses its Cancel (or Close),
            // and nothing else reaches the window behind it.
            if ke.key_code == KeyCode::Escape {
                self.press_dialog_cancel(cx);
            }
            return;
        }
        let comandos = teclado::comandos(ke.key_code, &ke.modifiers, do_campo);
        if comandos.is_empty() {
            return;
        }
        self.status = None;
        let no_editor = self.host.center().is_text();
        let teclado_do_texto = self.editor_com_foco(cx);
        for comando in comandos {
            if no_editor && !matches!(comando, teclado::Comando::Acao(_)) {
                continue;
            }
            if let teclado::Comando::Acao(id) = comando
                && teclado::not_here(id, teclado_do_texto, no_editor)
            {
                continue;
            }
            match comando {
                teclado::Comando::Acao(id) => self.execute_action(cx, id),
                teclado::Comando::Seta { dx, dy, modo } => self.seta(dx, dy, modo),
                teclado::Comando::Cancelar => self.cancelar_gesto(cx),
            }
        }
        self.reabrir_no_canvas(cx);
        self.refresh(cx);
    }

    #[note("ide-app-key-dispatch-ideapp-seta")]
    pub(super) fn seta(&mut self, dx: f32, dy: f32, modo: crawrad_designer::ArrowNudge) {
        let Some(d) = self.host.doc() else { return };
        let sel: Vec<NodeId> = d.selection().to_vec();
        let passo = self.designer.arrow_step(modo);
        let tx = if modo == crawrad_designer::ArrowNudge::Resize {
            self.designer
                .nudge_size(d.ir(), &sel, dx * passo, dy * passo)
        } else {
            self.designer.nudge(d.ir(), &sel, dx * passo, dy * passo)
        };
        let Some(tx) = tx else { return };
        if let Some(dm) = self.host.doc_mut()
            && let Err(e) = dm.apply(tx)
        {
            self.status = Some(format!("doc: {e}"));
        }
    }

    #[note("ide-app-key-dispatch-ideapp-cancelar-gesto")]
    pub(super) fn cancelar_gesto(&mut self, cx: &mut Cx) {
        let out = self.designer.cancel();
        if let Some(tx) = out.tx
            && let Some(dm) = self.host.doc_mut()
            && let Err(e) = dm.apply(tx)
        {
            self.status = Some(format!("doc: {e}"));
        }
        self.tool_armed = None;
        let canvas = self.canvas();
        if let Some(mut c) = canvas.borrow_mut::<DesignCanvas>() {
            let rt = c.runtime_mut();
            rt.set_marquee(None);
            rt.set_guides(&[]);
        }
        canvas.redraw(cx);
    }
}
