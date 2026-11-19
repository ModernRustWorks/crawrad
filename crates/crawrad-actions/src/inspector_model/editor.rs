//! What an editor of the Object Inspector **is**, as a contract — and what a
//! gesture on it means.
//!
//! # Why a trait and not an `enum`
//!
//! This module used to be one `enum Editor` with three `match`es over it: the
//! one that chose it from the `PropKind`, the one that wrote it into the
//! widget, and the one that read the gesture back. Adding a kind of editor
//! meant editing all three, in two files, and only the first of them was
//! exhaustive — the other two end in a catch-all arm, so a new variant reached
//! the screen silently drawn as a text field and silently parsed as a string.
//!
//! With a trait, **a new editor is a new file** and nothing else moves. What
//! the editor knows about itself travels with it: the template it asks the
//! `PortalList` for, what goes in each of its slots, what a typed text or a
//! flipped box means, and the rows that appear under it when it is open. The
//! panel assembles rows; it never asks what kind they are.
//!
//! The one place that still has to hear about a new editor is
//! `super::editors::MAKERS`, and that is deliberate rather than forgotten —
//! the argument is written there, and it is the same one
//! `ide_app/explorer/rows.rs` makes about `sample_rows`.
//!
//! # The one thing the trait does NOT let an editor do
//!
//! An editor cannot invent a template name. It can only name one, and the DSL
//! gate reads the names off [`PropEditor::template`] of the samples and charges
//! them against the DSL — because a template that does not exist makes
//! `PortalList::item` hand back an empty widget and the row draws **nothing**,
//! with nothing failing (`portal_list.rs:601` only warns in the log).

use std::fmt::Debug;

use crate::Args;
use crawrad_ir::Value;
use crawrad_note::note;

#[note("inspector-model-editor-slot")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Slot {
    /// The single editor of a one-value row.
    Value,
    /// The short word at the right edge that says why a row does not edit.
    Note,
    /// The button that opens the window behind a property.
    Button,
    /// The patch of colour next to a hexadecimal value.
    Swatch,
    /// The chevron that unfolds the sub-rows of a property.
    Fold,
    /// The button that empties a property — the `×` of a picture.
    Clear,
}

#[note("inspector-model-editor-cell")]
#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    /// A one-line field the user types in.
    Field { slot: Slot, text: String },
    /// A check box.
    Check { slot: Slot, on: bool },
    /// A closed set of options, and which one is current.
    ///
    /// `Vec<String>` and not `&'static [&'static str]`, and the reason is the
    /// `Reference` editor: the options of `Images` are the `ImageList`
    /// components **of the open document**, so they cannot be static. Making
    /// the catalogue-fed editors carry owned strings too is the price of one
    /// shape instead of two.
    ///
    /// `current: None` is a real state and not a missing value: it is the
    /// document holding a value that is not in the set, and the drop-down must
    /// not slide to the first option and pretend.
    Choice {
        slot: Slot,
        options: Vec<String>,
        current: Option<usize>,
    },
    /// A word the row only shows.
    Caption { slot: Slot, text: String },
    /// A button, with what is written on it.
    Press { slot: Slot, text: String },
    /// A patch of colour, as `r g b a`.
    Swatch { slot: Slot, rgba: [u8; 4] },
}

impl Cell {
    #[note("inspector-model-editor-cell-slot")]
    pub fn slot(&self) -> Slot {
        match self {
            Self::Field { slot, .. }
            | Self::Check { slot, .. }
            | Self::Choice { slot, .. }
            | Self::Caption { slot, .. }
            | Self::Press { slot, .. }
            | Self::Swatch { slot, .. } => *slot,
        }
    }
}

#[note("inspector-model-editor-gesture")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture<'a> {
    /// The text in the field, as it stands now.
    Typed(&'a str),
    /// The check box was flipped to this.
    Toggled(bool),
    /// This option of the closed set was chosen.
    Chose(usize),
    /// The button was pressed.
    Pressed,
}

#[note("inspector-model-editor-outcome")]
#[derive(Debug)]
pub enum Outcome {
    /// Write this value into the property of the row.
    ///
    /// The shell turns it into `component.set_property` over the whole
    /// selection, which is what gives "editing with N selected undoes in the N
    /// at once" — the editor never builds a `Transaction`.
    Set(Value),
    /// Run this catalogue action with these arguments.
    ///
    /// The shell does not know which action: it takes the pair and hands it to
    /// `execute_action`, the same door the menu, the keyboard and the protocol
    /// use.
    Run(&'static str, Args),
    /// Say this sentence in the status bar, and change nothing.
    ///
    /// **Refusing in silence would be the dead field back**: the user types
    /// `abc` in `Position`, leaves the field, and nothing happens without a
    /// word. The row returns to the value of the document on the next frame.
    Refuse(String),
    /// Nothing happens. Most gestures on most slots are nothing.
    Ignore,
}

#[note("inspector-model-editor-propeditor")]
pub trait PropEditor: Debug {
    #[note("inspector-model-editor-propeditor-template")]
    fn template(&self) -> &'static str;

    #[note("inspector-model-editor-propeditor-cells")]
    fn cells(&self) -> Vec<Cell>;

    #[note("inspector-model-editor-propeditor-gesture")]
    fn gesture(&self, _slot: Slot, _gesture: Gesture<'_>) -> Outcome {
        Outcome::Ignore
    }

    #[note("inspector-model-editor-propeditor-children")]
    fn children(&self) -> Vec<(String, Editor)> {
        Vec::new()
    }

    #[note("inspector-model-editor-propeditor-window-text")]
    fn window_text(&self) -> Option<String> {
        None
    }

    #[note("inspector-model-editor-propeditor-window-value")]
    fn window_value(&self, _text: &str) -> Outcome {
        Outcome::Ignore
    }

    #[note("inspector-model-editor-propeditor-window-color")]
    fn window_color(&self) -> Option<crawrad_ir::Color> {
        None
    }

    #[note("inspector-model-editor-propeditor-window-items")]
    fn window_items(&self) -> Option<bool> {
        None
    }
}

#[note("inspector-model-editor-editor")]
pub type Editor = Box<dyn PropEditor>;
