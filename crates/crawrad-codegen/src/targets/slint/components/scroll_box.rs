//! Slint component implementation for `ScrollBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ScrollBoxCodegen;

impl SlintComponentCodegen for ScrollBoxCodegen {
    fn class(&self) -> &'static str {
        "ScrollBox"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::scroll_box::scroll_box();
        let mut fields = util::common_control_fields();
        fields.push("    pub auto_scroll: bool,".to_string());
        fields.push("    pub horz_scroll_bar: bool,".to_string());
        fields.push("    pub vert_scroll_bar: bool,".to_string());
        fields.push("    pub border_width: i64,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "auto_scroll",
                "horz_scroll_bar",
                "vert_scroll_bar",
                "border_width",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ScrollBox {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::scroll_box::scroll_box();
        let auto_scroll = !matches!(node.props.get("auto_scroll"), Some(Value::Bool(false)));
        let horz_scroll = !matches!(node.props.get("horz_scroll_bar"), Some(Value::Bool(false)));
        let vert_scroll = !matches!(node.props.get("vert_scroll_bar"), Some(Value::Bool(false)));
        let bw = match node.props.get("border_width") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let mut inits = util::common_control_inits(node);
        inits.push(format!("auto_scroll: {auto_scroll}"));
        inits.push(format!("horz_scroll_bar: {horz_scroll}"));
        inits.push(format!("vert_scroll_bar: {vert_scroll}"));
        inits.push(format!("border_width: {bw}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "auto_scroll",
                "horz_scroll_bar",
                "vert_scroll_bar",
                "border_width",
            ],
        ));
        let body = inits.join(", ");
        format!("ScrollBox {{ {body} }}")
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
        let name = identificador(node.name())?;
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let bw = match node.props.get("border_width") {
            Some(Value::Int(i)) => (*i).max(0),
            _ => 1,
        };
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            background: #1e1e2e;\n");
        out.push_str(&format!("            border-width: {bw}px;\n"));
        out.push_str("            border-color: #45475a;\n");
        out.push_str("            clip: true;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        if !children.is_empty() {
            out.push_str(&children);
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }
}
