//! Slint component implementation for `Panel`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node};

pub struct PanelCodegen;

impl SlintComponentCodegen for PanelCodegen {
    fn class(&self) -> &'static str {
        "Panel"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        // target doc
        _doc: &Document,
        node: &Node,
        // field name
        field: &str,
        // emitted nodes
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(node, field)
    }

    fn emit_slint(
        &self,
        // target doc
        doc: &Document,
        reg: &crawrad_registry::Registry,
        // target node
        node: &Node,
        // coords
        x: f32,
        y: f32,
        // dimensions
        w: f32,
        // height
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(doc, reg, node, x, y, w, h)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        if layout::event_handler(node, "on_click").is_some() {
            vec![format!("callback {name}_clicked();")]
        } else {
            Vec::new()
        }
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::panel();
    util::standard_text_control_struct("Panel", &desc)
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::panel();
    util::standard_text_control_default("Panel", node, &desc)
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    util::emit_click_action(node, field)
}

pub fn emit_slint(
    // panel hierarchy document
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    // panel layout bounds
    x: f32,
    y: f32,
    w: f32,
    // component height
    h: f32,
) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos = layout::position(x, y);
    let children = super::emit_children(doc, reg, node, "    ")?;

    // Panel container rectangle
    let mut out = String::new();
    out.push_str(&format!("        {name} := Rectangle {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            background: #f0f0f0;\n");
    out.push_str("            border-width: 1px;\n");
    out.push_str("            border-color: #c0c0c0;\n");
    out.push_str("            clip: true;\n");
    if layout::event_handler(node, "on_click").is_some() {
        out.push_str(&format!(
            "            TouchArea {{\n\
             \x20               clicked => {{ root.{name}_clicked(); }}\n\
             \x20           }}\n"
        ));
    }
    if !children.is_empty() {
        out.push_str(&children);
    }
    out.push_str("        }\n");
    Ok(out)
}
