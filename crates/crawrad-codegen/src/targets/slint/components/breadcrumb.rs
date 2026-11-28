//! Slint component implementation for `Breadcrumb`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct BreadcrumbCodegen;

impl SlintComponentCodegen for BreadcrumbCodegen {
    fn class(&self) -> &'static str {
        "Breadcrumb"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::breadcrumb::breadcrumb();
        let mut fields = util::common_control_fields();
        fields.push("    pub items: Strings,".to_string());
        fields.push("    pub separator: String,".to_string());
        fields.push("    pub item_index: i64,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "items", "separator", "item_index"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Breadcrumb {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::breadcrumb::breadcrumb();
        let (items_slice, _, _) = util::extract_items_and_selected(node);
        let sep = match node.props.get("separator") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "›",
        };
        let item_idx = match node.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let mut inits = util::common_control_inits(node);
        inits.push(format!("items: Strings::from_slice(&[{items_slice}])"));
        inits.push(format!("separator: {}.to_string()", crate::literal(sep)));
        inits.push(format!("item_index: {item_idx}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "items", "separator", "item_index"],
        ));
        let body = inits.join(", ");
        format!("Breadcrumb {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // breadcrumb action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // breadcrumb action call
        let mut calls = String::new();
        // breadcrumb click & change
        for ev in ["on_click", "on_change"] {
            if let Some(h) = layout::event_handler(node, ev) {
                let h_name = identificador(&h)?;
                calls.push_str(&format!("                    f.form.{h_name}();\n"));
            }
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_clicked(move |idx| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.item_index = idx as i64;\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // breadcrumb doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // breadcrumb bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // breadcrumb id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let (_, _, raw_items) = util::extract_items_and_selected(node);
        let sep = match node.props.get("separator") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "›",
        };
        let sep_esc = util::escape_slint_string(sep);
        let item_idx = match node.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let seg_color = match node.props.get("segment_color") {
            Some(Value::Color(c)) => c.to_hex(),
            _ => "#45464B".to_string(),
        };
        let cur_color = match node.props.get("current_color") {
            Some(Value::Color(c)) => c.to_hex(),
            _ => "#E0612B".to_string(),
        };

        let items_escaped: Vec<String> = raw_items
            .iter()
            .map(|s| format!("\"{}\"", util::escape_slint_string(s)))
            .collect();
        let items_lit = items_escaped.join(", ");
        let count = raw_items.len();

        let mut out = util::emit_horizontal_layout_header(node, x, y, w, h)?;
        out.push_str("            spacing: 4px;\n");
        out.push_str("            alignment: start;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        out.push_str(&format!(
            "            for item[index] in [{items_lit}]: HorizontalLayout {{\n"
        ));
        out.push_str("                spacing: 4px;\n");
        out.push_str("                alignment: center;\n");
        out.push_str("                Text {\n");
        out.push_str("                    text: item;\n");
        out.push_str(&format!(
            "                    color: index == {item_idx} ? {cur_color} : {seg_color};\n"
        ));
        out.push_str("                    font-size: 12px;\n");
        out.push_str("                    vertical-alignment: center;\n");
        out.push_str("                    TouchArea {\n");
        out.push_str(&format!(
            "                        clicked => {{ root.{name}_clicked(index); }}\n"
        ));
        out.push_str("                    }\n");
        out.push_str("                }\n");
        if count > 1 {
            let limit = count.saturating_sub(1);
            out.push_str(&format!("                if index < {limit}: Text {{\n"));
            out.push_str(&format!("                    text: \"{sep_esc}\";\n"));
            out.push_str(&format!("                    color: {seg_color};\n"));
            out.push_str("                    font-size: 12px;\n");
            out.push_str("                    vertical-alignment: center;\n");
            out.push_str("                }\n");
        }
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_clicked(int);")]
    }
}
