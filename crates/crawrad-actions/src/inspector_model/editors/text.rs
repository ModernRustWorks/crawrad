//! Text of ONE line — `Name`, `Caption`, `Hint`.
//!
//! One line, always, and that is a decision rather than a limit of the row.
//! The field that grows with the content lived in the egui shell for one PR,
//! behind a `linhas_do_campo(p)` that handed back 3 to whoever declared
//! `multilinha`; thirty lines typed into it pushed `Layout`, `Behavior` and
//! `Events` out of sight. Whoever has several lines does not reach this file:
//! [`super::lines`] takes the row first and puts the growth in a window, which
//! is where growth fits.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-text-text")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl PropEditor for Text {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: self.text.clone(),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            // Nothing is rejected and nothing is trimmed: a `Caption` of two
            // spaces is a caption of two spaces, and a Hint that ends in a
            // space is the user's business. The one key that refuses anything
            // is `name`, and whoever refuses it is `component.rename` in the
            // catalogue — never this row (ADR 0008).
            (Slot::Value, Gesture::Typed(t)) => Outcome::Set(Value::Str(t.to_string())),
            _ => Outcome::Ignore,
        }
    }
}

/// A field typed as free text because its column list could not be read,
/// with the reason under it — never a text box that stays silent.
#[note("inspector-model-editors-text-unlisted")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unlisted {
    text: String,
    reason: String,
}

impl Unlisted {
    pub fn new(text: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            reason: reason.into(),
        }
    }
}

impl PropEditor for Unlisted {
    // The `When` row's shape: the box, and the reason on the line below.
    fn template(&self) -> &'static str {
        "RowWhenBad"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Field {
                slot: Slot::Value,
                text: self.text.clone(),
            },
            Cell::Caption {
                slot: Slot::Note,
                text: self.reason.clone(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => Outcome::Set(Value::Str(t.to_string())),
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    // The multiline flag of the catalogue is asked by `super::lines`, which
    // comes later in `KINDS` — but this file has to let it through, because a
    // `Memo.Lines` is a `PropKind::String` carrying `multilinha` and it must
    // not be caught here first.
    if s.desc.multiline {
        return None;
    }
    match (&s.desc.kind, s.value) {
        // A `FieldOf` whose dataset columns are not known yet is typed as text.
        (PropKind::String | PropKind::FieldOf(_), Value::Str(t)) => Some(Box::new(Text::new(t))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Text::new("Button1"))
}
