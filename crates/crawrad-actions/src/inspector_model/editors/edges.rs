//! Four sides — `Margins`, and anything else the catalogue declares
//! `PropKind::Edges` over a `Value::Edges`.
//!
//! # It FOLDS, and that is the third shape of a row
//!
//! A row of the Object Inspector is not "label plus field". It has three
//! shapes, and this file is the third: a **summary that opens in place**. Shut,
//! it reads `3, 3, 3, 3`, which is what whoever only wants to check the number
//! needs; open, it is four rows, one per side, each a number that is typed
//! over. The classic RAD draws `Margins` exactly this way and so does the egui
//! shell (`crates/crawrad-ide/src/inspector.rs:791-810`).
//!
//! Folded state lives in the shell and is toggled through the catalogue
//! (`inspector.toggle_group`), which is why the chevron hands back an
//! [`Outcome::Run`] and not a value: an agent gets to unfold `Margins` with a
//! tool, without a mouse.
//!
//! # Every side writes the WHOLE record
//!
//! There is no partial-field command in `crawrad-doc`, and there should not be —
//! one batch is one `Ctrl+Z`, and `component.set_property` already carries the
//! selection. Each side therefore holds a copy of the four numbers and hands
//! back all four.
//!
//! # Whole numbers, and a floor of zero
//!
//! A margin is an `Integer` in a classic RAD IDE and the reference print shows `3`, so the
//! field shows `3` and not `3.00` — the IDE inventing precision the property
//! does not have. The floor is zero because a margin is slack: negative here is
//! the control climbing into its neighbour, and `alinhar_filhos` cuts at zero
//! anyway.

use crate::Args;
use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::decimal::Decimal;
use super::{Kind, Subject, note};

#[note("inspector-model-editors-edges-ceiling")]
const CEILING: f64 = 1000.0;

#[note("inspector-model-editors-edges-sides")]
const SIDES: [(&str, Side); 4] = [
    ("Left", Side::Left),
    ("Top", Side::Top),
    ("Right", Side::Right),
    ("Bottom", Side::Bottom),
];

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-edges-side")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Top,
    Right,
    Bottom,
}

#[note("inspector-model-editors-edges-edges")]
#[derive(Debug, Clone, PartialEq)]
pub struct Edges {
    key: &'static str,
    value: crawrad_ir::Edges,
    open: bool,
}

impl Edges {
    pub fn new(key: &'static str, value: crawrad_ir::Edges, open: bool) -> Self {
        Self { key, value, open }
    }
}

impl PropEditor for Edges {
    fn template(&self) -> &'static str {
        "RowFold"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Press {
                slot: Slot::Fold,
                text: if self.open {
                    icon::FOLD_OPEN.to_string()
                } else {
                    icon::FOLD_CLOSED.to_string()
                },
            },
            Cell::Caption {
                slot: Slot::Value,
                text: Value::Edges(self.value).describe(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Fold, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_TOGGLE_GROUP,
                Args::text("key", self.key.to_string()),
            ),
            _ => Outcome::Ignore,
        }
    }

    fn children(&self) -> Vec<(String, Editor)> {
        if !self.open {
            return Vec::new();
        }
        SIDES
            .into_iter()
            .map(|(label, side)| {
                (
                    label.to_string(),
                    Box::new(EdgeSide {
                        whole: self.value,
                        side,
                    }) as Editor,
                )
            })
            .collect()
    }
}

#[note("inspector-model-editors-edges-edgeside")]
#[derive(Debug, Clone, Copy, PartialEq)]
struct EdgeSide {
    whole: crawrad_ir::Edges,
    side: Side,
}

impl EdgeSide {
    fn current(&self) -> f32 {
        match self.side {
            Side::Left => self.whole.left,
            Side::Top => self.whole.top,
            Side::Right => self.whole.right,
            Side::Bottom => self.whole.bottom,
        }
    }
}

impl PropEditor for EdgeSide {
    fn template(&self) -> &'static str {
        "SubText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: Decimal::show(f64::from(self.current())),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        let (Slot::Value, Gesture::Typed(text)) = (slot, gesture) else {
            return Outcome::Ignore;
        };
        let parsed = match Decimal::parse(text, 0.0, CEILING) {
            Ok(v) => v as f32,
            Err(why) => return Outcome::Refuse(why),
        };
        let mut v = self.whole;
        match self.side {
            Side::Left => v.left = parsed,
            Side::Top => v.top = parsed,
            Side::Right => v.right = parsed,
            Side::Bottom => v.bottom = parsed,
        }
        Outcome::Set(Value::Edges(v))
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Edges, Value::Edges(e)) => Some(Box::new(Edges::new(s.desc.key, *e, s.open))),
        _ => None,
    }
}

#[note("inspector-model-editors-edges-sample")]
fn sample() -> Editor {
    Box::new(Edges::new("margins", crawrad_ir::Edges::uniform(3.0), true))
}
