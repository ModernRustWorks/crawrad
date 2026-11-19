//! A closed set — the drop-down of every `PropKind::Enum` that is not `Align`.
//!
//! The options come from the **descriptor**, never from here: this file does
//! not know one enumerated value by name, which is what makes a component new
//! in the catalogue draw its own enums without a line edited in the shell.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-choice-choice")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    options: Vec<String>,
    current: Option<usize>,
}

impl Choice {
    #[note("inspector-model-editors-choice-choice-new")]
    pub fn new<S: AsRef<str>>(options: &[S], value: &str) -> Self {
        Self {
            options: options.iter().map(|o| o.as_ref().to_string()).collect(),
            current: options.iter().position(|o| o.as_ref() == value),
        }
    }
}

impl PropEditor for Choice {
    fn template(&self) -> &'static str {
        "RowChoice"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Choice {
            slot: Slot::Value,
            options: self.options.clone(),
            current: self.current,
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Chose(i)) => match self.options.get(i) {
                Some(o) => Outcome::Set(Value::Enum(o.clone())),
                // An index the list does not have is the drop-down and the
                // list disagreeing about what was on screen, and writing the
                // first option instead would be a value the user never saw.
                None => Outcome::Ignore,
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Enum(options), Value::Enum(v)) => Some(Box::new(Choice::new(options, v))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Choice::new(&["Left", "Right"], "Left"))
}
