//! # crawrad-ir — the CrawRAD document
//!
//! This crate is the **`.dfm` of CrawRAD**: the component tree of a
//! form, with typed properties and event bindings, in pure data.
//!
//! Three rules that have held since the first line, because none of them can be
//! retrofitted (see the design notes):
//!
//! 1. **Nothing here knows about runtimes.** There is no mention of Win32, egui or any
//!    toolkit. Whoever consumes the IR is the one that knows how to paint.
//! 2. **Every property obeys the contract** in [`contract`]: either it is in the
//!    common core, or it is namespaced by target. `validate()` rejects the rest.
//! 3. **The format is versioned** since v1, with a clear error for future versions.

// This crate has no `unsafe`, and the compiler now guarantees it. CrawRAD
// does not have ONE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and that boundary is just as
// important as the one in ADR 0002.
#![forbid(unsafe_code)]

/// **THE DRAWING VOCABULARY** — how a component paints itself, in data.
///
/// Lives here, and not in the catalog, because the adapter that PAINTS intentionally
/// does not depend on the catalog (ADR 0027: widget translates gesture, designer decides).
/// `crawrad-ir` is what both sides already have — it is the same reason `Value`
/// lives here.
pub mod drawing;
pub mod icon;

pub mod binding;
pub mod char_case;
pub mod contract;
pub mod date_time;
pub mod design_rows;
pub mod grid_columns;
pub mod handler_name;
pub mod item;
pub mod mask;
pub mod scrolling;
pub mod style_rule;
pub mod translation_key;
pub mod value;

pub use binding::{BindingDef, BindingMode};
pub use contract::{ContractError, PropKey};
pub use handler_name::{
    default_handler_name, is_derived_handler_name, is_handler_ident, legacy_handler_name,
};
pub use item::Item;
pub use translation_key::{TRANSLATABLE, text_key};
pub use value::{
    Color, Cursor, DateTime, Edges, FilePath, Font, ImageFormat, Key, ModalResult, Picture, Rect,
    Shortcut, Value,
};

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[note("format-version")]
pub const FORMAT_VERSION: u32 = 2;

#[note("form-retired-keys")]
pub const FORM_RETIRED_KEYS: &[&str] = &[
    "enabled",
    "visible",
    "anchors",
    "align",
    "margins",
    "align_with_margins",
    "tab_order",
    "tab_stop",
];

#[note("nodeid")]
pub type NodeId = String;

#[note("node")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub class: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub props: BTreeMap<String, Value>,
    /// event -> handler name in user code (`on_click` -> `btn_okClick`)
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub events: BTreeMap<String, String>,
    /// Order matters: it is the stacking order and the basis of tab order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeId>,
    /// Declarative property bindings to data sources (ADR 0041).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bindings: BTreeMap<String, BindingDef>,
}

impl Node {
    pub fn new(id: impl Into<NodeId>, class: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            class: class.into(),
            props: BTreeMap::new(),
            events: BTreeMap::new(),
            children: Vec::new(),
            bindings: BTreeMap::new(),
        }
    }

    pub fn with_prop(mut self, key: &str, v: Value) -> Self {
        self.props.insert(key.to_string(), v);
        self
    }

    pub fn with_binding(mut self, prop: &str, binding: BindingDef) -> Self {
        self.bindings.insert(prop.to_string(), binding);
        self
    }

    pub fn binding(&self, prop: &str) -> Option<&BindingDef> {
        self.bindings.get(prop)
    }

    pub fn set_binding(&mut self, prop: &str, binding: BindingDef) -> Option<BindingDef> {
        self.bindings.insert(prop.to_string(), binding)
    }

    pub fn remove_binding(&mut self, prop: &str) -> Option<BindingDef> {
        self.bindings.remove(prop)
    }

    pub fn rect(&self) -> Option<Rect> {
        self.props.get("rect")?.as_rect()
    }

    pub fn set_rect(&mut self, r: Rect) {
        self.props.insert("rect".into(), Value::Rect(r));
    }

    #[note("node-name")]
    pub fn name(&self) -> &str {
        match self.props.get("name") {
            Some(Value::Str(s)) if !s.is_empty() => s,
            _ => &self.id,
        }
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.props.insert("name".into(), Value::Str(name.into()));
    }

    pub fn text(&self) -> Option<&str> {
        self.props.get("text")?.as_str()
    }

    pub fn set_text(&mut self, s: impl Into<String>) {
        self.props.insert("text".into(), Value::Str(s.into()));
    }

    #[note("node-props-for")]
    pub fn props_for<'a>(&'a self, target: &'a str) -> impl Iterator<Item = (PropKey, &'a Value)> {
        self.props.iter().filter_map(move |(k, v)| {
            let key = PropKey::parse(k).ok()?;
            key.applies_to(target).then_some((key, v))
        })
    }
}

#[note("document")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub format: u32,
    pub root: NodeId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<String>,
    pub nodes: BTreeMap<NodeId, Node>,
}

impl Document {
    #[note("document-new")]
    pub fn new(root_id: impl Into<NodeId>, root_class: impl Into<String>) -> Self {
        let root = root_id.into();
        let mut nodes = BTreeMap::new();
        let mut root_node = Node::new(root.clone(), root_class);
        root_node.set_name(root.clone());
        nodes.insert(root.clone(), root_node);
        Self {
            format: FORMAT_VERSION,
            root,
            inherits: None,
            nodes,
        }
    }

    #[note("document-new-inherited")]
    pub fn new_inherited(
        root_id: impl Into<NodeId>,
        root_class: impl Into<String>,
        base_form: impl Into<String>,
    ) -> Self {
        let mut doc = Self::new(root_id, root_class);
        doc.inherits = Some(base_form.into());
        doc
    }

    #[note("document-inherits")]
    pub fn inherits(&self) -> Option<&str> {
        self.inherits.as_deref()
    }

    #[note("document-set-inherits")]
    pub fn set_inherits(&mut self, base: Option<impl Into<String>>) {
        self.inherits = base.map(Into::into);
    }

    #[note("document-is-inherited")]
    pub fn is_inherited(&self, node_id: &str, base: &Document) -> bool {
        node_id != self.root && base.nodes.contains_key(node_id)
    }

    #[note("document-resolve-inheritance")]
    pub fn resolve_inheritance(&self, base: &Document) -> Result<Document, IrError> {
        if let Some(base_name) = &self.inherits {
            if base_name == &self.root {
                return Err(IrError::SelfInheritance(self.root.clone()));
            }
            if let Some(parent_base) = &base.inherits
                && parent_base == &self.root
            {
                return Err(IrError::InheritanceCycle {
                    parent: base.root.clone(),
                });
            }
        }

        let mut resolved = base.clone();
        resolved.inherits = self.inherits.clone();

        // 1. Replaces the root with the derived root, inheriting base children and merging props
        if let Some(derived_root) = self.nodes.get(&self.root) {
            let base_root = resolved
                .nodes
                .remove(&base.root)
                .unwrap_or_else(|| Node::new(base.root.clone(), "Form"));

            let mut merged_root = Node::new(self.root.clone(), derived_root.class.clone());
            merged_root.props = base_root.props;
            merged_root.props.extend(derived_root.props.clone());
            merged_root.events = base_root.events;
            merged_root.events.extend(derived_root.events.clone());
            merged_root.bindings = base_root.bindings;
            merged_root.bindings.extend(derived_root.bindings.clone());

            let mut children = base_root.children;
            for c in &derived_root.children {
                if !children.contains(c) {
                    children.push(c.clone());
                }
            }
            merged_root.children = children;
            resolved.nodes.insert(self.root.clone(), merged_root);
            resolved.root = self.root.clone();
        }

        // 2. Merges non-root nodes from the derived document
        for (id, derived_node) in &self.nodes {
            if id == &self.root {
                continue;
            }
            if let Some(existing) = resolved.nodes.get_mut(id) {
                existing.props.extend(derived_node.props.clone());
                existing.events.extend(derived_node.events.clone());
                existing.bindings.extend(derived_node.bindings.clone());
                for c in &derived_node.children {
                    if !existing.children.contains(c) {
                        existing.children.push(c.clone());
                    }
                }
            } else {
                resolved.nodes.insert(id.clone(), derived_node.clone());
            }
        }

        resolved.validate()?;
        Ok(resolved)
    }

    #[note("document-resolve-chain")]
    pub fn resolve_chain(bases: &[Document]) -> Option<Document> {
        let (top, below) = bases.split_last()?;
        let mut resolved = top.clone();
        for base in below.iter().rev() {
            resolved = base
                .resolve_inheritance(&resolved)
                .unwrap_or_else(|_| base.clone());
        }
        Some(resolved)
    }

    #[note("document-own-part")]
    pub fn own_part(&self, base: &Document) -> Document {
        let differs = |id: &str, n: &Node| -> Node {
            let b = base.nodes.get(id);
            let mut out = Node::new(n.id.clone(), n.class.clone());
            for (k, v) in &n.props {
                if b.and_then(|b| b.props.get(k)) != Some(v) {
                    out.props.insert(k.clone(), v.clone());
                }
            }
            for (k, v) in &n.events {
                if b.and_then(|b| b.events.get(k)) != Some(v) {
                    out.events.insert(k.clone(), v.clone());
                }
            }
            for (k, v) in &n.bindings {
                if b.and_then(|b| b.bindings.get(k)) != Some(v) {
                    out.bindings.insert(k.clone(), v.clone());
                }
            }
            out
        };
        let inherited = |id: &str| self.is_inherited(id, base);
        let mut kept: BTreeMap<NodeId, Node> = BTreeMap::new();
        for (id, n) in &self.nodes {
            let entry = if inherited(id) {
                differs(id, n)
            } else if id == &self.root {
                differs(&base.root, n)
            } else {
                n.clone()
            };
            let changed =
                !entry.props.is_empty() || !entry.events.is_empty() || !entry.bindings.is_empty();
            if !inherited(id) || changed {
                kept.insert(id.clone(), entry);
            }
        }
        // a kept node may point at an inherited one: it has to be in the file too
        let mut pending: Vec<NodeId> = kept
            .values()
            .flat_map(|n| n.props.values())
            .filter_map(|v| match v {
                Value::Ref(t) if !t.is_empty() => Some(t.clone()),
                _ => None,
            })
            .chain(kept.keys().cloned())
            .collect();
        // and every kept node hangs from the root, through its inherited parents
        while let Some(id) = pending.pop() {
            if !kept.contains_key(&id)
                && let Some(n) = self.nodes.get(&id)
            {
                kept.insert(id.clone(), Node::new(n.id.clone(), n.class.clone()));
            }
            if let Some(parent) = self.parent_of(&id)
                && !kept.contains_key(parent)
            {
                pending.push(parent.clone());
            }
        }
        for (id, n) in &self.nodes {
            let own_children: Vec<NodeId> = n
                .children
                .iter()
                .filter(|c| kept.contains_key(*c))
                .cloned()
                .collect();
            if let Some(entry) = kept.get_mut(id) {
                entry.children = own_children;
            }
        }
        Document {
            format: self.format,
            root: self.root.clone(),
            inherits: self.inherits.clone(),
            nodes: kept,
        }
    }

    #[note("document-retire-form-keys")]
    fn retire_form_keys(&mut self) {
        let root = self.root.clone();
        if let Some(n) = self.nodes.get_mut(&root) {
            for key in FORM_RETIRED_KEYS {
                n.props.remove(*key);
            }
        }
    }

    pub fn from_json(s: &str) -> Result<Self, IrError> {
        let mut doc: Self = serde_json::from_str(s)?;
        if doc.format > FORMAT_VERSION {
            return Err(IrError::FutureFormat {
                found: doc.format,
                supported: FORMAT_VERSION,
            });
        }
        doc.migrate();
        doc.retire_form_keys();
        doc.validate()?;
        Ok(doc)
    }

    #[note("document-migrate")]
    fn migrate(&mut self) {
        if self.format >= FORMAT_VERSION {
            return;
        }
        for (id, n) in self.nodes.iter_mut() {
            if !matches!(n.props.get("name"), Some(Value::Str(s)) if !s.is_empty()) {
                n.props.insert("name".into(), Value::Str(id.clone()));
            }
        }
        self.format = FORMAT_VERSION;
    }

    pub fn to_json(&self) -> Result<String, IrError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, IrError> {
        let p = path.as_ref();
        let txt = std::fs::read_to_string(p).map_err(|e| IrError::Io {
            path: p.display().to_string(),
            source: e,
        })?;
        Self::from_json(&txt)
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<(), IrError> {
        let p = path.as_ref();
        std::fs::write(p, self.to_json()?).map_err(|e| IrError::Io {
            path: p.display().to_string(),
            source: e,
        })
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    #[note("document-node-by-name")]
    pub fn node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.values().find(|n| n.name() == name)
    }

    #[note("document-add-child")]
    pub fn add_child(&mut self, parent: &str, node: Node) -> Result<(), IrError> {
        if self.nodes.contains_key(&node.id) {
            return Err(IrError::DuplicateId(node.id));
        }
        if !self.nodes.contains_key(parent) {
            return Err(IrError::MissingNode(parent.to_string()));
        }
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        self.nodes
            .get_mut(parent)
            .expect("verified above")
            .children
            .push(id);
        Ok(())
    }

    #[note("document-remove-subtree")]
    pub fn remove_subtree(&mut self, id: &str) -> Vec<NodeId> {
        if id == self.root || !self.nodes.contains_key(id) {
            return Vec::new();
        }
        let mut doomed = Vec::new();
        let mut stack = vec![id.to_string()];
        while let Some(cur) = stack.pop() {
            if let Some(n) = self.nodes.get(&cur) {
                stack.extend(n.children.iter().cloned());
            }
            doomed.push(cur);
        }
        for d in &doomed {
            self.nodes.remove(d);
        }
        for n in self.nodes.values_mut() {
            n.children.retain(|c| !doomed.contains(c));
        }
        doomed
    }

    #[note("document-walk")]
    pub fn walk(&self) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut stack = vec![self.root.clone()];
        while let Some(id) = stack.pop() {
            if let Some(n) = self.nodes.get(&id) {
                out.push(n);
                for c in n.children.iter().rev() {
                    stack.push(c.clone());
                }
            }
        }
        out
    }

    #[note("document-subtree")]
    pub fn subtree(&self, id: &str) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut queue = vec![id.to_string()];
        while let Some(current) = queue.pop() {
            let Some(n) = self.node(&current) else {
                continue;
            };
            out.push(current.clone());
            queue.extend(n.children.iter().cloned());
        }
        out
    }

    #[note("document-referrers")]
    pub fn referrers(&self, target: &str) -> Vec<NodeId> {
        self.nodes
            .values()
            .filter(|n| {
                n.props
                    .values()
                    .any(|v| matches!(v, Value::Ref(r) if r == target))
            })
            .map(|n| n.id.clone())
            .collect()
    }

    #[note("document-parent-of")]
    pub fn parent_of(&self, id: &str) -> Option<&NodeId> {
        if id == self.root {
            return None;
        }
        self.nodes
            .iter()
            .find(|(_, n)| n.children.iter().any(|c| c == id))
            .map(|(parent, _)| parent)
    }

    #[note("document-ancestors")]
    pub fn ancestors(&self, id: &str) -> Vec<NodeId> {
        let mut out = Vec::new();
        let mut current = id.to_string();
        // The limit exists because `validate` might not have run: a
        // corrupted file with a cycle would freeze the IDE, and freezing is worse than ignoring.
        for _ in 0..64 {
            match self.parent_of(&current) {
                Some(parent) => {
                    out.push(parent.clone());
                    current = parent.clone();
                }
                None => break,
            }
        }
        out
    }

    #[note("document-parent-origin")]
    pub fn parent_origin(&self, id: &str) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;
        for a in self.ancestors(id) {
            if a == self.root {
                continue;
            }
            if let Some(r) = self.node(&a).and_then(|n| n.rect()) {
                x += r.x;
                y += r.y;
            }
        }
        (x, y)
    }

    /// The caption a component takes from the `Action` it points to: the
    /// action's `text` (or `simple_text`), the same one the canvas and the
    /// running app show. `None` when there is no linked action or it has no
    /// caption, and then the component's own caption stands.
    #[note("document-linked-action-text")]
    pub fn linked_action_text(&self, id: &str) -> Option<&str> {
        let action = match self.node(id)?.props.get("action")? {
            Value::Ref(r) if !r.is_empty() => self.node(r)?,
            _ => return None,
        };
        match action
            .props
            .get("text")
            .or_else(|| action.props.get("simple_text"))?
        {
            Value::Str(t) => Some(t.as_str()),
            _ => None,
        }
    }

    #[note("document-rect-in-form")]
    pub fn rect_in_form(&self, id: &str) -> Option<Rect> {
        let r = self.node(id)?.rect()?;
        let (dx, dy) = self.parent_origin(id);
        Some(Rect::new(r.x + dx, r.y + dy, r.w, r.h))
    }

    #[note("document-in-paint-order")]
    pub fn in_paint_order(&self) -> Vec<NodeId> {
        let mut out = Vec::new();
        self.descend(&self.root.clone(), &mut out, 0);
        out
    }

    fn descend(&self, id: &NodeId, out: &mut Vec<NodeId>, level: usize) {
        if level > 64 {
            return;
        }
        let Some(n) = self.node(id) else { return };
        for f in &n.children {
            out.push(f.clone());
            self.descend(f, out, level + 1);
        }
    }

    pub fn validate(&self) -> Result<(), IrError> {
        if !self.nodes.contains_key(&self.root) {
            return Err(IrError::MissingNode(self.root.clone()));
        }

        for (id, n) in &self.nodes {
            if id != &n.id {
                return Err(IrError::IdMismatch {
                    key: id.clone(),
                    inner: n.id.clone(),
                });
            }
            for k in n.props.keys() {
                PropKey::parse(k).map_err(|e| IrError::Contract {
                    node: id.clone(),
                    source: e,
                })?;
            }
            for c in &n.children {
                if !self.nodes.contains_key(c) {
                    return Err(IrError::MissingNode(c.clone()));
                }
            }
            // Dangling reference is silent garbage: the button keeps
            // pointing to the action that has already been deleted, and it is only discovered
            // on the day of code generation.
            for (k, v) in &n.props {
                if let Value::Ref(target) = v
                    && !target.is_empty()
                    && !self.nodes.contains_key(target)
                {
                    return Err(IrError::DanglingRef {
                        node: id.clone(),
                        key: k.clone(),
                        target: target.clone(),
                    });
                }
            }
        }

        // `Name` is unique within the form, cannot be empty, and requires
        // substance — see `name_has_alphanumeric`.
        let mut names: BTreeSet<&str> = BTreeSet::new();
        for n in self.nodes.values() {
            let name = n.name();
            if name.trim().is_empty() {
                return Err(IrError::EmptyName(n.id.clone()));
            }
            if !name_has_alphanumeric(name) {
                return Err(IrError::NameWithoutAlnum(name.to_string()));
            }
            if !names.insert(name) {
                return Err(IrError::DuplicateName(name.to_string()));
            }
        }

        // each node (except root) has exactly one parent
        let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
        for n in self.nodes.values() {
            for c in &n.children {
                *seen.entry(c.as_str()).or_insert(0) += 1;
            }
        }
        for (id, count) in &seen {
            if *count > 1 {
                return Err(IrError::MultipleParents((*id).to_string()));
            }
        }

        let reachable: BTreeSet<&str> = self.walk().iter().map(|n| n.id.as_str()).collect();
        if reachable.len() != self.nodes.len() {
            let orphan = self
                .nodes
                .keys()
                .find(|k| !reachable.contains(k.as_str()))
                .cloned()
                .unwrap_or_default();
            return Err(IrError::Orphan(orphan));
        }

        Ok(())
    }
}

#[note("name-has-alphanumeric")]
pub fn name_has_alphanumeric(name: &str) -> bool {
    name.chars().any(char::is_alphanumeric)
}

#[note("is-snake-case-name")]
pub fn is_snake_case_name(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[note("snake-case-hint")]
pub fn snake_case_hint(name: &str) -> Option<String> {
    let mut suggestion = String::new();
    let mut prev_upper = false;
    let mut prev_separator = true;
    for c in name.chars() {
        if c == '_' || c == '-' || c == ' ' {
            if !prev_separator && !suggestion.is_empty() {
                suggestion.push('_');
            }
            prev_separator = true;
            prev_upper = false;
        } else if c.is_uppercase() {
            if !prev_upper && !prev_separator && !suggestion.is_empty() {
                suggestion.push('_');
            }
            suggestion.extend(c.to_lowercase());
            prev_upper = true;
            prev_separator = false;
        } else {
            suggestion.push(c);
            prev_upper = false;
            prev_separator = false;
        }
    }
    (suggestion != name && is_snake_case_name(&suggestion) && name_has_alphanumeric(&suggestion))
        .then_some(suggestion)
}

#[note("irerror")]
#[derive(Debug, thiserror::Error)]
pub enum IrError {
    #[error("format {found} is newer than the supported one ({supported}) — update CrawRAD")]
    FutureFormat { found: u32, supported: u32 },

    #[error("node '{0}' does not exist")]
    MissingNode(String),

    #[error("duplicate id: '{0}'")]
    DuplicateId(String),

    // `Name` remains strictly unique — codegen transforms it into a
    // field, and two fields with the same name do not compile. What changed in ADR
    // 0008 is that uniqueness stopped being structural (map key) and became a
    // property rule, which is where it should always have lived.
    #[error("there is already a component named '{0}'")]
    DuplicateName(String),

    #[error("node '{0}' has no name")]
    EmptyName(String),

    // Separated from `EmptyName` on purpose: `___` **has** characters, and saying
    // "has no name" to someone who just typed three underscores does not help at
    // all. The message that the user reads must say what to do.
    #[error("'{0}' is not a valid identifier: use at least one letter or digit in the name")]
    NameWithoutAlnum(String),

    // The rejection that replaced silent translation. It is not satisfied with
    // merely saying no: `hint` carries the correct form when there is one, because whoever
    // just typed `btnOK` does not want a lecture on conventions — they want to know what
    // to write. See `IrError::not_snake_case`.
    #[error("'{name}' is not snake_case: the Name becomes a field in the generated code, {hint}")]
    NameNotSnakeCase { name: String, hint: String },

    #[error("node indexed as '{key}' but carrying id '{inner}'")]
    IdMismatch { key: String, inner: String },

    #[error("node '{0}' appears as a child of more than one parent")]
    MultipleParents(String),

    #[error("node '{0}' is not reachable from the root")]
    Orphan(String),

    #[error("'{node}.{key}' points to '{target}', which does not exist")]
    DanglingRef {
        node: String,
        key: String,
        target: String,
    },

    #[error("in node '{node}': {source}")]
    Contract {
        node: String,
        #[source]
        source: ContractError,
    },

    #[error("{path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("cannot inherit from '{parent}': cycle detected")]
    InheritanceCycle { parent: String },

    #[error("cannot inherit from self: '{0}'")]
    SelfInheritance(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl IrError {
    #[note("irerror-not-snake-case")]
    pub fn not_snake_case(name: impl Into<String>) -> Self {
        let name = name.into();
        let hint = match snake_case_hint(&name) {
            Some(s) => format!("use `{s}`"),
            None => "use lowercase letters, digits and underscores — like `btn_ok`".to_string(),
        };
        Self::NameNotSnakeCase { name, hint }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "test/style_rule/test.rs"]
mod style_rule_test;
