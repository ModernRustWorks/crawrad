//! Slint component implementation for `ComboBox`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node, Value};

pub struct ComboBoxCodegen;

impl SlintComponentCodegen for ComboBoxCodegen {
    fn class(&self) -> &'static str {
        "ComboBox"
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
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        emit_actions(node, field)
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
        emit_slint(node, x, y, w, h)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_selected(int, string);")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::combo_box();
    let mut fields = vec![
        util::FIELD_VISIBLE.to_string(),
        util::FIELD_ENABLED.to_string(),
        util::FIELD_SELECTED.to_string(),
        util::FIELD_ITEMS.to_string(),
        util::FIELD_TEXT.to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "items", "item_index", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct ComboBox {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let items = match node.props.get("items") {
        Some(v) => util::extract_string_items(v),
        None => Vec::new(),
    };
    let items_quoted: Vec<String> = items.iter().map(|it| crate::literal(it)).collect();
    let items_slice = items_quoted.join(", ");
    let selected = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => format!("Some({i})"),
        _ => "None".to_string(),
    };
    let text = match node.props.get("text") {
        Some(Value::Str(s)) if !s.is_empty() => crate::literal(s),
        _ => match node.props.get("item_index") {
            Some(Value::Int(i)) if *i >= 0 && (*i as usize) < items.len() => {
                crate::literal(&items[*i as usize])
            }
            _ => "\"\"".to_string(),
        },
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("selected: {selected}"),
        format!("items: Strings::from_slice(&[{items_slice}])"),
        format!("text: {text}.to_string()"),
    ];
    let desc = crawrad_registry::components::combo_box();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "items", "item_index", "text"],
    ));
    let body = inits.join(", ");
    format!("ComboBox {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let mut calls = String::new();
    for ev in ["on_select", "on_change"] {
        if let Some(h) = layout::event_handler(node, ev) {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
    }
    code.push_str(&format!(
        "        {{\n\
         \x20           let form = app.clone();\n\
         \x20           window.on_{field}_selected(move |idx, val| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.selected = if idx >= 0 {{ Some(idx as usize) }} else {{ None }};\n\
         \x20                   f.form.{field}.text = val.to_string();\n\
         {calls}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let items = match node.props.get("items") {
        Some(v) => util::extract_string_items(v),
        None => Vec::new(),
    };
    let items_esc: Vec<String> = items
        .iter()
        .map(|it| format!("\"{}\"", util::escape_slint_string(it)))
        .collect();
    let items_slice = items_esc.join(", ");
    let selected_idx = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => *i,
        _ => -1,
    };
    let en = layout::is_enabled(node);
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := ComboBox {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            model: [{items_slice}];\n"));
    if selected_idx >= 0 {
        out.push_str(&format!("            current-index: {selected_idx};\n"));
    }
    if !en {
        out.push_str("            enabled: false;\n");
    }
    out.push_str(&format!(
        "            selected(val) => {{ root.{name}_selected(self.current-index, val); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
