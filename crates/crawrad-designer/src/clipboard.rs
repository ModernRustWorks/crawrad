//! Copy, paste and duplicate components.
//!
//! The clipboard stores **the IR itself** — the entire nodes, with
//! properties and events. Pasting into a different form works, and what
//! is copied survives any editing in the source document, because it is a
//! copy of data, not a reference.

use crawrad_doc::{Command, Transaction};
use crawrad_ir::{Document, Node, NodeId, Rect};
use crawrad_note::note;

#[note("clipboard-clipboard")]
#[derive(Debug, Default, Clone)]
pub struct Clipboard {
    nodes: Vec<Node>,
}

#[note("clipboard-offset")]
const OFFSET: f32 = 8.0;

#[note("clipboard-mark")]
const MARK: &str = "crawrad.components";

impl Clipboard {
    #[note("clipboard-clipboard-to-text")]
    pub fn to_text(&self) -> String {
        serde_json::json!({ "kind": MARK, "nodes": self.nodes }).to_string()
    }

    #[note("clipboard-clipboard-from-text")]
    pub fn from_text(txt: &str) -> Option<Self> {
        let v: serde_json::Value = serde_json::from_str(txt).ok()?;
        if v.get("kind")?.as_str()? != MARK {
            return None;
        }
        let nodes: Vec<Node> = serde_json::from_value(v.get("nodes")?.clone()).ok()?;
        (!nodes.is_empty()).then_some(Self { nodes })
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[note("clipboard-clipboard-copy")]
    pub fn copy(&mut self, doc: &Document, sel: &[NodeId]) -> usize {
        self.nodes = sel
            .iter()
            .filter(|id| *id != &doc.root)
            .filter_map(|id| doc.node(id).cloned())
            .collect();
        self.nodes.len()
    }

    #[note("clipboard-clipboard-paste")]
    pub fn paste(
        &self,
        doc: &Document,
        new_id: impl Fn(&str) -> String,
    ) -> Option<(Transaction, Vec<NodeId>)> {
        if self.nodes.is_empty() {
            return None;
        }
        let root = doc.node(&doc.root)?;
        let first_index = root.children.len();

        // The copy enters at the end of the tab sequence: inheriting the original's
        // number would leave two components contending for the same position.
        let mut next = crate::arrangement::next_tab_order(doc);

        let mut tx = Transaction::user(if self.nodes.len() == 1 {
            "Paste".to_string()
        } else {
            format!("Paste {} components", self.nodes.len())
        });
        let mut created = Vec::new();

        for (index, template) in (first_index..).zip(self.nodes.iter()) {
            let mut n = template.clone();
            n.id = new_id(&template.class);
            if let Some(r) = n.rect() {
                n.set_rect(Rect::new(r.x + OFFSET, r.y + OFFSET, r.w, r.h));
            }
            if n.props.contains_key("tab_order") {
                n.props
                    .insert("tab_order".into(), crawrad_ir::Value::Int(next));
                next += 1;
            }
            // Event handlers do NOT come along: user code is attached
            // to the original component, and duplicating the binding would create two
            // components calling the same function without them asking for it.
            n.events.clear();

            created.push(n.id.clone());
            tx.push(Command::AddNode {
                parent: doc.root.clone(),
                node: n,
                index,
            });
        }
        Some((tx, created))
    }
}

#[note("clipboard-duplicate")]
pub fn duplicate(
    doc: &Document,
    sel: &[NodeId],
    new_id: impl Fn(&str) -> String,
) -> Option<(Transaction, Vec<NodeId>)> {
    let mut cb = Clipboard::default();
    if cb.copy(doc, sel) == 0 {
        return None;
    }
    let (mut tx, created) = cb.paste(doc, new_id)?;
    tx.label = if created.len() == 1 {
        "Duplicate".to_string()
    } else {
        format!("Duplicate {} components", created.len())
    };
    Some((tx, created))
}

#[cfg(test)]
#[path = "test/clipboard/test.rs"]
mod tests;
