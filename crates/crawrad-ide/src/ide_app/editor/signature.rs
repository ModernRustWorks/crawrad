//! **The parameter card**: the signature of the call around the caret,
//! its current parameter in Rust orange. Typing `(` or `,` opens it, every
//! later edit asks again (so a deleted `,` moves the mark back), Esc closes
//! it, and the server's empty answer — the caret left the call — closes it.
//! When to ask is `crawrad_insight::typing::signature_edit`; the request and its
//! reading are `crawrad_lsp::signature`.

use super::*;
use crawrad_insight::typing::SignatureEdit;
use crawrad_lsp::signature::{Asked, Outcome, Signature, Trigger};
use makepad_code_editor::code_view::CodeView;

impl IdeApp {
    /// After every edit, as the completion list follows it.
    pub(crate) fn signature_follows_typing(&mut self, cx: &mut Cx) {
        let (line, byte) = self.cursor_do_code_view(cx);
        let text = self.texto_na_vista.lines().nth(line).unwrap_or("");
        let before = text.get(..byte).unwrap_or(text);
        let triggers = self.lsp.as_ref().map(|l| l.signature_triggers());
        let card = &self.nav.signature;
        let open = card.shown().is_some() || card.pending_ticket().is_some();
        let edit =
            crawrad_insight::typing::signature_edit(before, &triggers.unwrap_or_default(), open);
        let trigger = match edit {
            Some(SignatureEdit::Typed(c)) => Trigger::Character(c),
            Some(SignatureEdit::Follow) => Trigger::ContentChange,
            None => return,
        };
        self.ask_signature((line, byte), trigger, open);
    }

    /// No edit, but the caret moved under an open card: it follows.
    pub(crate) fn signature_follows_caret(&mut self, cx: &mut Cx) {
        let caret = self.cursor_do_code_view(cx);
        if self.nav.signature.caret_moved(caret) {
            self.ask_signature(caret, Trigger::ContentChange, true);
        }
    }

    /// `editor.parameter_hints`: the card at the caret, asked by key.
    pub(super) fn signature_at_caret(&mut self, cx: &mut Cx) {
        let caret = self.cursor_do_code_view(cx);
        let open = self.nav.signature.shown().is_some();
        if !self.ask_signature(caret, Trigger::Invoked, open) {
            self.status = Some("Parameter hints need rust-analyzer and a code file".into());
        }
    }

    fn ask_signature(&mut self, caret: (usize, usize), trigger: Trigger, retrigger: bool) -> bool {
        let view = self.code_view();
        let at = view.borrow::<CodeView>().and_then(|cv| cv.caret_anchor());
        let (Some(lsp), Some(file), Some(at)) = (self.lsp.clone(), self.abas.active_file(), at)
        else {
            return false;
        };
        let file = file.to_path_buf();
        let (x, y, line_height) = (at.at.x, at.at.y, at.line_height);
        let anchor = crawrad_host::hover_card::Anchor { x, y, line_height };
        let ticket = self.nav.signature.ask(anchor, trigger == Trigger::Invoked);
        let asked = Asked {
            file: &file,
            version: self.lsp_versao,
            text: &self.texto_na_vista,
            caret,
            trigger,
            retrigger,
        };
        self.lsp_hints.signature.request(lsp, ticket, &asked)
    }

    /// The answer: the card shown or moved, or closed when there is no call.
    pub(super) fn poll_signature(&mut self, cx: &mut Cx) {
        let Some(ticket) = self.nav.signature.pending_ticket() else {
            return;
        };
        let Some(answer) = self.lsp_hints.signature.take(ticket) else {
            return;
        };
        let Some((anchor, by_key)) = self.nav.signature.answered(ticket) else {
            return;
        };
        match answer.outcome {
            Outcome::Shown(sig) if answer.version == self.lsp_versao => {
                self.show_signature(cx, &sig, anchor)
            }
            Outcome::Shown(_) => {}
            Outcome::Nothing => {
                if by_key {
                    self.status = Some("No call around the caret".into());
                }
                self.close_signature(cx);
            }
            Outcome::NoAnswer(_) if by_key => {
                self.status = Some("Language server did not answer".into())
            }
            Outcome::NoAnswer(_) => {}
        }
    }

    fn show_signature(
        &mut self,
        cx: &mut Cx,
        sig: &Signature,
        anchor: crawrad_host::hover_card::Anchor,
    ) {
        let card = self.signature_card();
        let (before, active, after) = sig.parts();
        card.label(id!(sig_before)).set_text(cx, before);
        card.label(id!(sig_active)).set_text(cx, active);
        card.label(id!(sig_after)).set_text(cx, after);
        let width = sig.label.chars().count() as f64 * 8.0 + 24.0;
        let size = (width, anchor.line_height.max(16.0) + 12.0);
        // The card's own size, as the hover card has: `Fit` alone left it
        // open by the MCP read and drawn nowhere (seen on the manual's capture).
        card.apply_over(cx, live! { width: (size.0), height: (size.1) });
        self.place_card(cx, &card, anchor, size);
        self.nav.signature.shown_at(anchor);
    }

    /// Closes the parameter card; whether one was up.
    pub(super) fn close_signature(&mut self, cx: &mut Cx) -> bool {
        let was = self.nav.signature.close();
        if was {
            self.signature_card().set_visible(cx, false);
            self.ui.redraw(cx);
        }
        was
    }

    /// What `editor.get_signature_help` reads: the card, its signature and
    /// the parameter marked.
    pub(super) fn signature_json(&self) -> serde_json::Value {
        let card = self.signature_card();
        let open = self.nav.signature.shown().is_some();
        let [before, active, after] =
            [id!(sig_before), id!(sig_active), id!(sig_after)].map(|p| card.label(p).text());
        let (label, active) = match open {
            true => (format!("{before}{active}{after}"), active),
            false => (String::new(), String::new()),
        };
        serde_json::json!({ "open": open, "label": label, "active_parameter": active })
    }

    fn signature_card(&self) -> ViewRef {
        self.painel(Panel::Design)
            .view(id!(code_container.signature_box))
    }
}
