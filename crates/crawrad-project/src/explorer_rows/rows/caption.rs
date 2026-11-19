//! The label of a group inside the panel — `Group`, `Forms`, `src/`.
//!
//! It is the one row with no icon on purpose. In the reference a section label
//! carries no drawing either: what an icon would say here (*this is a group*)
//! is already said by the label being a label.

use super::super::row::ExplorerRow;
use crawrad_note::note;

#[note("explorer-rows-rows-caption-caption")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caption {
    text: String,
    indent: usize,
}

impl Caption {
    pub fn new(text: impl Into<String>, indent: usize) -> Self {
        Self {
            text: text.into(),
            indent,
        }
    }
}

impl ExplorerRow for Caption {
    fn template(&self) -> &'static str {
        "Caption"
    }

    fn text(&self) -> &str {
        &self.text
    }

    fn indent(&self) -> usize {
        self.indent
    }
}
