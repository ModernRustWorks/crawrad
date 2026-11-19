//! **FLOATING FIND & REPLACE FOR THE EDITOR**
//!
//! Pure model and dispatcher for the floating search and replace bar (Ctrl+F / Ctrl+H).

/// The model moved to `crawrad_host::find` (pure, tested there).
pub use crawrad_host::find::EditorFind;

use super::*;

impl IdeApp {
    pub(crate) fn selecionar_ocorrencia(
        &mut self,
        cx: &mut Cx,
        linha: usize,
        byte: usize,
        len: usize,
    ) {
        use makepad_code_editor::code_view::CodeView;
        let vista = self.code_view();
        let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
            return;
        };
        cv.lazy_init_session();
        if let Some(sess) = cv.session.as_ref() {
            let p_start = makepad_code_editor::text::Position {
                line_index: linha,
                byte_index: byte,
            };
            let p_end = makepad_code_editor::text::Position {
                line_index: linha,
                byte_index: byte + len,
            };
            sess.set_selection(
                p_start,
                makepad_code_editor::selection::Affinity::Before,
                makepad_code_editor::session::SelectionMode::Simple,
                makepad_code_editor::history::NewGroup::Yes,
            );
            sess.move_to(
                p_end,
                makepad_code_editor::selection::Affinity::After,
                makepad_code_editor::history::NewGroup::No,
            );
        }
        cv.editor.keep_cursor_in_view = makepad_code_editor::code_editor::KeepCursorInView::Once;
        drop(cv);
        vista.redraw(cx);
    }

    pub(crate) fn open_find(
        &mut self,
        cx: &mut Cx,
        modo_substituir: bool,
        termo_inicial: Option<String>,
    ) {
        self.find
            .abrir(modo_substituir, termo_inicial, &self.texto_na_vista);
        let vista = self.painel(Panel::Design).widget(id!(code_container));
        let find_box = vista.view(id!(find_box));
        find_box.set_visible(cx, true);
        let replace_row = find_box.view(id!(replace_row));
        replace_row.set_visible(cx, self.find.modo_substituir);
        let btn_toggle = find_box.button(id!(btn_toggle_replace));
        btn_toggle.set_text(cx, if self.find.modo_substituir { "v" } else { ">" });
        let lbl_count = find_box.label(id!(find_count));
        lbl_count.set_text(cx, &self.find.contagem_texto());
        let find_input = find_box.text_input(id!(find_input));
        find_input.set_text(cx, &self.find.busca);
        cx.set_key_focus(find_input.area());
        if let Some((l, b)) = self
            .find
            .indice_atual
            .and_then(|i| self.find.ocorrencias.get(i).copied())
        {
            self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
        }
        self.ui.redraw(cx);
    }

    pub(crate) fn close_find(&mut self, cx: &mut Cx) {
        self.find.fechar();
        let vista = self.painel(Panel::Design).widget(id!(code_container));
        vista.view(id!(find_box)).set_visible(cx, false);
        let cv = self.code_view();
        cx.set_key_focus(cv.area());
        self.ui.redraw(cx);
    }

    pub(crate) fn draw_find_replace(&mut self, cx: &mut Cx) {
        let code_container = self.painel(Panel::Design).widget(id!(code_container));
        let find_box = code_container.view(id!(find_box));
        let visivel = self.find.aberto && code_container.visible();
        find_box.set_visible(cx, visivel);
        if visivel {
            find_box
                .view(id!(replace_row))
                .set_visible(cx, self.find.modo_substituir);
            find_box
                .button(id!(btn_toggle_replace))
                .set_text(cx, if self.find.modo_substituir { "v" } else { ">" });
            find_box
                .label(id!(find_count))
                .set_text(cx, &self.find.contagem_texto());
        }
    }

    pub(crate) fn atender_find_replace(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.find.aberto {
            return;
        }
        let code_container = self.painel(Panel::Design).widget(id!(code_container));
        let find_box = code_container.view(id!(find_box));

        if find_box.button(id!(btn_find_close)).clicked(actions) {
            self.close_find(cx);
            return;
        }

        if find_box.button(id!(btn_toggle_replace)).clicked(actions) {
            self.find.alternar_modo_substituir();
            self.draw_find_replace(cx);
            self.ui.redraw(cx);
            return;
        }

        let input_busca = find_box.text_input(id!(find_input));
        if let Some(txt) = input_busca.changed(actions) {
            self.find.busca = txt;
            self.find.atualizar(&self.texto_na_vista);
            if let Some((l, b)) = self
                .find
                .indice_atual
                .and_then(|i| self.find.ocorrencias.get(i).copied())
            {
                self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
            }
            find_box
                .label(id!(find_count))
                .set_text(cx, &self.find.contagem_texto());
            self.ui.redraw(cx);
            return;
        }

        if find_box.button(id!(btn_find_next)).clicked(actions) {
            if let Some((l, b)) = self.find.proxima() {
                self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
                find_box
                    .label(id!(find_count))
                    .set_text(cx, &self.find.contagem_texto());
                self.ui.redraw(cx);
            }
            return;
        }

        if find_box.button(id!(btn_find_prev)).clicked(actions) {
            if let Some((l, b)) = self.find.anterior() {
                self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
                find_box
                    .label(id!(find_count))
                    .set_text(cx, &self.find.contagem_texto());
                self.ui.redraw(cx);
            }
            return;
        }

        let input_repl = find_box.text_input(id!(replace_input));
        if let Some(txt) = input_repl.changed(actions) {
            self.find.substituicao = txt;
        }

        if find_box.button(id!(btn_replace)).clicked(actions) {
            if let Some((novo_texto, (l, b))) = self.find.substituir_atual(&self.texto_na_vista) {
                self.show_edited_source(cx, novo_texto);
                self.selecionar_ocorrencia(cx, l, b, self.find.substituicao.len());
                self.draw_find_replace(cx);
                self.ui.redraw(cx);
            }
            return;
        }

        if find_box.button(id!(btn_replace_all)).clicked(actions) {
            let (novo_texto, _) = self.find.substituir_tudo(&self.texto_na_vista);
            self.show_edited_source(cx, novo_texto);
            self.draw_find_replace(cx);
            self.ui.redraw(cx);
        }
    }
}
