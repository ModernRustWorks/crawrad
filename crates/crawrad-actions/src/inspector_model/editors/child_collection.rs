//! The child collection of a component — the `Columns` of a `Grid`, the
//! `Panels` of a `StatusBar`.
//!
//! Not a property: the items are CHILDREN in the document, declared by
//! `Children::Collection` in the catalogue. The row exists so the Object
//! Inspector leads to the collection dialog the way a classic RAD IDE does,
//! with `…`; double-clicking the component on the form opens the same one.

use crate::Args;
use crawrad_ir::NodeId;

use super::super::editor::{Cell, Gesture, Outcome, PropEditor, Slot};

/// The action the `…` runs.
pub const ACTION_OPEN_COLLECTION: &str = "inspector.open_collection";

#[derive(Debug, Clone, PartialEq)]
pub struct ChildCollection {
    node: NodeId,
    count: usize,
}

impl ChildCollection {
    pub fn new(node: NodeId, count: usize) -> Self {
        Self { node, count }
    }
}

impl PropEditor for ChildCollection {
    fn template(&self) -> &'static str {
        "RowOpen"
    }

    fn cells(&self) -> Vec<Cell> {
        super::collection::open_cells(format!("({})", self.count))
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                ACTION_OPEN_COLLECTION,
                Args::text("node", self.node.clone()),
            ),
            _ => Outcome::Ignore,
        }
    }
}
