//! Layout utility functions, property extraction, and hierarchy resolution for Slint.

use crate::identificador;
use crawrad_ir::{Document, Node, Value};

/// Computes x/y position strings for Slint markup.
pub fn position(x: f32, y: f32) -> String {
    format!("x: {x:.1}px; y: {y:.1}px;")
}

/// Extracts text of node stripped of accelerator character (&).
pub fn text(node: &Node) -> String {
    crate::sem_acelerador(node.text().unwrap_or(node.name()))
}

/// Retrieves a string property value if present.
pub fn str_val<'a>(node: &'a Node, key: &str) -> Option<&'a str> {
    match node.props.get(key) {
        Some(Value::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

/// Retrieves an event handler name if defined.
pub fn event_handler(node: &Node, event: &str) -> Option<String> {
    node.events
        .get(event)
        .filter(|h| !h.trim().is_empty())
        .cloned()
}

/// Checks if a component class has visual presentation.
pub fn has_widget(reg: &crawrad_registry::Registry, class: &str) -> bool {
    if let Some(desc) = reg.get(class) {
        desc.visual && !desc.derived_geometry
    } else {
        false
    }
}

/// Checks if a node resides inside a visual container.
pub fn is_inside_container(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node_id: &str,
) -> bool {
    let mut current_id = node_id.to_string();
    while let Some(parent_id) = doc.parent_of(&current_id) {
        if let Some(parent) = doc.node(parent_id) {
            if parent.class == "Form" {
                return false;
            }
            if let Some(desc) = reg.get(&parent.class)
                && desc.children_area()
            {
                return true;
            }
            current_id = parent.id.clone();
        } else {
            break;
        }
    }
    false
}

/// Computes the runtime visibility expression for a control.
pub fn visibility_expression(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
) -> Result<String, crate::Erro> {
    let field = identificador(node.name())?;
    let mut conditions = vec![format!("self.form.{field}.visible")];

    let mut current_id = node.id.clone();
    while let Some(parent_id) = doc.parent_of(&current_id) {
        if let Some(parent) = doc.node(parent_id) {
            if let Some(parent_desc) = reg.get(&parent.class)
                && parent.class != "Form"
                && parent_desc.children_area()
            {
                let parent_field = identificador(parent.name())?;
                conditions.push(format!("self.form.{parent_field}.visible"));
            }
            current_id = parent.id.clone();
        } else {
            break;
        }
    }

    Ok(conditions.join(" && "))
}

/// Checks if a component node is set to visible (defaults to true).
pub fn is_visible(node: &Node) -> bool {
    !matches!(node.props.get("visible"), Some(Value::Bool(false)))
}

/// Checks if a component node is enabled (defaults to true).
pub fn is_enabled(node: &Node) -> bool {
    !matches!(node.props.get("enabled"), Some(Value::Bool(false)))
}

/// Checks if a component node is checked.
pub fn is_checked(node: &Node) -> bool {
    matches!(node.props.get("checked"), Some(Value::Bool(true)))
}
