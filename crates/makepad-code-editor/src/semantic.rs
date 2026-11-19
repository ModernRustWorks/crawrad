//! **Semantic colours over the tokenizer's**: spans the language
//! server classified (a macro, a trait, a `mut` variable…) drawn in their own
//! colour; everywhere else the tokenizer's colour holds, so a file looks
//! right while the server is still indexing.
//!
//! The spans follow every edit as the squiggles do ([`Position::apply_edit`]),
//! so typing does not make the colours jump until the server answers again.

use crate::text::{Edit, Position};

/// The classes the editor colours on its own.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SemanticKind {
    Macro,
    Trait,
    /// A binding or parameter declared `mut`.
    Mutable,
    Parameter,
    Field,
    EnumMember,
    /// The `unsafe` keyword, also inside an attribute, and what is unsafe to
    /// use.
    Unsafe,
}

impl SemanticKind {
    /// Every kind, by the index a language client sends (`crawrad_lsp::semantic::Class`).
    pub const ALL: [SemanticKind; 7] = [
        SemanticKind::Macro,
        SemanticKind::Trait,
        SemanticKind::Mutable,
        SemanticKind::Parameter,
        SemanticKind::Field,
        SemanticKind::EnumMember,
        SemanticKind::Unsafe,
    ];
}

/// One classified span of the text: its colour (`None` keeps the
/// tokenizer's), and whether it names something deprecated, struck through
/// while typing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SemanticSpan {
    pub start: Position,
    pub end: Position,
    pub kind: Option<SemanticKind>,
    pub struck: bool,
}

/// The spans of a document, ordered by start.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SemanticSet {
    spans: Vec<SemanticSpan>,
}

impl SemanticSet {
    /// The server's spans for the current text; any order.
    pub fn set(&mut self, mut spans: Vec<SemanticSpan>) {
        spans.sort_by_key(|s| s.start);
        self.spans = spans;
    }

    pub fn as_spans(&self) -> &[SemanticSpan] {
        &self.spans
    }

    /// The text changed: every span moves with it. A span the edit swallowed
    /// collapses to nothing and is dropped.
    pub fn apply_edit(&mut self, edit: &Edit) {
        for span in &mut self.spans {
            span.start = span.start.apply_edit(edit);
            span.end = span.end.apply_edit(edit);
        }
        self.spans.retain(|s| s.start < s.end);
    }
}

/// Walks the spans in text order: for each position asked in increasing
/// order, the span covering it, if any.
pub struct SemanticCursor<'a> {
    spans: &'a [SemanticSpan],
    next: usize,
}

impl<'a> SemanticCursor<'a> {
    pub fn new(spans: &'a [SemanticSpan]) -> Self {
        Self { spans, next: 0 }
    }

    /// The span at `at`; positions must not go backwards between calls.
    pub fn at(&mut self, at: Position) -> Option<SemanticSpan> {
        while self.spans.get(self.next).is_some_and(|s| s.end <= at) {
            self.next += 1;
        }
        let span = self.spans.get(self.next)?;
        (span.start <= at).then_some(*span)
    }
}

#[cfg(test)]
#[path = "test/semantic/test.rs"]
mod tests;
