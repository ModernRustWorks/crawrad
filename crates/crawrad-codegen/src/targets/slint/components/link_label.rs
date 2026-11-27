//! Slint component implementation for `LinkLabel`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crawrad_ir::{Document, Node};

pub struct LinkLabelCodegen;

impl SlintComponentCodegen for LinkLabelCodegen {
    fn class(&self) -> &'static str {
        "LinkLabel"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::link_label();
        let body = util::struct_fields_with_text(&desc);
        format!("#[derive(Debug, Clone, Default)]\npub struct LinkLabel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::link_label();
        let body = util::struct_inits_with_text(node, &desc);
        format!("LinkLabel {{ {body} }}")
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
        util::emit_click_action(node, field)
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
        let (name, caption_esc, pos) = util::layout_props_caption(node, x, y)?;

        let mut out = format!(
            "        {name} := TouchArea {{\n            {pos}\n            width: {w:.1}px;\n            height: {h:.1}px;\n            mouse-cursor: pointer;\n            Text {{\n                text: \"{caption_esc}\";\n                color: #0066cc;\n            }}\n"
        );
        if crate::targets::slint::layout::event_handler(node, "on_click").is_some() {
            out.push_str(&format!(
                "            clicked => {{ root.{name}_clicked(); }}\n"
            ));
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        util::click_callbacks(node)
    }
}
