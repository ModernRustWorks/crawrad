//! **THE EDITOR KEYBOARD** — the modern editor navigation the framework leaves dead.
//!
//! The request, and the framework's empty arms that made it necessary:
//! the shell module notes, under *The editor keyboard*.

use makepad_widgets::{Cx, Event, KeyCode, KeyModifiers, LiveId, WidgetRef, id};

pub(super) use crawrad_host::word_motion::target as alvo;

pub(super) use crawrad_host::editor_chords::Command;
#[cfg(test)]
pub(super) use crawrad_host::editor_chords::LineDirection;
use crawrad_runtime_makepad::keys::matches_shortcut;

/// The chord, or `None` when this key belongs to the framework or to the IDE.
pub(super) fn comando(key_code: KeyCode, m: &KeyModifiers) -> Option<Command> {
    command_with_scheme(key_code, m, super::teclado::active_scheme())
}

/// The chord in a given keymap scheme: see [`crawrad_host::editor_chords`].
pub(super) fn command_with_scheme(
    key_code: KeyCode,
    m: &KeyModifiers,
    scheme: crawrad_actions::KeymapScheme,
) -> Option<Command> {
    let is = |c: &str| matches_shortcut(c, key_code, m);
    let classic = scheme == crawrad_actions::KeymapScheme::Classic;
    crawrad_host::editor_chords::command(&is, classic)
}

/// A `Ctrl` navigation key: see [`crawrad_host::editor_chords::navigation`].
#[cfg(test)]
pub(super) fn navegacao(
    key_code: KeyCode,
    m: &KeyModifiers,
) -> Option<crawrad_host::editor_chords::Navigation> {
    crawrad_host::editor_chords::navigation(&|c: &str| matches_shortcut(c, key_code, m))
}

impl super::IdeApp {
    pub(super) fn tecla_do_editor_de_texto(&mut self, cx: &mut Cx, event: &Event) -> bool {
        use makepad_code_editor::code_view::CodeView;

        if self.rolar_completar(cx, event) {
            return true;
        }
        let Event::KeyDown(ke) = event else {
            return false;
        };
        // Ctrl+Shift+N over a noted item's card opens that note.
        let note_keys =
            ke.modifiers.control && ke.modifiers.shift && ke.key_code.to_char(false) == Some('n');
        if note_keys && self.open_hover_note(cx) {
            return true;
        }
        // A key closes the hover card, a modifier alone does not (the
        // Ctrl+Shift+N above); Esc only does that.
        let modifier = matches!(
            ke.key_code,
            KeyCode::Control | KeyCode::Shift | KeyCode::Alt | KeyCode::Logo
        );
        if !modifier && self.close_hover(cx) && ke.key_code == KeyCode::Escape {
            return true;
        }
        // Esc closes the parameter card — after the completion list,
        // which Esc closes first, as in modern editors.
        if ke.key_code == KeyCode::Escape && !self.complete.aberto && self.close_signature(cx) {
            return true;
        }
        // Intercepts diff review gate shortcuts (Tab to accept, Esc to reject)
        if let Some(active_file) = self.reviewed_file() {
            if ke.key_code == KeyCode::Tab
                || (ke.key_code == KeyCode::ReturnKey && ke.modifiers.control)
            {
                let _ = self.accept_diff_review(cx, &active_file);
                return true;
            }
            if ke.key_code == KeyCode::Escape {
                let _ = self.reject_diff_review(cx, &active_file);
                return true;
            }
        }
        if ke.key_code == KeyCode::Escape && self.find.aberto {
            self.close_find(cx);
            return true;
        }
        if self.complete.aberto {
            match ke.key_code {
                KeyCode::Escape => {
                    self.complete.aberto = false;
                    self.complete.quiet_pending = false;
                    self.draw_autocomplete(cx);
                    self.ui.redraw(cx);
                    return true;
                }
                KeyCode::ArrowDown => {
                    if !self.complete.sugestoes.is_empty() {
                        let total = self.complete.sugestoes.len();
                        self.complete.selecionado = (self.complete.selecionado + 1) % total;
                        self.draw_autocomplete(cx);
                        self.ui.redraw(cx);
                    }
                    return true;
                }
                KeyCode::ArrowUp => {
                    if !self.complete.sugestoes.is_empty() {
                        let total = self.complete.sugestoes.len();
                        self.complete.selecionado = if self.complete.selecionado == 0 {
                            total.saturating_sub(1)
                        } else {
                            self.complete.selecionado - 1
                        };
                        self.draw_autocomplete(cx);
                        self.ui.redraw(cx);
                    }
                    return true;
                }
                KeyCode::ReturnKey | KeyCode::Tab => {
                    self.aplicar_autocomplete_selecionado(cx);
                    return true;
                }
                KeyCode::ArrowLeft
                | KeyCode::ArrowRight
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => {
                    self.complete.aberto = false;
                    self.draw_autocomplete(cx);
                    self.ui.redraw(cx);
                }
                _ => {}
            }
        }
        // A snippet being filled: Tab to its next stop, Esc out.
        let (tab, esc) = (ke.key_code == KeyCode::Tab, ke.key_code == KeyCode::Escape);
        if (tab || esc) && !ke.modifiers.shift && self.snippet_key(cx, tab) {
            return true;
        }
        let Some(pedido) = comando(ke.key_code, &ke.modifiers) else {
            return false;
        };
        if !self.editor_com_foco(cx) {
            return false;
        }
        let vista: WidgetRef = self.painel(super::Panel::Design).widget(id!(code_view));
        let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
            return false;
        };
        cv.lazy_init_session();
        let Some(sessao) = cv.session.as_ref() else {
            return false;
        };
        match pedido {
            Command::Move(n) => {
                let de = match sessao.selections().first() {
                    Some(s) => (s.cursor.position.line_index, s.cursor.position.byte_index),
                    None => (0, 0),
                };
                let (linha, byte) = {
                    let texto = sessao.document().as_text();
                    alvo(texto.as_lines(), de, n.motion)
                };
                let destino = makepad_code_editor::text::Position {
                    line_index: linha,
                    byte_index: byte,
                };
                use makepad_code_editor::history::NewGroup;
                use makepad_code_editor::selection::Affinity;
                if n.extend {
                    sessao.move_to(destino, Affinity::Before, NewGroup::Yes);
                } else {
                    sessao.set_selection(
                        destino,
                        Affinity::Before,
                        makepad_code_editor::session::SelectionMode::Simple,
                        NewGroup::Yes,
                    );
                }
            }
            Command::ToggleComment
            | Command::MoveLines(_)
            | Command::DuplicateLines(_)
            | Command::DeleteLine => {
                let Some(arquivo) = self.editavel.clone() else {
                    return true;
                };
                let (start_line, end_line, cursor_col) = match sessao.selections().first() {
                    Some(s) => {
                        let (s_line, e_line) = crawrad_host::line_edits::selection_line_range(
                            s.start().line_index,
                            s.end().line_index,
                            s.end().byte_index,
                        );
                        (s_line, e_line, s.cursor.position.byte_index)
                    }
                    None => (0, 0, 0),
                };

                let texto = self.texto_na_vista.clone();
                let mut linhas: Vec<String> = texto.lines().map(|s| s.to_string()).collect();
                if linhas.is_empty() {
                    linhas.push(String::new());
                }

                let (novas_linhas, nova_linha_cursor) = crawrad_host::line_edits::apply_line_edit(
                    pedido, &linhas, start_line, end_line,
                )
                .expect("unhandled line edit command");

                let novo_texto = novas_linhas.join("\n");
                self.host.set_buffer(arquivo, novo_texto.clone());
                self.texto_na_vista = novo_texto.clone();

                drop(cv);
                vista.set_text(cx, &novo_texto);
                let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
                    return true;
                };
                cv.lazy_init_session();
                if let Some(sess) = cv.session.as_ref() {
                    let max_byte = novas_linhas.get(nova_linha_cursor).map_or(0, |l| l.len());
                    let byte = cursor_col.min(max_byte);
                    let pos = makepad_code_editor::text::Position {
                        line_index: nova_linha_cursor,
                        byte_index: byte,
                    };
                    sess.set_selection(
                        pos,
                        makepad_code_editor::selection::Affinity::Before,
                        makepad_code_editor::session::SelectionMode::Simple,
                        makepad_code_editor::history::NewGroup::Yes,
                    );
                }
                cv.editor.keep_cursor_in_view =
                    makepad_code_editor::code_editor::KeepCursorInView::Once;
                drop(cv);
                self.refresh_change_bars(cx);
                self.redraw_editor_tabs(cx);
                vista.redraw(cx);
                return true;
            }
            Command::NextOccurrence => {
                let (primeira, ultima) = {
                    let sels = sessao.selections();
                    (sels.first().copied(), sels.last().copied())
                };
                let Some(primeira) = primeira else {
                    return false;
                };
                if primeira.is_empty() {
                    let pos = primeira.cursor.position;
                    sessao.set_selection(
                        pos,
                        makepad_code_editor::selection::Affinity::Before,
                        makepad_code_editor::session::SelectionMode::Word,
                        makepad_code_editor::history::NewGroup::Yes,
                    );
                } else {
                    let p1 = primeira.start();
                    let p2 = primeira.end();
                    let ultima = ultima.unwrap_or(primeira);
                    let apos = (ultima.end().line_index, ultima.end().byte_index);
                    let antes = (p1.line_index, p1.byte_index);

                    let (ocorrencia, termo_len) = {
                        let texto = sessao.document().as_text();
                        let linhas = texto.as_lines();
                        if p1.line_index == p2.line_index && p1.byte_index < p2.byte_index {
                            if let Some(linha_str) = linhas.get(p1.line_index) {
                                if let Some(termo) = linha_str.get(p1.byte_index..p2.byte_index) {
                                    let res = crawrad_host::line_edits::next_occurrence(
                                        linhas, termo, apos, antes,
                                    );
                                    (res, termo.len())
                                } else {
                                    (None, 0)
                                }
                            } else {
                                (None, 0)
                            }
                        } else {
                            (None, 0)
                        }
                    };

                    if let Some((l, b)) = ocorrencia {
                        let start_pos = makepad_code_editor::text::Position {
                            line_index: l,
                            byte_index: b,
                        };
                        let end_pos = makepad_code_editor::text::Position {
                            line_index: l,
                            byte_index: b + termo_len,
                        };
                        sessao.add_selection(
                            start_pos,
                            makepad_code_editor::selection::Affinity::Before,
                            makepad_code_editor::session::SelectionMode::Simple,
                        );
                        sessao.move_to(
                            end_pos,
                            makepad_code_editor::selection::Affinity::After,
                            makepad_code_editor::history::NewGroup::No,
                        );
                    }
                }
                cv.editor.keep_cursor_in_view =
                    makepad_code_editor::code_editor::KeepCursorInView::Once;
                drop(cv);
                vista.redraw(cx);
                return true;
            }
            Command::OpenFind(modo_substituir) => {
                let termo = {
                    let sels = sessao.selections();
                    if let Some(s) = sels.first() {
                        if !s.is_empty() && s.start().line_index == s.end().line_index {
                            let texto = sessao.document().as_text();
                            let linhas = texto.as_lines();
                            linhas
                                .get(s.start().line_index)
                                .and_then(|l| l.get(s.start().byte_index..s.end().byte_index))
                                .map(|t| t.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };
                drop(cv);
                self.open_find(cx, modo_substituir, termo);
                return true;
            }
            Command::FindNext => {
                drop(cv);
                if let Some((l, b)) = self.find.proxima() {
                    self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
                    self.draw_find_replace(cx);
                    self.ui.redraw(cx);
                }
                return true;
            }
            Command::FindPrevious => {
                drop(cv);
                if let Some((l, b)) = self.find.anterior() {
                    self.selecionar_ocorrencia(cx, l, b, self.find.busca.len());
                    self.draw_find_replace(cx);
                    self.ui.redraw(cx);
                }
                return true;
            }
            Command::OpenGoto => {
                drop(cv);
                self.open_goto(cx);
                return true;
            }
            Command::Bookmark { number, set } => {
                drop(cv);
                let command = if set {
                    crawrad_actions::EditorCommand::ToggleBookmark(number)
                } else {
                    crawrad_actions::EditorCommand::GoToBookmark(number)
                };
                self.editor_command(cx, Some(command));
                return true;
            }
            Command::RunAction(id) => {
                drop(cv);
                self.execute_action(cx, id);
                return true;
            }
            Command::Fold => {
                sessao.fold();
                drop(cv);
                vista.redraw(cx);
                return true;
            }
            Command::Unfold => {
                sessao.unfold();
                drop(cv);
                vista.redraw(cx);
                return true;
            }
            Command::RenameSymbol => {
                drop(cv);
                self.rename_symbol(cx);
                return true;
            }
            Command::Complete => {
                drop(cv);
                self.complete.ignorar_proximo_text_input = true;
                self.disparar_autocomplete(cx);
                return true;
            }
        }
        cv.editor.keep_cursor_in_view = makepad_code_editor::code_editor::KeepCursorInView::Once;
        drop(cv);
        vista.redraw(cx);
        true
    }
}

#[cfg(test)]
#[path = "../../test/ide_app/editor_keys.rs"]
mod test;
