//! Slint component implementation for `Card`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct CardCodegen;

impl SlintComponentCodegen for CardCodegen {
    fn class(&self) -> &'static str {
        "Card"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::card::card();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.push("    pub subtitle: String,".to_string());
        fields.push("    pub corner_radius: i64,".to_string());
        fields.push("    pub elevation: i64,".to_string());
        fields.push("    pub show_header: bool,".to_string());
        // card text catalogue fields
        fields.extend(util::catalogue_fields(
            &desc,
            // card fields
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "subtitle",
                "corner_radius",
                "elevation",
                "show_header",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Card {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::card::card();
        let subtitle = match node.props.get("subtitle") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let corner_radius = match node.props.get("corner_radius") {
            Some(Value::Int(i)) => *i,
            _ => 8,
        };
        let elevation = match node.props.get("elevation") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let show_header = !matches!(node.props.get("show_header"), Some(Value::Bool(false)));

        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.push(format!(
            "subtitle: {}.to_string()",
            crate::literal(subtitle)
        ));
        inits.push(format!("corner_radius: {corner_radius}"));
        inits.push(format!("elevation: {elevation}"));
        inits.push(format!("show_header: {show_header}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            // card inits
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "subtitle",
                "corner_radius",
                "elevation",
                "show_header",
            ],
        ));
        let body = inits.join(", ");
        format!("Card {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // card action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // card action call
        util::emit_click_action(node, field)
    }

    fn emit_slint(
        &self,
        // card doc
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        // card bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // card id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let _en = layout::is_enabled(node);
        // card title text
        let caption = layout::text(node);
        let caption_esc = util::escape_slint_string(&caption);
        let subtitle = match node.props.get("subtitle") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let subtitle_esc = util::escape_slint_string(subtitle);
        let corner_radius = match node.props.get("corner_radius") {
            Some(Value::Int(i)) => *i,
            _ => 8,
        };
        let show_header = !matches!(node.props.get("show_header"), Some(Value::Bool(false)));
        let header_h = if subtitle.is_empty() { 28.0 } else { 42.0 };
        // card child nodes
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = util::emit_rectangle_header(node, x, y, w, h)?;
        let cr_str = util::border_radius_px(node, corner_radius as f32);
        out.push_str(&format!("            border-radius: {cr_str};\n"));
        out.push_str("            background: #252538;\n");
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #3e3e56;\n");
        out.push_str("            clip: true;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        if show_header {
            out.push_str("            Rectangle {\n");
            out.push_str("                x: 0px;\n");
            out.push_str("                y: 0px;\n");
            out.push_str("                width: parent.width;\n");
            out.push_str(&format!("                height: {header_h:.1}px;\n"));
            out.push_str("                background: #2d2d42;\n");
            out.push_str("                border-width: 1px;\n");
            out.push_str("                border-color: #3e3e56;\n");
            out.push_str("                Text {\n");
            out.push_str("                    x: 10px;\n");
            out.push_str("                    y: 6px;\n");
            out.push_str(&format!("                    text: \"{caption_esc}\";\n"));
            out.push_str("                    color: #ffffff;\n");
            out.push_str("                    font-size: 11px;\n");
            out.push_str("                    font-weight: 700;\n");
            out.push_str("                }\n");
            if !subtitle.is_empty() {
                out.push_str("                Text {\n");
                out.push_str("                    x: 10px;\n");
                out.push_str("                    y: 20px;\n");
                out.push_str(&format!("                    text: \"{subtitle_esc}\";\n"));
                out.push_str("                    color: #a0a0b0;\n");
                out.push_str("                    font-size: 9px;\n");
                out.push_str("                }\n");
            }
            out.push_str("            }\n");
        }
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
