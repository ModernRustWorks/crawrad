//! The link between the form and the code — ADR 0015.
//!
//! The double-click on the event, the handler created in the user's file, and the cursor
//! landing INSIDE its body. It is the reason the editor exists in this shell.
//!
//! It is part of `impl IdeApp`, not a new type: the shell remains
//! ONE, and the child module reaches its private fields.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-link-ideapp-event-line-at")]
    pub(super) fn event_line_at(&self, p: DVec2) -> Option<usize> {
        self.rects_dos_eventos
            .iter()
            .find(|(_, r)| r.contains(p))
            .map(|(i, _)| *i)
    }

    #[note("ide-app-link-ideapp-inherited-event-line-at")]
    pub(super) fn inherited_event_line_at(&self, p: DVec2) -> Option<usize> {
        let i = self.event_line_at(p)?;
        let rows = self.linhas_do_inspector();
        matches!(rows.get(i), Some(inspector::Row::Event(e)) if e.base_handler.is_some())
            .then_some(i)
    }

    #[note("ide-app-link-ideapp-open-handler")]
    pub(super) fn open_handler(&mut self, cx: &mut Cx, jump: &crawrad_actions::HandlerJump) {
        if !jump.inherited {
            return self.abrir_no_codigo(cx, &jump.name);
        }
        let base = self
            .host
            .doc()
            .and_then(|d| d.ir().inherits().map(str::to_string));
        let (Some(form), Some(base)) = (self.host.active_file(), base) else {
            return;
        };
        let unit = form.with_file_name(format!("{base}.rs"));
        let text = std::fs::read_to_string(&unit).unwrap_or_default();
        let found = crawrad_codegen::elo::localizar_handler(&text, &jump.name);
        self.abrir_no_editor(cx, unit.clone());
        self.pouso_pendente = found
            .as_ref()
            .map(|d| PousoCodigo::cursor(d.linha, d.coluna));
        self.status = Some(match found {
            Some(d) => format!("{} — {}:{}", jump.name, unit.display(), d.linha),
            None => format!("{} is not in {}", jump.name, unit.display()),
        });
    }

    #[note("ide-app-link-ideapp-open-component-menu")]
    pub(super) fn open_component_menu(&mut self, cx: &mut Cx, at: DVec2) -> bool {
        let canvas = self.canvas();
        if self.host.center().is_text() || !canvas.area().rect(cx).contains(at) {
            return false;
        }
        let t = canvas.borrow::<DesignCanvas>().map(|c| c.transformacao());
        let hit = t.zip(self.host.doc()).and_then(|(t, d)| {
            let (x, y) = crawrad_runtime_makepad::canvas::point_in_design(&t, at);
            self.designer
                .hit(d.ir(), x, y)
                .filter(|n| *n != d.ir().root)
        });
        let Some(node) = hit else {
            return false;
        };
        self.host.select_one(node);
        self.sincronizar(cx);
        if self.badge_hint.take().is_some() {
            self.ui.tooltip(id!(ide_tooltip)).hide(cx);
        }
        self.open_menu = Some(Aberto::ComponentContext { em: at });
        self.show_menu(cx);
        true
    }

    #[note("ide-app-link-ideapp-hint-inherited-badge")]
    pub(super) fn hint_inherited_badge(&mut self, cx: &mut Cx, at: DVec2) {
        let canvas = self.canvas();
        let over = (self.open_menu.is_none() && !self.host.center().is_text())
            .then(|| canvas.borrow::<DesignCanvas>()?.inherited_badge_at(at))
            .flatten();
        if over == self.badge_hint {
            return;
        }
        let tip = self.ui.tooltip(id!(ide_tooltip));
        let base = self
            .host
            .doc()
            .and_then(|d| d.ir().inherits().map(str::to_string));
        match (&over, base) {
            (Some(_), Some(base)) => {
                let text = format!("Inherited from {base}");
                tip.show_with_options(cx, at + dvec2(12.0, 20.0), &text);
            }
            _ => tip.hide(cx),
        }
        self.badge_hint = over;
    }

    #[note("ide-app-link-ideapp-show-inspector-tab")]
    pub(super) fn show_inspector_tab(&mut self, cx: &mut Cx, tab: InspectorTab) {
        self.aba = tab;
        self.aplicar_arranjo(cx, crawrad_actions::LayoutCommand::Show(Panel::Inspector));
        self.edicao = None;
        self.edicao_de_evento = None;
        self.tab_button(tab)
            .as_radio_button()
            .select(cx, &mut Scope::empty());
        self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
    }

    #[note("ide-app-link-ideapp-abrir-handler-da-linha")]
    pub(super) fn abrir_handler_da_linha(&mut self, cx: &mut Cx, i: usize) {
        let linhas = self.linhas_do_inspector();
        let Some(inspector::Row::Event(e)) = linhas.get(i) else {
            return;
        };
        let Some(no) = self.host.doc().and_then(|d| d.selection().first().cloned()) else {
            return;
        };
        let evento = e.event;
        if e.is_inherited() {
            let args =
                crawrad_actions::Args::text("node", no).and("event", Value::Str(evento.into()));
            return self.execute_with(cx, "event.override", args);
        }
        let ligado = e.handler.trim().to_string();
        let handler = if ligado.is_empty() {
            let Some(nome) = self
                .host
                .doc()
                .and_then(|d| d.ir().node(&no))
                .map(|n| n.name().to_string())
            else {
                return;
            };
            crawrad_codegen::handler_padrao(&nome, evento)
        } else {
            ligado.clone()
        };
        drop(linhas);
        if ligado.is_empty() {
            self.bind_event(cx, no, evento, &handler);
        }
        self.abrir_no_codigo(cx, &handler);
    }

    #[note("ide-app-link-ideapp-bind-event")]
    pub(super) fn bind_event(&mut self, cx: &mut Cx, no: NodeId, evento: &str, handler: &str) {
        let args = crawrad_actions::Args::text("node", no)
            .and("event", Value::Str(evento.to_string()))
            .and("handler", Value::Str(handler.to_string()));
        self.execute_with(cx, "event.bind", args);
        self.sincronizar(cx);
        self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspector_list));
    }

    #[note("ide-app-link-ideapp-abrir-handler-do-no")]
    pub(super) fn abrir_handler_do_no(&mut self, cx: &mut Cx, no: &NodeId) {
        if let Some(classe) = self
            .host
            .doc()
            .and_then(|d| d.ir().node(no))
            .map(|n| n.class.clone())
        {
            if classe == CLASSE_IMAGE_LIST {
                self.open_imagelist(cx, no.clone());
                return;
            }
            let reg = self.designer.registry.as_deref();
            // the ActionList is a flat collection too: its Actions have no children
            if reg.is_some_and(|r| action_list::is_flat_collection(r, &classe)) {
                self.open_actions(cx, no.clone());
                return;
            }
            let tem_colecao = reg.and_then(|r| r.get(&classe)).is_some_and(|d| {
                d.props
                    .iter()
                    .any(|p| matches!(p.kind, crawrad_registry::PropKind::Collection { .. }))
            });
            if tem_colecao {
                self.open_items(cx, no.clone());
                return;
            }
        }
        let achado = self.host.doc().and_then(|d| d.ir().node(no)).and_then(|n| {
            let desc = self.designer.registry.as_deref()?.get(&n.class)?;
            Some((
                crawrad_codegen::elo::elo_do_duplo_clique(n, desc),
                n.name().to_string(),
            ))
        });
        let Some((elo, nome)) = achado else { return };
        let Some(elo) = elo else {
            self.status = Some(format!("{nome} has no events to wire"));
            return;
        };
        if !elo.ja_ligado {
            self.bind_event(cx, no.clone(), &elo.evento, &elo.handler);
        }
        self.abrir_no_codigo(cx, &elo.handler);
    }

    #[note("ide-app-link-ideapp-abrir-no-codigo")]
    pub(super) fn abrir_no_codigo(&mut self, cx: &mut Cx, handler: &str) {
        let Some(caminho) = self.host.active_file().map(|p| p.to_path_buf()) else {
            self.status = Some("code: no form open — a form lives in <member>/src/".into());
            return;
        };
        let unidade = source::unit_path(&caminho);
        let achar = |u: &std::path::Path| {
            std::fs::read_to_string(u).ok().and_then(|t| {
                let def = crawrad_codegen::elo::localizar_handler(&t, handler);
                let dig = crawrad_codegen::elo::ponto_de_digitacao(&t, handler);
                // The typing point first: the cursor lands inside the body, at its
                // indentation, ready to type (the classic designer's double-click).
                match (def, dig) {
                    (_, Some(dig)) => Some((
                        crate::ide_app::PousoCodigo::cursor(dig.linha, dig.coluna),
                        dig.linha,
                        dig.coluna,
                    )),
                    (Some(d), None) => {
                        let l_idx = d.linha.saturating_sub(1);
                        let line_text = t.lines().nth(l_idx).unwrap_or_default();
                        let start_byte = line_text.find(|c: char| !c.is_whitespace()).unwrap_or(0);
                        let end_byte = if let Some(brace) = line_text.find('{') {
                            brace + 1
                        } else {
                            line_text.trim_end().len()
                        };
                        Some((
                            crate::ide_app::PousoCodigo::selection(d.linha, start_byte, end_byte),
                            d.linha,
                            d.coluna,
                        ))
                    }
                    (None, None) => None,
                }
            })
        };
        let mut local = achar(&unidade);
        let criado = local.is_none();
        if criado {
            self.gerar_projeto(cx);
            if self
                .status
                .as_deref()
                .is_some_and(crawrad_project::generator::refused)
            {
                return;
            }
            local = achar(&unidade);
        }
        self.abrir_no_editor(cx, unidade.clone());
        self.pouso_pendente = local.as_ref().map(|(p, _, _)| *p);
        self.status = Some(match local {
            Some((_, l_num, c_num)) => format!(
                "{}{handler} — {}:{}:{}",
                if criado { "created " } else { "" },
                unidade.display(),
                l_num,
                c_num
            ),
            // The ADR 0015 case: the user renamed the `fn` by hand.
            None => format!("{handler} is not in {}", unidade.display()),
        });
    }

    #[note("ide-app-link-ideapp-pousar-cursor")]
    pub(super) fn pousar_cursor(&mut self, cx: &mut Cx, pouso: &crate::ide_app::PousoCodigo) {
        use makepad_code_editor::code_view::CodeView;
        // A tab just opened left its kept caret pending: this landing replaces it.
        self.pouso_pendente = self.pouso_pendente.and(Some(*pouso));
        let vista = self.code_view();
        let Some(mut cv) = vista.borrow_mut::<CodeView>() else {
            return;
        };
        cv.lazy_init_session();
        let Some(sessao) = cv.session.as_ref() else {
            return;
        };
        let p_start = makepad_code_editor::text::Position {
            line_index: pouso.linha,
            byte_index: pouso.start_byte,
        };
        sessao.set_selection(
            p_start,
            makepad_code_editor::selection::Affinity::Before,
            makepad_code_editor::session::SelectionMode::Simple,
            makepad_code_editor::history::NewGroup::Yes,
        );
        if pouso.end_byte > pouso.start_byte {
            let p_end = makepad_code_editor::text::Position {
                line_index: pouso.linha,
                byte_index: pouso.end_byte,
            };
            sessao.move_to(
                p_end,
                makepad_code_editor::selection::Affinity::After,
                makepad_code_editor::history::NewGroup::No,
            );
            cv.set_landing_line(pouso.linha, pouso.start_byte, pouso.end_byte);
        } else {
            cv.set_landing_line(pouso.linha, 0, 0);
        }
        cv.editor.reveal_cursor(cx);
        let area = cv.editor.area();
        drop(cv);
        cx.set_key_focus(area);
        vista.redraw(cx);
    }
}
