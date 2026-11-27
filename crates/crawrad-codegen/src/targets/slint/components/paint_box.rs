//! Slint component implementation for `PaintBox`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct PaintBoxCodegen;

impl SlintComponentCodegen for PaintBoxCodegen {
    fn class(&self) -> &'static str {
        "PaintBox"
    }

    fn struct_definition(&self) -> String {
        util::standard_control_struct("PaintBox", &crawrad_registry::components::paint_box())
    }

    fn emit_default(&self, node: &Node) -> String {
        util::standard_control_default("PaintBox", node, &crawrad_registry::components::paint_box())
    }

    fn emit_slint(
        &self,
        // paint box document reference
        _doc: &crawrad_ir::Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // paint box layout bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        util::emit_simple_rectangle(node, x, y, w, h, "            background: transparent;\n")
    }
}
