//! The count line of a member — `4 files · 2 not generated yet`.
//!
//! No icon: it is a total, not a thing. A drawing next to it would make the
//! eye look for a file that is not there.

use super::super::row::ExplorerRow;
use crawrad_note::note;

#[note("explorer-rows-rows-summary-summary")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Summary {
    text: String,
    indent: usize,
}

impl Summary {
    pub fn new(text: impl Into<String>, indent: usize) -> Self {
        Self {
            text: text.into(),
            indent,
        }
    }
}

impl ExplorerRow for Summary {
    fn template(&self) -> &'static str {
        "Summary"
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn indent(&self) -> usize {
        self.indent
    }
}
