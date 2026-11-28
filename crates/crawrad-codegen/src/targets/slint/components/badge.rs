//! Slint component implementation for `Badge`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct BadgeCodegen;

impl SlintComponentCodegen for BadgeCodegen {
    fn class(&self) -> &'static str {
        "Badge"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::badge::badge();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "text", "caption"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Badge {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::badge::badge();
        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "text", "caption"],
        ));
        let body = inits.join(", ");
        format!("Badge {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // badge action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // badge action call
        util::emit_click_action(node, field)
    }

    fn emit_slint(
        &self,
        // badge doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // badge bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // badge id
        let name = identificador(node.name())?;
        let caption = layout::text(node);
        let count = match node.props.get("count") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        };
        let max_count = match node.props.get("max_count") {
            Some(Value::Int(i)) => *i,
            _ => 99,
        };
        let effective_text = if count > 0 {
            if count > max_count {
                format!("{max_count}+")
            } else {
                count.to_string()
            }
        } else {
            caption
        };
        let text_esc = util::escape_slint_string(&effective_text);

        let kind = match node.props.get("kind") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "bkPrimary",
        };
        let effective_kind = if count > 0 && (kind == "bkPrimary" || kind.is_empty()) {
            "bkDanger"
        } else {
            kind
        };
        let outline = matches!(node.props.get("outline"), Some(Value::Bool(true)));

        let (base_bg, base_fg) = match effective_kind {
            "bkDefault" => ("#383847", "#f2f2fa"),
            "bkPrimary" => ("#3a598c", "#f2f2fa"),
            "bkInfo" => ("#2e80b8", "#f2f2fa"),
            "bkWarning" => ("#d18c26", "#1a1a26"),
            "bkDanger" => ("#cc3838", "#f2f2fa"),
            _ => ("#3a598c", "#f2f2fa"),
        };
        let (bg_color, border_color, text_color) = if outline {
            ("transparent", base_bg, base_bg)
        } else {
            (base_bg, base_bg, base_fg)
        };

        let radius = (h * 0.5).min(w * 0.5);
        let vis = layout::is_visible(node);

        let mut out = util::emit_rectangle_header(node, x, y, w, h)?;
        let br_str = if node.props.contains_key("slint.border-radius") {
            util::border_radius_px(node, radius)
        } else {
            format!("{radius:.1}px")
        };
        out.push_str(&format!("            border-radius: {br_str};\n"));
        out.push_str(&format!("            background: {bg_color};\n"));
        out.push_str(&format!("            border-color: {border_color};\n"));
        out.push_str("            border-width: 1px;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str("            Text {\n");
        out.push_str(&format!("                text: \"{text_esc}\";\n"));
        out.push_str(&format!("                color: {text_color};\n"));
        out.push_str("                font-size: 9px;\n");
        out.push_str("                horizontal-alignment: center;\n");
        out.push_str("                vertical-alignment: center;\n");
        out.push_str("            }\n");
        if layout::event_handler(node, "on_click").is_some() {
            out.push_str(&util::emit_touch_area_click(&name));
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        util::click_callbacks(node)
    }
}
