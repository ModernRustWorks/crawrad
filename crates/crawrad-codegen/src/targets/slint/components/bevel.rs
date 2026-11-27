//! Slint component implementation for `Bevel`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::Node;

pub struct BevelCodegen;

impl SlintComponentCodegen for BevelCodegen {
    fn class(&self) -> &'static str {
        "Bevel"
    }

    fn struct_definition(&self) -> String {
        util::standard_control_struct("Bevel", &crawrad_registry::components::bevel())
    }

    fn emit_default(&self, node: &Node) -> String {
        util::standard_control_default("Bevel", node, &crawrad_registry::components::bevel())
    }

    fn emit_slint(
        &self,
        // bevel document reference
        _doc: &crawrad_ir::Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // bevel layout bounds
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
            "            border-width: 1px;\n            border-color: #a0a0a0;\n            background: transparent;\n",
        )
    }
}
