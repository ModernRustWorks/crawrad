//! Date and time editor — `DateTime`.
//!
//! A formatted text field editing ISO-8601 timestamps `YYYY-MM-DD HH:MM:SS`.

use crawrad_ir::{DateTime, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-date-time-datetimeeditor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateTimeEditor {
    value: DateTime,
}

impl DateTimeEditor {
    pub fn new(value: DateTime) -> Self {
        Self { value }
    }
}

impl PropEditor for DateTimeEditor {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: self.value.to_iso8601(),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => match DateTime::parse(t) {
                Ok(dt) => Outcome::Set(Value::DateTime(dt)),
                Err(_) => Outcome::Ignore,
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::DateTime, Value::DateTime(dt)) => Some(Box::new(DateTimeEditor::new(*dt))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(DateTimeEditor::new(DateTime::new(2026, 9, 19, 12, 0, 0)))
}
