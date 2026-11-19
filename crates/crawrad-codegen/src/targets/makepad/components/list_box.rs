//! Makepad component implementation for `ListBox`.
//!
//! The rows are the form's at run time: the DSL holds a
//! `crawrad_runtime_makepad::list::CrawradListBox`, the sync hands it the items and
//! the selection, and one question in `handle_actions` answers for any row.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util::{
    self, FIELD_ENABLED, FIELD_ITEMS, FIELD_SELECTED, FIELD_SELECTION, FIELD_VISIBLE,
};
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::shared_geometry::LIST_ROW_HEIGHT;
use crawrad_ir::{Document, Item, Node, Value};

pub struct ListBoxCodegen;

impl MakepadComponentCodegen for ListBoxCodegen {
    fn class(&self) -> &'static str {
        "ListBox"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_list_box")
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
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

    #[crawrad_note::note("targets-makepad-components-list-box-listboxcodegen-app-state")]
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
    let desc = crawrad_registry::components::list_box();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        FIELD_SELECTED.to_string(),
        FIELD_SELECTION.to_string(),
        FIELD_ITEMS.to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "items", "item_index"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct ListBox {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let items = match node.props.get("items") {
        Some(v) => Item::list_from(v),
        None => Vec::new(),
    };
    let items_quoted: Vec<String> = items.iter().map(|it| crate::literal(&it.text)).collect();
    let items_slice = items_quoted.join(", ");
    let (selected, selection) = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => (format!("Some({i})"), format!("vec![{i}]")),
        _ => ("None".to_string(), "Vec::new()".to_string()),
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("selected: {selected}"),
        format!("selection: {selection}"),
        format!("items: Strings::from_slice(&[{items_slice}])"),
    ];
    let desc = crawrad_registry::components::list_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "items", "item_index"],
    ));
    let body = inits.join(", ");
    format!("ListBox {{ {body} }}")
}

#[crawrad_note::note("targets-makepad-components-list-box-emit-sync")]
pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.form.{field}"),
        crate::targets::shared::Selection::Optional,
    );
    format!(
        "{forget}        if self.form.{field}.sorted {{\n\
         \x20           crawrad_runtime::list_selection::sort(&mut self.form.{field}.items, &mut self.form.{field}.selected, &mut self.form.{field}.selection);\n\
         \x20       }}\n\
         \x20       crawrad_runtime::list_selection::settle(&mut self.form.{field}.selection, self.form.{field}.selected, self.form.{field}.items.len(), self.form.{field}.multi_select);\n\
         \x20       let list_box = self.ui.crawrad_list_box(id!({field}));\n\
         \x20       list_box.set_items(cx, &self.form.{field}.items);\n\
         \x20       list_box.set_selected(cx, self.form.{field}.selected);\n\
         \x20       list_box.set_selection(cx, &self.form.{field}.selection);\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let on_click_call = match layout::handler(node, "on_click") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
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
    // one question for any row, the ones added at run time too;
    // OnChange only when the row is another one, as on egui
    let change = if on_change_call.is_empty() {
        "            let _ = changed;\n".to_string()
    } else {
        format!("            if changed {{\n{on_change_call}            }}\n")
    };
    code.push_str(&format!(
        "        if let Some(_val) = self.ui.crawrad_list_box(id!({field})).selected(actions) {{\n\
         \x20           let changed = self.form.{field}.selected != Some(_val);\n\
         \x20           let (toggle, extend) = self.ui.crawrad_list_box(id!({field})).keys(actions).unwrap_or_default();\n\
         \x20           crawrad_runtime::list_selection::click(&mut self.form.{field}.selection, self.form.{field}.selected, _val, self.form.{field}.multi_select, toggle, extend);\n\
         \x20           self.form.{field}.selected = Some(_val);\n\
         {on_click_call}\
         {change}\
         \x20       }}\n"
    ));
    if let Some(h) = layout::handler(node, "on_dbl_click") {
        code.push_str(&format!(
            "        if self.ui.crawrad_list_box(id!({field})).double_clicked(actions).is_some() {{\n\
             \x20           self.form.{}();\n\
             \x20       }}\n",
            identificador(&h)?
        ));
    }
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    // the rows are drawn by `crawrad_runtime_makepad::list::CrawradListBox`,
    // as many as the form holds; the items are not baked here any more. The
    // row height still comes from `shared_geometry`, the file the designer's
    // canvas reads — it was `20.0` here against 18.0 there once, and the
    // designer showed one number of rows and the app another.
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    let row = format!("{LIST_ROW_HEIGHT:.1}");
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
