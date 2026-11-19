//! Autocomplete and IntelliSense in the Code Editor, as in classic RAD IDEs.
//!
//! Contextual suggestions for form properties and methods (self.), Rust keywords,
//! primitive types, and visual components from the CrawRAD library.

use super::*;
use crawrad_insight::item::filtered;
use crawrad_lsp::complete::Trigger;

/// How many rows the completion list draws (`comp_item_0` … `comp_item_9`).
const COMPLETION_SLOTS: usize = 10;

/// The path of completion row `i`, as the DSL names it.
fn completion_slot(i: usize) -> [LiveId; 2] {
    [
        live_id!(comp_rows),
        LiveId::from_str(&format!("comp_item_{i}")),
    ]
}

/// The list's item types live in `crawrad-insight` (pure); re-exported for the shell.
pub use crawrad_insight::item::{ItemKind, ItemSugestao};

/// The list's state is pure, so it lives in `crawrad-insight` (it paid the
/// shell's ratchet with it).
pub use crawrad_insight::list::EditorComplete;

impl IdeApp {
    /// Triggers autocomplete at the current cursor position (Ctrl+Space) as in classic RAD IDEs.
    pub(crate) fn disparar_autocomplete(&mut self, cx: &mut Cx) {
        use makepad_code_editor::code_view::CodeView;

        let vista = self.code_view();
        let (linha_idx, prefixo, contexto, start_byte, col_idx) = {
            let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
                return;
            };
            cv.lazy_init_session();
            let Some(ref sessao) = cv.session else {
                return;
            };
            let at = sessao.selections().first().map(|s| s.cursor.position);
            let (l_idx, b_idx) = at.map_or((0, 0), |p| (p.line_index, p.byte_index));
            let doc_texto = sessao.document().as_text();
            let linha_str = doc_texto.as_lines().get(l_idx).map_or("", String::as_str);
            let (ctx, pref, s_byte) = crawrad_insight::typing::dot_context(linha_str, b_idx);
            (l_idx, pref, ctx, s_byte, b_idx)
        };

        // 1. What the IDE already knows, at once (ADR 0050): the component's
        // fields as the generator emitted them — never a guess from the name.
        let rad = contexto
            .as_deref()
            .filter(|base| *base != "self")
            .map(|base| self.membros_do_componente(base))
            .unwrap_or_default();
        let mut candidatos = rad.clone();
        // After `x.` only the server knows the type: no loose words.
        let quiet_trigger = self.complete.quiet && self.lsp.is_some();
        if candidatos.is_empty() && !quiet_trigger && (contexto.is_none() || self.lsp.is_none()) {
            let text = self.texto_na_vista.clone();
            candidatos = EditorComplete::coletar_itens(&prefixo, &text, contexto.as_deref());
        }

        // 2. rust-analyzer answers on its own thread, in UTF-16 columns; its
        // items join this list when they arrive (`receber_completar_do_lsp`).
        self.complete.rad = rad;
        self.complete.esperando_lsp = false;
        let asked = self.ask_completion(linha_idx, col_idx, self.complete.trigger);
        if asked && candidatos.is_empty() && !self.complete.quiet {
            candidatos.push(ItemSugestao::novo("rust-analyzer…", ItemKind::Pending));
        }
        let quiet = std::mem::take(&mut self.complete.quiet) && self.complete.esperando_lsp;
        self.complete.quiet_pending = quiet && candidatos.is_empty();
        self.complete.trigger = Trigger::Invoked;

        if candidatos.is_empty() && !quiet {
            self.complete.aberto = false;
            self.draw_autocomplete(cx);
            self.refresh(cx);
            return;
        }

        drop(vista);
        self.open_list(cx, candidatos, (linha_idx, start_byte), prefixo);
    }

    /// The code templates at the caret (`Ctrl+J`): the shipped set and
    /// the user's file, filtered by the word before the caret (all of them
    /// when nothing matches). The language server is not asked: Enter writes
    /// the body as a snippet and Tab walks its stops.
    pub(crate) fn open_templates(&mut self, cx: &mut Cx) {
        use makepad_code_editor::code_view::CodeView;
        let view = self.code_view();
        let at = {
            let Some(mut cv) = view.borrow_mut::<CodeView>() else {
                return;
            };
            cv.lazy_init_session();
            let Some(ref sessao) = cv.session else {
                return;
            };
            let at = sessao.selections().first().map(|s| s.cursor.position);
            let (line, byte) = at.map_or((0, 0), |p| (p.line_index, p.byte_index));
            let text = sessao.document().as_text();
            let line_text = text.as_lines().get(line).map_or("", String::as_str);
            let (_, prefix, start) = crawrad_insight::typing::dot_context(line_text, byte);
            (line, start, prefix)
        };
        let (line, mut start, mut prefix) = at;
        let user = crawrad_project::code_templates::read_user();
        let all = crawrad_insight::templates::items(&crawrad_insight::templates::all(&user));
        if filtered(&all, &prefix).is_empty() {
            // Nothing starts so: the whole list, and Enter writes at the caret.
            start += prefix.len();
            prefix.clear();
        }
        self.complete.ticket += 1; // a late answer from the server is dropped
        (self.complete.esperando_lsp, self.complete.incompleto) = (false, false);
        self.complete.quiet_pending = false;
        self.complete.rad.clear();
        self.open_list(cx, all, (line, start), prefix);
    }

    /// Opens the list with `items` for the word starting at `at`
    /// (line, byte), `typed` already typed.
    fn open_list(
        &mut self,
        cx: &mut Cx,
        items: Vec<ItemSugestao>,
        at: (usize, usize),
        typed: String,
    ) {
        use makepad_code_editor::code_view::CodeView;
        let (line, start_byte) = at;
        let view = self.code_view();
        // Placed when it opens, then frozen (it does not chase the caret): at
        // the start of the word, in the container's coordinates.
        let word = view.borrow::<CodeView>().and_then(|cv| {
            let a = cv.place_anchor(line, start_byte)?;
            Some(([a.at.x, a.at.y], [cv.cell_size().x, a.line_height]))
        });
        drop(view);
        let container = self.painel(Panel::Design).widget(id!(code_container));
        let r = container.area().rect(cx);
        let [_, _, w, h] = self.complete.area;
        let size = if w > 0.0 { [w, h] } else { [480.0, 220.0] };
        if let Some((word, cell)) = word {
            let frame = xywh(r);
            let a = crawrad_insight::anchor::place(word, 0, cell, frame, size);
            (self.complete.pos_x, self.complete.pos_y) = (a.left, a.top);
            self.complete.word_at = a.word;
        }
        self.complete.linha_abertura = line;
        self.complete.aberto = !items.is_empty();
        self.complete.prefixo = typed;
        self.complete.start_byte = start_byte;
        (self.complete.top, self.complete.top_sel) = (0, 0);
        self.complete.sugestoes = filtered(&items, &self.complete.prefixo);
        self.complete.todos = items;
        self.complete.selecionado = 0;
        self.draw_autocomplete(cx);
        self.refresh(cx);
    }

    /// The members of the form component `base` (`crawrad_insight::component_items`),
    /// with the `.designer.rs` beside the file in the editor.
    fn membros_do_componente(&self, base: &str) -> Vec<ItemSugestao> {
        let designer = self.editavel.as_ref().and_then(|arq| {
            let stem = arq.file_stem()?.to_str()?;
            std::fs::read_to_string(arq.with_file_name(format!("{stem}.designer.rs"))).ok()
        });
        let form = self.host.doc().map(|d| d.ir());
        crawrad_insight::component_items(form, designer.as_deref(), base)
    }

    /// After every edit: the open list follows what is typed, as VS Code's —
    /// narrowed with each key, closed when the caret leaves the name; a `.`
    /// after a name opens a new one by itself.
    pub(crate) fn completar_acompanha_digitacao(&mut self, cx: &mut Cx) {
        let (line, byte) = self.cursor_do_code_view(cx);
        let line_text = self.texto_na_vista.lines().nth(line).unwrap_or("");
        let before = line_text.get(..byte).unwrap_or(line_text).to_string();
        if self.complete.quiet_pending
            && (line != self.complete.linha_abertura
                || crawrad_insight::typing::typed_since(&before, self.complete.start_byte)
                    .is_none())
        {
            self.complete.quiet_pending = false;
        }
        if self.complete.aberto {
            let typed = (line == self.complete.linha_abertura)
                .then(|| crawrad_insight::typing::typed_since(&before, self.complete.start_byte))
                .flatten();
            let shown = typed.map(|t| (t.to_string(), filtered(&self.complete.todos, t)));
            match shown {
                Some((t, list)) if !list.is_empty() || self.complete.incompleto => {
                    self.complete.prefixo = t;
                    self.complete.sugestoes = list;
                    self.complete.selecionado = 0;
                    // LSP `isIncomplete`: asked again at the caret, ranked for
                    // what is typed now; the filtered list stays until then.
                    if self.complete.incompleto {
                        self.ask_completion(line, byte, Trigger::Incomplete);
                    }
                }
                _ => self.complete.aberto = false,
            }
            self.draw_autocomplete(cx);
            self.ui.redraw(cx);
        }
        // Without a server the dot still opens the form's fields.
        let triggers = self
            .lsp
            .as_ref()
            .map_or_else(|| vec![".".into()], |l| l.completion_triggers());
        if let Some((c, how)) = crawrad_insight::typing::typed_trigger(&before, &triggers)
            && !self.complete.aberto
        {
            self.complete.trigger = Trigger::Character(c);
            self.complete.quiet = how == crawrad_insight::typing::Opening::Quiet;
            self.disparar_autocomplete(cx);
        }
    }

    /// The wheel over the open list scrolls it, three rows a notch, and the
    /// editor below does not scroll. `true` = the event was the list's.
    pub(crate) fn rolar_completar(&mut self, cx: &mut Cx, event: &Event) -> bool {
        let Event::Scroll(e) = event else {
            return false;
        };
        let c_box = self.painel(Panel::Design).widget(id!(code_container));
        let rect = c_box.view(id!(completion_box)).area().rect(cx);
        if !self.complete.aberto || !rect.contains(e.abs) || e.scroll.y == 0.0 {
            return false;
        }
        let rows = if e.scroll.y > 0.0 { 3 } else { -3 };
        let (total, sel) = (self.complete.sugestoes.len(), self.complete.selecionado);
        self.complete.top =
            crawrad_insight::typing::list_top(self.complete.top, sel, false, rows, total);
        self.draw_autocomplete(cx);
        self.ui.redraw(cx);
        true
    }

    /// Asks rust-analyzer for the list at `(line, byte)` of the view's text,
    /// in the server's columns (UTF-16 by default). The ticket moves on either
    /// way, so an older answer is dropped; `false` = no server to ask.
    fn ask_completion(&mut self, line: usize, byte: usize, why: Trigger) -> bool {
        self.complete.ticket += 1;
        let (Some(lsp), Some(arq)) = (self.lsp.clone(), self.editavel.clone()) else {
            return false;
        };
        let line_text = self.texto_na_vista.lines().nth(line).unwrap_or("");
        let at = crawrad_lsp::Position {
            line: u32::try_from(line).unwrap_or(u32::MAX),
            character: lsp.position_encoding().column(line_text, byte),
        };
        self.complete.esperando_lsp = true;
        let ticket = self.complete.ticket;
        self.lsp_hints
            .completion
            .request(lsp, &arq, at, ticket, why);
        true
    }

    /// rust-analyzer's items for the open list, when they arrive: the
    /// component's own fields first, then the server's, without repeats.
    pub(crate) fn receber_completar_do_lsp(&mut self, cx: &mut Cx) {
        if !self.complete.esperando_lsp {
            return;
        }
        let Some(answer) = self.lsp_hints.completion.take(self.complete.ticket) else {
            return;
        };
        let items = answer.items;
        self.complete.incompleto = answer.incomplete;
        self.complete.esperando_lsp = false;
        // A quiet trigger (`(`, `'`) opened nothing yet: it shows only now, and only with items.
        let opens = std::mem::take(&mut self.complete.quiet_pending) && !items.is_empty();
        if !self.complete.aberto && !opens {
            return;
        }
        self.complete.aberto = true;
        let mut list = std::mem::take(&mut self.complete.rad);
        for it in crawrad_insight::item::from_lsp(items) {
            if !list.iter().any(|i| i.rotulo == it.rotulo) {
                list.push(it);
            }
        }
        let shown = filtered(&list, &self.complete.prefixo);
        if !shown.is_empty() {
            self.complete.sugestoes = shown;
            self.complete.todos = list;
            self.complete.selecionado = 0;
        } else if self
            .complete
            .sugestoes
            .iter()
            .all(|i| i.kind == ItemKind::Pending)
        {
            self.complete.aberto = false;
        }
        self.draw_autocomplete(cx);
        self.ui.redraw(cx);
    }

    /// Draws the floating autocomplete popup directly below the cursor (classic RAD IDEs / VS Code).
    pub(crate) fn draw_autocomplete(&mut self, cx: &mut Cx) {
        let code_container = self.painel(Panel::Design).widget(id!(code_container));
        let comp_box = code_container.view(id!(completion_box));
        let visivel = self.complete.aberto && code_container.visible();
        comp_box.set_visible(cx, visivel);
        if !visivel {
            return self.follow_item_doc(cx);
        }

        // Placement frozen on opening: the box does NOT follow the cursor
        let (pos_x, pos_y) = (self.complete.pos_x, self.complete.pos_y);
        comp_box.apply_over(cx, live! { margin: { left: (pos_x), top: (pos_y) } });

        let (total, sel) = (self.complete.sugestoes.len(), self.complete.selecionado);

        let r = comp_box.area().rect(cx);
        self.complete.area = xywh(r);
        let moved = sel != self.complete.top_sel;
        let start = crawrad_insight::typing::list_top(self.complete.top, sel, moved, 0, total);
        (self.complete.top, self.complete.top_sel) = (start, sel);

        for i in 0..COMPLETION_SLOTS {
            let row = comp_box.view(&completion_slot(i));
            let Some(it) = self.complete.sugestoes.get(start + i) else {
                row.set_visible(cx, false);
                continue;
            };
            let cols = it.columns();
            let c = crawrad_insight::item::RowColours::of(start + i == sel);
            let [bg, hover, text, soft, icon] = [c.bg, c.hover, c.text, c.soft, it.kind.colour()]
                .map(|[r, g, b, a]| vec4(r, g, b, a));
            // Widths in the row's own font; the first frame has no rect yet.
            let row_width = Some(row.area().rect(cx).size.x).filter(|w| *w > 0.0);
            let laid = cols.layout(row_width.unwrap_or(462.0), |s| {
                let l = self
                    .measure
                    .layout(cx, 0., 0., None, false, Align::default(), s);
                l.size_in_lpxs.width as f64
            });
            let (from, to) = laid.strike;
            // The button under the columns: the row's colour and the strike.
            row.button(id!(bg)).apply_over(
                cx,
                live! {
                    draw_bg: {
                        color: (bg), color_hover: (hover),
                        strike_from: (from), strike_to: (to), strike_color: (text),
                    }
                },
            );
            let cells = [
                ("icon", cols.icon, icon),
                ("label", cols.label.as_str(), text),
                ("origin", &cols.origin, soft),
                ("signature", &laid.signature, soft),
            ];
            for (name, shown, colour) in cells {
                let cell = row.label(&[live_id!(cols), LiveId::from_str(name)]);
                cell.set_text(cx, shown);
                cell.apply_over(cx, live! { draw_text: { color: (colour) } });
            }
            row.set_visible(cx, true);
            row.redraw(cx);
        }
        let (height, top) = crawrad_insight::item::thumb(total, COMPLETION_SLOTS, start, 22.0);
        let thumb = comp_box.view(id!(comp_track.comp_thumb));
        thumb.apply_over(cx, live! { height: (height), margin: { top: (top) } });
        self.follow_item_doc(cx);
    }

    /// The list as the MCP reads it, its box measured now, not before a draw.
    pub(crate) fn completion_json(&mut self, cx: &mut Cx) -> serde_json::Value {
        let comp_box = self
            .painel(Panel::Design)
            .widget(id!(code_container.completion_box));
        let r = comp_box.area().rect(cx);
        self.complete.area = xywh(r);
        self.complete.to_json()
    }

    /// Enter/Tab: writes the selected item and closes the list. With nothing
    /// to write (only the `rust-analyzer…` row) it only closes the list.
    pub(crate) fn aplicar_autocomplete_selecionado(&mut self, cx: &mut Cx) {
        let escolha = self
            .complete
            .sugestoes
            .get(self.complete.selecionado)
            .filter(|it| self.complete.aberto && it.kind != ItemKind::Pending)
            .map(|it| {
                (
                    it.insert_text.clone().unwrap_or_else(|| it.rotulo.clone()),
                    it.snippet,
                )
            });
        self.complete.aberto = false;
        self.draw_autocomplete(cx);
        if let Some((escolha, snippet)) = escolha {
            self.inserir_sugestao_no_buffer(cx, &escolha, snippet);
        }
    }

    /// A snippet being filled: Tab selects its next stop (the last
    /// is `$0`, where it ends), Esc leaves it. `false`: no snippet, the key
    /// goes on.
    pub(crate) fn snippet_key(&mut self, cx: &mut Cx, tab: bool) -> bool {
        use makepad_code_editor::code_view::CodeView;
        use makepad_code_editor::{history::NewGroup, selection::Affinity};
        let view = self.code_view();
        let Some(cv) = view.borrow::<CodeView>() else {
            return false;
        };
        let Some(sess) = cv
            .session
            .as_ref()
            .filter(|s| s.document().snippet_active())
        else {
            return false;
        };
        match sess.document().snippet_next().filter(|_| tab) {
            Some((a, b)) => {
                let simple = makepad_code_editor::session::SelectionMode::Simple;
                sess.set_selection(a, Affinity::Before, simple, NewGroup::Yes);
                sess.move_to(b, Affinity::Before, NewGroup::No);
            }
            None => sess.document().end_snippet(),
        }
        drop(cv);
        view.redraw(cx);
        true
    }

    fn inserir_sugestao_no_buffer(&mut self, cx: &mut Cx, escolha: &str, snippet: bool) {
        use makepad_code_editor::code_view::CodeView;
        use makepad_code_editor::{history::NewGroup, selection::Affinity};
        let vista = self.code_view();
        {
            let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
                return;
            };
            cv.lazy_init_session();
            let Some(sess) = cv.session.as_mut() else {
                return;
            };
            let Some(caret) = sess.selections().first().map(|s| s.cursor.position) else {
                return;
            };
            if caret.line_index != self.complete.linha_abertura
                || caret.byte_index < self.complete.start_byte
            {
                return;
            }
            let mut start = caret;
            start.byte_index = self.complete.start_byte;
            let simple = makepad_code_editor::session::SelectionMode::Simple;
            sess.set_selection(start, Affinity::Before, simple, NewGroup::Yes);
            sess.move_to(caret, Affinity::Before, NewGroup::No);
            // A snippet: expanded (the editor indents its lines), its
            // stops placed on the lines as they landed, the first selected.
            let expanded = crawrad_insight::snippet::expand(escolha, "");
            let written = if snippet {
                expanded.text.as_str()
            } else {
                escolha
            };
            sess.insert(written.into());
            sess.handle_changes(); // the insert reaches the selections first
            if snippet {
                let all = sess.document().as_text().as_lines().to_vec();
                let from = (start.line_index, start.byte_index);
                let stops = crawrad_insight::snippet::placed(written, &expanded.stops, from, &all);
                sess.document().start_snippet(&stops);
                if let Some((a, b)) = sess.document().snippet_next() {
                    sess.set_selection(a, Affinity::Before, simple, NewGroup::Yes);
                    sess.move_to(b, Affinity::Before, NewGroup::No);
                }
            }
            cv.editor.keep_cursor_in_view =
                makepad_code_editor::code_editor::KeepCursorInView::Once;
        }
        vista.redraw(cx);
        self.colher_do_editor(cx);
        self.refresh(cx);
    }

    /// Handles mouse clicks on Code Insight items.
    pub(crate) fn atender_autocomplete(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.complete.aberto {
            return;
        }
        let code_container = self.painel(Panel::Design).widget(id!(code_container));
        let comp_box = code_container.view(id!(completion_box));

        let start = self.complete.top;
        for i in 0..COMPLETION_SLOTS {
            if comp_box
                .view(&completion_slot(i))
                .button(id!(bg))
                .clicked(actions)
            {
                let idx = start + i;
                if idx < self.complete.sugestoes.len() {
                    self.complete.selecionado = idx;
                    self.aplicar_autocomplete_selecionado(cx);
                }
                return;
            }
        }
    }

    /// Closes the popups of the caret: a file switch must not carry them over.
    pub(crate) fn close_caret_popups(&mut self, cx: &mut Cx) {
        if self.complete.aberto {
            self.complete.aberto = false;
            self.complete.quiet_pending = false;
            self.draw_autocomplete(cx);
            self.follow_item_doc(cx);
        }
        self.close_signature(cx);
        if matches!(self.open_menu, Some(super::Aberto::CodeActions { .. })) {
            self.close_menu(cx);
        }
        self.ui.redraw(cx);
    }
}
