//! Slint component implementation for `Shape`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct ShapeCodegen;

impl SlintComponentCodegen for ShapeCodegen {
    fn class(&self) -> &'static str {
        "Shape"
    }

    fn struct_definition(&self) -> String {
        util::standard_control_struct("Shape", &crawrad_registry::components::shape())
    }

    fn emit_default(&self, node: &Node) -> String {
        util::standard_control_default("Shape", node, &crawrad_registry::components::shape())
    }

    fn emit_slint(
        &self,
        // shape document reference
        _doc: &crawrad_ir::Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // shape layout bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        util::emit_simple_rectangle(
            node,
            x,
            y,
            w,
            h,
            "            background: #ffffff;\n            border-width: 1px;\n            border-color: #000000;\n",
        )
    }
}
