//! Slint component implementation for `Image`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct ImageCodegen;

impl SlintComponentCodegen for ImageCodegen {
    fn class(&self) -> &'static str {
        "Image"
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
    let desc = crawrad_registry::components::image();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Image {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    let desc = crawrad_registry::components::image();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("Image {{ {body} }}")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := Image {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            image-fit: contain;\n");
    out.push_str("        }\n");
    Ok(out)
}
