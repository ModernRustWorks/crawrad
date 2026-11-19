//! `Align` — and it opens a **drawing of the form**, not a list of names.
//!
//! # Why this one property gets a window of its own
//!
//! The owner sent the classic RAD print and said *"with egui it was already like
//! this"*. It is true: `crates/crawrad-ide/src/align_picker.rs` has done it for
//! weeks, and its own doc writes the argument out —
//!
//! > *"Instead of a list with the names, a drawing of the form you click a
//! > region of: the top band is `Top`, the middle is `Client`, and so on. It is
//! > faster to use and does not ask you to remember the names."*
//!
//! `Align` is a `PropKind::Enum` like `Alignment` or `BorderStyle`, so nothing
//! about the type says it deserves this. What says it is the **meaning**: the
//! five values are five places on a rectangle, and a picture of a rectangle
//! answers "which place?" faster than five words can.
//!
//! This is the one editor chosen by the **key** of the property and not by its
//! kind, and that is a declared exception rather than an oversight. The
//! alternative is a flag in the catalogue (`PropDesc::visual_picker`) whose
//! only reader would be this file, for its only user — a fact in the data whose
//! whole meaning lives in the interface, which is the boundary ADR 0005 draws.
//! If a second property ever wants the picker, the flag is the right answer
//! then and this comment is the note that says so.
//!
//! # Not to be confused with its neighbour
//!
//! `crates/crawrad-ide/src/paleta_alinhamento.rs` aligns components **with each
//! other**. That is another front entirely; this is the property of one
//! component.

use crate::Args;
use crawrad_ir::{NodeId, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

#[note("inspector-model-editors-align-key")]
const KEY: &str = "align";

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-align-align")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Align {
    node: NodeId,
    current: String,
}

impl Align {
    pub fn new(node: NodeId, current: impl Into<String>) -> Self {
        Self {
            node,
            current: current.into(),
        }
    }
}

impl PropEditor for Align {
    fn template(&self) -> &'static str {
        "RowPick"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            // The button carries the VALUE, not an ellipsis: the reference
            // print writes `Align [None]`, and a row whose only text is `…`
            // would hide the one thing worth reading at a glance.
            Cell::Press {
                slot: Slot::Button,
                text: self.current.clone(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_OPEN_EDITOR,
                Args::text("node", self.node.clone()).and("key", Value::Str(KEY.to_string())),
            ),
            _ => Outcome::Ignore,
        }
    }

    // No `window_text`: what opens here is not a pad. The shell reads the key
    // and draws the regions, and what comes back is one of the five names,
    // which goes home through `component.set_property` like everything else.
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Enum(_), Value::Enum(v)) if s.desc.key == KEY => {
            Some(Box::new(Align::new(s.node.clone(), v)))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Align::new("Button1".to_string(), "None"))
}
