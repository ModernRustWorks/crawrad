//! Slint component implementation for `Chips`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ChipsCodegen;

impl SlintComponentCodegen for ChipsCodegen {
    fn class(&self) -> &'static str {
        "Chips"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::chips::chips();
        let mut fields = util::common_control_fields();
        fields.push("    pub items: Strings,".to_string());
        fields.push("    pub item_index: i64,".to_string());
        fields.push("    pub removable: bool,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "items", "item_index", "removable"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Chips {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::chips::chips();
        let (items_slice, _, _) = util::extract_items_and_selected(node);
        let item_idx = match node.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let removable = matches!(node.props.get("removable"), Some(Value::Bool(true)));
        let mut inits = util::common_control_inits(node);
        inits.push(format!("items: Strings::from_slice(&[{items_slice}])"));
        inits.push(format!("item_index: {item_idx}"));
        inits.push(format!("removable: {removable}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "items", "item_index", "removable"],
        ));
        let body = inits.join(", ");
        format!("Chips {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // chips action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // chips action call
        let mut calls = String::new();
        // chips click & change
        for ev in ["on_click", "on_change"] {
            if let Some(h) = layout::event_handler(node, ev) {
                let h_name = identificador(&h)?;
                calls.push_str(&format!("                    f.form.{h_name}();\n"));
            }
        }
        let mut out = format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_selected(move |idx| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.item_index = idx as i64;\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        );
        let removable = matches!(node.props.get("removable"), Some(Value::Bool(true)));
        if removable {
            let mut remove_calls = String::new();
            if let Some(h) = layout::event_handler(node, "on_change") {
                let h_name = identificador(&h)?;
                remove_calls.push_str(&format!("                    f.form.{h_name}();\n"));
            }
            out.push_str(&format!(
                "        {{\n\
                 \x20           let form = app.clone();\n\
                 \x20           window.on_{field}_removed(move |idx| {{\n\
                 \x20               if let Ok(mut f) = form.lock() {{\n\
                 \x20                   let uidx = idx as usize;\n\
                 \x20                   if uidx < f.form.{field}.items.len() {{\n\
                 \x20                       f.form.{field}.items.remove(uidx);\n\
                 \x20                       if f.form.{field}.item_index == idx as i64 {{\n\
                 \x20                           f.form.{field}.item_index = -1;\n\
                 \x20                       }} else if f.form.{field}.item_index > idx as i64 {{\n\
                 \x20                           f.form.{field}.item_index -= 1;\n\
                 \x20                       }}\n\
                 {remove_calls}\
                 \x20                   }}\n\
                 \x20               }}\n\
                 \x20           }});\n\
                 \x20       }}\n"
            ));
        }
        Ok(out)
    }

    fn emit_slint(
        &self,
        // chips doc
        _doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        // chips bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // chips id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let (_, _, mut raw_items) = util::extract_items_and_selected(node);
        if raw_items.is_empty()
            && let Some(p) = reg
                .get("Chips")
                .and_then(|desc| desc.props.iter().find(|p| p.key == "items"))
        {
            raw_items = util::extract_string_items(&p.default);
        }
        let item_idx = match node.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let removable = matches!(node.props.get("removable"), Some(Value::Bool(true)));

        let items_escaped: Vec<String> = raw_items
            .iter()
            .map(|s| format!("\"{}\"", util::escape_slint_string(s)))
            .collect();
        let items_lit = items_escaped.join(", ");
        let br_chip = util::border_radius_px(node, 12.0);

        let mut out = util::emit_horizontal_layout_header(node, x, y, w, h)?;
        out.push_str("            spacing: 6px;\n");
        out.push_str("            alignment: start;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!(
            "            for item[index] in [{items_lit}]: Rectangle {{\n"
        ));
        out.push_str(&format!("                border-radius: {br_chip};\n"));
        out.push_str(&format!(
            "                background: index == {item_idx} ? #3a598c : #e0e0e0;\n"
        ));
        out.push_str("                border-width: 1px;\n");
        out.push_str("                border-color: #cccccc;\n");
        out.push_str("                horizontal-stretch: 0;\n");
        out.push_str("                HorizontalLayout {\n");
        out.push_str("                    padding-left: 8px;\n");
        out.push_str("                    padding-right: 8px;\n");
        out.push_str("                    spacing: 4px;\n");
        out.push_str("                    alignment: center;\n");
        out.push_str("                    Text {\n");
        out.push_str("                        text: item;\n");
        out.push_str(&format!(
            "                        color: index == {item_idx} ? #ffffff : #222222;\n"
        ));
        out.push_str("                        font-size: 11px;\n");
        out.push_str("                        vertical-alignment: center;\n");
        out.push_str("                    }\n");
        if removable {
            out.push_str("                    Text {\n");
            out.push_str("                        text: \"×\";\n");
            out.push_str("                        color: #888888;\n");
            out.push_str("                        font-size: 12px;\n");
            out.push_str("                        vertical-alignment: center;\n");
            out.push_str("                    }\n");
        }
        out.push_str("                }\n");
        out.push_str("                TouchArea {\n");
        out.push_str(&format!("                    enabled: {en};\n"));
        out.push_str(&format!(
            "                    clicked => {{ root.{name}_selected(index); }}\n"
        ));
        out.push_str("                }\n");
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        let mut cbs = vec![format!("callback {name}_selected(int);")];
        let removable = matches!(node.props.get("removable"), Some(Value::Bool(true)));
        if removable {
            cbs.push(format!("callback {name}_removed(int);"));
        }
        cbs
    }
}
