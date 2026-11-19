//! **Semantic colours**: what the language server classified, over
//! the tokenizer's colours. Asked on every open and edit (a delta after the
//! first, `crawrad_lsp::semantic`); the spans follow the edits in the document,
//! so the colours do not jump while the next answer is on its way.

use super::*;
use makepad_code_editor::code_view::CodeView;
use makepad_code_editor::semantic::{SemanticKind, SemanticSpan};
use makepad_code_editor::text::Position;
use std::sync::atomic::{AtomicU64, Ordering};

/// The newest request; an older answer is dropped.
static TICKET: AtomicU64 = AtomicU64::new(0);

impl IdeApp {
    /// After an open or an edit of `file` at the current version.
    pub(crate) fn ask_semantic(&self, file: &std::path::Path) {
        if let Some(lsp) = self.lsp.clone() {
            let ticket = TICKET.fetch_add(1, Ordering::Relaxed) + 1;
            let version = self.lsp_versao;
            let assist = &self.lsp_hints;
            assist.code_lenses.request(lsp.clone(), file, version);
            assist.semantic.request(lsp, ticket, file, version);
        }
    }

    /// The answer for the text in view: its spans into the document.
    pub(super) fn poll_semantic(&mut self, cx: &mut Cx) {
        let again = self.lsp.as_ref().is_some_and(|l| l.take_semantic_refresh());
        if let Some(file) = self.editavel.clone().filter(|_| again) {
            self.ask_semantic(&file);
        }
        let ticket = TICKET.load(Ordering::Relaxed);
        let (Some(answer), Some(lsp)) = (self.lsp_hints.semantic.take(ticket), self.lsp.clone())
        else {
            return;
        };
        let same = self.editavel.as_deref() == Some(answer.file.as_path());
        let (Some(data), true) = (answer.data, same && answer.version == self.lsp_versao) else {
            return;
        };
        let (legend, enc) = (lsp.semantic_legend(), lsp.position_encoding());
        let spans = crawrad_lsp::semantic::decode(&data, &legend, &self.texto_na_vista, enc);
        let at = |line_index, byte_index| Position {
            line_index,
            byte_index,
        };
        let spans = spans.iter().map(|s| SemanticSpan {
            start: at(s.line, s.start),
            end: at(s.line, s.end),
            kind: s.class.map(|c| SemanticKind::ALL[c as usize]),
            struck: s.deprecated,
        });
        let view = self.code_view();
        let doc = view
            .borrow::<CodeView>()
            .and_then(|cv| Some(cv.session.as_ref()?.document().clone()));
        if let Some(doc) = doc {
            doc.set_semantic(spans.collect());
        }
        view.redraw(cx);
    }

    /// What `editor.get_semantic_tokens` reads: each coloured span, 1-based
    /// line, its bytes, its kind and its text.
    pub(super) fn semantic_json(&self) -> serde_json::Value {
        let doc = |cv: &CodeView| Some(cv.session.as_ref()?.document().semantic_json());
        self.with_code_view(doc).flatten().unwrap_or_default()
    }
}
