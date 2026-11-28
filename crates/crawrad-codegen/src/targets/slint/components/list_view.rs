//! Slint component implementation for `ListView` and `ListColumn`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Item, Node, Value};

pub struct ListViewCodegen;

impl SlintComponentCodegen for ListViewCodegen {
    fn class(&self) -> &'static str {
        "ListView"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::list_view::list_view();
        let mut fields = util::common_control_fields();
        fields.push("    pub item_index: i64,".to_string());
        fields.push("    pub items: Vec<ListViewItem>,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "item_index", "items"],
        ));
        let body = fields.join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct ListView {{\n{body}\n}}\n\n\
             #[derive(Clone, Debug, Default, PartialEq, Eq)]\n\
             pub struct ListViewItem {{\n\
             \x20   pub text: String,\n\
             \x20   pub sub_items: Vec<String>,\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let item_index = match node.props.get("item_index") {
            Some(Value::Int(i)) => *i,
            _ => -1,
        };
        let itens = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
        };
        let items_init = emit_items_init(&itens);
        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("item_index: {item_index}"),
            format!("items: {items_init}"),
        ];
        let desc = crawrad_registry::components::list_view::list_view();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "item_index", "items"],
        ));
        let body = inits.join(", ");
        format!("ListView {{ {body} }}")
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
        let on_dbl_click_call = match layout::event_handler(node, "on_dbl_click") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };

        code.push_str(&format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_selected(move |idx| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.item_index = idx as i64;\n\
             {on_click_call}\
             {on_change_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ));

        if !on_dbl_click_call.is_empty() {
            code.push_str(&format!(
                "        {{\n\
                 \x20           let form = app.clone();\n\
                 \x20           window.on_{field}_double_clicked(move |idx| {{\n\
                 \x20               if let Ok(mut f) = form.lock() {{\n\
                 \x20                   f.form.{field}.item_index = idx as i64;\n\
                 {on_dbl_click_call}\
                 \x20               }}\n\
                 \x20           }});\n\
                 \x20       }}\n"
            ));
        }

        Ok(code)
    }

    fn emit_slint(
        &self,
        doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let name = identificador(node.name())?;
        let columns = columns_of(doc, node);
        let items = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
        };
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let show_headers = matches!(
            node.props.get("show_column_headers"),
            None | Some(Value::Bool(true))
        );

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

        // Column headers
        if show_headers && !columns.is_empty() {
            out.push_str(&format!("                {name}_header := Rectangle {{\n"));
            out.push_str("                    height: 24px;\n");
            out.push_str("                    background: #f0f0f0;\n");
            out.push_str("                    border-width: 1px;\n");
            out.push_str("                    border-color: #dddddd;\n");
            out.push_str("                    HorizontalLayout {\n");
            out.push_str("                        padding-left: 6px;\n");
            out.push_str("                        padding-right: 6px;\n");
            out.push_str("                        spacing: 8px;\n");
            for col in &columns {
                let col_text = util::escape_slint_string(&col.text);
                let col_w = col.width;
                out.push_str("                        Text {\n");
                out.push_str(&format!("                            width: {col_w}px;\n"));
                out.push_str(&format!(
                    "                            text: \"{col_text}\";\n"
                ));
                out.push_str("                            font-weight: 700;\n");
                out.push_str("                            font-size: 12px;\n");
                out.push_str("                            vertical-alignment: center;\n");
                out.push_str("                        }\n");
            }
            out.push_str("                    }\n");
            out.push_str("                }\n");
        }

        // Rows
        for (i, it) in items.iter().enumerate() {
            let it_esc = util::escape_slint_string(&it.text);
            out.push_str(&format!("                {name}_row_{i} := Rectangle {{\n"));
            out.push_str("                    height: 22px;\n");
            out.push_str("                    HorizontalLayout {\n");
            out.push_str("                        padding-left: 6px;\n");
            out.push_str("                        padding-right: 6px;\n");
            out.push_str("                        spacing: 8px;\n");
            out.push_str("                        Text {\n");
            if !columns.is_empty() {
                let w0 = columns[0].width;
                out.push_str(&format!("                            width: {w0}px;\n"));
            }
            out.push_str(&format!(
                "                            text: \"{it_esc}\";\n"
            ));
            out.push_str("                            font-size: 12px;\n");
            out.push_str("                            vertical-alignment: center;\n");
            out.push_str("                        }\n");
            for (sub_i, sub) in it.children.iter().enumerate() {
                let sub_esc = util::escape_slint_string(&sub.text);
                out.push_str("                        Text {\n");
                if sub_i + 1 < columns.len() {
                    let w_col = columns[sub_i + 1].width;
                    out.push_str(&format!("                            width: {w_col}px;\n"));
                }
                out.push_str(&format!(
                    "                            text: \"{sub_esc}\";\n"
                ));
                out.push_str("                            font-size: 12px;\n");
                out.push_str("                            vertical-alignment: center;\n");
                out.push_str("                        }\n");
            }
            out.push_str("                    }\n");
            out.push_str("                    TouchArea {\n");
            if !en {
                out.push_str("                        enabled: false;\n");
            }
            out.push_str(&format!(
                "                        clicked => {{ root.{name}_selected({i}); }}\n"
            ));
            out.push_str(&format!(
                "                        double-clicked => {{ root.{name}_double_clicked({i}); }}\n"
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
        vec![
            format!("callback {name}_selected(int);"),
            format!("callback {name}_double_clicked(int);"),
        ]
    }
}

pub struct ListColumnCodegen;

impl SlintComponentCodegen for ListColumnCodegen {
    fn class(&self) -> &'static str {
        "ListColumn"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::list_view::list_column();
        let mut fields = vec![
            "    pub text: String,".to_string(),
            "    pub width: i64,".to_string(),
            "    pub alignment: String,".to_string(),
        ];
        fields.extend(util::catalogue_fields(
            &desc,
            &["text", "width", "alignment"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ListColumn {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let text = layout::text(node);
        let width = match node.props.get("width") {
            Some(Value::Int(i)) => *i,
            _ => 100,
        };
        let alignment = match node.props.get("alignment") {
            Some(Value::Str(s)) | Some(Value::Enum(s)) => s.as_str(),
            _ => "taLeftJustify",
        };
        let mut inits = vec![
            format!("text: {}.to_string()", crate::literal(&text)),
            format!("width: {width}"),
            format!("alignment: {}.to_string()", crate::literal(alignment)),
        ];
        let desc = crawrad_registry::components::list_view::list_column();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["text", "width", "alignment"],
        ));
        let body = inits.join(", ");
        format!("ListColumn {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_slint(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }
}

pub struct ColumnInfo {
    pub text: String,
    pub width: i64,
}

pub fn columns_of(doc: &Document, node: &Node) -> Vec<ColumnInfo> {
    node.children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "ListColumn")
        .map(|c| {
            let text = layout::text(c);
            let width = match c.props.get("width") {
                Some(Value::Int(i)) => *i,
                _ => 100,
            };
            ColumnInfo { text, width }
        })
        .collect()
}

pub fn emit_items_init(itens: &[Item]) -> String {
    if itens.is_empty() {
        return "Vec::new()".to_string();
    }
    let mut out = String::from("vec![\n");
    for i in itens {
        let subs: Vec<String> = i
            .children
            .iter()
            .map(|c| format!("{}.to_string()", crate::literal(&c.text)))
            .collect();
        out.push_str(&format!(
            "                ListViewItem {{ text: {}.to_string(), sub_items: vec![{}] }},\n",
            crate::literal(&i.text),
            subs.join(", ")
        ));
    }
    out.push_str("            ]");
    out
}
