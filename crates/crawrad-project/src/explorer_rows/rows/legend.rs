//! One pair of the legend: the word above, the phrase that explains it below.
//!
//! It stopped being a badge row long before this round, and the reason still
//! holds: the phrase is PROSE, and prose in two columns fits in neither — even
//! at the panel's starting width `every Generate replaces it…` came out cut.
//!
//! No icon here either, and it is the same argument as [`super::summary`]: the
//! subject of the row is the WORD, and a drawing to the left of a word that is
//! itself the subject would be one indirection too many.

use super::super::row::ExplorerRow;
use crawrad_note::note;

#[note("explorer-rows-rows-legend-legend")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Legend {
    mark: &'static str,
    phrase: &'static str,
    warn: bool,
}

impl Legend {
    pub fn new(mark: &'static str, phrase: &'static str, warn: bool) -> Self {
        Self { mark, phrase, warn }
    }
}

impl ExplorerRow for Legend {
    fn template(&self) -> &'static str {
        if self.warn { "LegendWarn" } else { "Legend" }
    }

    fn text(&self) -> &str {
        self.mark
    }

    fn badge(&self) -> &str {
        self.phrase
    }

    fn indent(&self) -> usize {
        // Inside the section that contains it, like every section body row.
        super::CHILD
    }
}
