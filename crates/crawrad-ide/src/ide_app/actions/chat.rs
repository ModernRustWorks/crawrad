//! Aefos AI panel actions (Chat, prompts, queue, permission and plan cards).

use super::super::*;
use crawrad_agent::chat_config as menus;

impl IdeApp {
    pub(in super::super) fn handle_chat_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Aefos AI Actions
        let chat_panel = self.painel(Panel::Chat);
        let btn_send_clicked = self.chat_tool("btn_prompt_send").clicked(actions);
        let btn_stop_clicked = self.chat_tool("btn_prompt_stop").clicked(actions);
        let prompt_input = self.get_chat_prompt_input();
        let prompt_returned = prompt_input.returned(actions).is_some();
        let hit = |path: &[LiveId]| chat_panel.button(path).clicked(actions);

        for (i, markdown) in self.chat.copy_texts.clone().iter().enumerate() {
            let card = chat_panel.view(&chat_bar::chat_card("reply_card", i));
            if let Some(code) = card
                .crawrad_segment_list(id!(r_blocks))
                .copy_code_clicked(actions)
            {
                cx.copy_to_clipboard(&code);
            }
            if card.button(id!(r_meta_row.r_copy)).clicked(actions) {
                cx.copy_to_clipboard(markdown);
            }
        }

        if btn_stop_clicked {
            self.parar_ia_prompt(cx);
        } else if (btn_send_clicked || prompt_returned) && !self.chat.audio_gravando {
            self.send_ai_prompt(cx);
        } else if hit(id!(ai_header.btn_ai_new)) || hit(id!(ai_header.btn_ai_clear)) {
            self.new_chat_session(cx);
        } else if hit(id!(ai_header.btn_ai_history)) {
            self.toggle_history(cx);
        } else if self.chat_tool("btn_form_toggle").clicked(actions) {
            self.chat.form_off = !self.chat.form_off;
            self.renderizar_historico_chat(cx);
        } else if self.chat_tool("btn_sel_toggle").clicked(actions) {
            self.chat.selection_off = !self.chat.selection_off;
            self.renderizar_historico_chat(cx);
        } else if hit(id!(ai_header.btn_ai_settings)) {
            self.open_options(cx);
            self.secao = options::Category::AiAcp;
            self.redraw_options(cx);
        } else if chat_panel
            .button(id!(ai_agent_chip_row.ai_agent_chip.btn_chip))
            .clicked(actions)
            || chat_panel
                .button(id!(ai_agent_chip_row.btn_chip))
                .clicked(actions)
        {
            if self.is_agent_failing() {
                self.open_acp_diagnostic(cx);
            }
        } else if self.chat_tool("btn_ai_attach").clicked(actions) {
            let p = self.painel(Panel::Chat).view(id!(ai_context_popup));
            let abrir = !p.visible();
            if abrir {
                self.posicionar_context_popup(cx);
                self.chat_shows(cx, id!(ai_slash_popup), false);
                self.chat_shows(cx, id!(ai_model_dropdown), false);
            }
            p.set_visible(cx, abrir);
            self.painel(Panel::Chat).redraw(cx);
        } else if let Some(cat) = if self.chat_tool("btn_ai_model").clicked(actions) {
            Some("model")
        } else if self.chat_tool("btn_ai_effort").clicked(actions) {
            Some("thought_level")
        } else {
            None
        } {
            self.chat_shows(cx, id!(ai_context_popup), false);
            self.chat_shows(cx, id!(ai_slash_popup), false);
            let drop = self.painel(Panel::Chat).view(id!(ai_model_dropdown));
            if self.chat.open_menu.as_deref() == Some(cat) && drop.visible() {
                drop.set_visible(cx, false);
                self.chat.open_menu = None;
            } else {
                self.position_config_menu(cx, cat);
            }
            self.painel(Panel::Chat).redraw(cx);
        } else if self.chat_tool("btn_ai_mic").clicked(actions) {
            self.toggle_audio_mic(cx);
        } else if hit(id!(ai_queue_card.queue_header.btn_queue_toggle)) {
            self.alternar_ai_queue_expanded(cx);
        } else if self.despachar_cliques_fila_chat(cx, actions) {
            // handled in the queue dispatcher
        } else if self.dispatch_attachment_clicks(cx, actions) {
        } else if let Some((_, cmd)) = crawrad_agent::chat_layout::HERO_SHORTCUTS
            .into_iter()
            .find(|(card, _)| hit(&[LiveId::from_str(&format!("btn_card_{card}"))]))
        {
            self.get_chat_prompt_input().set_text(cx, cmd);
        }

        // Plan Card Actions: Approve, Reject, Toggle Auto-Approve
        let plan = chat_panel.view(id!(ai_messages_view.plan_card));
        if plan.button(id!(btn_plan_approve)).clicked(actions) {
            plan.label(id!(plan_status)).set_text(cx, "🟢 Approved");
            self.execute_ai_prompt(cx, "Execute the approved plan step by step.");
        } else if plan.button(id!(btn_plan_reject)).clicked(actions) {
            plan.label(id!(plan_status)).set_text(cx, "🔴 Rejected");
            plan.set_visible(cx, false);
            self.display_chat_response(cx, "Plan execution cancelled by user.");
        } else if plan.button(id!(btn_plan_auto_approve)).clicked(actions) {
            self.acp_config.auto_approve_plans = !self.acp_config.auto_approve_plans;
            let _ = self.acp_config.save();
            let on = self.acp_config.auto_approve_plans;
            let (_, label) = crawrad_agent::slash_commands::auto_approve_labels(on);
            plan.button(id!(btn_plan_auto_approve)).set_text(cx, label);
            if on {
                plan.label(id!(plan_status))
                    .set_text(cx, "🟢 Auto-Approved");
            }
        }
        // the `+` menu's items and the permission card's answers (the same path as MCP)
        let named = |path: [LiveId; 2], name: String| [path[0], path[1], LiveId::from_str(&name)];
        let ctx = [live_id!(ai_context_popup), live_id!(ctx_list)];
        let perm = [live_id!(permission_card), live_id!(perm_actions_row)];
        for item in crawrad_agent::chat_layout::CONTEXT_ITEMS {
            if hit(&named(ctx, format!("btn_ai_{item}"))) {
                self.context_item(cx, item);
            }
        }
        for choice in crawrad_agent::chat_layout::PERMISSION_CHOICES {
            if hit(&named(perm, format!("btn_perm_{choice}"))) {
                self.answer_permission(cx, choice);
            }
        }

        // Detection of "/" to show or hide the slash commands popup
        let prompt_input = self.get_chat_prompt_input();
        if let Some(texto) = prompt_input.changed(actions) {
            let mostra_popup =
                texto.trim_start().starts_with('/') && !texto.trim_start().contains(' ');
            if mostra_popup {
                self.posicionar_slash_popup(cx);
                self.chat.slash_selected = 0;
                self.update_slash_popup_selection(cx);
            }
            self.chat_shows(cx, id!(ai_slash_popup), mostra_popup);
        }

        // Click actions on slash commands popup items
        let slash_popup = chat_panel.view(id!(ai_slash_popup));
        let row_hit = |i: &usize| slash_popup.button(&slash_row(*i)).clicked(actions);
        if let Some(idx) = (0..SLASH_ROWS.len()).find(row_hit) {
            self.chat.slash_selected = idx;
            self.run_slash_command(cx, idx);
        }
        self.history_actions(cx, actions);

        // Clicks in dynamic configuration menu slots
        let model_drop = chat_panel.view(id!(ai_model_dropdown));
        for i in 0..chat_bar::MENU_SLOTS {
            if model_drop.button(&chat_bar::menu_slot(i)).clicked(actions) {
                let open = self.chat.open_menu.take().unwrap_or_default();
                let menu = menus::menu_or_catalogue(&self.chat.live, &self.acp_config, &open);
                if let Some(menu) = menu {
                    let picked = menus::pick(&mut self.chat.live, &mut self.acp_config, &menu, i);
                    if let Err(e) = picked {
                        self.status = Some(format!("Could not change {}: {e}", menu.title));
                    }
                    self.sync_model_button(cx);
                }
                self.chat_shows(cx, id!(ai_model_dropdown), false);
                self.painel(Panel::Chat).redraw(cx);
                break;
            }
        }
    }
}

/// The slash popup's rows, in order; the first five write `/<name> ` in the prompt.
const SLASH_ROWS: [&str; 8] = [
    "explain", "refactor", "test", "docs", "find", "memory", "mcp", "new",
];

fn slash_row(index: usize) -> [LiveId; 2] {
    let name = format!("btn_cmd_{}", SLASH_ROWS[index % SLASH_ROWS.len()]);
    [live_id!(commands_list), LiveId::from_str(&name)]
}

impl IdeApp {
    /// Row `index` of the slash popup, as its click does.
    pub(crate) fn run_slash_command(&mut self, cx: &mut Cx, index: usize) {
        self.chat_shows(cx, id!(ai_slash_popup), false);
        match index {
            0..5 => {
                (self.get_chat_prompt_input()).set_text(cx, &format!("/{} ", SLASH_ROWS[index]))
            }
            5 => self.open_memory(cx),
            6 => self.open_mcp(cx),
            _ => self.open_new_project(cx, None),
        }
    }

    /// The keyboard's row looks like a hovered one.
    pub(crate) fn update_slash_popup_selection(&mut self, cx: &mut Cx) {
        let popup = self.painel(Panel::Chat).view(id!(ai_slash_popup));
        for i in 0..SLASH_ROWS.len() {
            // CRAWRAD_SELECAO under the chosen row, CRAWRAD_CONTROLE elsewhere
            let (bg, hover) = match i == self.chat.slash_selected {
                true => (vec4(0.114, 0.176, 0.239, 1.0), 1.0),
                false => (vec4(0.169, 0.169, 0.176, 1.0), 0.0),
            };
            let look = live! { draw_bg: { color_hover: (bg), hover: (hover) }, draw_text: { hover: (hover) } };
            popup.button(&slash_row(i)).apply_over(cx, look);
        }
        popup.redraw(cx);
    }

    /// ↑↓ move the popup's row, Enter runs it, Esc closes — none reach the prompt.
    pub(crate) fn slash_popup_keys(&mut self, cx: &mut Cx, event: &Event) -> bool {
        let popup = self.painel(Panel::Chat).view(id!(ai_slash_popup));
        let ke = match event {
            Event::KeyDown(ke) if popup.visible() && self.chat_prompt_focused(cx) => ke,
            _ => return false,
        };
        let (current, total) = (self.chat.slash_selected, SLASH_ROWS.len());
        match ke.key_code {
            KeyCode::ArrowUp => self.chat.slash_selected = (current + total - 1) % total,
            KeyCode::ArrowDown => self.chat.slash_selected = (current + 1) % total,
            KeyCode::ReturnKey => self.run_slash_command(cx, current),
            KeyCode::Escape => self.chat_shows(cx, id!(ai_slash_popup), false),
            _ => return false,
        }
        self.update_slash_popup_selection(cx);
        true
    }
}
