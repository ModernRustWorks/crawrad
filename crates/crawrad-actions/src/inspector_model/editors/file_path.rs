//! File path editor — `FilePath`.
//!
//! An editable path string with file existence awareness.

use crawrad_ir::{FilePath, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-file-path-filepatheditor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePathEditor {
    path: String,
}

impl FilePathEditor {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl PropEditor for FilePathEditor {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: self.path.clone(),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => Outcome::Set(Value::FilePath(FilePath::new(t))),
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::FilePath, Value::FilePath(fp)) => {
            Some(Box::new(FilePathEditor::new(fp.as_str())))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(FilePathEditor::new("src/main.rs"))
}
