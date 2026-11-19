//! # crawrad-doc — the live document
//!
//! **The project's single mutation gateway.** The UI, plugins, and agents
//! all enter through here; nothing modifies [`crawrad_ir::Document`] behind its back.
//!
//! This rule provides, all at once:
//!
//! - **generic undo** — every command knows how to invert itself;
//! - **batch = one transaction = one Ctrl+Z** — an agent that makes a mistake is undone with
//!   a shortcut, not with git;
//! - **auditing with author** — `user`, `plugin:x`, or `agent:y` on each change;
//! - **native `preview`** — [`Doc::preview`] answers what would change without changing,
//!   which was nearly impossible over a classic RAD IDE's extension API (§12.5 of the study).
//!
//! When RTA and MCP arrive (Phase 3), they are merely a protocol shell
//! on top of this crate.

// This crate has no `unsafe`, and the compiler now guarantees it. CrawRAD
// does not have A SINGLE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is as
// important as that of ADR 0002.
#![forbid(unsafe_code)]

pub mod command;

// The new gates are born in English (ADR 0030); the older neighbours keep
// their Portuguese names until maintenance touches them.
#[cfg(test)]
mod architecture_gate;

#[cfg(test)]
mod class_dispatch_gate;

#[cfg(test)]
mod target_command_gate;

#[cfg(test)]
mod silenced_lint_gate;

#[cfg(test)]
mod portao_ingles;

#[cfg(test)]
mod pt_identifier_gate;

#[cfg(test)]
mod duplication_gate;

#[cfg(test)]
mod test_layout_gate;

// The gate that keeps `Sdf2d` in reach of every shader that calls it.
#[cfg(test)]
mod shader_import_gate;

// The gate against writing a shader uniform at run time.
#[cfg(test)]
mod uniform_write_gate;

// The gate that keeps `unsafe` in one crate — ADR 0009, finally armed.
#[cfg(test)]
mod trademark_gate;

#[cfg(test)]
mod unsafe_gate;

// The runtime that ships inside user apps never depends on a GPL IDE crate.
#[cfg(test)]
mod runtime_license_gate;

#[cfg(test)]
mod tests;

pub use command::Command;

use crawrad_ir::{Document, IrError, NodeId};
use crawrad_note::note;
use std::collections::BTreeSet;

#[note("author")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Author {
    User,
    Plugin(String),
    Agent(String),
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Plugin(p) => write!(f, "plugin:{p}"),
            Self::Agent(a) => write!(f, "agent:{a}"),
        }
    }
}

#[note("transaction")]
#[derive(Debug, Clone)]
pub struct Transaction {
    pub label: String,
    pub author: Author,
    pub commands: Vec<Command>,
    /// Which continuous **gesture** it belongs to, if any.
    ///
    /// A drag is not one change: it is dozens, one per frame, because the IDE
    /// applies on every `PointerMove` so the drawing tracks the hand. Without this,
    /// each frame pushed an undo step — and `Ctrl+Z` undid ONE
    /// FRAME, a displacement you cannot even see. Measured: a short drag on
    /// screen took the document from `rev 0` to `rev 70`, and since `max_undo` is 100,
    /// **two drags evicted the user's entire history**, including the
    /// `AddNode` that created the component.
    ///
    /// The number is coined by whoever knows where the gesture begins and ends — the
    /// `Designer`, on `PointerDown`. Being monotonic is enough: comparison is only
    /// against the top of the stack.
    pub gesto: Option<u64>,
}

impl Transaction {
    pub fn new(label: impl Into<String>, author: Author) -> Self {
        Self {
            label: label.into(),
            author,
            commands: Vec::new(),
            gesto: None,
        }
    }

    #[note("transaction-no-gesto")]
    pub fn no_gesto(mut self, id: u64) -> Self {
        self.gesto = Some(id);
        self
    }

    pub fn user(label: impl Into<String>) -> Self {
        Self::new(label, Author::User)
    }

    pub fn with(mut self, c: Command) -> Self {
        self.commands.push(c);
        self
    }

    pub fn push(&mut self, c: Command) {
        self.commands.push(c);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

#[note("applied")]
#[derive(Debug, Clone, PartialEq)]
pub struct Applied {
    pub label: String,
    pub author: Author,
    pub touched: BTreeSet<NodeId>,
    /// Revision number of the document after the change.
    pub revision: u64,
}

#[note("auditentry")]
#[derive(Debug, Clone, PartialEq)]
pub struct AuditEntry {
    pub revision: u64,
    pub author: Author,
    pub label: String,
    pub touched: BTreeSet<NodeId>,
    /// `true` when the entry is an undo/redo, not a new edit.
    pub undo: bool,
}

#[note("doc")]
pub struct Doc {
    doc: Document,
    undo: Vec<(Transaction, String)>,
    redo: Vec<(Transaction, String)>,
    selection: Vec<NodeId>,
    revision: u64,
    audit: Vec<AuditEntry>,
    max_undo: usize,
    base: Option<Document>,
}

impl Doc {
    pub fn new(doc: Document) -> Self {
        Self {
            doc,
            undo: Vec::new(),
            redo: Vec::new(),
            selection: Vec::new(),
            revision: 0,
            audit: Vec::new(),
            max_undo: 100,
            base: None,
        }
    }

    #[note("doc-inheriting")]
    pub fn inheriting(own: &Document, base: Document) -> Result<Self, DocError> {
        let resolved = own.resolve_inheritance(&base)?;
        let mut doc = Self::new(resolved);
        doc.base = Some(base);
        Ok(doc)
    }

    pub fn ir(&self) -> &Document {
        &self.doc
    }

    /// The resolved base form this one inherits from, when it inherits.
    pub fn base(&self) -> Option<&Document> {
        self.base.as_ref()
    }

    /// Does this node come from the base form?
    pub fn is_inherited(&self, id: &str) -> bool {
        self.base
            .as_ref()
            .is_some_and(|b| self.doc.is_inherited(id, b))
    }

    /// The nodes that come from the base form, in id order.
    pub fn inherited_nodes(&self) -> Vec<NodeId> {
        self.doc
            .nodes
            .keys()
            .filter(|id| self.is_inherited(id))
            .cloned()
            .collect()
    }

    #[note("doc-own-ir")]
    pub fn own_ir(&self) -> std::borrow::Cow<'_, Document> {
        match &self.base {
            Some(b) => std::borrow::Cow::Owned(self.doc.own_part(b)),
            None => std::borrow::Cow::Borrowed(&self.doc),
        }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn audit(&self) -> &[AuditEntry] {
        &self.audit
    }

    // ---------------------------------------------------------- mutation

    #[note("doc-apply")]
    pub fn apply(&mut self, tx: Transaction) -> Result<Applied, DocError> {
        if tx.is_empty() {
            return Err(DocError::EmptyTransaction);
        }
        self.reject_non_snake_case_name(&tx)?;
        self.reject_inherited_structure(&tx)?;
        let touched: BTreeSet<NodeId> = tx.commands.iter().flat_map(|c| c.touches()).collect();

        let mut inverse = Vec::with_capacity(tx.commands.len());
        for cmd in tx.commands.iter().cloned() {
            match cmd.apply(&mut self.doc) {
                Ok(inv) => inverse.push(inv),
                Err(e) => {
                    // rollback what has already been applied, in reverse order
                    for inv in inverse.into_iter().rev() {
                        let _ = inv.apply(&mut self.doc);
                    }
                    return Err(e);
                }
            }
        }
        inverse.reverse();

        // Changed an action? Whatever points to it changed as well, even without any
        // command citing it: this is what makes the button follow the action's caption
        // without the IDE needing to know this rule.
        let mut touched = touched;
        for id in touched.clone() {
            touched.extend(self.doc.referrers(&id));
        }

        self.revision += 1;
        let applied = Applied {
            label: tx.label.clone(),
            author: tx.author.clone(),
            touched: touched.clone(),
            revision: self.revision,
        };
        self.audit.push(AuditEntry {
            revision: self.revision,
            author: tx.author.clone(),
            label: tx.label.clone(),
            touched,
            undo: false,
        });

        // Next frame of the SAME gesture: the undo step is already on the stack,
        // and what counts is the inverse of the FIRST frame — it is what restores the
        // component to where it was before the hand touched it. The inverses of
        // intermediate frames would lead back to the middle of the drag, which is nowhere:
        // the user did not stop there, they passed through there.
        //
        // Discarding instead of merging command-by-command is what makes this
        // cheap and correct at the same time. It holds because `SetRect` overwrites the
        // entire field; if a command that ACCUMULATES is ever added here, this
        // line needs to know.
        let mesmo_gesto = tx.gesto.is_some()
            && self
                .undo
                .last()
                .is_some_and(|(anterior, _)| anterior.gesto == tx.gesto);
        if !mesmo_gesto {
            let label = tx.label.clone();
            self.undo.push((
                Transaction {
                    label,
                    author: tx.author,
                    commands: inverse,
                    gesto: tx.gesto,
                },
                tx.label,
            ));
            if self.undo.len() > self.max_undo {
                self.undo.remove(0);
            }
        }
        self.redo.clear();
        Ok(applied)
    }

    #[note("doc-apply-one")]
    pub fn apply_one(&mut self, label: impl Into<String>, c: Command) -> Result<Applied, DocError> {
        self.apply(Transaction::user(label).with(c))
    }

    #[note("doc-reject-inherited-structure")]
    fn reject_inherited_structure(&self, tx: &Transaction) -> Result<(), DocError> {
        for cmd in &tx.commands {
            let node = match cmd {
                Command::RemoveNode { node }
                | Command::Rename { node, .. }
                | Command::Reparent { node, .. } => node,
                Command::Reorder { parent, from, .. } => {
                    match self.doc.node(parent).and_then(|p| p.children.get(*from)) {
                        Some(child) => child,
                        None => continue,
                    }
                }
                _ => continue,
            };
            if self.is_inherited(node) {
                let name = self.doc.node(node).map_or(node.as_str(), |n| n.name());
                return Err(DocError::InheritedNode(name.to_string()));
            }
        }
        Ok(())
    }

    #[note("doc-reject-non-snake-case-name")]
    fn reject_non_snake_case_name(&self, tx: &Transaction) -> Result<(), DocError> {
        for cmd in &tx.commands {
            if let Command::Rename { node, name, .. } = cmd {
                // Renaming to what it already is enters no new name:
                // `Command::Rename` treats this as a no-op, and rejecting here
                // would make the IDE complain about a gesture that changes nothing.
                if self.doc.node(node).is_some_and(|n| n.name() == name) {
                    continue;
                }
                // An EMPTY name and `___` have their own rejection, and it says more —
                // "has no name" and "use at least one letter or digit". What
                // writes them is `Command::Rename`, right ahead; overriding them
                // here with "is not snake_case" would replace a precise sentence with
                // a generic one, which is the opposite of what a rejection needs.
                if name.trim().is_empty() || !crawrad_ir::name_has_alphanumeric(name) {
                    continue;
                }
                if !crawrad_ir::is_snake_case_name(name) {
                    return Err(DocError::Ir(IrError::not_snake_case(name.clone())));
                }
            }
        }
        Ok(())
    }

    #[note("doc-preview")]
    pub fn preview(&self, tx: &Transaction) -> Result<Preview, DocError> {
        let mut copy = self.doc.clone();
        for cmd in tx.commands.iter().cloned() {
            cmd.apply(&mut copy)?;
        }
        copy.validate()?;
        Ok(Preview {
            touched: tx.commands.iter().flat_map(|c| c.touches()).collect(),
            result: copy,
        })
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    #[note("doc-undo-label")]
    pub fn undo_label(&self) -> Option<&str> {
        self.undo.last().map(|(_, l)| l.as_str())
    }

    pub fn redo_label(&self) -> Option<&str> {
        self.redo.last().map(|(_, l)| l.as_str())
    }

    pub fn undo(&mut self) -> Result<Applied, DocError> {
        let (tx, label) = self.undo.pop().ok_or(DocError::NothingToUndo)?;
        self.replay(tx, label, Stack::Redo)
    }

    pub fn redo(&mut self) -> Result<Applied, DocError> {
        let (tx, label) = self.redo.pop().ok_or(DocError::NothingToRedo)?;
        self.replay(tx, label, Stack::Undo)
    }

    #[note("doc-replay")]
    fn replay(
        &mut self,
        tx: Transaction,
        label: String,
        push_to: Stack,
    ) -> Result<Applied, DocError> {
        let touched: BTreeSet<NodeId> = tx.commands.iter().flat_map(|c| c.touches()).collect();

        let mut inverse = Vec::with_capacity(tx.commands.len());
        for cmd in tx.commands.iter().cloned() {
            inverse.push(cmd.apply(&mut self.doc)?);
        }
        inverse.reverse();

        // Changed an action? Whatever points to it changed as well, even without any
        // command citing it: this is what makes the button follow the action's caption
        // without the IDE needing to know this rule.
        let mut touched = touched;
        for id in touched.clone() {
            touched.extend(self.doc.referrers(&id));
        }

        self.revision += 1;
        let back = Transaction {
            label: label.clone(),
            author: tx.author.clone(),
            commands: inverse,
            // History replay is NOT a gesture: the step is already coalesced, and
            // tagging it would cause the next undo to merge into it. A `Ctrl+Z`
            // is always a single step.
            gesto: None,
        };
        match push_to {
            Stack::Redo => self.redo.push((back, label.clone())),
            Stack::Undo => self.undo.push((back, label.clone())),
        }

        self.audit.push(AuditEntry {
            revision: self.revision,
            author: tx.author.clone(),
            label: label.clone(),
            touched: touched.clone(),
            undo: true,
        });

        Ok(Applied {
            label,
            author: tx.author,
            touched,
            revision: self.revision,
        })
    }

    // ---------------------------------------------------------- selection

    pub fn selection(&self) -> &[NodeId] {
        &self.selection
    }

    pub fn select(&mut self, ids: impl IntoIterator<Item = NodeId>) {
        self.selection = ids
            .into_iter()
            .filter(|i| self.doc.node(i).is_some())
            .collect();
    }

    pub fn select_one(&mut self, id: impl Into<NodeId>) {
        self.select(std::iter::once(id.into()));
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    pub fn toggle(&mut self, id: impl Into<NodeId>) {
        let id = id.into();
        if let Some(p) = self.selection.iter().position(|s| s == &id) {
            self.selection.remove(p);
        } else if self.doc.node(&id).is_some() {
            self.selection.push(id);
        }
    }
}

#[note("stack")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stack {
    Undo,
    Redo,
}

#[derive(Debug, Clone)]
pub struct Preview {
    pub touched: BTreeSet<NodeId>,
    pub result: Document,
}

#[note("docerror")]
#[derive(Debug, thiserror::Error)]
pub enum DocError {
    #[error("node '{0}' does not exist")]
    NoNode(String),

    #[error("the root cannot be removed")]
    CannotRemoveRoot,

    #[error("'{0}' comes from the base form, so remove, rename or move it there")]
    InheritedNode(String),

    #[error(
        "'{0}' has children: remove the descendants first (or use a transaction with one RemoveNode per node)"
    )]
    RemoveWithChildren(String),

    #[error("invalid index in reorder: {from} -> {to}")]
    BadIndex { from: usize, to: usize },

    #[error("empty transaction")]
    EmptyTransaction,

    #[error("nothing to undo")]
    NothingToUndo,

    #[error("nothing to redo")]
    NothingToRedo,

    #[error(transparent)]
    Ir(#[from] IrError),
}
