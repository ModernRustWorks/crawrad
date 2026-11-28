//! Slint component implementation for `CheckListBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node};

pub struct CheckListBoxCodegen;

impl SlintComponentCodegen for CheckListBoxCodegen {
    fn class(&self) -> &'static str {
        "CheckListBox"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::check_list_box::check_list_box();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_ITEMS.to_string());
        fields.push("    pub checked_items: Strings,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "items", "checked_items"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct CheckListBox {{\n{body}\n}}\n")
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

        let checked = match node.props.get("checked_items") {
            Some(v) => util::extract_string_items(v),
            None => Vec::new(),
        };
        let checked_quoted: Vec<String> = checked.iter().map(|it| crate::literal(it)).collect();
        let checked_slice = checked_quoted.join(", ");

        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("items: Strings::from_slice(&[{items_slice}])"),
            format!("checked_items: Strings::from_slice(&[{checked_slice}])"),
        ];
        let desc = crawrad_registry::components::check_list_box::check_list_box();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "items", "checked_items"],
        ));
        let body = inits.join(", ");
        format!("CheckListBox {{ {body} }}")
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
        let on_change_call = match layout::event_handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_toggled(move |_idx, _on| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             {on_change_call}\
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
        let items = match node.props.get("items") {
            Some(v) => util::extract_string_items(v),
            None => Vec::new(),
        };
        let checked_set: std::collections::HashSet<String> = match node.props.get("checked_items") {
            Some(v) => util::extract_string_items(v).into_iter().collect(),
            None => std::collections::HashSet::new(),
        };
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
        out.push_str("            VerticalLayout {\n");
        out.push_str("                padding: 4px;\n");
        out.push_str("                spacing: 2px;\n");
        out.push_str("                alignment: start;\n");
        for (i, item) in items.iter().enumerate() {
            let item_esc = util::escape_slint_string(item);
            let is_checked = checked_set.contains(item);
            out.push_str("                HorizontalLayout {\n");
            out.push_str("                    spacing: 6px;\n");
            out.push_str("                    alignment: center;\n");
            out.push_str(&format!(
                "                    {name}_chk_{i} := CheckBox {{\n"
            ));
            if is_checked {
                out.push_str("                        checked: true;\n");
            }
            if !en {
                out.push_str("                        enabled: false;\n");
            }
            out.push_str(&format!(
                "                        toggled => {{ root.{name}_toggled({i}, self.checked); }}\n"
            ));
            out.push_str("                    }\n");
            out.push_str("                    Text {\n");
            out.push_str(&format!("                        text: \"{item_esc}\";\n"));
            out.push_str("                        font-size: 13px;\n");
            out.push_str("                        vertical-alignment: center;\n");
            out.push_str("                    }\n");
            out.push_str("                }\n");
        }
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_toggled(int, bool);")]
    }
}
