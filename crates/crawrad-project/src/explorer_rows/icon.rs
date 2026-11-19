//! The drawings the Explorer puts at the left of a row.
//!
//! # Why they are constants and not a `match`
//!
//! Each row kind is its own file (see [`super::row`]), and a `match` here
//! would pull the choice back into one place — the row would say *which
//! variant it is* and this file would decide what it looks like, which is
//! exactly the central switch the decomposition removed. A constant is
//! additive: a row that needs a drawing nobody used yet adds one line here and
//! references it, and no existing line moves.
//!
//! # Why Codicons, and why a table of our own
//!
//! Codicons is the family the whole IDE already draws — the rail, the bar, the
//! panel headers — and the reason is written out in `ide_app/icon.rs`: it maps
//! **zero** code points below `U+0100`, so it can sit in front of a text face
//! in one font family, which Font Awesome cannot.
//!
//! This is a *second* table in the same crate, and that is deliberate.
//! `ide_app/icon.rs` holds `Chrome`, which answers *what does this button DO?*
//! and is compared character by character against the egui shell's
//! `icones.rs` — a role added there means editing the other binary's file.
//! These answer a third question, *what KIND of thing is this row about?*,
//! they are drawn by no other shell, and putting them in `Chrome` would make
//! the divergence test of two shells charge a set only one of them has.
//!
//! What keeps them honest is the same oracle:
//! `every_explorer_icon_exists_in_codicons` reads the `cmap` of the font file
//! and charges every drawing a sample row asks for. A code point the font does
//! not have is an **empty box** on screen, and nothing complains.
//!
//! # The code points go through RUST, never through the DSL
//!
//! The `live_design!` tokenizer knows three escapes — `\\`, `\"` and `\n`
//! (`makepad-live-tokenizer-1.0.0/src/tokenizer.rs:530-550`) — so a
//! `"\u{ea83}"` written in the DSL shows up spelled out on screen. Here it is
//! Rust, `\u{}` means what it says, and the shell writes the text into the
//! label of every row it draws.

use crawrad_note::note;
#[note("explorer-rows-icon-rowicon")]
pub type RowIcon = &'static str;

#[note("explorer-rows-icon-chevron-closed")]
pub const CHEVRON_CLOSED: RowIcon = crate::codicons::CHEVRON_CLOSED;

#[note("explorer-rows-icon-chevron-open")]
pub const CHEVRON_OPEN: RowIcon = crate::codicons::CHEVRON_OPEN;

#[note("explorer-rows-icon-group-root")]
pub const GROUP_ROOT: RowIcon = "\u{eb46}";

#[note("explorer-rows-icon-member")]
pub const MEMBER: RowIcon = "\u{ea83}";

#[note("explorer-rows-icon-member-open")]
pub const MEMBER_OPEN: RowIcon = "\u{eaf7}";

#[note("explorer-rows-icon-form")]
pub const FORM: RowIcon = "\u{eb7f}";

#[note("explorer-rows-icon-source")]
pub const SOURCE: RowIcon = "\u{eae9}";

#[note("explorer-rows-icon-manifest")]
pub const MANIFEST: RowIcon = "\u{eb29}";

#[note("explorer-rows-icon-pending")]
pub const PENDING: RowIcon = "\u{ea7f}";

#[note("explorer-rows-icon-warning")]
pub const WARNING: RowIcon = "\u{ea6c}";

#[note("explorer-rows-icon-error")]
pub const ERROR: RowIcon = "\u{ea87}";

#[note("explorer-rows-icon-unsaved")]
pub const UNSAVED: RowIcon = "\u{ea71}";

#[note("explorer-rows-icon-note")]
pub const NOTE: RowIcon = "\u{ea74}";
