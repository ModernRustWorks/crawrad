//! Slint component implementation for `Bevel`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct BevelCodegen;

impl SlintComponentCodegen for BevelCodegen {
    fn class(&self) -> &'static str {
        "Bevel"
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
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_slint(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::bevel();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Bevel {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    let desc = crawrad_registry::components::bevel();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("Bevel {{ {body} }}")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := Rectangle {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            border-width: 1px;\n");
    out.push_str("            border-color: #a0a0a0;\n");
    out.push_str("            background: transparent;\n");
    out.push_str("        }\n");
    Ok(out)
}
