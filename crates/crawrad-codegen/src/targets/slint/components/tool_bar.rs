//! Slint component implementation for `ToolBar`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ToolBarCodegen;

impl SlintComponentCodegen for ToolBarCodegen {
    fn class(&self) -> &'static str {
        "ToolBar"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::tool_bar::tool_bar();
        let mut fields = util::common_control_fields();
        fields.push("    pub flat: bool,".to_string());
        fields.push("    pub show_captions: bool,".to_string());
        fields.push("    pub button_width: i64,".to_string());
        fields.push("    pub button_height: i64,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "flat",
                "show_captions",
                "button_width",
                "button_height",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ToolBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::tool_bar::tool_bar();
        let flat = !matches!(node.props.get("flat"), Some(Value::Bool(false)));
        let show_captions = matches!(node.props.get("show_captions"), Some(Value::Bool(true)));
        let bw = match node.props.get("button_width") {
            Some(Value::Int(i)) => *i,
            _ => 24,
        };
        let bh = match node.props.get("button_height") {
            Some(Value::Int(i)) => *i,
            _ => 24,
        };
        let mut inits = util::common_control_inits(node);
        inits.push(format!("flat: {flat}"));
        inits.push(format!("show_captions: {show_captions}"));
        inits.push(format!("button_width: {bw}"));
        inits.push(format!("button_height: {bh}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "flat",
                "show_captions",
                "button_width",
                "button_height",
            ],
        ));
        let body = inits.join(", ");
        format!("ToolBar {{ {body} }}")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        util::emit_click_action(node, field)
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
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            background: #2d3139;\n");
        out.push_str("            border-bottom-width: 1px;\n");
        out.push_str("            border-color: #1e1e24;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        if layout::event_handler(node, "on_click").is_some() {
            out.push_str("            TouchArea {\n");
            out.push_str(&format!(
                "                clicked => {{ root.{name}_clicked(); }}\n"
            ));
            out.push_str("            }\n");
        }
        if !children.is_empty() {
            out.push_str(&children);
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        util::click_callbacks(node)
    }
}
