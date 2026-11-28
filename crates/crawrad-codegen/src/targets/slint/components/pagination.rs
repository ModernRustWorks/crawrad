//! Slint component implementation for `Pagination`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct PaginationCodegen;

impl SlintComponentCodegen for PaginationCodegen {
    fn class(&self) -> &'static str {
        "Pagination"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::pagination::pagination();
        let mut fields = util::common_control_fields();
        fields.push("    pub page_count: i64,".to_string());
        fields.push("    pub page_index: i64,".to_string());
        fields.push("    pub max_buttons: i64,".to_string());
        fields.push("    pub show_arrows: bool,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "page_count",
                "page_index",
                "max_buttons",
                "show_arrows",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Pagination {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::pagination::pagination();
        let page_count = match node.props.get("page_count") {
            Some(Value::Int(i)) => *i,
            _ => 10,
        };
        let page_index = match node.props.get("page_index") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        };
        let max_buttons = match node.props.get("max_buttons") {
            Some(Value::Int(i)) => *i,
            _ => 7,
        };
        let show_arrows = !matches!(node.props.get("show_arrows"), Some(Value::Bool(false)));

        let mut inits = util::common_control_inits(node);
        inits.push(format!("page_count: {page_count}"));
        inits.push(format!("page_index: {page_index}"));
        inits.push(format!("max_buttons: {max_buttons}"));
        inits.push(format!("show_arrows: {show_arrows}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &[
                "visible",
                "enabled",
                "page_count",
                "page_index",
                "max_buttons",
                "show_arrows",
            ],
        ));
        let body = inits.join(", ");
        format!("Pagination {{ {body} }}")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut calls = String::new();
        if let Some(h) = layout::event_handler(node, "on_change") {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_page_changed(move |idx| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.page_index = idx as i64;\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
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
        let page_count = match node.props.get("page_count") {
            Some(Value::Int(i)) => *i,
            _ => 10,
        };
        let page_index = match node.props.get("page_index") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        };
        let max_buttons = match node.props.get("max_buttons") {
            Some(Value::Int(i)) => *i,
            _ => 7,
        };
        let show_arrows = !matches!(node.props.get("show_arrows"), Some(Value::Bool(false)));

        let mut out = String::new();
        out.push_str(&format!("        {name} := HorizontalLayout {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            spacing: 4px;\n");
        out.push_str("            alignment: center;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));

        let prev_page = (page_index - 1).max(0);
        let next_page = (page_index + 1).min(page_count - 1);

        if show_arrows {
            out.push_str("            Rectangle {\n");
            out.push_str("                width: 24px;\n");
            out.push_str(&format!("                height: {h:.1}px;\n"));
            out.push_str("                border-radius: 4px;\n");
            out.push_str("                background: #f0f0f0;\n");
            out.push_str("                border-width: 1px;\n");
            out.push_str("                border-color: #d0d0d0;\n");
            out.push_str("                Text { text: \"‹\"; font-size: 14px; horizontal-alignment: center; vertical-alignment: center; }\n");
            out.push_str("                TouchArea {\n");
            out.push_str(&format!(
                "                    clicked => {{ root.{name}_page_changed({prev_page}); }}\n"
            ));
            out.push_str("                }\n");
            out.push_str("            }\n");
        }

        let buttons_to_show = page_count.min(max_buttons);
        for p in 0..buttons_to_show {
            let active = p == page_index;
            let bg = if active { "#3a598c" } else { "#f0f0f0" };
            let fg = if active { "#ffffff" } else { "#333333" };
            let label = p + 1;
            out.push_str("            Rectangle {\n");
            out.push_str("                width: 24px;\n");
            out.push_str(&format!("                height: {h:.1}px;\n"));
            out.push_str("                border-radius: 4px;\n");
            out.push_str(&format!("                background: {bg};\n"));
            out.push_str("                border-width: 1px;\n");
            out.push_str("                border-color: #d0d0d0;\n");
            out.push_str(&format!(
                "                Text {{ text: \"{label}\"; color: {fg}; font-size: 12px; horizontal-alignment: center; vertical-alignment: center; }}\n"
            ));
            out.push_str("                TouchArea {\n");
            out.push_str(&format!(
                "                    clicked => {{ root.{name}_page_changed({p}); }}\n"
            ));
            out.push_str("                }\n");
            out.push_str("            }\n");
        }

        if show_arrows {
            out.push_str("            Rectangle {\n");
            out.push_str("                width: 24px;\n");
            out.push_str(&format!("                height: {h:.1}px;\n"));
            out.push_str("                border-radius: 4px;\n");
            out.push_str("                background: #f0f0f0;\n");
            out.push_str("                border-width: 1px;\n");
            out.push_str("                border-color: #d0d0d0;\n");
            out.push_str("                Text { text: \"›\"; font-size: 14px; horizontal-alignment: center; vertical-alignment: center; }\n");
            out.push_str("                TouchArea {\n");
            out.push_str(&format!(
                "                    clicked => {{ root.{name}_page_changed({next_page}); }}\n"
            ));
            out.push_str("                }\n");
            out.push_str("            }\n");
        }

        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_page_changed(int);")]
    }
}
