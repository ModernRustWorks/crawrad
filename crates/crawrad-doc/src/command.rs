//! Commands: the only way to mutate the document.
//!
//! Every command is **invertible** — applying returns the command that undoes it. That is
//! where generic undo comes from, without requiring every property editor to know
//! what undo is (§6.4 of the study).

use crawrad_ir::{Document, IrError, Node, NodeId, Rect, Value};

use crate::DocError;
use crawrad_note::note;

#[note("command-command")]
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Sets (or removes, with `value: None`) a property.
    SetProp {
        node: NodeId,
        key: String,
        value: Option<Value>,
    },
    /// Shortcut of `SetProp` for geometry — the designer's most used path.
    SetRect { node: NodeId, rect: Rect },
    /// Inserts a node as a child of `parent`, at `index`.
    AddNode {
        parent: NodeId,
        node: Node,
        index: usize,
    },
    /// Removes a node and its descendants.
    RemoveNode { node: NodeId },
    /// Changes the position of the node among siblings (Z-order and tab order baseline).
    Reorder {
        parent: NodeId,
        from: usize,
        to: usize,
    },
    /// Binds (or unbinds, with `handler: None`) an event to a handler in code.
    SetEvent {
        node: NodeId,
        event: String,
        handler: Option<String>,
    },
    /// Renames the node — changes the id everywhere referencing it.
    /// Changes the `Name` of a node. **Does not touch identity** — `id` has been stable
    /// since ADR 0008, so renaming no longer needs to reconnect referrers,
    /// and became what it always should have been: a property.
    Rename {
        node: NodeId,
        name: String,
        handlers: bool,
    },
    /// Moves a node to another parent, **with the new position included**.
    ///
    /// The `rect` is relative to the NEW parent, which is what makes this a single
    /// command. In a classic RAD IDE `SetParent` does not touch `Left`/`Top` — the control
    /// teleports, and every programmer has written `C.Parent := P; C.Left := x`
    /// by hand. Here there cannot exist, even for an instant, a component in the
    /// new parent with the old parent's coordinates
    /// (ADR 0012).
    Reparent {
        node: NodeId,
        parent: NodeId,
        index: usize,
        rect: Rect,
    },
    /// Sets (or removes, with `binding: None`) a property data binding (ADR 0041).
    SetBinding {
        node: NodeId,
        prop: String,
        binding: Option<crawrad_ir::BindingDef>,
    },
}

impl Command {
    #[note("command-command-touches")]
    pub fn touches(&self) -> Vec<NodeId> {
        match self {
            Self::SetProp { node, .. }
            | Self::SetRect { node, .. }
            | Self::RemoveNode { node }
            | Self::SetEvent { node, .. }
            | Self::SetBinding { node, .. } => vec![node.clone()],
            Self::AddNode { parent, node, .. } => vec![parent.clone(), node.id.clone()],
            Self::Reorder { parent, .. } => vec![parent.clone()],
            Self::Reparent { node, parent, .. } => vec![parent.clone(), node.clone()],
            Self::Rename { node, .. } => vec![node.clone()],
        }
    }

    #[note("command-command-label")]
    pub fn label(&self) -> String {
        match self {
            Self::SetProp { key, node, .. } => format!("{node}.{key}"),
            Self::SetRect { node, .. } => format!("move/resize {node}"),
            Self::AddNode { node, .. } => format!("add {}", node.class),
            Self::RemoveNode { node } => format!("remove {node}"),
            Self::Reorder { parent, .. } => format!("reorder in {parent}"),
            Self::Reparent { node, parent, .. } => {
                format!("move {node} into {parent}")
            }
            Self::SetEvent { node, event, .. } => format!("{node}.{event}"),
            Self::SetBinding { node, prop, .. } => format!("bind {node}.{prop}"),
            Self::Rename { node, name, .. } => format!("rename {node} to {name}"),
        }
    }

    #[note("command-command-apply")]
    pub(crate) fn apply(self, doc: &mut Document) -> Result<Command, DocError> {
        match self {
            Self::SetProp { node, key, value } => {
                // validates the contract before touching the document
                if value.is_some() {
                    crawrad_ir::PropKey::parse(&key).map_err(|e| {
                        DocError::Ir(IrError::Contract {
                            node: node.clone(),
                            source: e,
                        })
                    })?;
                }
                let n = doc
                    .node_mut(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                let old = match value {
                    Some(v) => n.props.insert(key.clone(), v),
                    None => n.props.remove(&key),
                };
                Ok(Self::SetProp {
                    node,
                    key,
                    value: old,
                })
            }

            Self::SetRect { node, rect } => {
                let n = doc
                    .node_mut(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                let old = n.rect();
                n.set_rect(rect);
                match old {
                    Some(r) => Ok(Self::SetRect { node, rect: r }),
                    // had no rect: the inverse is deleting the property
                    None => Ok(Self::SetProp {
                        node,
                        key: "rect".into(),
                        value: None,
                    }),
                }
            }

            Self::AddNode {
                parent,
                node,
                index,
            } => {
                if doc.node(&node.id).is_some() {
                    return Err(DocError::Ir(IrError::DuplicateId(node.id)));
                }
                let id = node.id.clone();
                let subtree: Vec<Node> = std::iter::once(node).collect();
                for n in subtree {
                    doc.nodes.insert(n.id.clone(), n);
                }
                let p = doc
                    .node_mut(&parent)
                    .ok_or_else(|| DocError::NoNode(parent.clone()))?;
                let at = index.min(p.children.len());
                p.children.insert(at, id.clone());
                Ok(Self::RemoveNode { node: id })
            }

            Self::RemoveNode { node } => {
                if node == doc.root {
                    return Err(DocError::CannotRemoveRoot);
                }
                let (parent, index) = doc
                    .nodes
                    .iter()
                    .find_map(|(pid, p)| {
                        p.children
                            .iter()
                            .position(|c| c == &node)
                            .map(|i| (pid.clone(), i))
                    })
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;

                let removed = doc
                    .node(&node)
                    .cloned()
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;

                // for now the designer only removes leaves or entire subtrees; the
                // inverse stores the node, and descendants return via the commands
                // that created them (the transaction is undone in reverse order).
                if !removed.children.is_empty() {
                    return Err(DocError::RemoveWithChildren(node));
                }
                doc.nodes.remove(&node);
                if let Some(p) = doc.node_mut(&parent) {
                    p.children.retain(|c| c != &node);
                }
                Ok(Self::AddNode {
                    parent,
                    node: removed,
                    index,
                })
            }

            Self::Reparent {
                node,
                parent,
                index,
                rect,
            } => {
                // The old parent, the old position, and the old rectangle — that is what
                // the inverse needs to restore everything all at once.
                let (velho_pai, velho_indice) = doc
                    .parent_of(&node)
                    .cloned()
                    .and_then(|p| {
                        doc.node(&p)?
                            .children
                            .iter()
                            .position(|c| c == &node)
                            .map(|i| (p, i))
                    })
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                let velho_rect = doc
                    .node(&node)
                    .and_then(|n| n.rect())
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;

                // A node cannot become a child of its own descendants: the tree
                // would cease to be a tree, and `validate` would only notice later.
                if parent == node || doc.ancestors(&parent).contains(&node) {
                    return Err(DocError::NoNode(parent));
                }

                if let Some(p) = doc.node_mut(&velho_pai) {
                    p.children.retain(|c| c != &node);
                }
                let destino = doc
                    .node_mut(&parent)
                    .ok_or_else(|| DocError::NoNode(parent.clone()))?;
                let i = index.min(destino.children.len());
                destino.children.insert(i, node.clone());
                if let Some(n) = doc.node_mut(&node) {
                    n.set_rect(rect);
                }
                Ok(Self::Reparent {
                    node,
                    parent: velho_pai,
                    index: velho_indice,
                    rect: velho_rect,
                })
            }

            Self::Reorder { parent, from, to } => {
                let p = doc
                    .node_mut(&parent)
                    .ok_or_else(|| DocError::NoNode(parent.clone()))?;
                if from >= p.children.len() || to >= p.children.len() {
                    return Err(DocError::BadIndex { from, to });
                }
                let id = p.children.remove(from);
                p.children.insert(to, id);
                Ok(Self::Reorder {
                    parent,
                    from: to,
                    to: from,
                })
            }

            Self::SetEvent {
                node,
                event,
                handler,
            } => {
                let n = doc
                    .node_mut(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                let old = match handler {
                    Some(h) => n.events.insert(event.clone(), h),
                    None => n.events.remove(&event),
                };
                Ok(Self::SetEvent {
                    node,
                    event,
                    handler: old,
                })
            }

            Self::Rename {
                node,
                name,
                handlers,
            } => {
                let anterior = doc
                    .node(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?
                    .name()
                    .to_string();
                if anterior == name {
                    return Ok(Self::Rename {
                        node,
                        name,
                        handlers,
                    });
                }
                if name.trim().is_empty() {
                    return Err(DocError::Ir(IrError::EmptyName(node)));
                }
                // `"___".trim()` is **not** empty, and therefore bypassed the guard
                // above — the generator then derived an empty type and the rename
                // migrator deleted the word `FormMain` from inside the handwritten
                // file. User code loss, not an ugly file.
                //
                // The rule belongs to `crawrad-ir` (`nome_tem_alfanumerico`), not to
                // `crawrad-codegen`: the document does not know about the generator, and doesn't
                // need to. Rust-specific cases — `1abc`, `fn`, `meu botão` — are
                // already rejected by `crawrad-codegen::identificador`, and rejected
                // inside `gerar()`, **before** any file is touched. Only names without
                // alphanumerics reached disk.
                if !crawrad_ir::name_has_alphanumeric(&name) {
                    return Err(DocError::Ir(IrError::NameWithoutAlnum(name)));
                }
                // `snake_case` is NOT enforced here, and the reason is recorded in
                // `Doc::apply` — what passes through here includes UNDO,
                // which restores a name that was already in the document.
                //
                // Unique among NAMES, not among ids: these are distinct
                // concepts since ADR 0008.
                if doc.nodes.values().any(|n| n.id != node && n.name() == name) {
                    return Err(DocError::Ir(IrError::DuplicateName(name)));
                }
                let n = doc
                    .node_mut(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                n.set_name(&name);
                if handlers {
                    for (event, handler) in &mut n.events {
                        if *handler == crawrad_ir::default_handler_name(&anterior, event) {
                            *handler = crawrad_ir::default_handler_name(&name, event);
                        } else if *handler == crawrad_ir::legacy_handler_name(&anterior, event) {
                            *handler = crawrad_ir::legacy_handler_name(&name, event);
                        }
                    }
                }
                Ok(Self::Rename {
                    node,
                    name: anterior,
                    handlers,
                })
            }

            Self::SetBinding {
                node,
                prop,
                binding,
            } => {
                let n = doc
                    .node_mut(&node)
                    .ok_or_else(|| DocError::NoNode(node.clone()))?;
                let old = match binding {
                    Some(b) => n.set_binding(&prop, b),
                    None => n.remove_binding(&prop),
                };
                Ok(Self::SetBinding {
                    node,
                    prop,
                    binding: old,
                })
            }
        }
    }
}

#[cfg(test)]
#[path = "test/command/test.rs"]
mod tests;
