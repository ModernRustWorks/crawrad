//! Text with line breaks — the `Lines` of a `Memo`.
//!
//! # The row shows HOW MANY lines, never the lines
//!
//! It is a `Value::Str` like a `Caption`, so the summary of the panel would
//! hand back the whole `String` — which on a 21-pixel row is
//! `Line1□Line2□Line3` glued into one thread. The egui shell paid for that
//! once (`crates/crawrad-ide/src/inspector.rs:1010-1024`); the count is what
//! reads.
//!
//! # And the WHOLE thing is what the catalogue declares, not the name of the key
//!
//! The key of `Memo.Lines` is `text` — the same key as `Button`, `Label`,
//! `Edit`, `GroupBox` and `TabSheet`. A branch on `"text"` would give the pad
//! to all six, and the `Caption` of a button has no window behind it. What
//! separates them is a **fact declared in the catalogue**,
//! `crawrad_registry::PropDesc::multilinha`, and it is what this file asks. It
//! is the same fix `colecao_em_arvore` made for the Items Editor, and the
//! `class_dispatch_gate` fails the other one.

use crate::Args;
use crawrad_ir::{NodeId, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-lines-summary")]
pub fn summary(text: &str) -> String {
    let n = if text.is_empty() {
        0
    } else {
        text.split('\n').count()
    };
    match n {
        0 => "(empty)".to_string(),
        1 => "1 line".to_string(),
        n => format!("{n} lines"),
    }
}

#[note("inspector-model-editors-lines-lines")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lines {
    node: NodeId,
    key: &'static str,
    text: String,
}

impl Lines {
    pub fn new(node: NodeId, key: &'static str, text: impl Into<String>) -> Self {
        Self {
            node,
            key,
            text: text.into(),
        }
    }
}

impl PropEditor for Lines {
    fn template(&self) -> &'static str {
        "RowOpen"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            // The button comes BEFORE the summary: it is what gets clicked, and
            // hunting for a target that moves with the length of the text is
            // friction nobody notices and everybody tires of.
            Cell::Press {
                slot: Slot::Button,
                text: icon::OPEN_EDITOR.to_string(),
            },
            Cell::Caption {
                slot: Slot::Value,
                text: summary(&self.text),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_OPEN_EDITOR,
                Args::text("node", self.node.clone()).and("key", Value::Str(self.key.to_string())),
            ),
            _ => Outcome::Ignore,
        }
    }

    fn window_text(&self) -> Option<String> {
        Some(self.text.clone())
    }

    fn window_value(&self, text: &str) -> Outcome {
        // Straight through: the pad IS the value for this kind. Nothing is
        // trimmed — a trailing blank line in a `Memo` is a blank line the user
        // typed.
        Outcome::Set(Value::Str(text.to_string()))
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    let Value::Str(t) = s.value else { return None };
    let multiline = s.desc.multiline || matches!(s.desc.kind, PropKind::Multiline);
    multiline.then(|| Box::new(Lines::new(s.node.clone(), s.desc.key, t)) as Editor)
}

fn sample() -> Editor {
    Box::new(Lines::new("Memo1".to_string(), "text", "Line1\nLine2"))
}
