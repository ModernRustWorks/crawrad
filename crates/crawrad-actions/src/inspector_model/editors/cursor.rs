//! Mouse cursor editor — `Cursor`.
//!
//! A drop-down of system and RAD mouse cursors.

use crawrad_ir::{Cursor, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

pub const CURSOR_OPTIONS: &[&str] = &[
    "default",
    "arrow",
    "hand",
    "text",
    "crosshair",
    "move",
    "wait",
    "not_allowed",
    "resize_ns",
    "resize_ew",
    "resize_nesw",
    "resize_nwse",
];

#[note("inspector-model-editors-cursor-cursoreditor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorEditor {
    options: Vec<String>,
    current: Option<usize>,
}

impl CursorEditor {
    pub fn new(cursor: Cursor) -> Self {
        let name = cursor.as_str();
        Self {
            options: CURSOR_OPTIONS.iter().map(|&s| s.to_string()).collect(),
            current: CURSOR_OPTIONS.iter().position(|&o| o == name),
        }
    }
}

impl PropEditor for CursorEditor {
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
                Some(o) => {
                    if let Some(c) = Cursor::from_str_name(o) {
                        Outcome::Set(Value::Cursor(c))
                    } else {
                        Outcome::Ignore
                    }
                }
                None => Outcome::Ignore,
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Cursor, Value::Cursor(c)) => Some(Box::new(CursorEditor::new(*c))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(CursorEditor::new(Cursor::Hand))
}
