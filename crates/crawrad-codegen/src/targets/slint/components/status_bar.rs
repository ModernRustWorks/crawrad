//! Slint component implementation for `StatusBar` and `StatusPanel`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct StatusBarCodegen;

impl SlintComponentCodegen for StatusBarCodegen {
    fn class(&self) -> &'static str {
        "StatusBar"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::status_bar::status_bar();
        let mut fields = util::common_control_fields();
        fields.push("    pub simple_text: String,".to_string());
        fields.push("    pub simple_panel: bool,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "simple_text", "simple_panel"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct StatusBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::status_bar::status_bar();
        let simple_text = match node.props.get("simple_text") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let simple_panel = !matches!(node.props.get("simple_panel"), Some(Value::Bool(false)));

        let mut inits = util::common_control_inits(node);
        inits.push(format!(
            "simple_text: {}.to_string()",
            crate::literal(simple_text)
        ));
        inits.push(format!("simple_panel: {simple_panel}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "simple_text", "simple_panel"],
        ));
        let body = inits.join(", ");
        format!("StatusBar {{ {body} }}")
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
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
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
        let txt = match node.props.get("simple_text") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let txt_esc = util::escape_slint_string(txt);

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            background: #252526;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        out.push_str("            Text {\n");
        out.push_str("                x: 4px;\n");
        out.push_str(&format!("                text: \"{txt_esc}\";\n"));
        out.push_str("                color: #cccccc;\n");
        out.push_str("                font-size: 9px;\n");
        out.push_str("                vertical-alignment: center;\n");
        out.push_str("            }\n");
        if layout::event_handler(node, "on_click").is_some() {
            out.push_str("            TouchArea {\n");
            out.push_str(&format!(
                "                clicked => {{ root.{name}_clicked(); }}\n"
            ));
            out.push_str("            }\n");
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        util::click_callbacks(node)
    }
}

pub struct StatusPanelCodegen;

impl SlintComponentCodegen for StatusPanelCodegen {
    fn class(&self) -> &'static str {
        "StatusPanel"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::status_bar::status_panel();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.push("    pub width: i64,".to_string());
        fields.push("    pub alignment: String,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "width",
                "panel_width",
                "alignment",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct StatusPanel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::status_bar::status_panel();
        let width = match node.props.get("panel_width") {
            Some(Value::Int(i)) => *i,
            _ => 50,
        };
        let alignment = match node.props.get("alignment") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "taLeftJustify",
        };
        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.push(format!("width: {width}"));
        inits.push(format!(
            "alignment: {}.to_string()",
            crate::literal(alignment)
        ));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "width",
                "panel_width",
                "alignment",
            ],
        ));
        let body = inits.join(", ");
        format!("StatusPanel {{ {body} }}")
    }
}
