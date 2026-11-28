//! Slint component implementation for `DateTimePicker`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct DateTimePickerCodegen;

impl SlintComponentCodegen for DateTimePickerCodegen {
    fn class(&self) -> &'static str {
        "DateTimePicker"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::date_time_picker::date_time_picker();
        let mut fields = util::common_control_fields();
        fields.push("    pub kind: String,".to_string());
        fields.push("    pub date_time: String,".to_string());
        fields.push("    pub min_date: String,".to_string());
        fields.push("    pub max_date: String,".to_string());
        fields.push("    pub shown: String,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "kind",
                "date_time",
                "min_date",
                "max_date",
                "shown",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct DateTimePicker {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let kind = match node.props.get("kind") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "dtkDate",
        };
        let date_time = match node.props.get("date_time") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "2000-01-01",
        };
        let min_date = match node.props.get("min_date") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let max_date = match node.props.get("max_date") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let shown = crawrad_ir::date_time::shown_of(node);
        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("kind: {}.to_string()", crate::literal(kind)),
            format!("date_time: {}.to_string()", crate::literal(date_time)),
            format!("min_date: {}.to_string()", crate::literal(min_date)),
            format!("max_date: {}.to_string()", crate::literal(max_date)),
            format!("shown: {}.to_string()", crate::literal(&shown)),
        ];
        let desc = crawrad_registry::components::date_time_picker::date_time_picker();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            // date_time_picker inits
            &[
                "visible",
                "enabled",
                "kind",
                "date_time",
                "min_date",
                "max_date",
                "shown",
            ],
        ));
        let body = inits.join(", ");
        format!("DateTimePicker {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        // date_time_picker action target
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // date_time_picker on_change handler
        let on_change_call = util::format_handler_call(node, "on_change")?;
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_edited(move |val| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.shown = val.to_string();\n\
             \x20                   f.form.{field}.date_time = val.to_string();\n\
             {on_change_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // date_time_picker document reference
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // date_time_picker layout bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // date_time_picker component name
        let name = identificador(node.name())?;
        let shown = crawrad_ir::date_time::shown_of(node);
        let shown_esc = util::escape_slint_string(&shown);
        let kind = match node.props.get("kind") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "dtkDate",
        };
        let icon = if kind == "dtkTime" { "🕒" } else { "📅" };
        let en = layout::is_enabled(node);

        let mut out = util::emit_container_header(node, x, y, w, h, 4.0, false)?;
        out.push_str("            HorizontalLayout {\n");
        out.push_str("                padding-left: 6px;\n");
        out.push_str("                padding-right: 6px;\n");
        out.push_str("                spacing: 4px;\n");
        out.push_str("                alignment: center;\n");
        out.push_str(&format!("                {name}_input := LineEdit {{\n"));
        out.push_str("                    horizontal-stretch: 1;\n");
        out.push_str(&format!("                    text: \"{shown_esc}\";\n"));
        if !en {
            out.push_str("                    enabled: false;\n");
        }
        out.push_str(&format!(
            "                    edited(val) => {{ root.{name}_edited(val); }}\n"
        ));
        out.push_str("                }\n");
        out.push_str("                Text {\n");
        out.push_str(&format!("                    text: \"{icon}\";\n"));
        out.push_str("                    font-size: 13px;\n");
        out.push_str("                    vertical-alignment: center;\n");
        out.push_str("                }\n");
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_edited(string);")]
    }
}
