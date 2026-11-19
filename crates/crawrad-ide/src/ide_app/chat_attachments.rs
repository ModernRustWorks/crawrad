//! The editor's selection as a chat attachment, drawn as a chip with ✕.

use super::*;
use crawrad_agent::chat_attachment::{MAX_ATTACHMENTS, SelectionAttachment};
use crawrad_note::note;

fn chip_slot(i: usize) -> [LiveId; 1] {
    [LiveId::from_str(&format!("att_{i}"))]
}

impl IdeApp {
    #[note("ide-app-chat-attachments-ideapp-capture-selection")]
    fn capture_selection(&self) -> Option<SelectionAttachment> {
        let (sel, path) = self
            .extrair_selecao_do_editor()
            .zip(self.caminho_ativo_para_contexto())?;
        let file = self.shown_path(&path);
        let text = &self.texto_na_vista;
        Some(SelectionAttachment::capture(&file, &sel, text, 12 * 1024))
    }

    #[note("ide-app-chat-attachments-ideapp-add-selection-to-chat")]
    pub fn add_selection_to_chat(&mut self, cx: &mut Cx) {
        let refusal = match self.capture_selection() {
            None => Some("select some code first"),
            Some(_) if self.chat.attachments.len() + self.chat.images.len() >= MAX_ATTACHMENTS => {
                Some("up to 4 attachments: remove one first")
            }
            Some(att) => {
                self.chat.attachments.push(att);
                None
            }
        };
        match refusal {
            Some(said) => self.status = Some(said.into()),
            None => self.show_panel(cx, Panel::Chat),
        }
        self.render_attachment_chips(cx);
        self.sincronizar(cx);
    }

    #[note("ide-app-chat-attachments-ideapp-add-file-to-chat")]
    pub fn add_file_to_chat(&mut self, cx: &mut Cx) {
        if let Some(path) = self.caminho_ativo_para_contexto() {
            let shown = self.shown_path(&path);
            let input = self.get_chat_prompt_input();
            let mut text = input.text();
            if !text.is_empty() && !text.ends_with(|c: char| c.is_whitespace()) {
                text.push(' ');
            }
            text.push('@');
            text.push_str(&shown);
            text.push(' ');
            input.set_text(cx, &text);
            self.show_panel(cx, Panel::Chat);
        } else {
            self.status = Some("open a file first".into());
        }
        self.sincronizar(cx);
    }

    #[note("ide-app-chat-attachments-ideapp-ask-about-selection")]
    pub fn ask_about_selection(&mut self, cx: &mut Cx, command: &str) {
        match self.capture_selection() {
            None => self.status = Some("select some code first".into()),
            Some(att) => {
                self.show_panel(cx, Panel::Chat);
                if self.chat.busy {
                    self.enfileirar_ai_prompt(cx, command, vec![att], vec![]);
                } else {
                    self.execute_prompt_with(cx, command, vec![att]);
                }
            }
        }
        self.sincronizar(cx);
    }

    #[note("ide-app-chat-attachments-ideapp-paste-image-into-chat")]
    pub fn paste_image_into_chat(&mut self, cx: &mut Cx) -> bool {
        let taken = self.chat.attachments.len() + self.chat.images.len();
        match crawrad_agent::paste_image(taken) {
            None => return false,
            Some(Ok(img)) => self.chat.images.push(img),
            Some(Err(why)) => self.status = Some(why),
        }
        self.render_attachment_chips(cx);
        true
    }

    #[note("ide-app-chat-attachments-ideapp-render-attachment-chips")]
    pub fn render_attachment_chips(&mut self, cx: &mut Cx) {
        let chips = self.get_chat_input_bar().view(id!(ai_attach_chips));
        let texts = self.attachment_chip_texts();
        chips.set_visible(cx, !texts.is_empty());
        for i in 0..MAX_ATTACHMENTS {
            let chip = chips.view(&chip_slot(i));
            chip.set_visible(cx, i < texts.len());
            if let Some(t) = texts.get(i) {
                chip.label(id!(att_text)).set_text(cx, t);
                let close = chip.button(id!(btn_att_remove));
                close.set_text(cx, icon::Chrome::Close.glyph());
            }
        }
        self.painel(Panel::Chat).redraw(cx);
    }

    #[note("ide-app-chat-attachments-ideapp-attachment-chip-texts")]
    fn attachment_chip_texts(&self) -> Vec<String> {
        let active = self
            .caminho_ativo_para_contexto()
            .map(|p| self.shown_path(&p));
        let now = |a: &SelectionAttachment| {
            (active.as_deref() == Some(a.file.as_str())).then_some(self.texto_na_vista.as_str())
        };
        let images = self.chat.images.iter().enumerate();
        let atts = self.chat.attachments.iter().map(|a| a.chip_text(now(a)));
        atts.chain(images.map(|(i, img)| img.chip_text(i)))
            .collect()
    }

    #[note("ide-app-chat-attachments-ideapp-attachments-json")]
    pub(super) fn attachments_json(&self, cx: &Cx) -> serde_json::Value {
        let chips = self.get_chat_input_bar().view(id!(ai_attach_chips));
        let texts = self.attachment_chip_texts().into_iter().enumerate();
        let rows = texts.map(|(i, text)| {
            let close = chips.view(&chip_slot(i)).button(id!(btn_att_remove));
            let r = close.area().rect(cx);
            serde_json::json!({"text": text, "remove": xywh(r)})
        });
        rows.collect::<Vec<_>>().into()
    }

    #[note("ide-app-chat-attachments-ideapp-shown-path")]
    fn shown_path(&self, path: &std::path::Path) -> String {
        let root = self.raiz();
        let shown = root.as_deref().and_then(|r| path.strip_prefix(r).ok());
        shown.unwrap_or(path).to_string_lossy().replace('\\', "/")
    }

    #[note("ide-app-chat-attachments-ideapp-dispatch-attachment-clicks")]
    pub fn dispatch_attachment_clicks(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let chips = self.get_chat_input_bar().view(id!(ai_attach_chips));
        let close = |i: usize| chips.view(&chip_slot(i)).button(id!(btn_att_remove));
        let Some(i) = (0..MAX_ATTACHMENTS).find(|i| close(*i).clicked(actions)) else {
            return false;
        };
        crawrad_agent::remove_chip(&mut self.chat.attachments, &mut self.chat.images, i);
        self.render_attachment_chips(cx);
        true
    }
}
