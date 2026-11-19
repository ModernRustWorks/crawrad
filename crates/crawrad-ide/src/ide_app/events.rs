//! The window's events: `MatchEvent` and `AppMain` (`handle_event`).
//! Cut out of `ide_app.rs` as a pure move, as `chat.rs` and `build.rs` were.

use super::*;
use crawrad_note::note;

impl MatchEvent for IdeApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        // The keymap is a preference; the generator target is the project's.
        let pref = crawrad_project::workspace::read_preferences();
        self.esquema_teclado = pref.keymap_scheme;
        self.auto_save = pref.auto_save;
        self.designer.grid = pref.designer_grid as f32;
        (self.designer.snap, self.ver_grade) = (pref.designer_snap, pref.designer_show_grid);
        self.show_tab_order = pref.designer_show_tab_order;
        self.zoom = pref.designer_scale as f32;
        teclado::set_active_scheme(pref.keymap_scheme);
        self.carregar(cx);
        // A folder on the command line is a project to open, not a form.
        let folder = std::env::args().nth(1).map(PathBuf::from);
        if let Some(dir) = folder.filter(|p| p.is_dir()) {
            self.open_project(cx, Some(dir));
        }
        self.aplicar_tema_editor(cx);
        // The remembered Designer zoom reaches the canvas, not just the number.
        self.aplicar_zoom(cx);
        self.apply_history_settings();
        self.montar_cromo(cx);
        if let Some(root) = self.desktop_root.clone() {
            self.restore_desktop(cx, &root, true);
        }
        self.refresh_notes(cx);
        // Window bounds are fitted in handle_event via fit_on_screen_once.
    }

    #[note("ide-app-events-ideapp-handle-draw-2d")]
    fn handle_draw_2d(&mut self, cx: &mut Cx2d) {
        self.draw_2d(cx);
    }

    #[note("ide-app-events-ideapp-handle-key-down")]
    fn handle_key_down(&mut self, cx: &mut Cx, ke: &KeyEvent) {
        if let Some(campo) = &self.campo_focado
            && cx.has_key_focus(campo.area())
        {
            return;
        }
        self.tecla(cx, ke, false);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.handle_input_actions(cx, actions);
        self.handle_menu_actions(cx, actions);
        self.handle_editor_actions(cx, actions);
        self.handle_designer_actions(cx, actions);
        self.handle_panels_actions(cx, actions);
        self.handle_history_actions(cx, actions);
        self.handle_welcome_actions(cx, actions);
        self.handle_chat_actions(cx, actions);
        self.handle_terminal_actions(cx, actions);
        self.handle_dialog_actions(cx, actions);
    }
}

impl AppMain for IdeApp {
    #[note("ide-app-events-ideapp-handle-event")]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        let escalado = self.com_ponteiro_na_escala(cx, event);
        let event = escalado.as_ref().unwrap_or(event);
        if self.pointer_outside_modal(cx, event) {
            return;
        }
        self.ask_before_closing(cx, event);
        if self.mcp_rx.is_none() {
            self.iniciar_mcp(cx);
        }
        self.fit_on_screen_once(cx);
        self.restore_desktop_tabs(cx);
        let welcome = id!(designer_row.welcome_container);
        if self.host.center() == Center::Welcome
            && !self.painel(Panel::Design).view(welcome).visible()
        {
            self.show_welcome(cx);
        }
        if self.initial_sync && !matches!(event, Event::Startup) {
            self.initial_sync = false;
            self.reabrir_no_canvas(cx);
            if self.status.is_none() {
                self.status = crawrad_project::installed::refusal_notice();
            }
            self.sincronizar(cx);
            self.montar_cromo(cx);
            self.warm_up_list_rows(cx);
            // The interface zoom the owner left (an IDE setting), now that the
            // window knows its DPI.
            let scale = crawrad_project::workspace::read_preferences().ide_scale;
            if scale != self.zoom_da_ide {
                self.set_ide_zoom(cx, scale);
                self.sincronizar(cx);
            }
            self.check_environment_on_startup(cx);
        }
        if let Event::WindowDragQuery(dq) = event {
            if self.point_in_window_drag(cx, dq.abs) {
                dq.response.set(WindowDragQueryResponse::Caption);
                cx.set_cursor(MouseCursor::Default);
            }
            return;
        }
        if let Event::MouseMove(mm) = event {
            self.hint_designer_bar(cx, mm.abs);
            self.hint_inherited_badge(cx, mm.abs);
            self.hint_form_check_pin(cx, mm.abs);
            self.hint_menu_row(cx, mm.abs);
            self.hint_toolbox_row(cx, mm.abs);
            // Back to the arrow only on LEAVING our divisor: a dock splitter sets
            // its own resize cursor once, on hover-in, and a reset every move undid it.
            let on_divisor = self.arrastando_divisor || self.mouse_no_divisor(cx, mm.abs);
            if on_divisor || self.cursor_on_divisor {
                let cursor = [MouseCursor::Default, MouseCursor::ColResize];
                cx.set_cursor(cursor[usize::from(on_divisor)]);
            }
            self.cursor_on_divisor = on_divisor;
        }
        if let Event::MouseUp(_) = event {
            self.arrastando_divisor = false;
        }
        if self.menu_keys(cx, event) || self.enter_no_editor_somente_leitura(cx, event) {
            return;
        }
        if let Event::KeyDown(k) = event
            && teclado::is_paste(k.key_code, &k.modifiers)
            && self.chat_prompt_focused(cx)
            && self.paste_image_into_chat(cx)
        {
            return;
        }
        if self.slash_popup_keys(cx, event) || self.tecla_do_editor_de_texto(cx, event) {
            return;
        }
        if let Event::TextInput(_) = event
            && self.complete.ignorar_proximo_text_input
        {
            self.complete.ignorar_proximo_text_input = false;
            return;
        }
        if self.match_event_with_draw_2d(cx, event).is_ok() {
            return;
        }
        let canvas = self.canvas();
        match event.drag_hits(cx, canvas.area()) {
            DragHit::Drag(f) if classe_arrastada(&f.items).is_some() => {
                *f.response.lock().unwrap() = DragResponse::Copy;
            }
            DragHit::Drop(f) => {
                let ponto = canvas.borrow::<DesignCanvas>().map(|c| {
                    crawrad_runtime_makepad::canvas::point_in_design(&c.transformacao(), f.abs)
                });
                if let (Some(classe), Some((x, y))) = (classe_arrastada(&f.items), ponto) {
                    self.colocar(cx, &classe, x, y);
                    self.tool_armed = None;
                }
            }
            _ => {}
        }

        if let Event::MouseDown(md) = event {
            if self.mouse_no_divisor(cx, md.abs) {
                self.arrastando_divisor = true;
                cx.set_cursor(MouseCursor::ColResize);
            }
            if self.open_menu.is_some() && !self.point_in_menu(cx, md.abs) {
                self.close_menu(cx);
            }
            if self.complete.aberto {
                let c_container = self.painel(Panel::Design).widget(id!(code_container));
                let comp_box = c_container.view(id!(completion_box));
                if comp_box.visible() {
                    let box_rect = comp_box.area().rect(cx);
                    if box_rect.size.x > 10.0
                        && box_rect.size.y > 10.0
                        && !box_rect.contains(md.abs)
                    {
                        self.complete.aberto = false;
                        self.draw_autocomplete(cx);
                        self.ui.redraw(cx);
                    }
                }
            }
            let chat_drop = self.painel(Panel::Chat).view(id!(ai_model_dropdown));
            if chat_drop.visible() {
                let drop_rect = chat_drop.area().rect(cx);
                let btn_m = self.chat_tool("btn_ai_model").area().rect(cx);
                let btn_e = self.chat_tool("btn_ai_effort").area().rect(cx);
                let outside_drop = drop_rect.size.x <= 10.0 || !drop_rect.contains(md.abs);
                let outside_m = btn_m.size.x <= 0.0 || !btn_m.contains(md.abs);
                let outside_e = btn_e.size.x <= 0.0 || !btn_e.contains(md.abs);
                if outside_drop && outside_m && outside_e {
                    chat_drop.set_visible(cx, false);
                    self.chat.open_menu = None;
                    self.painel(Panel::Chat).redraw(cx);
                }
            }
            // the history list closes on a click outside it and its clock button
            let (chat, at) = (self.painel(Panel::Chat), md.abs);
            let (popup, clock) = (
                chat.view(id!(ai_history_popup)),
                chat.button(id!(btn_ai_history)),
            );
            let list = popup.area().rect(cx);
            if popup.visible() && !list.contains(at) && !clock.area().rect(cx).contains(at) {
                self.close_history(cx);
            }
            let ctx_popup = self.painel(Panel::Chat).view(id!(ai_context_popup));
            if ctx_popup.visible() {
                let ctx_rect = ctx_popup.area().rect(cx);
                let btn = self.chat_tool("btn_ai_attach");
                let btn_rect = btn.area().rect(cx);
                let outside_popup = if ctx_rect.size.x > 10.0 && ctx_rect.size.y > 10.0 {
                    !ctx_rect.contains(md.abs)
                } else {
                    false
                };
                let outside_btn = if btn_rect.size.x > 0.0 && btn_rect.size.y > 0.0 {
                    !btn_rect.contains(md.abs)
                } else {
                    true
                };
                if outside_popup && outside_btn {
                    ctx_popup.set_visible(cx, false);
                    self.painel(Panel::Chat).redraw(cx);
                }
            }
            let slash_popup = self.painel(Panel::Chat).view(id!(ai_slash_popup));
            if slash_popup.visible() {
                let slash_rect = slash_popup.area().rect(cx);
                let input = self.get_chat_prompt_input();
                let input_rect = input.area().rect(cx);
                let outside_popup = if slash_rect.size.x > 10.0 && slash_rect.size.y > 10.0 {
                    !slash_rect.contains(md.abs)
                } else {
                    false
                };
                let outside_input = if input_rect.size.x > 0.0 && input_rect.size.y > 0.0 {
                    !input_rect.contains(md.abs)
                } else {
                    true
                };
                if outside_popup && outside_input {
                    slash_popup.set_visible(cx, false);
                    self.painel(Panel::Chat).redraw(cx);
                }
            }
            if md.button.is_primary() {
                let duplo = crawrad_runtime_makepad::canvas::is_double_tap(
                    self.ultimo_toque,
                    md.abs,
                    md.time,
                );
                self.ultimo_toque = Some((md.abs, md.time));
                if duplo && let Some(i) = self.event_line_at(md.abs) {
                    self.abrir_handler_da_linha(cx, i);
                }
            }
            if md.button.is_secondary() {
                let alvo = self
                    .painel_do_trilho_em(md.abs)
                    .or_else(|| self.painel_no_corpo_em(cx, &self.arranjo(), md.abs));
                if alvo == Some(Panel::Explorer)
                    && !self
                        .rail_rects
                        .iter()
                        .any(|(p, r)| *p == Panel::Explorer && r.contains(md.abs))
                {
                    let row = self
                        .explorer_rects
                        .iter()
                        .find(|(_, r)| r.contains(md.abs))
                        .and_then(|&(i, _)| self.explorer_row_targets.get(i).cloned().flatten());
                    let root = || self.resolver_caminho_do_explorer(std::path::Path::new(""));
                    let target = row.unwrap_or_else(|| (root(), true));
                    self.open_menu = Some(Aberto::ExplorerContext {
                        is_dir: target.1,
                        em: md.abs,
                    });
                    self.explorer_context_target = Some(target);
                    self.show_menu(cx);
                } else if let Some(row) = self.inherited_event_line_at(md.abs) {
                    self.open_menu = Some(Aberto::EventContext { row, em: md.abs });
                    self.show_menu(cx);
                } else if alvo == Some(Panel::Design)
                    && (self.open_editor_menu(cx, md.abs) || self.open_component_menu(cx, md.abs))
                {
                } else if let Some(painel) = alvo {
                    self.open_menu = Some(Aberto::Painel { painel, em: md.abs });
                    self.show_menu(cx);
                }
            }
        }
        if matches!(event, Event::Signal) {
            self.bombear_pacotes(cx);
            self.bombear_criacao(cx);
            self.bombear_build(cx);
            self.bombear_mcp(cx);
            self.pump_environment(cx);
            self.pump_disk_changes(cx);
            self.pump_debugging(cx);
            self.bombear_audio(cx);
            self.bombear_ia(cx);
            self.pump_doctest(cx);
            self.pump_dataset_columns(cx);
        }
        if self.chat.recording_clock.is_event(event).is_some() && self.chat.audio_gravando {
            self.bombear_audio(cx);
            self.chat.recording_clock = cx.start_timeout(1.0);
        }
        if self.chat.busy_clock.is_event(event).is_some() && self.chat.busy {
            self.show_live_status(cx);
            self.chat.busy_clock = cx.start_timeout(1.0);
        }
        self.auto_save_event(cx, event);
        self.recovery_tick(cx, event);
        self.spin_lsp_icon(cx, event);
        if self.build_clock.is_event(event).is_some() {
            if let Some(status) = self.build.status_line_now() {
                self.status = Some(status);
            }
            self.draw_build(cx);
            if self.build.needs_tick() {
                self.build_clock = cx.start_timeout(0.1);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
        self.follow_diff_review(cx);
        self.bombear(cx);
        self.remake_editor_view(cx);
        if self.host.center().is_text() {
            self.colher_do_editor(cx);
            self.sincronizar_minimap(cx);
            self.escrever_rodape_do_editor(cx);
        }
        if let Some(pouso) = self.pouso_pendente {
            let area = self.with_code_view(|cv| cv.editor.area());
            if area.is_some_and(|a| a != Area::Empty) {
                self.pouso_pendente = None;
                self.pousar_cursor(cx, &pouso);
                self.reveal_after_landing = 3;
                cx.new_next_frame();
            } else {
                // Redraw if code editor area is empty so landing executes next frame.
                self.ui.redraw(cx);
            }
        }
        // A landing on a tab opened this frame happens before the editor is
        // laid out, so its reveal scrolls nothing: reveal again once drawn.
        if self.reveal_after_landing > 0 && matches!(event, Event::NextFrame(_) | Event::Draw(_)) {
            self.reveal_after_landing -= 1;
            let view = self.code_view();
            if let Some(mut cv) = view.borrow_mut::<makepad_code_editor::code_view::CodeView>() {
                cv.editor.reveal_cursor(cx);
            }
            view.redraw(cx);
            self.escrever_rodape_do_editor(cx);
            if self.reveal_after_landing > 0 {
                cx.new_next_frame();
            }
        }
        // Persist dock layout changes when modified.
        self.gravar_arranjo_se_mudou(cx);
        self.lsp_process_diagnostics(cx);
        self.lsp_apply_hints(cx);
    }
}
