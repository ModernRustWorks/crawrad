//! Slint component implementation for `RadioButton`.

use super::{codegen::SlintComponentCodegen, util};
use crate::identificador;
use crawrad_ir::{Document, Node};

pub struct RadioButtonCodegen;

impl SlintComponentCodegen for RadioButtonCodegen {
    fn class(&self) -> &'static str {
        "RadioButton"
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
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
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
        emit_slint(node, x, y, w, h)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_clicked();")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::radio_button();
    util::checked_control_struct("RadioButton", &desc)
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::radio_button();
    util::checked_control_default("RadioButton", node, &desc)
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    util::emit_toggle_action(node, field, "clicked", "checked")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let (name, caption_esc, checked, en, pos) = util::layout_props_checked(node, x, y)?;

    let mut out = String::new();
    out.push_str(&format!("        {name} := HorizontalLayout {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            spacing: 6px;\n");
    out.push_str("            Rectangle {\n");
    out.push_str("                width: 16px;\n");
    out.push_str("                height: 16px;\n");
    out.push_str("                border-radius: 8px;\n");
    out.push_str("                border-width: 1px;\n");
    out.push_str("                border-color: #606060;\n");
    out.push_str("                background: transparent;\n");
    if checked {
        out.push_str("                Rectangle {\n");
        out.push_str("                    width: 8px;\n");
        out.push_str("                    height: 8px;\n");
        out.push_str("                    border-radius: 4px;\n");
        out.push_str("                    background: #2a82da;\n");
        out.push_str("                }\n");
    }
    out.push_str("            }\n");
    out.push_str("            Text {\n");
    out.push_str(&format!("                text: \"{caption_esc}\";\n"));
    out.push_str("                vertical-alignment: center;\n");
    if !en {
        out.push_str("                color: #888888;\n");
    }
    out.push_str("            }\n");
    out.push_str("            TouchArea {\n");
    out.push_str(&format!(
        "                clicked => {{ root.{name}_clicked(); }}\n"
    ));
    out.push_str("            }\n");
    out.push_str("        }\n");
    Ok(out)
}
