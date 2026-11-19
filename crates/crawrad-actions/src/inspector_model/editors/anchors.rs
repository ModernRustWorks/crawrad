//! `Anchors` — four sides that are on or off, folded like [`super::edges`].
//!
//! # Why it is a file of its own, and not a branch of `edges`
//!
//! Because the catalogue declares it as `PropKind::Edges` and it holds a
//! `Value::Map` (`crawrad-registry/src/props.rs`, `Props::anchors`): **the same
//! kind over two different shapes**. Four check boxes are not four numbers, and
//! a branch inside the numbers file would be the central `match` sneaking back
//! in under another name. It comes before `edges` in `super::KINDS`, and that
//! order is the whole dispatch.
//!
//! # The four names are the classic RAD ones
//!
//! `akLeft`, `akTop`, `akRight`, `akBottom` — and the summary between brackets
//! (`[akLeft,akTop]`) is the reference print, character for character. The
//! **keys of the map** are `left`/`top`/`right`/`bottom` and they are what
//! `crawrad-designer` reads; the `ak` prefix never reaches the document.

use std::collections::BTreeMap;

use crate::Args;
use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::{Kind, Subject, note};

#[note("inspector-model-editors-anchors-sides")]
const SIDES: [(&str, &str); 4] = [
    ("left", "akLeft"),
    ("top", "akTop"),
    ("right", "akRight"),
    ("bottom", "akBottom"),
];

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-anchors-anchors")]
#[derive(Debug, Clone, PartialEq)]
pub struct Anchors {
    key: &'static str,
    map: BTreeMap<String, Value>,
    open: bool,
}

impl Anchors {
    pub fn new(key: &'static str, map: BTreeMap<String, Value>, open: bool) -> Self {
        Self { key, map, open }
    }
}

#[note("inspector-model-editors-anchors-on")]
fn on(map: &BTreeMap<String, Value>, key: &str) -> bool {
    matches!(map.get(key), Some(Value::Bool(true)))
}

impl PropEditor for Anchors {
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
            // The summary is built HERE and not by `describe`, and the
            // difference is the `ak` prefix: the panel's generic summary of a
            // map prints the keys of the document (`[left,top]`), and the
            // reference print writes the classic RAD names (`[akLeft,akTop]`). The
            // keys are what `crawrad-designer` reads; the names are what the user
            // reads, and the two never meet.
            Cell::Caption {
                slot: Slot::Value,
                text: format!(
                    "[{}]",
                    SIDES
                        .into_iter()
                        .filter(|(key, _)| on(&self.map, key))
                        .map(|(_, label)| label)
                        .collect::<Vec<_>>()
                        .join(",")
                ),
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
            .map(|(key, label)| {
                (
                    label.to_string(),
                    Box::new(AnchorSide {
                        whole: self.map.clone(),
                        side: key,
                    }) as Editor,
                )
            })
            .collect()
    }
}

#[note("inspector-model-editors-anchors-anchorside")]
#[derive(Debug, Clone, PartialEq)]
struct AnchorSide {
    whole: BTreeMap<String, Value>,
    side: &'static str,
}

impl PropEditor for AnchorSide {
    fn template(&self) -> &'static str {
        "SubCheck"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Check {
            slot: Slot::Value,
            on: on(&self.whole, self.side),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Toggled(v)) => {
                // The whole map goes back, for the reason `edges` writes out:
                // there is no partial-field command, and one batch is one
                // `Ctrl+Z`.
                let mut novo = self.whole.clone();
                novo.insert(self.side.to_string(), Value::Bool(v));
                Outcome::Set(Value::Map(novo))
            }
            _ => Outcome::Ignore,
        }
    }
}

#[note("inspector-model-editors-anchors-make")]
fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Set(_), Value::Map(m)) => {
            Some(Box::new(Anchors::new(s.desc.key, m.clone(), s.open)))
        }
        _ => None,
    }
}

#[note("inspector-model-editors-anchors-sample")]
fn sample() -> Editor {
    let map = SIDES
        .into_iter()
        .map(|(k, _)| (k.to_string(), Value::Bool(k == "left" || k == "top")))
        .collect();
    Box::new(Anchors::new("anchors", map, true))
}
