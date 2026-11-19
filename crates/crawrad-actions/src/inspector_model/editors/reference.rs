//! A pointer at another component of the same form — `Images`, `Action`.
//!
//! # The list comes out of the DOCUMENT, and it is filtered by class
//!
//! Only the components of the class the property accepts are offered: pointing
//! a button at an `ImageList` is not an option that should exist, and a
//! drop-down that offered it would be the IDE letting the user build a form
//! the code generator cannot emit. Which class it is comes from the catalogue
//! ([`crawrad_registry::PropKind::Reference`]), never from a name written here.
//!
//! # What the row SHOWS and what it WRITES are different strings
//!
//! It shows the **name** and writes the **id** (ADR 0008: the id is an
//! address, not a label, and it never reaches the interface). The egui shell
//! shows the id (`crates/crawrad-ide/src/inspector.rs:727-740`) because it
//! predates the ADR; here the two travel side by side, which costs one extra
//! `Vec` and buys the rename that does not break the pointer.

use crawrad_ir::Value;
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::{Kind, Subject, note};

#[note("inspector-model-editors-reference-none")]
pub const NONE: &str = crawrad_ir::value::NO_REFERENCE;

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-reference-reference")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// What the drop-down shows — the names, with `(none)` first.
    labels: Vec<String>,
    /// What each row of it writes — the ids, with the empty one first.
    ids: Vec<String>,
    current: Option<usize>,
}

impl Reference {
    #[note("inspector-model-editors-reference-reference-new")]
    pub fn new(pairs: Vec<(String, String)>, value: &str) -> Self {
        // `(none)` is ALWAYS the first row, and it is the reason this editor
        // cannot be a plain `Choice`: unbinding a reference is a value of the
        // set, not the absence of one, and a drop-down with no way back would
        // make `Images` a one-way door.
        let mut labels = vec![NONE.to_string()];
        let mut ids = vec![String::new()];
        for (id, name) in pairs {
            labels.push(name);
            ids.push(id);
        }
        let current = ids.iter().position(|i| i == value);
        Self {
            labels,
            ids,
            current,
        }
    }
}

impl PropEditor for Reference {
    fn template(&self) -> &'static str {
        "RowChoice"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Choice {
            slot: Slot::Value,
            options: self.labels.clone(),
            current: self.current,
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Chose(i)) => match self.ids.get(i) {
                Some(id) => Outcome::Set(Value::Ref(id.clone())),
                None => Outcome::Ignore,
            },
            _ => Outcome::Ignore,
        }
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    let Value::Ref(value) = s.value else {
        return None;
    };
    // The class comes from the kind first and from `ref_class` second, which
    // is the same order the egui shell reads them in: the two say the same
    // thing, and the descriptor builder fills both.
    let wanted = match &s.desc.kind {
        PropKind::Reference(c) => Some(*c),
        _ => s.desc.ref_class,
    }?;
    // **The name is a CAPABILITY, and a class answers to its own name.**
    //
    // `Button.action` asks for `"Action"` and the `Action` class answers, which
    // is what this line used to compare directly. What it could not do is the
    // other case: five provider classes declaring `provides = ["DbProvider"]`, and
    // one abstraction asking for `"DbProvider"` without knowing any of them.
    //
    // Listing the five inside the abstraction would work until the sixth, which
    // would have to edit the abstraction to exist -- the opposite of what an
    // abstraction is for.
    let pairs = s
        .doc
        .nodes
        .values()
        .filter(|n| {
            s.registry
                .get(&n.class)
                .is_some_and(|d| d.answers_to(wanted))
        })
        .map(|n| (n.id.clone(), n.name().to_string()))
        .collect();
    Some(Box::new(Reference::new(pairs, value)))
}

fn sample() -> Editor {
    Box::new(Reference::new(
        vec![("ImageList1".into(), "ImageList1".into())],
        "",
    ))
}
