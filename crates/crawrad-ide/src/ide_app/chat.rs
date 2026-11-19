//! The AI CHAT of the shell: slash commands, prompt execution, and audio recording.
//!
//! Extracted from `ide_app.rs` as per ADR 0043.
//! This module is a part of `impl IdeApp`, preserving visibility and private fields.

use super::*;
use crawrad_agent::chat_attachment::{MAX_ATTACHMENTS, SelectionAttachment};
use crawrad_note::note;
use crawrad_os::tasks::wake;

#[note("ide-app-chat-slash")]
pub(crate) enum Slash {
    Done,
    Send(String),
    No,
}

impl IdeApp {
    #[note("ide-app-chat-ideapp-despachar-slash-command")]
    pub(super) fn despachar_slash_command(&mut self, cx: &mut Cx, prompt: &str) -> Slash {
        let trimmed = prompt.trim();
        if !trimmed.starts_with('/') {
            return Slash::No;
        }
        let Some(found) = crawrad_agent::SlashCommand::match_exact(trimmed) else {
            return Slash::No;
        };
        // the first word matched ignoring case, and triggers are ASCII
        let rest = trimmed[found.trigger.len()..].trim();
        match found.id {
            "new" => {
                let chat = self.painel(Panel::Chat);
                for card in [
                    id!(user_card),
                    id!(reply_card),
                    id!(copilot_card),
                    id!(ai_hero_view),
                ] {
                    chat.view(id!(ai_messages_view))
                        .view(card)
                        .set_visible(cx, card == id!(ai_hero_view));
                }
                self.open_new_project(cx, None);
                self.display_chat_response(cx, "⚡ Opening New Project dialog...");
                Slash::Done
            }
            "clear" => {
                self.new_chat_session(cx);
                Slash::Done
            }
            "actions" => {
                let list = |d: &crawrad_doc::Doc| {
                    let mut nodes = d.ir().nodes.iter();
                    nodes
                        .find(|(_, n)| n.class == "ActionList")
                        .map(|(id, _)| id.clone())
                };
                if let Some(id) = self.host.doc().and_then(list) {
                    self.open_actions(cx, id);
                    self.display_chat_response(cx, "⚡ Opening ActionList Editor dialog...");
                } else {
                    self.display_chat_response(
                        cx,
                        "⚠️ No ActionList component found on active form. Add an ActionList component to use this action editor.",
                    );
                }
                Slash::Done
            }
            "memory" => {
                self.open_memory(cx);
                self.display_chat_response(cx, "🧠 Opening Aefos Memory dialog...");
                Slash::Done
            }
            "mcp" => {
                self.open_mcp(cx);
                self.display_chat_response(cx, "🔌 Opening MCP Servers dialog...");
                Slash::Done
            }
            "acp" => {
                if rest.is_empty() {
                    let status = crawrad_agent::slash_commands::format_acp_status(&self.acp_config);
                    self.display_chat_response(cx, &status);
                } else if rest.starts_with("test") {
                    let target = rest.strip_prefix("test").unwrap().trim().to_lowercase();
                    self.display_chat_response(
                        cx,
                        crawrad_agent::slash_commands::acp_probing_message(),
                    );
                    let acp_cfg = self.acp_config.clone();
                    let (probe_tx, probe_rx) = std::sync::mpsc::channel();
                    self.chat.probe_rx = Some(probe_rx);
                    std::thread::spawn(move || {
                        let res = crawrad_agent::slash_commands::acp_probe(&target, &acp_cfg);
                        let _ = probe_tx.send(res);
                        wake();
                    });
                } else if rest.starts_with("ask") {
                    let query = rest.strip_prefix("ask").unwrap().trim();
                    if query.is_empty() {
                        self.display_chat_response(
                            cx,
                            crawrad_agent::slash_commands::acp_ask_usage(),
                        );
                    } else {
                        return Slash::Send(query.to_string());
                    }
                } else {
                    self.display_chat_response(
                        cx,
                        crawrad_agent::slash_commands::acp_commands_help(),
                    );
                }
                Slash::Done
            }
            "use" => {
                let (cfg, slash) = (&self.acp_config, crawrad_agent::slash_commands::use_reply);
                let (switch, said) = slash(rest, &cfg.provider, &cfg.agent_id);
                // the Options' own path: saved, and the live agent warmed anew
                if let Some(p) = switch {
                    self.selecionar_provedor_ia(cx, p);
                }
                self.display_chat_response(cx, &said);
                Slash::Done
            }
            "settings" => {
                self.open_options(cx);
                self.secao = options::Category::AiAcp;
                self.redraw_options(cx);
                self.display_chat_response(
                    cx,
                    "⚙ Opening Tools -> Options (AI Assistant / ACP)...",
                );
                Slash::Done
            }
            "plan" => {
                let plan = id!(ai_messages_view.plan_card);
                if rest.eq_ignore_ascii_case("auto") {
                    self.acp_config.auto_approve_plans = !self.acp_config.auto_approve_plans;
                    let _ = self.acp_config.save();
                    let on = self.acp_config.auto_approve_plans;
                    let (state, button) = crawrad_agent::slash_commands::auto_approve_labels(on);
                    let card = self.painel(Panel::Chat).view(plan);
                    card.button(id!(btn_plan_auto_approve)).set_text(cx, button);
                    let said = crawrad_agent::slash_commands::plan_auto_toggled_message(state);
                    self.display_chat_response(cx, &said);
                } else if !rest.is_empty() {
                    self.chat_shows(cx, id!(ai_messages_view.ai_hero_view), false);
                    let card = self.painel(Panel::Chat).view(plan);
                    card.set_visible(cx, true);
                    let on = self.acp_config.auto_approve_plans;
                    card.label(id!(plan_status))
                        .set_text(cx, crawrad_agent::slash_commands::plan_status_label(on));
                    let (_, button) = crawrad_agent::slash_commands::auto_approve_labels(on);
                    card.button(id!(btn_plan_auto_approve)).set_text(cx, button);
                    return Slash::Send(crawrad_agent::slash_commands::plan_prompt_for(rest));
                } else {
                    self.display_chat_response(cx, crawrad_agent::slash_commands::plan_usage());
                }
                Slash::Done
            }
            "explain" | "refactor" | "test" | "docs" | "review" => {
                if let Some(prompt_text) =
                    crawrad_agent::slash_commands::build_command_prompt(found.trigger, rest)
                {
                    return Slash::Send(prompt_text);
                }
                Slash::Done
            }
            "find" => {
                self.open_quick_open(cx);
                if !rest.is_empty() {
                    self.quick_open_filtro = rest.to_string();
                    self.quick_open_filtrados = crawrad_host::quick_open::filter_files(
                        &self.quick_open_arquivos,
                        &self.quick_open_filtro,
                    );
                }
                self.display_chat_response(cx, "🔍 Opening Quick Open file search...");
                Slash::Done
            }
            "queue" => {
                if rest.is_empty() {
                    self.display_chat_response(cx, "💡 Usage: `/queue <prompt>`");
                } else {
                    self.enfileirar_ai_prompt(cx, rest, Vec::new(), Vec::new());
                }
                Slash::Done
            }
            "agent" => Slash::No,
            "help" => {
                self.display_chat_response(cx, &crawrad_agent::slash_commands::help_text());
                Slash::Done
            }
            "teach" | "why" | "fix-explain" | "compare" => self.teacher_command(cx, found.id, rest),
            _ => Slash::No,
        }
    }

    pub(super) fn display_chat_response(&mut self, cx: &mut Cx, resposta: &str) {
        self.chat_shows(cx, id!(ai_messages_view.thinking_card), false);
        self.chat
            .session
            .add_message(crawrad_agent::ChatMessage::assistant(
                format!("resp-{}", self.chat.session.messages.len()),
                resposta,
            ));
        self.renderizar_historico_chat(cx);
    }

    #[note("ide-app-chat-ideapp-send-ai-prompt")]
    pub fn send_ai_prompt(&mut self, cx: &mut Cx) {
        if self.chat.audio_gravando {
            return;
        }

        let input_widget = self.get_chat_prompt_input();
        let prompt = input_widget.text();
        if prompt.trim().is_empty() {
            return;
        }

        // If the agent is busy, the message automatically goes to the send queue!
        if self.chat.busy {
            let attachments = std::mem::take(&mut self.chat.attachments);
            let images = std::mem::take(&mut self.chat.images);
            self.render_attachment_chips(cx);
            input_widget.set_text(cx, "");
            self.enfileirar_ai_prompt(cx, &prompt, attachments, images);
            return;
        }

        input_widget.set_text(cx, "");
        self.execute_ai_prompt(cx, &prompt);
    }

    #[note("ide-app-chat-ideapp-renderizar-historico-chat")]
    pub fn renderizar_historico_chat(&mut self, cx: &mut Cx) {
        let chat = self.painel(Panel::Chat);
        let has_messages = !self.chat.session.messages.is_empty();

        chat.view(id!(ai_messages_view.ai_hero_view))
            .set_visible(cx, !has_messages);

        let default_sender = self.get_active_agent_name();
        let turns = crawrad_agent::chat_layout::group_chat_turns(
            &self.chat.session.messages,
            &default_sender,
        );

        let visible_turns = if turns.len() > chat_bar::CHAT_CARDS {
            &turns[turns.len() - chat_bar::CHAT_CARDS..]
        } else {
            &turns[..]
        };

        let view_rect = chat.view(id!(ai_messages_view)).area().rect(cx);
        let largura_vista = if view_rect.size.x > 100.0 {
            view_rect.size.x - 20.0
        } else {
            280.0
        };

        // what each card's Copy copies: the reply's whole markdown
        self.chat.copy_texts = visible_turns
            .iter()
            .map(|t| t.reply_text.clone().unwrap_or_default())
            .collect();
        for i in 0..chat_bar::CHAT_CARDS {
            // each card's widgets, found inside the card itself (no hand-made path list)
            let (u, r) = (
                chat.view(&chat_bar::chat_card("user_card", i)),
                chat.view(&chat_bar::chat_card("reply_card", i)),
            );
            if let Some(turn) = visible_turns.get(i) {
                if let Some(user_text) = &turn.user_text {
                    let meta = crawrad_agent::chat_layout::user_meta_text(turn);
                    let recuo_esq = crawrad_agent::chat_layout::user_bubble_indent(
                        user_text,
                        &meta,
                        largura_vista,
                    );
                    u.apply_over(
                        cx,
                        live! {
                            margin: { left: (recuo_esq) }
                        },
                    );
                    if recuo_esq <= crawrad_agent::chat_layout::USER_BUBBLE_MIN_INDENT {
                        u.view(id!(u_row)).apply_over(
                            cx,
                            live! {
                                flow: Down,
                                align: { x: 1.0, y: 1.0 },
                                u_meta_row: { align: { x: 1.0, y: 1.0 } }
                            },
                        );
                        u.text_input(id!(u_msg_text))
                            .apply_over(cx, live! { width: Fill });
                    } else {
                        u.view(id!(u_row)).apply_over(
                            cx,
                            live! {
                                flow: Right,
                                align: { y: 1.0 },
                                u_meta_row: { align: { x: 0.0, y: 1.0 } }
                            },
                        );
                        u.text_input(id!(u_msg_text))
                            .apply_over(cx, live! { width: Fit });
                    }
                    u.set_visible(cx, true);
                    u.text_input(id!(u_msg_text)).set_text(cx, user_text);
                    let time = turn.user_sent_at.as_deref().unwrap_or("");
                    u.label(id!(u_meta)).set_text(cx, time);
                    u.label(id!(u_mark))
                        .set_text(cx, turn.user_mark.unwrap_or(""));
                } else {
                    u.set_visible(cx, false);
                }

                if !turn.reply_segments.is_empty() || turn.has_reply {
                    r.set_visible(cx, true);
                    if let Some(sender) = &turn.sender_name {
                        r.label(id!(r_sender_name)).set_text(cx, sender);
                    }

                    let notice = turn.notice.as_deref();
                    r.view(id!(r_notice_row)).set_visible(cx, notice.is_some());
                    r.label(id!(r_notice_row.r_notice))
                        .set_text(cx, notice.unwrap_or(""));
                    let status = turn.status_line.clone().unwrap_or_default();
                    r.view(id!(r_status_row))
                        .set_visible(cx, turn.status_line.is_some());
                    r.label(id!(r_status_row.st_line))
                        .set_text(cx, &status.line);
                    r.label(id!(r_status_row.st_sub)).set_text(cx, &status.sub);

                    let blocks: Vec<_> = turn
                        .reply_segments
                        .iter()
                        .map(|s| s.view().into())
                        .collect();
                    r.crawrad_segment_list(id!(r_blocks))
                        .set_segments(cx, &blocks);
                    let r_meta = turn.reply_meta.as_deref().unwrap_or("");
                    r.label(id!(r_meta)).set_text(cx, r_meta);

                    if let Some(tool) = turn.tool_calls.first() {
                        r.view(id!(r_tool_card)).set_visible(cx, true);
                        r.label(id!(r_tool_card.tool_header.lbl_tool_name))
                            .set_text(cx, &tool.name);
                        r.label(id!(r_tool_card.lbl_tool_args))
                            .set_text(cx, &tool.arguments);
                        r.label(id!(r_tool_card
                            .tool_header
                            .badge_tool_status
                            .lbl_tool_status))
                            .set_text(cx, tool.status.badge_label());
                    } else {
                        r.view(id!(r_tool_card)).set_visible(cx, false);
                    }
                } else {
                    r.set_visible(cx, false);
                }
            } else {
                u.set_visible(cx, false);
                r.set_visible(cx, false);
            }
        }

        let chip = crawrad_agent::chat_layout::agent_chip(
            &self.chat.session.messages,
            &default_sender,
            self.chat.live.status(),
        );
        chat.label(id!(ai_agent_chip_row.chip_text))
            .set_text(cx, &chip.text);
        chat.view(id!(ai_agent_chip_row))
            .set_visible(cx, !chip.text.is_empty());
        for dot in crawrad_agent::chat_layout::CHIP_DOTS {
            chat.view(&[LiveId::from_str(dot)])
                .set_visible(cx, dot == chip.link.dot());
        }
        let count_str =
            crawrad_agent::chat_layout::message_count_label(self.chat.session.messages.len());
        chat.label(id!(ai_messages_view
            .ai_hero_view
            .session_row
            .lbl_session_count))
            .set_text(cx, &count_str);

        let resumo = crawrad_agent::chat_layout::session_summary(&self.chat.session.messages);
        chat.label(id!(ai_messages_view
            .ai_hero_view
            .session_row
            .lbl_session_status))
            .set_text(cx, &resumo);

        self.sync_model_button(cx);
        let offs = [self.chat.form_off, self.chat.selection_off];
        for ((id, name), off) in crawrad_agent::chat_layout::CONTEXT_SWITCHES
            .into_iter()
            .zip(offs)
        {
            self.chat_tool(id)
                .set_text(cx, &crawrad_agent::chat_layout::switch_label(name, !off));
        }
        self.chat.scroll_to_bottom_frames = 2;
        chat.view(id!(ai_messages_view))
            .set_scroll_pos(cx, dvec2(0.0, 100_000.0));
        chat.view(id!(ai_messages_view)).redraw(cx);
        chat.redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-chat-ideapp-chat-messages-json")]
    pub fn chat_messages_json(&self, cx: &Cx) -> serde_json::Value {
        let name = self.get_active_agent_name();
        let watch = self.chat.status.as_ref().filter(|_| self.chat.busy);
        let mut out = crawrad_agent::chat_layout::turns_json(
            &self.chat.session.messages,
            &name,
            self.chat.live.status(),
            watch.map(|w| w.current()).as_ref(),
        );
        let r = self.get_chat_prompt_input().area().rect(cx);
        out["prompt_rect"] = serde_json::json!(xywh(r));
        for (key, btn) in [
            ("model_button", self.chat_tool("btn_ai_model")),
            ("effort_button", self.chat_tool("btn_ai_effort")),
        ] {
            let r = btn.area().rect(cx);
            out[key] =
                serde_json::json!({"text": btn.text(), "visible": btn.visible(), "rect": xywh(r)});
        }
        let drop = self.painel(Panel::Chat).view(id!(ai_model_dropdown));
        let open = self.chat.open_menu.as_deref().filter(|_| drop.visible());
        let menu = open.and_then(|c| {
            crawrad_agent::chat_config::menu_or_catalogue(&self.chat.live, &self.acp_config, c)
        });
        let items = menu.map(|m| m.items).unwrap_or_default();
        let slots =
            (0..chat_bar::MENU_SLOTS).map(|i| drop.button(&chat_bar::menu_slot(i)).area().rect(cx));
        let items: Vec<_> = items.iter().zip(slots).map(|(it, r)| serde_json::json!({"label": it.label, "selected": it.selected, "rect": xywh(r)})).collect();
        out["menu"] = serde_json::json!({"open": open, "items": items});
        let rect = |r: Rect| serde_json::json!(xywh(r));
        let card = |i| {
            self.painel(Panel::Chat)
                .view(&chat_bar::chat_card("reply_card", i))
        };
        let cards: Vec<_> = (0..self.chat.copy_texts.len())
            .map(|i| {
                let list = card(i).crawrad_segment_list(id!(r_blocks));
                let (text_rect, selection) = list.first_text_state(cx);
                let copy = rect(card(i).button(id!(r_meta_row.r_copy)).area().rect(cx));
                crawrad_agent::chat_layout::card_json(
                    text_rect,
                    copy,
                    &selection,
                    &list.block_rects(cx),
                )
            })
            .collect();
        out["cards"] = cards.into();
        // What a script needs to drive the chat — busy, the switches, every control's rect
        let mut controls = serde_json::Map::new();
        for n in crawrad_agent::chat_layout::CHAT_CONTROLS {
            let b = self.chat_tool(n);
            controls.insert(
                n.into(),
                serde_json::json!({"rect": rect(b.area().rect(cx)), "visible": b.visible()}),
            );
        }
        out["controls"] = controls.into();
        out["busy"] = self.chat.busy.into();
        out["conversation"] = self.chat.conversation.clone().into();
        out["history_list"] = self.history_json(cx);
        out["prompt"] = self.get_chat_prompt_input().text().into();
        out["permission"] =
            crawrad_agent::chat_layout::permission_json(&self.chat.permission_queue);
        let queue = self.chat.queue.iter().map(|m| m.text.as_str());
        out["queue"] = queue.collect::<Vec<_>>().into();
        out["context"] =
            serde_json::json!({"form": !self.chat.form_off, "selection": !self.chat.selection_off});
        out["attachments"] = self.attachments_json(cx);
        out
    }

    #[note("ide-app-chat-ideapp-caminho-ativo-para-contexto")]
    pub fn caminho_ativo_para_contexto(&self) -> Option<std::path::PathBuf> {
        if self.host.center().is_text() {
            if let Some(path) = self.abas.active_file() {
                return Some(path.to_path_buf());
            }
            if let Some(ref p) = self.editavel {
                return Some(p.clone());
            }
        }
        self.host.active_file().map(|p| p.to_path_buf())
    }

    #[note("ide-app-chat-ideapp-extrair-selecao-do-editor")]
    pub fn extrair_selecao_do_editor(&self) -> Option<crawrad_agent::EditorSelectionContext> {
        if !self.host.center().is_text() {
            return None;
        }
        let [(start, from), (end, to)] = self.editor_selection_bounds()?;
        let (start_line, end_line) = (start + 1, end + 1);
        let selected_text = crawrad_agent::prompt_assembler::extract_selected_text(
            &self.texto_na_vista,
            start_line,
            from,
            end_line,
            to,
        );
        (!selected_text.trim().is_empty()).then_some(crawrad_agent::EditorSelectionContext {
            start_line,
            end_line,
            selected_text,
        })
    }

    #[note("ide-app-chat-ideapp-extrair-contexto-ativo-para-ia")]
    pub fn extrair_contexto_ativo_para_ia(&self) -> crawrad_agent::ProjectContext {
        let raiz = self.raiz();
        if self.host.center().is_text() {
            if let Some(path) = self.caminho_ativo_para_contexto() {
                let live_content = if !self.texto_na_vista.is_empty() {
                    self.texto_na_vista.clone()
                } else if let Some(editado) = self.host.buffer(&path) {
                    editado.to_owned()
                } else {
                    std::fs::read_to_string(&path).unwrap_or_default()
                };
                let sel = self.extrair_selecao_do_editor();
                return crawrad_agent::ProjectContext::for_code_editor(
                    &path,
                    raiz.as_deref(),
                    live_content,
                    sel,
                );
            }
        } else if let Some(doc) = self.host.doc() {
            let (name, sel, count) = crawrad_host::document::designer_summary(doc);
            return crawrad_agent::ProjectContext::for_designer(
                name,
                self.host.active_file(),
                raiz.as_deref(),
                count,
                sel,
            );
        }
        crawrad_agent::ProjectContext::None
    }

    #[note("ide-app-chat-ideapp-warm-live-agent")]
    pub fn warm_live_agent(&mut self, cx: &mut Cx) {
        let cwd = self.raiz().or_else(|| self.membro());
        let perms = self.chat.session_permissions.clone();
        if !cfg!(test) && self.chat.live.ensure_warm(&self.acp_config, cwd, perms) {
            self.renderizar_historico_chat(cx);
        }
    }

    #[note("ide-app-chat-ideapp-record-conversation")]
    fn record_conversation(&mut self) {
        let (agent, chat) = (self.get_active_agent_name(), &mut self.chat);
        let store = crawrad_agent::chat_history::HistoryStore::user_default();
        let (open, messages) = (&mut chat.conversation, &chat.session.messages);
        let _ = crawrad_agent::chat_history::record(&store, open, &agent, &chat.live, messages);
    }

    #[note("ide-app-chat-ideapp-open-conversation")]
    pub fn open_conversation(&mut self, cx: &mut Cx, id: &str) {
        let store = crawrad_agent::chat_history::HistoryStore::user_default();
        let Some(messages) = crawrad_agent::chat_history::reopen(&store, id, &mut self.chat.live)
        else {
            return;
        };
        self.new_chat_session(cx);
        (self.chat.session.messages, self.chat.conversation) = (messages, Some(id.to_string()));
        self.renderizar_historico_chat(cx);
    }

    #[note("ide-app-chat-ideapp-new-chat-session")]
    pub fn new_chat_session(&mut self, cx: &mut Cx) {
        self.chat.conversation = None;
        self.chat.session.clear();
        self.chat.live.reset();
        self.warm_live_agent(cx);
        self.chat.queue.clear();
        self.chat.permission_queue.clear();
        self.chat.session_permissions.lock().unwrap().clear();
        self.update_permission_card(cx);
        self.update_ai_queue_visual(cx);
        for card in [
            id!(ai_messages_view.thinking_card),
            id!(ai_messages_view.plan_card),
            id!(ai_messages_view.copilot_card),
        ] {
            self.painel(Panel::Chat).view(card).set_visible(cx, false);
        }
        self.renderizar_historico_chat(cx);
        self.get_chat_prompt_input().set_text(cx, "");
    }

    #[note("ide-app-chat-ideapp-context-item")]
    pub fn context_item(&mut self, cx: &mut Cx, item: &str) {
        self.chat_shows(cx, id!(ai_context_popup), false);
        match item {
            "media" | "mentions" => {
                let (active, root) = (self.caminho_ativo_para_contexto(), self.raiz());
                let token =
                    crawrad_agent::chat_layout::mention_token(active.as_deref(), root.as_deref());
                let token = if item == "media" {
                    "@media:assets/".to_string()
                } else {
                    token
                };
                let text = crawrad_agent::chat_layout::append_token(
                    &self.get_chat_prompt_input().text(),
                    &token,
                );
                self.get_chat_prompt_input().set_text(cx, &text);
                self.sincronizar(cx);
            }
            "actions" => {
                self.posicionar_slash_popup(cx);
                self.chat_shows(cx, id!(ai_slash_popup), true);
            }
            "memory" => self.open_memory(cx),
            _ => {
                self.open_options(cx);
                self.secao = options::Category::AiAcp;
                self.redraw_options(cx);
            }
        }
        self.painel(Panel::Chat).redraw(cx);
    }

    /// Shows or hides one view of the chat panel (a popup, the thinking card).
    pub(super) fn chat_shows(&self, cx: &mut Cx, path: &[LiveId], on: bool) {
        self.painel(Panel::Chat).view(path).set_visible(cx, on);
    }

    #[note("ide-app-chat-ideapp-answer-permission")]
    pub fn answer_permission(&mut self, cx: &mut Cx, choice: &str) {
        let Some(req) = self.chat.permission_queue.pop() else {
            return;
        };
        if choice == "allow_always" {
            self.chat
                .session_permissions
                .lock()
                .unwrap()
                .allow_always(&req.tool_name, req.target.clone());
        }
        let _ = req.respond(crawrad_agent::PermissionDecision::Selected(
            choice.to_string(),
        ));
        self.update_permission_card(cx);
    }

    #[note("ide-app-chat-ideapp-chat-mcp")]
    pub fn chat_mcp(&mut self, cx: &mut Cx, name: &str, args: &serde_json::Value) {
        match name {
            "chat.send" => {
                self.get_chat_prompt_input()
                    .set_text(cx, args["text"].as_str().unwrap_or_default());
                self.send_ai_prompt(cx);
            }
            "chat.stop" => self.parar_ia_prompt(cx),
            "chat.new_session" => self.new_chat_session(cx),
            "chat.history" => self.open_conversation(cx, args["open"].as_str().unwrap_or("")),
            "chat.context_menu" => self.context_item(cx, args["item"].as_str().unwrap_or_default()),
            "chat.permission" => {
                self.answer_permission(cx, args["choice"].as_str().unwrap_or_default())
            }
            "chat.queue" => {
                let index = args["index"].as_u64().unwrap_or(0) as usize;
                match args["action"].as_str().unwrap_or_default() {
                    "send" => self.enviar_ai_queue_item(cx, index),
                    "edit" => self.editar_ai_queue_item(cx, index),
                    _ => self.remove_ai_queue_item(cx, index),
                }
            }
            _ => {
                self.chat.form_off = !args["form"].as_bool().unwrap_or(!self.chat.form_off);
                self.chat.selection_off = !args["selection"]
                    .as_bool()
                    .unwrap_or(!self.chat.selection_off);
                self.renderizar_historico_chat(cx);
            }
        }
    }

    #[note("ide-app-chat-ideapp-execute-ai-prompt")]
    pub fn execute_ai_prompt(&mut self, cx: &mut Cx, prompt: &str) {
        let attachments = std::mem::take(&mut self.chat.attachments);
        self.render_attachment_chips(cx);
        self.execute_prompt_with(cx, prompt, attachments);
    }

    #[note("ide-app-chat-ideapp-execute-prompt-with")]
    pub fn execute_prompt_with(
        &mut self,
        cx: &mut Cx,
        prompt: &str,
        attachments: Vec<SelectionAttachment>,
    ) {
        if prompt.trim().is_empty() {
            return;
        }
        let agent_name = self.get_active_agent_name();
        self.chat.busy = true;
        self.chat_shows(cx, id!(ai_slash_popup), false);
        self.chat_shows(cx, id!(ai_context_popup), false);
        self.chat
            .session
            .add_message(crawrad_agent::ChatMessage::user(
                format!("msg-{}", self.chat.session.messages.len()),
                prompt,
            ));
        self.renderizar_historico_chat(cx);

        // The live status is not a thought: the card shows whatever `enable_thinking` says.
        let (status_tx, watch) = crawrad_agent::acp_status::StatusWatch::channel(&agent_name);
        self.chat.status = Some(watch);
        self.show_live_status(cx);
        self.chat.scroll_to_bottom_frames = 2;

        // the bubble shows what was typed; the agent gets what the command expands to
        let expanded = match self.despachar_slash_command(cx, prompt) {
            Slash::Done => {
                self.chat.busy = false;
                let draft = std::mem::replace(&mut self.chat.attachments, attachments);
                self.chat.attachments.extend(draft);
                self.chat.attachments.truncate(MAX_ATTACHMENTS);
                self.render_attachment_chips(cx);
                self.renderizar_historico_chat(cx);
                self.processar_proxima_da_fila(cx);
                return;
            }
            Slash::Send(p) => p,
            Slash::No => prompt.to_string(),
        };
        let prompt = expanded.as_str();
        let acp = self.acp_config.clone();
        let mem_path = crawrad_agent::AefosMemory::default_path();
        let global_mem = crawrad_agent::AefosMemory::load_from_file(&mem_path)
            .ok()
            .map(|m| m.instructions);
        let (form, sel) = (!self.chat.form_off, !self.chat.selection_off);
        let project_context = self.extrair_contexto_ativo_para_ia().filtered(form, sel);
        let mut live_buffers = self.host.editor_buffers().clone();
        if let Some(active_path) = self.caminho_ativo_para_contexto()
            && !self.texto_na_vista.is_empty()
        {
            live_buffers.insert(active_path, self.texto_na_vista.clone());
        }
        let mode = if prompt.trim_start().starts_with("/agent") {
            crawrad_agent::PromptMode::Agent
        } else {
            crawrad_agent::PromptMode::Chat
        };
        let prompt_montado =
            crawrad_agent::PromptAssembler::assemble(crawrad_agent::AssembleParams {
                mode,
                global_memory: global_mem.as_deref(),
                project_context: &project_context,
                user_query: prompt,
                session_history: &[],
                attachments: &attachments,
                workspace_root: self.raiz().as_deref(),
                live_buffers: Some(&live_buffers),
                max_bytes_per_file: 12 * 1024,
                max_total_prompt_bytes: 28 * 1024,
            });

        let (tx, rx) = std::sync::mpsc::channel();
        let (cancel_tx, cancel_rx) = std::sync::mpsc::channel();
        let (perm_tx, perm_rx) = std::sync::mpsc::channel();
        self.chat.resposta_rx = Some(rx);
        self.chat.cancel_tx = Some(cancel_tx);
        self.chat.permission_rx = Some(perm_rx);
        let write = self.chat.writes.open(wake);
        self.update_chat_buttons_state(cx);
        let cwd = self.raiz().or_else(|| self.membro());
        if let Some(ref dir) = cwd {
            let instance =
                std::env::var("CRAWRAD_INSTANCE").unwrap_or_else(|_| "default".to_string());
            crate::ide_app::mcp_host::sync_workspace_mcp_config(dir, &instance);
        }
        let (id, provider) = (&self.acp_config.agent_id, &self.acp_config.provider);
        let support = crawrad_agent::agent_profile::profile_for(id, provider).session;
        let cli_session_id = match support {
            crawrad_agent::SessionSupport::Pinned => {
                Some(self.chat.session.ensure_cli_session_id().to_string())
            }
            crawrad_agent::SessionSupport::Captured => self.chat.session.cli_session_id.clone(),
            crawrad_agent::SessionSupport::None => None,
        };
        let is_resume = self.chat.session.session_started && cli_session_id.is_some();

        let req = crawrad_agent::acp_live::TurnRequest {
            config: acp,
            live: self.chat.live.client(),
            prompt: prompt_montado,
            images: std::mem::take(&mut self.chat.images),
            cwd,
            buffers: live_buffers,
            perms: self.chat.session_permissions.clone(),
            cli_session: cli_session_id.map(|id| (id, is_resume)),
            agent: agent_name,
            status: status_tx,
        };
        self.render_attachment_chips(cx);
        std::thread::spawn(move || {
            let cancelled = || cancel_rx.try_recv().is_ok();
            let ask = |req| {
                drop(perm_tx.send(req));
                wake();
            };
            let result = crawrad_agent::acp_live::run_turn(req, cancelled, ask, write);
            let _ = tx.send(result);
            wake();
        });
        self.chat.busy_clock = cx.start_timeout(1.0);
        self.painel(Panel::Chat).redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-chat-ideapp-show-live-status")]
    pub(super) fn show_live_status(&mut self, cx: &mut Cx) {
        let Some(status) = self.chat.status.as_mut().map(|w| w.poll()) else {
            return;
        };
        let card = self.painel(Panel::Chat).view(id!(thinking_card));
        card.set_visible(cx, true);
        card.label(id!(st_line)).set_text(cx, &status.line);
        card.label(id!(st_sub)).set_text(cx, &status.sub);
    }

    #[note("ide-app-chat-ideapp-finalizar-ia-prompt")]
    pub fn finalizar_ia_prompt(&mut self, cx: &mut Cx) {
        if !self.chat.busy {
            return;
        }

        let resposta = match self.chat.resposta_rx.as_ref() {
            Some(rx) => rx.try_recv(),
            None => {
                self.chat_shows(cx, id!(ai_messages_view.thinking_card), false);
                self.chat.busy = false;
                self.update_chat_buttons_state(cx);
                self.renderizar_historico_chat(cx);
                self.processar_proxima_da_fila(cx);
                return;
            }
        };

        match resposta {
            Ok(resultado) => {
                self.chat.resposta_rx = None;
                self.chat.cancel_tx = None;
                self.chat_shows(cx, id!(ai_messages_view.thinking_card), false);

                let (id, provider) = (&self.acp_config.agent_id, &self.acp_config.provider);
                let support = crawrad_agent::agent_profile::profile_for(id, provider).session;
                let (reply_str, tool_records, via, status_line) = match resultado {
                    Ok((text, tools, via, status_line)) => {
                        let session = &mut self.chat.session;
                        let outcome = session.apply_turn_result_for_support(&Ok(text), support);
                        (outcome.reply_text(), tools, Some(via), Some(status_line))
                    }
                    Err(err) => {
                        if err.contains("terminated") || err.contains("closed communication stream")
                        {
                            self.chat.live.reset();
                            self.warm_live_agent(cx);
                        }
                        (format!("⚠️ Agent error: {err}"), Vec::new(), None, None)
                    }
                };

                let mut msg = crawrad_agent::ChatMessage::assistant(
                    format!("resp-{}", self.chat.session.messages.len()),
                    &reply_str,
                )
                .with_tool_calls(tool_records)
                .with_status_line(status_line);
                match via {
                    Some(crawrad_agent::chat_layout::Via::Acp { session }) => {
                        msg.acp_session = session
                    }
                    Some(crawrad_agent::chat_layout::Via::Cli { notice }) => {
                        msg.notice = Some(notice)
                    }
                    None => msg.status = crawrad_agent::MessageStatus::Error,
                }

                self.chat.session.add_message(msg);
                self.record_conversation();
                self.chat.busy = false;
                self.update_chat_buttons_state(cx);
                self.renderizar_historico_chat(cx);
                self.processar_proxima_da_fila(cx);
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // Still in flight: update the live status line immediately.
                self.show_live_status(cx);
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.chat.resposta_rx = None;
                self.chat.cancel_tx = None;
                self.chat.live.reset();
                self.warm_live_agent(cx);
                self.chat_shows(cx, id!(ai_messages_view.thinking_card), false);

                let error_msg = "⚠️ Agent error: Connection closed unexpectedly.";
                self.chat
                    .session
                    .add_message(crawrad_agent::ChatMessage::assistant(
                        format!("resp-{}", self.chat.session.messages.len()),
                        error_msg,
                    ));

                self.chat.busy = false;
                self.update_chat_buttons_state(cx);
                self.renderizar_historico_chat(cx);
                self.processar_proxima_da_fila(cx);
            }
        }
    }

    #[note("ide-app-chat-ideapp-bombear-ia")]
    pub fn bombear_ia(&mut self, cx: &mut Cx) {
        if let Some(ref rx) = self.chat.options_test_rx
            && let Ok(res) = rx.try_recv()
        {
            match res {
                Ok(msg) => self.chat.acp_status_msg = Some(msg),
                Err(err) => self.chat.acp_status_msg = Some(format!("🔴 Connection failed: {err}")),
            }
            self.chat.options_test_rx = None;
            self.redraw_options(cx);
        }
        if let Some(ref rx) = self.chat.probe_rx
            && let Ok(res) = rx.try_recv()
        {
            match res {
                Ok(msg) => {
                    let text = format!(
                        "🤖 **ACP Live Probe Result**\n\n{msg}\n• Provider: {}\n• Model: {}",
                        self.acp_config.provider, self.acp_config.model
                    );
                    self.display_chat_response(cx, &text);
                }
                Err(err) => {
                    let text = format!("🔴 **ACP Connection Failed**\n\n{err}");
                    self.display_chat_response(cx, &text);
                }
            }
            self.chat.probe_rx = None;
        }
        let rx = &self.chat.permission_rx;
        let asks: Vec<_> = rx.iter().flat_map(|r| r.try_iter()).collect();
        for req in asks {
            if !self.agent_edit_ask(cx, &req) {
                self.chat.permission_queue.push(req);
            }
        }
        for req in self.chat.writes.drain() {
            self.agent_write(cx, req);
        }
        if let Some(top) = self.chat.permission_queue.peek()
            && top.is_expired()
        {
            let _ = top.respond(crawrad_agent::PermissionDecision::Cancelled);
            let tool_name = top.tool_name.clone();
            self.chat.permission_queue.pop();
            self.display_chat_response(
                cx,
                &format!("⏱️ Permission request for `{tool_name}` timed out (cancelled)."),
            );
        }
        self.update_permission_card(cx);
        self.finalizar_ia_prompt(cx);
    }

    #[note("ide-app-chat-ideapp-update-permission-card")]
    pub fn update_permission_card(&mut self, cx: &mut Cx) {
        let card = self
            .painel(Panel::Chat)
            .view(id!(ai_messages_view.permission_card));
        let Some(req) = self.chat.permission_queue.peek() else {
            card.set_visible(cx, false);
            return;
        };
        card.set_visible(cx, true);
        let len = self.chat.permission_queue.len();
        let count = if len > 1 {
            format!("(1 of {len})")
        } else {
            String::new()
        };
        card.label(id!(perm_hdr_row.perm_count))
            .set_text(cx, &count);
        card.label(id!(perm_tool_row.lbl_perm_tool_name))
            .set_text(cx, &req.tool_name);
        card.view(id!(perm_target_row))
            .set_visible(cx, req.target.is_some());
        let target = req.target.as_deref().unwrap_or_default();
        card.label(id!(perm_target_row.lbl_perm_target))
            .set_text(cx, target);
        card.label(id!(lbl_perm_args))
            .set_text(cx, &req.arguments_summary);
    }

    #[note("ide-app-chat-ideapp-toggle-audio-mic")]
    pub fn toggle_audio_mic(&mut self, cx: &mut Cx) {
        if self.chat.audio_gravando {
            self.parar_gravacao_audio(cx);
        } else {
            self.iniciar_gravacao_audio(cx);
        }
    }

    #[note("ide-app-chat-ideapp-parar-ia-prompt")]
    pub fn parar_ia_prompt(&mut self, cx: &mut Cx) {
        if !self.chat.busy {
            return;
        }

        if let Some(ref tx) = self.chat.cancel_tx {
            let _ = tx.send(());
        }
        self.chat.cancel_tx = None;
        self.chat.resposta_rx = None;
        self.chat.busy = false;

        self.chat_shows(cx, id!(ai_messages_view.thinking_card), false);

        self.chat
            .session
            .add_message(crawrad_agent::ChatMessage::assistant(
                format!("resp-{}", self.chat.session.messages.len()),
                "🛑 Turn cancelled by user.",
            ));

        self.update_chat_buttons_state(cx);
        self.renderizar_historico_chat(cx);
    }

    #[note("ide-app-chat-ideapp-set-prompt-input-recording-state")]
    pub(super) fn set_prompt_input_recording_state(
        &mut self,
        cx: &mut Cx,
        recording: bool,
        texto: Option<&str>,
    ) {
        let prompt_input = self.get_chat_prompt_input();
        let btn_send = self.chat_tool("btn_prompt_send");
        let look = crawrad_audio::prompt::look(recording);
        let v = |c: [f32; 4]| vec4(c[0], c[1], c[2], c[3]);
        let (text, focus, cursor) = (v(look.text), v(look.text_focus), v(look.cursor));
        let (bg, bg_hover, label) = (v(look.button), v(look.button_hover), v(look.button_text));
        prompt_input.set_is_read_only(cx, recording);
        prompt_input.apply_over(
            cx,
            live! {
                draw_text: { color: (text) color_hover: (text) color_focus: (focus) }
                draw_cursor: { color: (cursor) }
            },
        );
        btn_send.set_enabled(cx, !recording);
        btn_send.apply_over(
            cx,
            live! {
                draw_bg: { disabled: (look.disabled) color: (bg) color_hover: (bg_hover) }
                draw_text: { color: (label) }
            },
        );
        if let Some(t) = texto {
            prompt_input.set_text(cx, t);
        }

        prompt_input.redraw(cx);
        btn_send.redraw(cx);
    }

    #[note("ide-app-chat-ideapp-iniciar-gravacao-audio")]
    pub fn iniciar_gravacao_audio(&mut self, cx: &mut Cx) {
        if self.chat.audio_sessao.is_none() {
            let recorder = crawrad_audio::CpalAudioRecorder::default();
            self.chat.audio_sessao = Some(crawrad_audio::AudioSession::new(recorder, wake));
        }

        let Some(session) = self.chat.audio_sessao.as_mut() else {
            return;
        };
        match session.start_recording() {
            Ok(()) => {
                self.chat.audio_gravando = true;
                self.set_mic_recording_visual(cx, 1.0);
                self.chat.recording_clock = cx.start_timeout(1.0);
                // what was typed stays, the listening note after it
                self.chat.audio_prompt_anterior = self.get_chat_prompt_input().text();
                let shown =
                    crawrad_audio::prompt::recording_started(&self.chat.audio_prompt_anterior);
                self.set_prompt_input_recording_state(cx, true, Some(&shown));
                self.say_on_status(cx, crawrad_audio::prompt::MIC_ON.to_string());
            }
            Err(err) => {
                self.chat.audio_gravando = false;
                self.set_mic_recording_visual(cx, 0.0);
                self.set_prompt_input_recording_state(cx, false, None);
                self.say_on_status(cx, format!("Could not open the microphone: {err}"));
            }
        }
    }

    #[note("ide-app-chat-ideapp-say-on-status")]
    fn say_on_status(&mut self, cx: &mut Cx, message: String) {
        self.status = Some(message);
        self.sincronizar(cx);
        self.painel(Panel::Chat).redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-chat-ideapp-parar-gravacao-audio")]
    pub fn parar_gravacao_audio(&mut self, cx: &mut Cx) {
        self.chat.audio_gravando = false;
        self.set_mic_recording_visual(cx, 0.0);

        if let Some(sessao) = self.chat.audio_sessao.as_mut() {
            let config = crawrad_audio::TranscriptionConfig::default();
            let transcriber = crawrad_audio::DefaultTranscriber;
            match sessao.stop_and_transcribe(transcriber, config) {
                Ok(()) => {
                    self.say_on_status(cx, crawrad_audio::prompt::TRANSCRIBING.to_string());
                }
                Err(err) => {
                    self.set_prompt_input_recording_state(cx, false, None);
                    self.say_on_status(cx, format!("Could not stop the recording: {err}"));
                }
            }
        } else {
            self.set_prompt_input_recording_state(cx, false, None);
        }
    }

    #[note("ide-app-chat-ideapp-set-mic-recording-visual")]
    pub(super) fn set_mic_recording_visual(&self, cx: &mut Cx, recording: f32) {
        let btn = self.chat_tool("btn_ai_mic");
        btn.apply_over(cx, live! { draw_bg: { recording: (recording) } });
        btn.redraw(cx);
    }

    #[note("ide-app-chat-ideapp-bombear-audio")]
    pub(super) fn bombear_audio(&mut self, cx: &mut Cx) {
        let status = if let Some(sessao) = self.chat.audio_sessao.as_mut() {
            sessao.poll().clone()
        } else {
            return;
        };

        match status {
            crawrad_audio::AudioSessionStatus::Recording { elapsed_secs, .. } => {
                if self.chat.audio_gravando {
                    self.status = Some(crawrad_audio::prompt::recording_status(elapsed_secs));
                    self.sincronizar(cx);
                    let previous = &self.chat.audio_prompt_anterior;
                    let shown = crawrad_audio::prompt::recording_prompt(previous, elapsed_secs);
                    self.set_prompt_input_recording_state(cx, true, Some(&shown));
                }
            }
            crawrad_audio::AudioSessionStatus::Transcribing => {
                self.status = Some(crawrad_audio::prompt::TRANSCRIBING.into());
                self.sincronizar(cx);
            }
            crawrad_audio::AudioSessionStatus::Finished(result) => {
                let previous = std::mem::take(&mut self.chat.audio_prompt_anterior);
                let joined = crawrad_audio::prompt::after_transcription(&previous, &result.text);
                // the prompt and Send come back, so the user can send it
                self.set_prompt_input_recording_state(cx, false, Some(&joined));
                self.set_mic_recording_visual(cx, 0.0);
                self.say_on_status(cx, crawrad_audio::prompt::transcribed_status(&result));
            }
            crawrad_audio::AudioSessionStatus::Error(err) => {
                self.set_mic_recording_visual(cx, 0.0);
                let previous = std::mem::take(&mut self.chat.audio_prompt_anterior);
                self.set_prompt_input_recording_state(cx, false, Some(&previous));
                self.say_on_status(cx, format!("Audio error: {err}"));
            }
            crawrad_audio::AudioSessionStatus::Idle => {}
        }
    }
}
