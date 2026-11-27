//! Slint component mapping and registry module.

pub mod button;
pub mod codegen;
pub mod label;
pub mod util;

pub use codegen::SlintComponentCodegen;

use crate::Erro;
use crawrad_ir::{Document, Node};

static REGISTRY: &[&'static dyn SlintComponentCodegen] =
    &[&button::ButtonCodegen, &label::LabelCodegen];

pub fn for_class(class: &str) -> Option<&'static dyn SlintComponentCodegen> {
    REGISTRY.iter().copied().find(|c| c.class_name() == class)
}

pub fn classes() -> Vec<&'static str> {
    REGISTRY.iter().map(|c| c.class()).collect()
}

pub fn component_structs(doc: &Document, reg: &crawrad_registry::Registry) -> String {
    let mut out = String::new();
    let mut used_classes = std::collections::BTreeSet::new();
    for node in doc.nodes.values() {
        used_classes.insert(node.class.as_str());
        used_classes.insert(reg.effective_class(&node.class));
    }

    for class in used_classes {
        if let Some(c) = for_class(class) {
            out.push_str(&c.struct_definition());
            out.push('\n');
        }
    }
    out
}

pub fn emit_slint(
    // Source document
    doc: &Document,
    // Component registry
    reg: &crawrad_registry::Registry,
    // Component node
    node: &Node,
    // Coordinates and dimensions
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, Erro> {
    if let Some(c) = for_class(&node.class) {
        c.emit_slint(doc, reg, node, x, y, w, h)
    } else {
        Ok(String::new())
    }
}

pub fn emit_default_com(_reg: &crawrad_registry::Registry, doc: &Document, node: &Node) -> String {
    if let Some(c) = for_class(&node.class) {
        c.emit_default_doc(doc, node)
    } else {
        format!("{}::default()", node.class)
    }
}

pub fn emit_actions(
    // Source document
    doc: &Document,
    // Target component node
    node: &Node,
    // Form struct field name
    field: &str,
    // Emitted nodes
    emitted: &[&Node],
) -> Result<String, Erro> {
    if let Some(c) = for_class(&node.class) {
        c.emit_actions(doc, node, field, emitted)
    } else {
        Ok(String::new())
    }
}

pub fn emit_sync(doc: &Document, node: &Node, field: &str) -> String {
    if let Some(c) = for_class(&node.class) {
        c.emit_sync(doc, node, field)
    } else {
        String::new()
    }
}

pub fn slint_callbacks(node: &Node) -> Vec<String> {
    if let Some(c) = for_class(&node.class) {
        c.slint_callbacks(node)
    } else {
        Vec::new()
    }
}

pub fn slint_properties(node: &Node) -> Vec<String> {
    if let Some(c) = for_class(&node.class) {
        c.slint_properties(node)
    } else {
        Vec::new()
    }
}
