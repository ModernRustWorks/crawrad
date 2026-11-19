//! A SECTION HEADER — a chevron, a title, and everything under it folds away.
//!
//! # Why the legend became one
//!
//! The legend `What the badges mean` was eating a third of the panel, always,
//! and it is the one thing on this screen nobody arrives knowing:
//! `Rewritten` / `Yours` / `active` is a concept of THIS product, unlike the
//! `M`/`U` of git, which the reference does not explain precisely because
//! everyone already knows it.
//!
//! So it could not be deleted — in this house, *what you take away, you later
//! forget* — and it could not stay open either. Folded, closed by default, it
//! leaves the screen without leaving the product, which is what the reference
//! does with `Outline` and `Timeline`.
//!
//! # The click is an ACTION, and that is the whole point
//!
//! Folding a section is a capability of the IDE, so it goes through
//! `crawrad_actions` like every other one. The row carries the action id and the
//! arguments it was drawn with; the shell hands the pair to the same door the
//! menu, the keyboard and the protocol use. Nothing here reads state at click
//! time, which is the window in which the drawing and the running disagree.

use crawrad_actions::Args;

use super::super::icon::{CHEVRON_CLOSED, CHEVRON_OPEN, RowIcon};
use super::super::row::{Activation, ExplorerRow};
use crawrad_note::note;

#[note("explorer-rows-rows-section-section")]
#[derive(Debug, Clone, PartialEq)]
pub struct Section {
    title: String,
    open: bool,
    /// The catalogue action that folds it.
    action: &'static str,
}

impl Section {
    pub fn new(title: impl Into<String>, open: bool, action: &'static str) -> Self {
        Self {
            title: title.into(),
            open,
            action,
        }
    }
}

impl ExplorerRow for Section {
    #[note("explorer-rows-rows-section-section-template")]
    fn template(&self) -> &'static str {
        "Section"
    }

    fn text(&self) -> &str {
        &self.title
    }

    fn chevron(&self) -> Option<RowIcon> {
        Some(if self.open {
            CHEVRON_OPEN
        } else {
            CHEVRON_CLOSED
        })
    }

    fn activation(&self) -> Activation {
        Activation::Run(self.action, Args::none())
    }
}
