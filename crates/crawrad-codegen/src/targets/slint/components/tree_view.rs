//! Slint component implementation for `TreeView`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Item, Node, Value};

pub struct TreeViewCodegen;

impl SlintComponentCodegen for TreeViewCodegen {
    fn class(&self) -> &'static str {
        "TreeView"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::tree_view::tree_view();
        let mut fields = vec![
            util::FIELD_VISIBLE.to_string(),
            util::FIELD_ENABLED.to_string(),
            util::FIELD_SELECTED.to_string(),
            "    pub items: Vec<crawrad_runtime::tree::TreeNode>,".to_string(),
            "    pub collapsed: std::collections::BTreeSet<usize>,".to_string(),
        ];
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "items", "item_index", "images"],
        ));
        let body = fields.join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct TreeView {{\n\
             {body}\n\
             }}\n\
             \n\
             impl TreeView {{\n\
             \x20   pub fn is_collapsed(&self, idx: usize) -> bool {{\n\
             \x20       self.collapsed.contains(&idx)\n\
             \x20   }}\n\
             \x20   \n\
             \x20   pub fn toggle_collapse(&mut self, idx: usize) {{\n\
             \x20       if self.collapsed.contains(&idx) {{\n\
             \x20           self.collapsed.remove(&idx);\n\
             \x20       }} else {{\n\
             \x20           self.collapsed.insert(idx);\n\
             \x20       }}\n\
             \x20   }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let items = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
        };
        let items_literal: Vec<String> = items
            .iter()
            .map(|it| crate::targets::shared::tree_node_literal(it, &[]))
            .collect();
        let items_vec = format!("vec![{}]", items_literal.join(", "));
        let selected = match node.props.get("item_index") {
            Some(Value::Int(i)) if *i >= 0 => format!("Some({i})"),
            _ => "None".to_string(),
        };
        let collapsed_indices = crate::targets::shared::collapsed_preorder(&items);
        let collapsed_init = if collapsed_indices.is_empty() {
            "collapsed: std::collections::BTreeSet::new()".to_string()
        } else {
            let list: Vec<String> = collapsed_indices.iter().map(|i| i.to_string()).collect();
            format!(
                "collapsed: std::collections::BTreeSet::from([{}])",
                list.join(", ")
            )
        };
        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("selected: {selected}"),
            format!("items: {items_vec}"),
            collapsed_init,
        ];
        let desc = crawrad_registry::components::tree_view::tree_view();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "items", "item_index", "images"],
        ));
        let body = inits.join(", ");
        format!("TreeView {{ {body} }}")
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
        let on_click_call = match layout::event_handler(node, "on_click") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };
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
             \x20           window.on_{field}_node_clicked(move |idx| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.selected = Some(idx as usize);\n\
             \x20                   f.form.{field}.toggle_collapse(idx as usize);\n\
             {on_click_call}\
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
        let items = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
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

        for (i, it) in items.iter().enumerate() {
            let it_esc = util::escape_slint_string(&it.text);
            let has_children = !it.children.is_empty();
            let arrow = if has_children { "▶ " } else { "  " };
            out.push_str(&format!(
                "                {name}_node_{i} := Rectangle {{\n"
            ));
            out.push_str("                    height: 20px;\n");
            out.push_str("                    HorizontalLayout {\n");
            out.push_str("                        padding-left: 4px;\n");
            out.push_str("                        spacing: 4px;\n");
            out.push_str("                        alignment: center;\n");
            out.push_str("                        Text {\n");
            out.push_str(&format!("                            text: \"{arrow}\";\n"));
            out.push_str("                            font-size: 11px;\n");
            out.push_str("                            vertical-alignment: center;\n");
            out.push_str("                        }\n");
            out.push_str("                        Text {\n");
            out.push_str(&format!(
                "                            text: \"{it_esc}\";\n"
            ));
            out.push_str("                            font-size: 13px;\n");
            out.push_str("                            vertical-alignment: center;\n");
            out.push_str("                        }\n");
            out.push_str("                    }\n");
            out.push_str("                    TouchArea {\n");
            if !en {
                out.push_str("                        enabled: false;\n");
            }
            out.push_str(&format!(
                "                        clicked => {{ root.{name}_node_clicked({i}); }}\n"
            ));
            out.push_str("                    }\n");
            out.push_str("                }\n");
        }

        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_node_clicked(int);")]
    }
}
