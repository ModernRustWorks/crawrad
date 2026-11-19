//! Modal result editor — `ModalResult`.
//!
//! A drop-down of standard modal dialog and form result codes.

use crawrad_ir::{ModalResult, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

pub const MODAL_RESULT_OPTIONS: &[&str] = &[
    "none", "ok", "cancel", "abort", "retry", "ignore", "yes", "no", "close",
];

#[note("inspector-model-editors-modal-result-modalresulteditor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModalResultEditor {
    options: Vec<String>,
    current: Option<usize>,
}

impl ModalResultEditor {
    pub fn new(modal_result: ModalResult) -> Self {
        let name = modal_result.as_str();
        Self {
            options: MODAL_RESULT_OPTIONS
                .iter()
                .map(|&s| s.to_string())
                .collect(),
            current: MODAL_RESULT_OPTIONS.iter().position(|&o| o == name),
        }
    }
}

impl PropEditor for ModalResultEditor {
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
                    if let Some(mr) = ModalResult::from_str_name(o) {
                        Outcome::Set(Value::ModalResult(mr))
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
        (PropKind::ModalResult, Value::ModalResult(mr)) => {
            Some(Box::new(ModalResultEditor::new(*mr)))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(ModalResultEditor::new(ModalResult::Ok))
}
