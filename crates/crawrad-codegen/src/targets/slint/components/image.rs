//! Slint component implementation for `Image`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::Node;

pub struct ImageCodegen;

impl SlintComponentCodegen for ImageCodegen {
    fn class(&self) -> &'static str {
        "Image"
    }

    fn struct_definition(&self) -> String {
        util::standard_control_struct("Image", &crawrad_registry::components::image())
    }

    fn emit_default(&self, node: &Node) -> String {
        util::standard_control_default("Image", node, &crawrad_registry::components::image())
    }

    fn emit_slint(
        &self,
        _doc: &crawrad_ir::Document,
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
        let name = identificador(node.name())?;
        let pos = layout::position(x, y);
        Ok(format!(
            "        {name} := Image {{\n\
             \x20           {pos}\n\
             \x20           width: {w:.1}px;\n\
             \x20           height: {h:.1}px;\n\
             \x20           image-fit: contain;\n\
             \x20       }}\n"
        ))
    }
}
