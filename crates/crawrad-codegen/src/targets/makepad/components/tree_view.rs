//! Makepad component implementation for `TreeView`.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util::{self, FIELD_ENABLED, FIELD_SELECTED, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::shared_geometry::TREE_ROW_HEIGHT;
use crawrad_ir::{Document, Item, Node, Value};

pub struct TreeViewCodegen;

impl MakepadComponentCodegen for TreeViewCodegen {
    fn class(&self) -> &'static str {
        "TreeView"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default_with_doc(None, node)
    }

    fn emit_default_doc(&self, doc: &Document, node: &Node) -> String {
        emit_default_with_doc(Some(doc), node)
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        emit_sync(doc, node, field)
    }

    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(doc, node, field, emitted)
    }

    #[crawrad_note::note("targets-makepad-components-tree-view-treeviewcodegen-app-state")]
    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::list::CrawradListBoxWidgetRefExt;\n".to_string(),
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
        emit_dsl(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::tree_view();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        FIELD_SELECTED.to_string(),
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
             pub fn is_collapsed(&self, idx: usize) -> bool {{\n\
                 self.collapsed.contains(&idx)\n\
             }}\n\
             \n\
             pub fn toggle_collapse(&mut self, idx: usize) {{\n\
                 if self.collapsed.contains(&idx) {{\n\
                     self.collapsed.remove(&idx);\n\
                 }} else {{\n\
                     self.collapsed.insert(idx);\n\
                 }}\n\
             }}\n\
         }}\n"
    )
}

pub fn emit_default(node: &Node) -> String {
    emit_default_with_doc(None, node)
}

pub fn emit_default_with_doc(doc: Option<&Document>, node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let items = match node.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let symbols = doc.map_or_else(Vec::new, |d| crate::targets::shared::image_symbols(d, node));
    let items_literal: Vec<String> = items
        .iter()
        .map(|it| crate::targets::shared::tree_node_literal(it, &symbols))
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
    let desc = crawrad_registry::components::tree_view();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "items", "item_index", "images"],
    ));
    let body = inits.join(", ");
    format!("TreeView {{ {body} }}")
}

pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    // the rows are the ListBox's widget, so the look is the list's whole look
    let look = crate::targets::makepad::look::whole_look(field, "crawrad_list_box");
    look + &format!(
        "        let list = self.ui.crawrad_list_box(id!({field}));\n\
         \x20       let rows = crawrad_runtime::tree::visible_rows(&self.form.{field}.items, &self.form.{field}.collapsed);\n\
         \x20       list.set_items(cx, &crawrad_runtime::tree::row_texts(&rows));\n\
         \x20       let indents: Vec<f64> = rows.iter().map(|r| r.level as f64 * 16.0 + 6.0).collect();\n\
         \x20       list.set_indents(cx, &indents);\n\
         \x20       list.set_selected(cx, crawrad_runtime::tree::row_of(&rows, self.form.{field}.selected));\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let on_click_call = match layout::handler(node, "on_click") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("                self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("                self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    // A node with children announces the change before it happens
    let mut announce = String::new();
    for (event, when) in [
        ("on_expanding", "is_collapsed"),
        ("on_collapsing", "!is_collapsed"),
    ] {
        if let Some(h) = layout::handler(node, event) {
            let h = identificador(&h)?;
            let (neg, method) = match when.strip_prefix('!') {
                Some(m) => ("!", m),
                None => ("", when),
            };
            announce.push_str(&format!(
                "                    if {neg}self.form.{field}.{method}(idx) {{\n\
                 \x20                       self.form.{h}();\n\
                 \x20                   }}\n"
            ));
        }
    }
    // `OnExpanded` / `OnCollapsed`: after the fold has happened
    let mut after = String::new();
    for (event, when) in [("on_expanded", "!"), ("on_collapsed", "")] {
        if let Some(h) = layout::handler(node, event) {
            let h = identificador(&h)?;
            after.push_str(&format!(
                "                    if {when}self.form.{field}.is_collapsed(idx) {{\n\
                 \x20                       self.form.{h}();\n\
                 \x20                   }}\n"
            ));
        }
    }
    // `OnChanging` first: setting `allow_change` to false keeps the old row,
    // and then nothing changed for `OnChange` either
    let (take, on_change_call) = match layout::handler(node, "on_changing") {
        Some(h) => {
            let h = identificador(&h)?;
            let change = if on_change_call.is_empty() {
                String::new()
            } else {
                format!(
                    "                if allow_change {{\n\
                     \x20   {on_change_call}\
                     \x20               }}\n"
                )
            };
            (
                format!(
                    "                let mut allow_change = true;\n\
                     \x20               self.form.{h}(&mut allow_change);\n\
                     \x20               if allow_change {{\n\
                     \x20                   self.form.{field}.selected = Some(idx);\n\
                     \x20               }}\n"
                ),
                change,
            )
        }
        None => (
            format!("                self.form.{field}.selected = Some(idx);\n"),
            on_change_call,
        ),
    };

    let fold_block = if announce.is_empty() && after.is_empty() {
        format!("                    self.form.{field}.toggle_collapse(idx);\n")
    } else {
        format!(
            "{announce}\
             \x20                   self.form.{field}.toggle_collapse(idx);\n\
             {after}"
        )
    };

    Ok(format!(
        "        if let Some(row) = self.ui.crawrad_list_box(id!({field})).selected(actions) {{\n\
         \x20           let rows = crawrad_runtime::tree::visible_rows(&self.form.{field}.items, &self.form.{field}.collapsed);\n\
         \x20           if let Some(hit) = rows.get(row) {{\n\
         \x20               let idx = hit.index;\n\
         {take}\
         \x20               if hit.has_children {{\n\
         {fold_block}\
         \x20               }}\n\
         {on_click_call}\
         {on_change_call}\
         \x20           }}\n\
         \x20       }}\n"
    ))
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    // The rows are drawn by `crawrad_runtime_makepad::list::CrawradListBox`,
    // as many as the form holds; the items are not baked here any more. The
    // row height still comes from `shared_geometry`, the file the designer's
    // canvas reads — it was `20.0` here against 17.0 there once, and the
    // designer showed one number of rows and the app another.
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let row = format!("{TREE_ROW_HEIGHT:.1}");
    Ok(format!(
        "            {name} = <CrawradListBox> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20               list = {{ row = {{ height: {row} }} row_selected = {{ height: {row} }} }}\n\
         \x20           }}\n"
    ))
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
