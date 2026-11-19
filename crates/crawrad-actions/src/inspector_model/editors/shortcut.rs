//! Keyboard shortcut editor — `Shortcut`.
//!
//! An editable shortcut representation (e.g. `Ctrl+S`, `F5`, `None`).

use crawrad_ir::{Shortcut, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-shortcut-shortcuteditor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutEditor {
    shortcut: Shortcut,
}

impl ShortcutEditor {
    pub fn new(shortcut: Shortcut) -> Self {
        Self { shortcut }
    }
}

impl PropEditor for ShortcutEditor {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: self.shortcut.to_string(),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => {
                if let Some(sc) = Shortcut::parse(t) {
                    Outcome::Set(Value::Shortcut(sc))
                } else {
                    Outcome::Ignore
                }
            }
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Shortcut, Value::Shortcut(sc)) => Some(Box::new(ShortcutEditor::new(*sc))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(ShortcutEditor::new(Shortcut::ctrl(crawrad_ir::Key::S)))
}
