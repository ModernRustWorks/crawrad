//! A box that is ticked or is not — `Enabled`, `Visible`, `AlignWithMargins`.
//!
//! No label next to the box: the name of the row is at the left, and a second
//! word beside the tick would be the name written twice.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-check-check")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Check {
    on: bool,
}

impl Check {
    pub fn new(on: bool) -> Self {
        Self { on }
    }
}

impl PropEditor for Check {
    fn template(&self) -> &'static str {
        "RowCheck"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Check {
            slot: Slot::Value,
            on: self.on,
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Toggled(on)) => Outcome::Set(Value::Bool(on)),
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Bool, Value::Bool(b)) => Some(Box::new(Check::new(*b))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Check::new(true))
}
