//! A sentence the panel says instead of a list.
//!
//! `No folder open.`, `No form here yet.`, `cannot plan: …`. The prose wraps,
//! which is why it is not a badge row: a sentence in two columns fits in
//! neither.

use super::super::icon::{NOTE, RowIcon, WARNING};
use super::super::row::ExplorerRow;
use crawrad_note::note;

#[note("explorer-rows-rows-note-note")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    text: String,
    warn: bool,
    indent: usize,
}

impl Note {
    pub fn new(text: impl Into<String>, warn: bool, indent: usize) -> Self {
        Self {
            text: text.into(),
            warn,
            indent,
        }
    }
}

impl ExplorerRow for Note {
    fn template(&self) -> &'static str {
        if self.warn { "NoteWarn" } else { "Note" }
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn indent(&self) -> usize {
        self.indent
    }

    fn icon(&self) -> Option<RowIcon> {
        Some(if self.warn { WARNING } else { NOTE })
    }
}
