//! What a row of the Explorer **is**, as a contract — and what clicking it
//! does.
//!
//! # Why a trait and not an `enum`
//!
//! This module used to be one `enum Row` with five `match`es over it:
//! `template`, `indent`, `text`, `badge`, and the dispatch of the click in the
//! shell. Adding a kind of row meant editing all five, in three files, and the
//! compiler only charged for three of them — the click dispatch ends in a `_
//! => {}` and the drawing ends in a `_ =>` too, so a new variant reached the
//! screen silently drawn as something else.
//!
//! With a trait, **a new kind of row is a new file** and nothing else moves.
//! What the row knows about itself travels with it: the template it asks the
//! `PortalList` for, how deep it sits, the icon, the text, the badge, the
//! sentence behind the badge and the catalogue command a click runs. The panel
//! assembles rows; it never asks what kind they are.
//!
//! # The one thing the trait does NOT let a row do
//!
//! A row cannot invent a template name. It can only name one, and the gate
//! `every_explorer_template_is_declared_in_the_dsl` reads the names off
//! [`ExplorerRow::template`] of the samples and charges them against the DSL —
//! because a template that does not exist makes `PortalList::item` hand back
//! an empty widget and the row draws **nothing**, with nothing failing
//! (`portal_list.rs:601` only warns in the log).

use std::fmt::Debug;
use std::path::PathBuf;

use crawrad_actions::Args;

use super::icon::RowIcon;
use crawrad_note::note;

#[note("explorer-rows-row-activation")]
#[derive(Debug, Clone, PartialEq)]
pub enum Activation {
    /// Run this catalogue action with these arguments.
    ///
    /// The shell does not know which action: it takes the pair and hands it to
    /// `execute_action`, the same door the menu, the keyboard and the protocol
    /// use.
    Run(&'static str, Args),
    /// Open this form in the designer.
    ///
    /// **A declared gap, and it predates this module.** There is no
    /// `file.open` in `crawrad_actions::catalogo()` — an agent cannot switch
    /// forms today, by any door. Turning the click into an action would mean
    /// adding an `Arquivo::Abrir` and an arm in both shells, which is a round
    /// of its own; what is NOT acceptable is leaving it looking like it is
    /// already a command. It is spelled out here instead.
    OpenForm(PathBuf),
    /// Nothing happens. Most rows are captions.
    Inert,
}

#[note("explorer-rows-row-explorerrow")]
pub trait ExplorerRow: Debug {
    #[note("explorer-rows-row-explorerrow-template")]
    fn template(&self) -> &'static str;

    #[note("explorer-rows-row-explorerrow-text")]
    fn text(&self) -> &str;

    #[note("explorer-rows-row-explorerrow-indent")]
    fn indent(&self) -> usize {
        0
    }

    #[note("explorer-rows-row-explorerrow-icon")]
    fn icon(&self) -> Option<RowIcon> {
        None
    }

    #[note("explorer-rows-row-explorerrow-chevron")]
    fn chevron(&self) -> Option<RowIcon> {
        None
    }

    #[note("explorer-rows-row-explorerrow-badge")]
    fn badge(&self) -> &str {
        ""
    }

    #[note("explorer-rows-row-explorerrow-activation")]
    fn activation(&self) -> Activation {
        Activation::Inert
    }

    #[note("explorer-rows-row-explorerrow-note")]
    fn note(&self) -> Option<String> {
        None
    }

    #[note("explorer-rows-row-explorerrow-item-target")]
    fn item_target(&self) -> Option<(PathBuf, bool)> {
        None
    }
}

#[note("explorer-rows-row-row")]
pub type Row = Box<dyn ExplorerRow>;
