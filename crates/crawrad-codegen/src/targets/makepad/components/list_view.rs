//! Makepad component implementation for `ListView` and `ListColumn`.

use super::codegen::{AppState, MakepadComponentCodegen};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Item, Node};

pub struct ListViewCodegen;

impl MakepadComponentCodegen for ListViewCodegen {
    fn class(&self) -> &'static str {
        "ListView"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ListView").expect("ListView desc");
        // Like ListBox: TabOrder stays in the Inspector, neither target honours it yet.
        let fields: Vec<_> = state::fields_of(desc).into_iter().collect();
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        // The ListView first: the field readers take the first struct as the
        // class's own; the row type follows it.
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct ListView {{\n{body}\n}}\n\n\
             /// One row of a `ListView` (#500).\n\
             #[derive(Clone, Debug, Default, PartialEq, Eq)]\n\
             pub struct ListViewItem {{\n\
             \x20   pub text: String,\n\
             \x20   pub sub_items: Vec<String>,\n\
             }}\n\n\
             impl crawrad_runtime_makepad::list_view::AsListViewItem for ListViewItem {{\n\
             \x20   fn text(&self) -> &str {{\n\
             \x20       &self.text\n\
             \x20   }}\n\
             \x20   fn sub_items(&self) -> &[String] {{\n\
             \x20       &self.sub_items\n\
             \x20   }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ListView").expect("ListView desc");
        let itens = match node.props.get("items") {
            Some(v) => Item::list_from(v),
            None => Vec::new(),
        };
        let items_init = emit_items_init(&itens);
        let inits: Vec<_> = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| {
                if n == "items" {
                    (n, items_init.clone())
                } else {
                    (n, v)
                }
            })
            .collect();
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("ListView {{ {body} }}")
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ListView").expect("ListView desc");
        // Like ListBox: TabOrder stays in the Inspector, neither target honours it yet.
        let fields: Vec<_> = state::fields_of(desc).into_iter().collect();
        let mut reads: Vec<String> = fields
            .iter()
            .map(|(f, _)| format!("&self.form.{field}.{f}"))
            .collect();
        let columns = columns_of(doc, node);
        for col in &columns {
            reads.push(format!("&self.form.{col}.text"));
            reads.push(format!("&self.form.{col}.width"));
            reads.push(format!("&self.form.{col}.alignment"));
        }
        let reads_str = reads.join(", ");

        let mut cols_tuple = String::new();
        for col in &columns {
            cols_tuple.push_str(&format!(
                "                (self.form.{col}.text.clone(), self.form.{col}.width as f32, self.form.{col}.alignment.clone()),\n"
            ));
        }

        let forget = crate::targets::shared::forget_selection_past_end(
            &format!("self.form.{field}"),
            crate::targets::shared::Selection::Signed,
        );
        format!(
            "{forget}        let _ = ({reads_str});\n\
             \x20       let list_view = self.ui.crawrad_list_view(id!({field}));\n\
             \x20       let cols: &[(String, f32, String)] = &[\n{cols_tuple}\x20       ];\n\
             \x20       list_view.set_columns(cx, cols);\n\
             \x20       list_view.set_items(cx, &self.form.{field}.items);\n\
             \x20       let sel = if self.form.{field}.item_index >= 0 {{ Some(self.form.{field}.item_index as usize) }} else {{ None }};\n\
             \x20       list_view.set_selected(cx, sel);\n\
             \x20       list_view.set_row_select(cx, self.form.{field}.row_select);\n\
             \x20       list_view.set_grid_lines(cx, self.form.{field}.grid_lines);\n\
             \x20       list_view.set_show_column_headers(cx, self.form.{field}.show_column_headers);\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = String::new();
        let on_click_call = match layout::handler(node, "on_click") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    self.form.{h_name}();\n")
            }
            None => String::new(),
        };
        let on_change_call = match layout::handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    self.form.{h_name}();\n")
            }
            None => String::new(),
        };
        let on_dbl_click_call = match layout::handler(node, "on_dbl_click") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                self.form.{h_name}();\n")
            }
            None => String::new(),
        };

        // `OnSelectItem`: the row that leaves, then the row that arrives
        let select_item = match layout::handler(node, "on_select_item") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!(
                    "            if before_item != _val as i64 {{\n\
                     \x20               if before_item >= 0 {{\n\
                     \x20                   self.form.{h_name}(before_item as usize, false);\n\
                     \x20               }}\n\
                     \x20               self.form.{h_name}(_val, true);\n\
                     \x20           }}\n"
                )
            }
            None => String::new(),
        };
        let before_item = if select_item.is_empty() {
            String::new()
        } else {
            format!("            let before_item = self.form.{field}.item_index;\n")
        };

        // Only an `OnChange` compares with the index from before the click.
        let (change_check, change_dispatch) = if on_change_call.is_empty() {
            (String::new(), String::new())
        } else {
            (
                format!("            let changed = self.form.{field}.item_index != _val as i64;\n"),
                format!("            if changed {{\n{on_change_call}            }}\n"),
            )
        };
        code.push_str(&format!(
            "        if let Some(_val) = self.ui.crawrad_list_view(id!({field})).selected(actions) {{\n\
             {change_check}\
             {before_item}\
             \x20           self.form.{field}.item_index = _val as i64;\n\
             {on_click_call}\
             {select_item}\
             {change_dispatch}\
             \x20       }}\n"
        ));

        if !on_dbl_click_call.is_empty() {
            code.push_str(&format!(
                "        if let Some(_val) = self.ui.crawrad_list_view(id!({field})).double_clicked(actions) {{\n\
                 \x20           self.form.{field}.item_index = _val as i64;\n\
                 {on_dbl_click_call}\
                 \x20       }}\n"
            ));
        }

        if let Some(h) = layout::handler(node, "on_column_click") {
            let h_name = identificador(&h)?;
            code.push_str(&format!(
                "        if let Some(_col) = self.ui.crawrad_list_view(id!({field})).column_clicked(actions) {{\n\
                 \x20           self.form.{h_name}(_col);\n\
                 \x20       }}\n"
            ));
        }

        Ok(code)
    }

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::list_view::CrawradListViewWidgetRefExt;\n"
                .to_string(),
            ..AppState::default()
        }))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        Ok(format!(
            "            {name} = <CrawradListView> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20           }}\n"
        ))
    }
}

fn columns_of(doc: &Document, node: &Node) -> Vec<String> {
    node.children
        .iter()
        .filter_map(|c| doc.node(c))
        .filter(|c| c.class == "ListColumn")
        .filter_map(|c| identificador(c.name()).ok())
        .collect()
}

fn emit_items_init(itens: &[Item]) -> String {
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

pub struct ListColumnCodegen;

impl MakepadComponentCodegen for ListColumnCodegen {
    fn class(&self) -> &'static str {
        "ListColumn"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ListColumn").expect("ListColumn desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ListColumn {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ListColumn").expect("ListColumn desc");
        let body = state::inits_of(node, desc)
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
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

    fn emit_dsl(
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
}
