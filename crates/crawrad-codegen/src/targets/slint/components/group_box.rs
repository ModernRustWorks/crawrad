//! Slint component implementation for `GroupBox`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct GroupBoxCodegen;

impl SlintComponentCodegen for GroupBoxCodegen {
    fn class(&self) -> &'static str {
        "GroupBox"
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
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(doc, reg, node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::group_box();
    let mut fields = util::common_control_fields();
    fields.push(util::FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct GroupBox {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));
    let desc = crawrad_registry::components::group_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("GroupBox {{ {body} }}")
}

pub fn emit_slint(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let title_esc = util::escape_slint_string(&caption);
    let pos = layout::position(x, y);
    let children = super::emit_children(doc, reg, node, "    ")?;

    let mut out = String::new();
    out.push_str(&format!("        {name} := GroupBox {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            title: \"{title_esc}\";\n"));
    if !children.is_empty() {
        out.push_str(&children);
    }
    out.push_str("        }\n");
    Ok(out)
}
