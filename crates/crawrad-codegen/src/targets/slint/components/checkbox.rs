//! Slint component implementation for `CheckBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::identificador;
use crawrad_ir::{Document, Node};

pub struct CheckBoxCodegen;

impl SlintComponentCodegen for CheckBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckBox"
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
        vec![format!("callback {name}_toggled(bool);")]
    }

    fn std_widgets(&self) -> &'static [&'static str] {
        &["CheckBox"]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::check_box();
    util::checked_control_struct("CheckBox", &desc)
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::check_box();
    util::checked_control_default("CheckBox", node, &desc)
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    util::emit_toggle_action(node, field, "toggled", "checked")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let (name, caption_esc, checked, en, pos) = util::layout_props_checked(node, x, y)?;

    let mut out = String::new();
    out.push_str(&format!("        {name} := CheckBox {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            text: \"{caption_esc}\";\n"));
    if checked {
        out.push_str("            checked: true;\n");
    }
    if !en {
        out.push_str("            enabled: false;\n");
    }
    out.push_str(&format!(
        "            toggled => {{ root.{name}_toggled(self.checked); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
