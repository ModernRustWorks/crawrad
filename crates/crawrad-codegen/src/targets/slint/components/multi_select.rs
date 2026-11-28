//! Slint component implementation for `MultiSelect`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct MultiSelectCodegen;

impl SlintComponentCodegen for MultiSelectCodegen {
    fn class(&self) -> &'static str {
        "MultiSelect"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::multi_select::multi_select();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_ITEMS.to_string());
        fields.push("    pub selected_items: Strings,".to_string());
        fields.push("    pub text_hint: String,".to_string());
        fields.push("    pub drop_down_count: i32,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "items",
                "selected_items",
                "text_hint",
                "drop_down_count",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct MultiSelect {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let items = match node.props.get("items") {
            Some(v) => util::extract_string_items(v),
            None => Vec::new(),
        };
        let items_quoted: Vec<String> = items.iter().map(|it| crate::literal(it)).collect();
        let items_slice = items_quoted.join(", ");

        let selected_items = match node.props.get("selected_items") {
            Some(v) => util::extract_string_items(v),
            None => Vec::new(),
        };
        let sel_quoted: Vec<String> = selected_items.iter().map(|it| crate::literal(it)).collect();
        let sel_slice = sel_quoted.join(", ");

        let hint = match node.props.get("text_hint") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "Select…",
        };
        let count = match node.props.get("drop_down_count") {
            Some(Value::Int(i)) => *i as i32,
            _ => 6,
        };

        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("items: Strings::from_slice(&[{items_slice}])"),
            format!("selected_items: Strings::from_slice(&[{sel_slice}])"),
            format!("text_hint: {}.to_string()", crate::literal(hint)),
            format!("drop_down_count: {count}"),
        ];
        let desc = crawrad_registry::components::multi_select::multi_select();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "items",
                "selected_items",
                "text_hint",
                "drop_down_count",
            ],
        ));
        let body = inits.join(", ");
        format!("MultiSelect {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = String::new();
        let on_change_call = match layout::event_handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };
        code.push_str(&format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_clicked(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             {on_change_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ));
        Ok(code)
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
        let selected_items = match node.props.get("selected_items") {
            Some(v) => util::extract_string_items(v),
            None => Vec::new(),
        };
        let hint = match node.props.get("text_hint") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "Select…",
        };
        let (display_text, text_color) = if selected_items.is_empty() {
            (hint.to_string(), "#888888")
        } else {
            (selected_items.join(", "), "#1c1b1f")
        };
        let display_esc = util::escape_slint_string(&display_text);
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #cccccc;\n");
        out.push_str("            border-radius: 4px;\n");
        out.push_str("            background: #ffffff;\n");
        out.push_str("            clip: true;\n");
        if !vis {
            out.push_str("            visible: false;\n");
        }
        out.push_str("            HorizontalLayout {\n");
        out.push_str("                padding-left: 8px;\n");
        out.push_str("                padding-right: 8px;\n");
        out.push_str("                spacing: 4px;\n");
        out.push_str("                alignment: center;\n");
        out.push_str(&format!("                {name}_summary := Text {{\n"));
        out.push_str("                    horizontal-stretch: 1;\n");
        out.push_str(&format!("                    text: \"{display_esc}\";\n"));
        out.push_str(&format!("                    color: {text_color};\n"));
        out.push_str("                    font-size: 13px;\n");
        out.push_str("                    vertical-alignment: center;\n");
        out.push_str("                    overflow: elide;\n");
        out.push_str("                }\n");
        out.push_str("                Text {\n");
        out.push_str("                    text: \"▼\";\n");
        out.push_str("                    font-size: 10px;\n");
        out.push_str("                    color: #666666;\n");
        out.push_str("                    vertical-alignment: center;\n");
        out.push_str("                }\n");
        out.push_str("            }\n");
        out.push_str("            TouchArea {\n");
        if !en {
            out.push_str("                enabled: false;\n");
        }
        out.push_str(&format!(
            "                clicked => {{ root.{name}_clicked(); }}\n"
        ));
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_clicked();")]
    }
}
