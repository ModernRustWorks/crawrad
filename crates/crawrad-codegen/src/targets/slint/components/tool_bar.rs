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
            // tool_bar inits
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
        // tool_bar action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // tool_bar action call
        util::emit_click_action(node, field)
    }

    fn emit_slint(
        &self,
        // tool_bar doc
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        // tool_bar bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // tool_bar id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let _en = layout::is_enabled(node);
        // tool_bar child nodes
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = util::emit_rectangle_header(node, x, y, w, h)?;
        out.push_str("            background: #2d3139;\n");
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #1e1e24;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        if layout::event_handler(node, "on_click").is_some() {
            out.push_str(&util::emit_touch_area_click(&name));
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
