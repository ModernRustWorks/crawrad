//! **The completion item's documentation**: a panel beside the list
//! with the selected item's doc. What to ask is `crawrad_insight::item::DocPanel`;
//! the request is `crawrad_lsp::resolve`.

use super::*;
use crawrad_insight::item::DocStep;

impl IdeApp {
    /// After the list is drawn and on every wake: the selection followed,
    /// the answer taken, the panel drawn when something changed.
    pub(crate) fn follow_item_doc(&mut self, cx: &mut Cx) {
        let c = &self.complete;
        let selected = c
            .aberto
            .then(|| c.sugestoes.get(c.selecionado))
            .flatten()
            .cloned();
        let mut changed = match self.complete.doc.select(selected.as_ref()) {
            DocStep::Ask(ticket, label, data) => {
                if let Some(lsp) = self.lsp.clone() {
                    self.lsp_hints.docs.request(lsp, ticket, &label, data);
                }
                true
            }
            DocStep::Redraw => true,
            DocStep::Same => false,
        };
        let ticket = self.complete.doc.pending();
        if let Some(answer) = ticket.and_then(|t| self.lsp_hints.docs.take(t)) {
            changed |= self.complete.doc.answered(answer.ticket, answer.doc);
        }
        if changed {
            self.draw_item_doc(cx);
        }
    }

    /// The panel at the list's right edge, its top aligned with the list's.
    fn draw_item_doc(&mut self, cx: &mut Cx) {
        let container = self.painel(Panel::Design).widget(id!(code_container));
        let panel = container.view(id!(completion_doc));
        match self
            .complete
            .doc
            .text
            .as_deref()
            .filter(|_| self.complete.aberto)
        {
            Some(text) => {
                let text = crawrad_host::hover_card::clip(text, 20);
                panel.text_input(id!(doc_text)).set_text(cx, &text);
                let [x, y, w, _] = self.complete.area;
                let origin = container.area().rect(cx).pos;
                let (x, y) = (x + w + 4.0 - origin.x, y - origin.y);
                panel.apply_over(cx, live! { margin: { left: (x), top: (y) } });
                panel.set_visible(cx, true);
            }
            None => panel.set_visible(cx, false),
        }
        self.ui.redraw(cx);
    }
}
