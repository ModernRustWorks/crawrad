//! The `When` of a `StyleRule` — text, with its verdict beside it.
//!
//! Green check when the expression is right; red box and the reason under it
//! when it is not (`crate::style_rule_check`). The row only shows the verdict:
//! what is written is written, exactly as the text row does.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject};

pub const KIND: Kind = Kind { make, sample };

/// The `When` row: the text typed and what is wrong with it, if anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct When {
    text: String,
    verdict: Result<(), String>,
}

impl When {
    pub fn new(text: impl Into<String>, verdict: Result<(), String>) -> Self {
        Self {
            text: text.into(),
            verdict,
        }
    }
}

impl PropEditor for When {
    fn template(&self) -> &'static str {
        match self.verdict {
            Ok(()) => "RowWhenOk",
            Err(_) => "RowWhenBad",
        }
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Field {
                slot: Slot::Value,
                text: self.text.clone(),
            },
            Cell::Caption {
                slot: Slot::Note,
                text: match &self.verdict {
                    Ok(()) => "\u{2713}".to_string(),
                    Err(reason) => reason.clone(),
                },
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
    let is_when = s.desc.key == "when"
        && matches!(s.desc.kind, PropKind::String)
        && s.doc
            .node(s.node)
            .is_some_and(|n| n.class == crate::style_rule_check::STYLE_RULE_CLASS);
    match s.value {
        Value::Str(t) if is_when => Some(Box::new(When::new(
            t,
            crate::style_rule_check::when_check(s.doc, s.node),
        ))),
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(When::new(
        "balance <",
        Err("expected a value after \"<\" (at 9)".to_string()),
    ))
}
