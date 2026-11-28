//! Slint component implementation for `GroupBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
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
        // target doc
        _doc: &Document,
        _node: &Node,
        // field name
        _field: &str,
        // emitted nodes
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
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

    fn std_widgets(&self) -> &'static [&'static str] {
        &["GroupBox"]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::group_box();
    util::standard_text_control_struct("GroupBox", &desc)
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::group_box();
    util::standard_text_control_default("GroupBox", node, &desc)
}

pub fn emit_slint(
    // groupbox hierarchy document
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    // groupbox layout bounds
    x: f32,
    y: f32,
    w: f32,
    // component height
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
