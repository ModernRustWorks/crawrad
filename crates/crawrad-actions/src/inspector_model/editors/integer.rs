//! A whole number, with the range the catalogue declares — `TabOrder`,
//! `ItemIndex`.
//!
//! # The clamp is a clamp of GESTURE
//!
//! A typed number outside the range goes in clamped, and **nothing rewrites
//! the document on its own**. It is the same ruler the `clamp_existing_to_range(false)`
//! bought in the egui shell (`crates/crawrad-ide/src/inspector.rs:645`), for the
//! same reason: a range is a limit on what can be *asked for*, not a claim
//! about what the document may contain. With the passive clamp on, merely
//! selecting a component with the `Layout` section open was enough to write
//! `w`/`h` back as 8.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-integer-integer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Integer {
    value: i64,
    min: i64,
    max: i64,
}

impl Integer {
    pub fn new(value: i64, min: i64, max: i64) -> Self {
        Self { value, min, max }
    }

    #[note("inspector-model-editors-integer-integer-parse")]
    pub fn parse(text: &str, min: i64, max: i64) -> Result<i64, String> {
        match text.trim().parse::<i64>() {
            Ok(v) => Ok(v.clamp(min, max)),
            Err(_) => Err(format!("`{text}` is not a whole number")),
        }
    }
}

impl PropEditor for Integer {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        // No invented decimals: `3`, never `3.00`. Whoever types over it does
        // not want to delete two places nobody asked for.
        vec![Cell::Field {
            slot: Slot::Value,
            text: self.value.to_string(),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => match Self::parse(t, self.min, self.max) {
                Ok(v) => Outcome::Set(Value::Int(v)),
                Err(why) => Outcome::Refuse(why),
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Int { min, max }, Value::Int(v)) => Some(Box::new(Integer::new(*v, *min, *max))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Integer::new(7, 0, 1000))
}
