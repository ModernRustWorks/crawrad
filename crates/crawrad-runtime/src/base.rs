//! Base hierarchy for runtime components and controls.
//!
//! Inspired by the layered model of the classic RAD IDE and LCL (`TComponent` -> `TControl` -> `TWinControl`),
//! adapted for idiomatic Rust via composition and [`std::ops::Deref`].
//!
//! Allows any runtime (egui, makepad, slint, etc.) to share the same base of
//! geometric, visibility, action, parentage, and tab order properties,
//! eliminating redundant rewrites when extracting from the document [`crawrad_ir::Document`].

use crawrad_ir::{Document, Node, NodeId, Rect, Value};
use crawrad_note::note;
use std::ops::{Deref, DerefMut};

#[note("base-componentbase")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentBase {
    pub id: NodeId,
    pub name: String,
    pub class: String,
}

impl ComponentBase {
    pub fn new(id: impl Into<NodeId>, name: impl Into<String>, class: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            class: class.into(),
        }
    }

    pub fn from_node(id: &NodeId, n: &Node) -> Self {
        Self {
            id: id.clone(),
            name: n.name().to_string(),
            class: n.class.clone(),
        }
    }
}

#[note("base-visualcontrolbase")]
#[derive(Debug, Clone, PartialEq)]
pub struct VisualControlBase {
    pub base: ComponentBase,
    pub rect: Rect,
    pub visible: bool,
    pub enabled: bool,
    pub text: String,
    pub visual: bool,
    /// The parent imposes the rectangle (see [`crate::DERIVED_GEOMETRY`]).
    pub derived_geometry: bool,
}

impl Deref for VisualControlBase {
    type Target = ComponentBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for VisualControlBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl VisualControlBase {
    pub fn from_doc(id: &NodeId, n: &Node, doc: &Document, visual: bool) -> Self {
        let base = ComponentBase::from_node(id, n);

        // The action link: caption and state come FROM THE ACTION when the component
        // points to one.
        let action = match n.props.get("action") {
            Some(Value::Ref(r)) if !r.is_empty() => doc.node(r),
            _ => None,
        };
        let get_text = |node: &Node| match node
            .props
            .get("text")
            .or_else(|| node.props.get("simple_text"))
        {
            Some(Value::Str(t)) => Some(t.clone()),
            _ => None,
        };
        let get_bool = |node: &Node, k: &str, default: bool| match node.props.get(k) {
            Some(Value::Bool(b)) => *b,
            _ => default,
        };

        let mut text = get_text(n).unwrap_or_default();
        let mut enabled = get_bool(n, "enabled", true);
        if let Some(t) = doc.linked_action_text(id) {
            text = t.to_string();
        }
        if let Some(a) = action {
            enabled = get_bool(a, "enabled", enabled);
        }

        let rect = doc
            .rect_in_form(id)
            .unwrap_or(Rect::new(0.0, 0.0, 80.0, 24.0));
        let visible = get_bool(n, "visible", true);
        let derived_geometry = crate::DERIVED_GEOMETRY.contains(&n.class.as_str());

        Self {
            base,
            rect,
            visible,
            enabled,
            text,
            visual,
            derived_geometry,
        }
    }
}

#[note("base-windowcontrolbase")]
#[derive(Debug, Clone, PartialEq)]
pub struct WindowControlBase {
    pub visual_base: VisualControlBase,
    pub parent: NodeId,
    pub children: Vec<NodeId>,
    pub clip: Option<Rect>,
    pub tab_order: i64,
}

impl Deref for WindowControlBase {
    type Target = VisualControlBase;

    fn deref(&self) -> &Self::Target {
        &self.visual_base
    }
}

impl DerefMut for WindowControlBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.visual_base
    }
}

impl WindowControlBase {
    pub fn from_doc(id: &NodeId, n: &Node, doc: &Document, visual: bool) -> Self {
        let visual_base = VisualControlBase::from_doc(id, n, doc, visual);

        let parent = doc
            .parent_of(id)
            .cloned()
            .unwrap_or_else(|| doc.root.clone());

        let children = n.children.clone();

        let clip = doc
            .parent_of(id)
            .filter(|p| **p != doc.root)
            .and_then(|p| doc.rect_in_form(p));

        let tab_order = match n.props.get("tab_order") {
            Some(Value::Int(i)) => *i,
            _ => i64::MAX,
        };

        Self {
            visual_base,
            parent,
            children,
            clip,
            tab_order,
        }
    }
}
