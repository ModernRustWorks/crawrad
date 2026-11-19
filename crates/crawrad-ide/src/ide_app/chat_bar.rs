//! The chat's input bar: the prompt queue, its popups and buttons. Cut out of `ide_app.rs`.

use super::*;
use crawrad_agent::chat_attachment::SelectionAttachment;
use crawrad_agent::chat_history::{self, HistoryStore};
use crawrad_note::note;

#[note("ide-app-chat-bar-queue-slots")]
pub(super) const QUEUE_SLOTS: usize = 8;

#[note("ide-app-chat-bar-queue-slot")]
fn queue_slot(i: usize) -> LiveId {
    LiveId::from_str(&format!("q_item_{i}"))
}

#[note("ide-app-chat-bar-menu-slots")]
pub(super) const MENU_SLOTS: usize = 16;

#[note("ide-app-chat-bar-chat-cards")]
pub(super) const CHAT_CARDS: usize = 16;

#[note("ide-app-chat-bar-chat-card")]
pub(super) fn chat_card(kind: &str, i: usize) -> [LiveId; 2] {
    let name = format!("{kind}_{i}");
    [
        live_id!(ai_messages_view),
        LiveId::from_str(name.strip_suffix("_0").unwrap_or(&name)),
    ]
}

#[note("ide-app-chat-bar-menu-slot")]
pub(super) fn menu_slot(i: usize) -> [LiveId; 1] {
    [LiveId::from_str(&format!("opt_{i}"))]
}

#[note("ide-app-chat-bar-history-slots")]
pub(super) const HISTORY_SLOTS: usize = 10;

fn history_slot(i: usize) -> [LiveId; 1] {
    [LiveId::from_str(&format!("hist_{i}"))]
}

impl IdeApp {
    #[note("ide-app-chat-bar-ideapp-toggle-history")]
    pub fn toggle_history(&mut self, cx: &mut Cx) {
        let chat = self.painel(Panel::Chat);
        let popup = chat.view(id!(ai_history_popup));
        if self.close_history(cx) {
            return;
        }
        let panel = chat.area().rect(cx);
        let btn = chat.button(id!(ai_header.btn_ai_history)).area().rect(cx);
        let (left, top) = (
            panel.size.x - 338.0,
            btn.pos.y + btn.size.y + 4.0 - panel.pos.y,
        );
        popup.apply_over(cx, live! { margin: { left: (left.max(0.0)), top: (top) } });
        for other in ["ai_context_popup", "ai_model_dropdown", "ai_slash_popup"] {
            chat.view(&[LiveId::from_str(other)]).set_visible(cx, false);
        }
        popup.set_visible(cx, true);
        popup.text_input(id!(hist_search)).set_text(cx, "");
        self.fill_history(cx);
    }

    #[note("ide-app-chat-bar-ideapp-close-history")]
    pub fn close_history(&mut self, cx: &mut Cx) -> bool {
        let popup = self.painel(Panel::Chat).view(id!(ai_history_popup));
        let was_open = popup.visible();
        popup.set_visible(cx, false);
        self.painel(Panel::Chat).redraw(cx);
        was_open
    }

    #[note("ide-app-chat-bar-ideapp-fill-history")]
    pub fn fill_history(&mut self, cx: &mut Cx) {
        let popup = self.painel(Panel::Chat).view(id!(ai_history_popup));
        let (store, open) = (
            HistoryStore::user_default(),
            self.chat.conversation.as_deref(),
        );
        let query = popup.text_input(id!(hist_search)).text();
        let found = chat_history::mcp_json(&store, open, &query)["history"].clone();
        let all = found.as_array().cloned().unwrap_or_default();
        let rows = chat_history::rows(&all, open, HISTORY_SLOTS);
        popup
            .label(id!(hist_count))
            .set_text(cx, &format!("{} conversations", all.len()));
        popup.view(id!(hist_empty)).set_visible(cx, rows.is_empty());
        for i in 0..HISTORY_SLOTS {
            let (slot, row) = (popup.view(&history_slot(i)), rows.get(i));
            slot.set_visible(cx, row.is_some());
            let Some(r) = row else { continue };
            slot.view(id!(h_group_box))
                .set_visible(cx, r.group.is_some());
            let group = r.group.as_deref().unwrap_or_default();
            let texts = [group, r.title.as_str(), &r.meta, &r.when, r.resume];
            for (name, text) in ["h_group", "h_title", "h_meta", "h_when", "h_resume"]
                .iter()
                .zip(texts)
            {
                slot.label(&[LiveId::from_str(name)]).set_text(cx, text);
            }
            slot.view(id!(h_now)).set_visible(cx, r.open);
            slot.view(id!(h_bar)).set_visible(cx, r.open);
        }
        self.chat.history_rows = rows.into_iter().map(|r| r.id).collect();
        self.painel(Panel::Chat).redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-history-actions")]
    pub(super) fn history_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let popup = self.painel(Panel::Chat).view(id!(ai_history_popup));
        let search = popup.text_input(id!(hist_search));
        if search.escaped(actions) {
            self.close_history(cx);
            return;
        }
        if search.changed(actions).is_some() {
            return self.fill_history(cx);
        }
        let hit = (0..HISTORY_SLOTS).find(|&i| {
            popup
                .view(&history_slot(i))
                .button(id!(h_hit))
                .clicked(actions)
        });
        if let Some(id) = hit.and_then(|i| self.chat.history_rows.get(i).cloned()) {
            self.close_history(cx);
            self.open_conversation(cx, &id);
        }
    }

    #[note("ide-app-chat-bar-ideapp-history-json")]
    pub(super) fn history_json(&self, cx: &Cx) -> serde_json::Value {
        let popup = self.painel(Panel::Chat).view(id!(ai_history_popup));
        let rect = |r: Rect| serde_json::json!(xywh(r));
        let rows = self.chat.history_rows.iter().enumerate().map(|(i, id)| {
            let item = popup
                .view(&history_slot(i))
                .view(id!(h_item))
                .area()
                .rect(cx);
            serde_json::json!({"id": id, "rect": rect(item)})
        });
        let search = rect(popup.text_input(id!(hist_search)).area().rect(cx));
        serde_json::json!({"open": popup.visible(), "search_rect": search, "rows": rows.collect::<Vec<_>>()})
    }
    #[note("ide-app-chat-bar-ideapp-enfileirar-ai-prompt")]
    pub fn enfileirar_ai_prompt(
        &mut self,
        cx: &mut Cx,
        prompt: &str,
        attachments: Vec<SelectionAttachment>,
        images: Vec<crawrad_agent::ImageAttachment>,
    ) {
        if prompt.trim().is_empty() {
            return;
        }
        let seq = self.chat.queue.len() + 1;
        self.chat.queue.push(
            crawrad_agent::QueuedMessage::new(format!("queue-{seq}"), prompt.trim())
                .with_attachments(attachments)
                .with_images(images),
        );
        self.chat.queue_collapsed = false;
        self.update_ai_queue_visual(cx);
    }

    #[note("ide-app-chat-bar-ideapp-remove-ai-queue-item")]
    pub fn remove_ai_queue_item(&mut self, cx: &mut Cx, index: usize) {
        if index < self.chat.queue.len() {
            self.chat.queue.remove(index);
            self.update_ai_queue_visual(cx);
        }
    }

    #[note("ide-app-chat-bar-ideapp-editar-ai-queue-item")]
    pub fn editar_ai_queue_item(&mut self, cx: &mut Cx, index: usize) {
        if index < self.chat.queue.len() {
            let item = self.chat.queue.remove(index);
            self.get_chat_prompt_input().set_text(cx, &item.text);
            self.chat.attachments.extend(item.attachments);
            self.chat.images.extend(item.images);
            self.render_attachment_chips(cx);
            self.update_ai_queue_visual(cx);
        }
    }

    #[note("ide-app-chat-bar-ideapp-enviar-ai-queue-item")]
    pub fn enviar_ai_queue_item(&mut self, cx: &mut Cx, index: usize) {
        if index < self.chat.queue.len() {
            let item = self.chat.queue.remove(index);
            self.send_queued(cx, item);
        }
    }

    #[note("ide-app-chat-bar-ideapp-alternar-ai-queue-expanded")]
    pub fn alternar_ai_queue_expanded(&mut self, cx: &mut Cx) {
        self.chat.queue_collapsed = !self.chat.queue_collapsed;
        self.update_ai_queue_visual(cx);
    }

    #[note("ide-app-chat-bar-ideapp-despachar-cliques-fila-chat")]
    pub fn despachar_cliques_fila_chat(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let chat_panel = self.painel(Panel::Chat);
        for idx in 0..QUEUE_SLOTS {
            let v = chat_panel.view(&[
                live_id!(ai_queue_card),
                live_id!(queue_items),
                queue_slot(idx),
            ]);
            if v.button(id!(btn_q_send)).clicked(actions) {
                self.enviar_ai_queue_item(cx, idx);
                return true;
            } else if v.button(id!(btn_q_edit)).clicked(actions) {
                self.editar_ai_queue_item(cx, idx);
                return true;
            } else if v.button(id!(btn_q_del)).clicked(actions) {
                self.remove_ai_queue_item(cx, idx);
                return true;
            }
        }
        false
    }

    #[note("ide-app-chat-bar-ideapp-processar-proxima-da-fila")]
    pub fn processar_proxima_da_fila(&mut self, cx: &mut Cx) {
        if !self.chat.queue.is_empty() && !self.chat.busy {
            let proxima = self.chat.queue.remove(0);
            self.send_queued(cx, proxima);
        }
    }

    /// Sends a queued message with ITS images; the draft's pasted ones stay.
    fn send_queued(&mut self, cx: &mut Cx, item: crawrad_agent::QueuedMessage) {
        let draft = std::mem::replace(&mut self.chat.images, item.images);
        self.update_ai_queue_visual(cx);
        self.execute_prompt_with(cx, &item.text, item.attachments);
        self.chat.images = draft;
        self.render_attachment_chips(cx);
    }

    #[note("ide-app-chat-bar-ideapp-position-config-menu")]
    pub fn position_config_menu(&mut self, cx: &mut Cx, category: &str) {
        let chat = self.painel(Panel::Chat);
        let btn = if category == "thought_level" {
            self.chat_tool("btn_ai_effort")
        } else {
            self.chat_tool("btn_ai_model")
        };
        let btn_rect = btn.area().rect(cx);
        let panel_rect = chat.area().rect(cx);
        let bar_rect = self.get_chat_input_bar().area().rect(cx);

        let menu = crawrad_agent::chat_config::menu_or_catalogue(
            &self.chat.live,
            &self.acp_config,
            category,
        );
        let (title, items) = menu.map(|m| (m.title, m.items)).unwrap_or_default();
        let visible_count = items.len().clamp(1, MENU_SLOTS);
        let drop_h = (visible_count as f64 * 26.0 + 34.0).max(40.0);
        let (bx, panel, bar_y) = (btn_rect.pos.x, panel_rect, bar_rect.pos.y);
        let (rel_x, rel_y) =
            crawrad_agent::chat_config::menu_origin(bx, xywh(panel), bar_y, drop_h);

        let drop = chat.view(id!(ai_model_dropdown));
        drop.apply_over(cx, live! { margin: { left: (rel_x), top: (rel_y) } });
        drop.label(id!(menu_title)).set_text(cx, &title);

        for i in 0..MENU_SLOTS {
            let slot = drop.button(&menu_slot(i));
            if let Some(it) = items.get(i) {
                let check = if it.selected { "✓ " } else { "   " };
                slot.set_text(cx, &format!("{check}{}", it.label));
                slot.set_visible(cx, true);
            } else {
                slot.set_visible(cx, false);
            }
        }

        self.chat.open_menu = Some(category.to_string());
        drop.set_visible(cx, true);
        self.painel(Panel::Chat).redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-posicionar-context-popup")]
    pub fn posicionar_context_popup(&mut self, cx: &mut Cx) {
        let chat = self.painel(Panel::Chat);
        let btn = self.chat_tool("btn_ai_attach");
        let btn_rect = btn.area().rect(cx);
        let panel_rect = chat.area().rect(cx);
        let bar = self.get_chat_input_bar();
        let bar_rect = bar.area().rect(cx);
        let popup = chat.view(id!(ai_context_popup));
        let p_rect = popup.area().rect(cx);

        // unsized before its first draw: 215 is its measured height (245 left a 30 px gap)
        let popup_h = Some(p_rect.size.y).filter(|h| *h > 60.0).unwrap_or(215.0);

        let panel = (panel_rect.pos.x, panel_rect.pos.y, panel_rect.size.y);
        let (x, y) = crawrad_agent::chat_layout::context_popup_margin(
            (btn_rect.pos.x, btn_rect.pos.y),
            (bar_rect.pos.x, bar_rect.pos.y),
            panel,
            popup_h,
        );

        popup.apply_over(
            cx,
            live! {
                margin: { left: (x), top: (y) }
            },
        );
        self.painel(Panel::Chat).redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-posicionar-slash-popup")]
    pub fn posicionar_slash_popup(&mut self, cx: &mut Cx) {
        let chat = self.painel(Panel::Chat);
        let panel_rect = chat.area().rect(cx);
        let bar = self.get_chat_input_bar();
        let bar_rect = bar.area().rect(cx);
        let popup = chat.view(id!(ai_slash_popup));
        let p_rect = popup.area().rect(cx);

        let popup_h = if p_rect.size.y > 60.0 {
            p_rect.size.y
        } else {
            280.0
        };

        let panel = (panel_rect.pos.x, panel_rect.pos.y, panel_rect.size.y);
        let bar = (bar_rect.pos.x, bar_rect.pos.y);
        let (x, y) = crawrad_agent::chat_layout::slash_popup_margin(bar, panel, popup_h);

        popup.apply_over(
            cx,
            live! {
                margin: { left: (x), top: (y) }
            },
        );
        self.painel(Panel::Chat).redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-update-ai-queue-visual")]
    pub fn update_ai_queue_visual(&mut self, cx: &mut Cx) {
        let chat = self.painel(Panel::Chat);
        let count = self.chat.queue.len();
        let mut card = chat.view(id!(chat_base.ai_queue_card));
        if card.is_empty() {
            card = chat.view(id!(ai_queue_card));
        }

        if count == 0 {
            card.set_visible(cx, false);
            card.redraw(cx);
            chat.redraw(cx);
            self.ui.redraw(cx);
            return;
        }

        card.set_visible(cx, true);
        card.label(id!(queue_header.badge_queue_count.lbl_queue_count))
            .set_text(cx, &count.to_string());

        let toggle_icon = if !self.chat.queue_collapsed {
            icon::Chrome::ChevronDown.glyph()
        } else {
            icon::Chrome::ChevronRight.glyph()
        };
        card.button(id!(queue_header.btn_queue_toggle))
            .set_text(cx, toggle_icon);

        let items_view = card.view(id!(queue_items));
        items_view.set_visible(cx, !self.chat.queue_collapsed);

        if !self.chat.queue_collapsed {
            for i in 0..QUEUE_SLOTS {
                let slot = card.view(&[live_id!(queue_items), queue_slot(i)]);
                if let Some(msg) = self.chat.queue.get(i) {
                    slot.set_visible(cx, true);
                    slot.label(id!(q_item_text)).set_text(cx, &msg.text);
                    slot.button(id!(btn_q_send))
                        .set_text(cx, icon::Chrome::ArrowRight.glyph());
                    slot.button(id!(btn_q_edit))
                        .set_text(cx, icon::Chrome::Edit.glyph());
                    slot.button(id!(btn_q_del))
                        .set_text(cx, icon::Chrome::ClearChat.glyph());
                } else {
                    slot.set_visible(cx, false);
                }
            }
        }
        card.redraw(cx);
        chat.redraw(cx);
        self.ui.redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-get-active-agent-name")]
    pub fn get_active_agent_name(&self) -> String {
        let profile = crawrad_agent::agent_profile::find_profile_by_id(&self.acp_config.agent_id)
            .unwrap_or_else(|| {
                crawrad_agent::agent_profile::resolve_profile(&self.acp_config.provider)
            });
        profile.display_name.to_string()
    }

    #[note("ide-app-chat-bar-ideapp-get-chat-prompt-input")]
    pub(crate) fn get_chat_prompt_input(&self) -> TextInputRef {
        let cp = self.painel(Panel::Chat);
        let i = cp.text_input(id!(chat_base.ai_input_bar.prompt_input));
        if !i.is_empty() {
            i
        } else {
            cp.text_input(id!(prompt_input))
        }
    }

    #[note("ide-app-chat-bar-ideapp-update-chat-buttons-state")]
    pub fn update_chat_buttons_state(&mut self, cx: &mut Cx) {
        let btn_send = self.chat_tool("btn_prompt_send");
        let btn_stop = self.chat_tool("btn_prompt_stop");
        if self.chat.busy {
            btn_send.set_visible(cx, false);
            btn_stop.set_visible(cx, true);
        } else {
            btn_send.set_visible(cx, true);
            btn_stop.set_visible(cx, false);
        }
        btn_send.redraw(cx);
        btn_stop.redraw(cx);
    }

    #[note("ide-app-chat-bar-ideapp-chat-tool")]
    pub(crate) fn chat_tool(&self, name: &str) -> ButtonRef {
        let cp = self.painel(Panel::Chat);
        let path = [
            live_id!(chat_base),
            live_id!(ai_input_bar),
            live_id!(ai_tools_bar),
            LiveId::from_str(name),
        ];
        Some(cp.button(&path))
            .filter(|b| !b.is_empty())
            .unwrap_or_else(|| cp.button(&path[3..]))
    }

    pub(super) fn get_chat_input_bar(&self) -> ViewRef {
        let chat_panel = self.painel(Panel::Chat);
        let mut bar = chat_panel.view(id!(chat_base.ai_input_bar));
        if bar.is_empty() {
            bar = chat_panel.view(id!(ai_input_bar));
        }
        bar
    }

    #[note("ide-app-chat-bar-ideapp-sync-model-button")]
    pub fn sync_model_button(&mut self, cx: &mut Cx) {
        let (model, effort) =
            crawrad_agent::chat_config::button_labels(&self.chat.live, &self.acp_config);
        self.chat_tool("btn_ai_model").set_text(cx, &model);
        let btn = self.chat_tool("btn_ai_effort");
        btn.set_visible(cx, effort.is_some());
        btn.set_text(cx, effort.as_deref().unwrap_or_default());
    }

    #[note("ide-app-chat-bar-ideapp-abrir-url-navegador")]
    pub fn abrir_url_navegador(&mut self, url: &str) {
        self.status = Some(crawrad_runtime::link::open_and_say(url));
    }
}
