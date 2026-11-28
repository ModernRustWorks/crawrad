//! Slint component implementation for `ListBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ListBoxCodegen;

impl SlintComponentCodegen for ListBoxCodegen {
    fn class(&self) -> &'static str {
        "ListBox"
    }

    fn struct_definition(&self) -> String {
        struct_definition()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        // target doc
        _doc: &Document,
        node: &Node,
        // field name
        field: &str,
        // emitted nodes
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(node, field)
    }

    fn emit_slint(
        &self,
        // target doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        // target node
        node: &Node,
        // coords
        x: f32,
        y: f32,
        // dimensions
        w: f32,
        // height
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(node, x, y, w, h)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_current_item_changed(int);")]
    }

    fn std_widgets(&self) -> &'static [&'static str] {
        &["StandardListView"]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::list_box();
    let mut fields = vec![
        util::FIELD_VISIBLE.to_string(),
        util::FIELD_ENABLED.to_string(),
        util::FIELD_SELECTED.to_string(),
        util::FIELD_SELECTION.to_string(),
        util::FIELD_ITEMS.to_string(),
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
    let (items_slice, selected, _) = util::extract_items_and_selected(node);
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("selected: {selected}"),
        "selection: Vec::new()".to_string(),
        format!("items: Strings::from_slice(&[{items_slice}])"),
    ];
    let desc = crawrad_registry::components::list_box();
    let extra = util::catalogue_inits(node, &desc, &["visible", "enabled", "items", "item_index"]);
    inits.extend(extra);
    let body = inits.join(", ");
    format!("ListBox {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let mut calls = String::new();
    // list_box click & change
    for ev in ["on_click", "on_change"] {
        if let Some(h) = layout::event_handler(node, ev) {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
    }
    code.push_str(&format!(
        "        {{\n\
         \x20           let form = app.clone();\n\
         \x20           window.on_{field}_current_item_changed(move |idx| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.selected = if idx >= 0 {{ Some(idx as usize) }} else {{ None }};\n\
         {calls}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
    // return listbox item changed callback wiring
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let selected_idx = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => *i,
        _ => -1,
    };
    let en = layout::is_enabled(node);
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := StandardListView {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    if selected_idx >= 0 {
        out.push_str(&format!("            current-item: {selected_idx};\n"));
    }
    if !en {
        out.push_str("            enabled: false;\n");
    }
    out.push_str(&format!(
        "            current-item-changed(idx) => {{ root.{name}_current_item_changed(idx); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
