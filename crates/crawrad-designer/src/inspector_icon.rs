//! The drawings the Object Inspector puts on a row.
//!
//! # Why a table of its own, and why constants
//!
//! The same two arguments `ide_app/explorer/icon.rs` makes, and they hold here
//! for the same reasons. A `match` would pull the choice back into one place —
//! the editor would say *which kind it is* and this file would decide what it
//! looks like, which is the central switch `super::editors` exists to
//! remove. A constant is additive.
//!
//! And it is not `Chrome` (`ide_app/icon.rs`): that table answers *what does
//! this button DO?* and is compared character by character against the egui
//! shell's `icones.rs`, so a role added there means editing the other binary's
//! file. These answer *what shape is this row?*, and no other shell draws them.
//!
//! What keeps them honest is the same oracle as the Explorer's:
//! `every_inspector_icon_exists_in_codicons` reads the `cmap` of the font file
//! and charges every drawing. A code point the font does not have is an
//! **empty box** on screen, and nothing complains.

use crawrad_note::note;
#[note("inspector-icon-rowicon")]
pub type RowIcon = &'static str;

#[note("inspector-icon-fold-closed")]
pub const FOLD_CLOSED: RowIcon = "\u{eb70}";

#[note("inspector-icon-fold-open")]
pub const FOLD_OPEN: RowIcon = "\u{eb6e}";

#[note("inspector-icon-open-editor")]
pub const OPEN_EDITOR: RowIcon = "\u{ea7c}";

#[note("inspector-icon-clear")]
pub const CLEAR: RowIcon = "\u{ea76}";

#[note("inspector-icon-all")]
pub const ALL: &[RowIcon] = &[FOLD_CLOSED, FOLD_OPEN, OPEN_EDITOR, CLEAR];
