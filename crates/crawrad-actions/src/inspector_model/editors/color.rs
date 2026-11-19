//! A colour — the patch you can see, the `#RRGGBB` you can type, and the `…` button opening the Color dialog.
//!
//! # Why the hexadecimal, and not an embedded colour wheel
//!
//! Because the wheel is a **window**, opened by the `…` button, and this row
//! is a cell in a grid 21 pixels tall.
//!
//! What the row must **not** do is what it did before this file existed: fall
//! through to the summary and print `color`, which is the name of the type and
//! not a value at all.
//!
//! # The patch is not decoration
//!
//! Hexadecimal is unreadable to a human eye — `#2B579A` and `#2B5A97` are the
//! same string at a glance. The patch is what makes the row answer *"which
//! colour is this?"* without being decoded, and it is why this editor has a
//! template of its own instead of borrowing [`super::text`]'s.

use crate::Args;
use crawrad_ir::{Color, NodeId, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-color-colorrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorRow {
    node: NodeId,
    key: &'static str,
    value: Color,
}

impl ColorRow {
    pub fn new(node: NodeId, key: &'static str, value: Color) -> Self {
        Self { node, key, value }
    }
}

impl PropEditor for ColorRow {
    fn template(&self) -> &'static str {
        "RowColor"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Swatch {
                slot: Slot::Swatch,
                rgba: [self.value.r, self.value.g, self.value.b, self.value.a],
            },
            // The same text the summary of a read-only row shows, so the two
            // ways of seeing a colour in this panel spell it identically.
            Cell::Field {
                slot: Slot::Value,
                text: Value::Color(self.value).describe(),
            },
            Cell::Press {
                slot: Slot::Button,
                text: icon::OPEN_EDITOR.to_string(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_OPEN_EDITOR,
                Args::text("node", self.node.clone()).and("key", Value::Str(self.key.to_string())),
            ),
            (Slot::Value, Gesture::Typed(text)) => {
                // `Color::from_hex` is the reader of `crawrad-ir` — the same one the
                // `.crawform` goes through — and it takes the `#` or leaves it.
                match Color::from_hex(text.trim()) {
                    Some(c) => Outcome::Set(Value::Color(c)),
                    None => Outcome::Refuse(format!(
                        "`{text}` is not a colour — write #RRGGBB or #RRGGBBAA"
                    )),
                }
            }
            _ => Outcome::Ignore,
        }
    }

    fn window_color(&self) -> Option<Color> {
        Some(self.value)
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Color, Value::Color(c)) => {
            Some(Box::new(ColorRow::new(s.node.clone(), s.desc.key, *c)))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(ColorRow::new(
        "sample".to_string(),
        "color",
        Color::rgb(0x2B, 0x57, 0x9A),
    ))
}
