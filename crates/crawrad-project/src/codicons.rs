//! Codicons drawings that more than one panel draws, defined once.
//!
//! Folding is not the Explorer's: the Object Inspector's categories fold too.
//! So the fold marks live here, where neither panel owns them, and both read
//! the same code point.

use crawrad_note::note;
#[note("codicons-chevron-closed")]
pub const CHEVRON_CLOSED: &str = "\u{eab6}";

#[note("codicons-chevron-open")]
pub const CHEVRON_OPEN: &str = "\u{eab4}";
