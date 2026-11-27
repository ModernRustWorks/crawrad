//! Slint component implementation for `Label`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node};

pub struct LabelCodegen;

impl SlintComponentCodegen for LabelCodegen {
    fn class(&self) -> &'static str {
        "Label"
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
        // Document
        _doc: &Document,
        // Target node
        _node: &Node,
        // Struct field
        _field: &str,
        // Emitted nodes
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_slint(
        &self,
        // Document
        _doc: &Document,
        // Registry
        _reg: &crawrad_registry::Registry,
        // Target node
        node: &Node,
        // Geometry
        x: f32,
        y: f32,
        // dimensions
        w: f32,
        // height
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::label();
    let body = util::struct_fields_with_text(&desc);
    format!("#[derive(Debug, Clone, Default)]\npub struct Label {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::label();
    let body = util::struct_inits_with_text(node, &desc);
    format!("Label {{ {body} }}")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = util::escape_slint_string(&caption);
    let pos = layout::position(x, y);

    Ok(format!(
        "        {name} := Text {{\n\
         \x20           {pos}\n\
         \x20           width: {w:.1}px;\n\
         \x20           height: {h:.1}px;\n\
         \x20           text: \"{caption_esc}\";\n\
         \x20       }}\n"
    ))
}
