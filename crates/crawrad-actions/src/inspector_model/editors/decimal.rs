//! A floating-point number, with the range the catalogue declares.
//!
//! The clamp is a clamp of GESTURE, for the reason written in
//! [`super::integer`] — and `NaN` and the infinities are refused outright,
//! because a rectangle whose width is `inf` is a component that stops being
//! drawn and a document that no longer round-trips.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-decimal-decimal")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decimal {
    value: f64,
    min: f64,
    max: f64,
}

impl Decimal {
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        Self { value, min, max }
    }

    #[note("inspector-model-editors-decimal-decimal-parse")]
    pub fn parse(text: &str, min: f64, max: f64) -> Result<f64, String> {
        match text.trim().parse::<f64>() {
            Ok(v) if v.is_finite() => Ok(v.clamp(min, max)),
            _ => Err(format!("`{text}` is not a number")),
        }
    }

    #[note("inspector-model-editors-decimal-decimal-show")]
    pub fn show(value: f64) -> String {
        let text = format!("{value}");
        if text == "-0" { "0".to_string() } else { text }
    }
}

impl PropEditor for Decimal {
    fn template(&self) -> &'static str {
        "RowText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: Self::show(self.value),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Typed(t)) => match Self::parse(t, self.min, self.max) {
                Ok(v) => Outcome::Set(Value::Float(v)),
                Err(why) => Outcome::Refuse(why),
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Float { min, max, .. }, Value::Float(v)) => {
            Some(Box::new(Decimal::new(*v, *min, *max)))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Decimal::new(1.5, 0.0, 100.0))
}
