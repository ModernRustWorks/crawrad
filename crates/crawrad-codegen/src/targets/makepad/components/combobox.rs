//! Makepad component implementation for `ComboBox`.
//!
//! Its items are the form's, at run time: the sync hands them to the
//! `DropDown` (`crawrad_runtime_makepad::items`), and the generated `App`
//! remembers the list on screen so it is only handed over when it changes.

use super::codegen::{AppState, MakepadComponentCodegen};
use super::util::{self, FIELD_ENABLED, FIELD_ITEMS, FIELD_SELECTED, FIELD_VISIBLE};
use crate::identificador;
use crate::targets::makepad::layout;
use crawrad_ir::{Document, Item, Node, Value};

pub struct ComboBoxCodegen;

impl MakepadComponentCodegen for ComboBoxCodegen {
    fn class(&self) -> &'static str {
        "ComboBox"
    }

    fn whole_look_widget(&self) -> Option<&'static str> {
        Some("crawrad_combo_box")
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

    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, crate::Erro> {
        Ok(Some(AppState {
            uses: "use crawrad_runtime_makepad::combo_box::CrawradComboBoxWidgetRefExt;\n"
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
        emit_dsl(node, x, y, w, h)
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::combo_box();
    let mut fields = vec![
        FIELD_VISIBLE.to_string(),
        FIELD_ENABLED.to_string(),
        FIELD_SELECTED.to_string(),
        FIELD_ITEMS.to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "items", "item_index"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct ComboBox {{\n{body}\n}}\n")
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
    let selected = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => format!("Some({i})"),
        _ => "None".to_string(),
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("selected: {selected}"),
        format!("items: Strings::from_slice(&[{items_slice}])"),
    ];
    let desc = crawrad_registry::components::combo_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "items", "item_index"],
    ));
    let body = inits.join(", ");
    format!("ComboBox {{ {body} }}")
}

#[crawrad_note::note("targets-makepad-components-combobox-emit-sync")]
pub fn emit_sync(_doc: &Document, _node: &Node, field: &str) -> String {
    let forget = crate::targets::shared::forget_selection_past_end(
        &format!("self.form.{field}"),
        crate::targets::shared::Selection::Optional,
    );
    let forget =
        forget + &crate::targets::shared::text_follows_selection(&format!("self.form.{field}"));
    // the items first: a selection set before them would point into the old list
    format!(
        "{forget}        if self.form.{field}.sorted {{\n\
         \x20           crawrad_runtime::list_selection::sort(&mut self.form.{field}.items, &mut self.form.{field}.selected, &mut []);\n\
         \x20       }}\n\
         \x20       {{\n\
         \x20           let combo_box = self.ui.crawrad_combo_box(id!({field}));\n\
         \x20           combo_box.set_editable(cx, self.form.{field}.style == \"DropDown\");\n\
         \x20           combo_box.set_text(cx, &self.form.{field}.text);\n\
         \x20           combo_box.set_items(cx, self.form.{field}.items.as_slice());\n\
         \x20           combo_box.set_selected(cx, self.form.{field}.selected);\n\
         \x20           combo_box.set_drop_down_count(cx, self.form.{field}.drop_down_count);\n\
         \x20       }}\n"
    )
}

pub fn emit_actions(
    _doc: &Document,
    node: &Node,
    field: &str,
    _emitted: &[&Node],
) -> Result<String, crate::Erro> {
    let mut code = String::new();
    // `OnSelect`: the item the user picked, the same one or not
    let on_select_call = match layout::handler(node, "on_select") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    // `OnChange`: only when the choice is a different one, as in the egui
    // target (makepad reports a pick of the same item too)
    let on_change_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!(
                "            if changed {{\n\
                 \x20               self.form.{h_name}();\n\
                 \x20           }}\n"
            )
        }
        None => String::new(),
    };
    let changed = if on_change_call.is_empty() {
        ""
    } else {
        "            let changed = self.form.FIELD.selected != Some(_val) || self.form.FIELD.text != self.form.FIELD.items.get(_val).map(|s| s.as_str()).unwrap_or(\"\");\n"
    }
    .replace("FIELD", field);
    code.push_str(&format!(
        "        if let Some(_val) = self.ui.crawrad_combo_box(id!({field})).selected(actions) {{\n\
         {changed}\
         \x20           self.form.{field}.selected = Some(_val);\n\
         \x20           if let Some(item_text) = self.form.{field}.items.get(_val) {{\n\
         \x20               self.form.{field}.text = item_text.clone();\n\
         \x20           }}\n\
         {on_select_call}\
         {on_change_call}\
         \x20       }}\n"
    ));
    let on_change_text_call = match layout::handler(node, "on_change") {
        Some(h) => {
            let h_name = identificador(&h)?;
            format!("            self.form.{h_name}();\n")
        }
        None => String::new(),
    };
    code.push_str(&format!(
        "        if let Some(_val) = self.ui.crawrad_combo_box(id!({field})).text_changed(actions) {{\n\
         \x20           self.form.{field}.text = _val.clone();\n\
         \x20           self.form.{field}.selected = self.form.{field}.items.iter().position(|it| it == &_val);\n\
         {on_change_text_call}\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let pos = layout::posicao(x, y);
    let name = identificador(node.name())?;
    Ok(format!(
        "            {name} = <CrawradComboBox> {{\n\
         \x20               {pos}\n\
         \x20               width: {w}, height: {h}\n\
         \x20           }}\n"
    ))
}

pub fn emitir_dsl(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    emit_dsl(node, x, y, w, h)
}
