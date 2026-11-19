//! A collection of items — the `Items` of a `ListBox`, a `ComboBox`, a
//! `TreeView`.
//!
//! # The window behind it is a TEXT PAD, and that is adaptation and not a cut
//!
//! the classic RAD editor for a `TStrings` **is** a text pad — one item per line,
//! and that is the whole of it. What a classic RAD IDE gives a grid to is the `TTreeView`,
//! because a tree is not a list of lines; so this file writes the tree **as**
//! lines, with two spaces of indent per level, which is the shape every
//! outliner, every `yaml` and every `Markdown` list already uses and nobody has
//! to be taught.
//!
//! What that buys is one window for three property kinds instead of three
//! windows: [`super::lines`] opens the same pad with its own text. What it
//! costs is written out under [`from_text`] — an item whose own text begins
//! with spaces cannot be spelled.
//!
//! # The row shows HOW MANY, never the items
//!
//! `4 items`, and the count is of the whole tree including descendants
//! ([`crawrad_ir::Item::total`]) — the same number the egui shell's summary shows.
//!
//! # `hierarchical` comes from the CATALOGUE
//!
//! Not from the class of the component. Deciding with `node.class ==
//! "ComboBox"` is what sent the `ListBox` to the tree editor once, and the
//! `class_dispatch_gate` fails it now.

use crate::Args;
use crawrad_ir::{Item, NodeId, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::{Kind, Subject, note};

/// The `…` and the caption beside it: the cells of every `RowOpen` row.
pub(super) fn open_cells(caption: String) -> Vec<Cell> {
    vec![
        Cell::Press {
            slot: Slot::Button,
            text: icon::OPEN_EDITOR.to_string(),
        },
        Cell::Caption {
            slot: Slot::Value,
            text: caption,
        },
    ]
}

#[note("inspector-model-editors-collection-indent")]
const INDENT: usize = 2;

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-collection-to-text")]
pub fn to_text(items: &[Item]) -> String {
    fn walk(out: &mut Vec<String>, items: &[Item], depth: usize) {
        for item in items {
            out.push(format!("{}{}", " ".repeat(depth * INDENT), item.text));
            walk(out, &item.children, depth + 1);
        }
    }
    let mut out = Vec::new();
    walk(&mut out, items, 0);
    out.join("\n")
}

#[note("inspector-model-editors-collection-from-text")]
pub fn from_text(text: &str, hierarchical: bool) -> Vec<Item> {
    /// The list a line at this depth is appended to, found by walking the
    /// indices of the items that came before it.
    ///
    /// Indices and not `&mut` kept across the loop: the borrow checker cannot
    /// see that "the node I just descended into" and "its children" are the
    /// same lifetime narrowed twice, and a path of `usize` sidesteps the whole
    /// argument for the price of one short walk per line.
    fn level<'a>(roots: &'a mut Vec<Item>, path: &[usize]) -> &'a mut Vec<Item> {
        let mut here = roots;
        for &i in path {
            here = &mut here[i].children;
        }
        here
    }

    let mut roots: Vec<Item> = Vec::new();
    // The index of the last item written at each depth, deepest last.
    let mut path: Vec<usize> = Vec::new();
    for line in text.split('\n') {
        // `\r` because a pad on Windows can be pasted into from anywhere, and
        // an item whose text ends in a carriage return is an item whose label
        // draws a box.
        let trimmed = line.trim_end_matches('\r');
        let body = if hierarchical {
            trimmed.trim_start_matches(' ')
        } else {
            trimmed
        };
        if body.trim().is_empty() {
            continue;
        }
        // No deeper than one level below whatever is already there — see the
        // second rule above.
        let asked = (trimmed.len() - body.len()) / INDENT;
        let depth = asked.min(path.len());
        let here = level(&mut roots, &path[..depth]);
        here.push(Item::new(body));
        let index = here.len() - 1;
        path.truncate(depth);
        path.push(index);
    }
    roots
}

#[note("inspector-model-editors-collection-collection")]
#[derive(Debug, Clone, PartialEq)]
pub struct Collection {
    node: NodeId,
    key: &'static str,
    items: Vec<Item>,
    hierarchical: bool,
}

impl Collection {
    pub fn new(node: NodeId, key: &'static str, items: Vec<Item>, hierarchical: bool) -> Self {
        Self {
            node,
            key,
            items,
            hierarchical,
        }
    }
}

impl PropEditor for Collection {
    fn template(&self) -> &'static str {
        "RowOpen"
    }

    fn cells(&self) -> Vec<Cell> {
        open_cells(Item::list_to(&self.items).describe())
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

    #[note("inspector-model-editors-collection-collection-window-items")]
    fn window_items(&self) -> Option<bool> {
        Some(self.hierarchical)
    }

    #[note("inspector-model-editors-collection-collection-window-text")]
    fn window_text(&self) -> Option<String> {
        Some(to_text(&self.items))
    }

    fn window_value(&self, text: &str) -> Outcome {
        Outcome::Set(Item::list_to(&from_text(text, self.hierarchical)))
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Collection { hierarchical, .. }, Value::List(_)) => {
            Some(Box::new(Collection::new(
                s.node.clone(),
                s.desc.key,
                Item::list_from(s.value),
                *hierarchical,
            )))
        }
        _ => None,
    }
}

fn sample() -> Editor {
    Box::new(Collection::new(
        "TreeView1".to_string(),
        "items",
        vec![Item::with_children("Node1", vec![Item::new("SubNode1")])],
        true,
    ))
}
