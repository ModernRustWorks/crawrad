//! Agnostic utility helpers for Slint component property formatting.

use crate::targets::slint::layout;
use crawrad_ir::{Node, Value};
use crawrad_registry::{ComponentDesc, PropDesc};

pub const FIELD_VISIBLE: &str = "    pub visible: bool,";
pub const FIELD_ENABLED: &str = "    pub enabled: bool,";
pub const FIELD_TEXT: &str = "    pub text: String,";
pub const FIELD_CHECKED: &str = "    pub checked: bool,";
pub const FIELD_SELECTED: &str = "    pub selected: Option<usize>,";
pub const FIELD_SELECTION: &str = "    pub selection: Vec<usize>,";
pub const FIELD_ITEMS: &str = "    pub items: Strings,";
pub const FIELD_LINES: &str = "    pub lines: Strings,";

/// Escapes a string for inclusion in `.slint` markup.
pub fn escape_slint_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Extracts string items from a property value supporting both string lists and item maps.
pub fn extract_string_items(v: &Value) -> Vec<String> {
    match v {
        Value::List(items) => items
            .iter()
            .filter_map(|it| match it {
                Value::Str(s) => Some(s.clone()),
                Value::Map(m) => match m.get("text") {
                    Some(Value::Str(s)) => Some(s.clone()),
                    _ => None,
                },
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

pub fn common_control_fields() -> Vec<String> {
    vec![FIELD_VISIBLE.to_string(), FIELD_ENABLED.to_string()]
}

pub fn common_control_inits(node: &Node) -> Vec<String> {
    vec![init_visible(node), init_enabled(node)]
}

pub fn init_visible(node: &Node) -> String {
    let vis = layout::is_visible(node);
    format!("visible: {vis}")
}

pub fn init_enabled(node: &Node) -> String {
    let en = layout::is_enabled(node);
    format!("enabled: {en}")
}

pub fn init_text(node: &Node) -> String {
    let txt = layout::text(node);
    let txt_lit = crate::literal(&txt);
    format!("text: {txt_lit}.to_string()")
}

pub fn init_checked(node: &Node) -> String {
    let chk = layout::is_checked(node);
    format!("checked: {chk}")
}

pub fn format_field(name: &str, ty: &str) -> String {
    format!("    pub {name}: {ty},")
}

pub fn value_to_rust_literal(val: &Value) -> String {
    match val {
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => format!("{f:.1}"),
        Value::Str(s) | Value::Enum(s) | Value::Ref(s) => {
            let lit = crate::literal(s);
            format!("{lit}.to_string()")
        }
        Value::DateTime(dt) => format!(
            "DateTime::new({}, {}, {}, {}, {}, {})",
            dt.year, dt.month, dt.day, dt.hour, dt.minute, dt.second
        ),
        Value::FilePath(fp) => {
            let lit = crate::literal(&fp.path);
            format!("FilePath::new({lit})")
        }
        Value::Cursor(c) => format!("Cursor::{c:?}"),
        Value::ModalResult(m) => format!("ModalResult::{m:?}"),
        Value::Shortcut(sc) => {
            if sc.is_none() {
                "Shortcut::NONE".to_string()
            } else {
                format!(
                    "Shortcut {{ key: crawrad_runtime::prelude::Key::{:?}, ctrl: {}, alt: {}, shift: {}, meta: {} }}",
                    sc.key, sc.ctrl, sc.alt, sc.shift, sc.meta
                )
            }
        }
        Value::Color(c) => format!("Color::rgba({}, {}, {}, {})", c.r, c.g, c.b, c.a),
        Value::Font(_) => crate::targets::shared::literal_da_peca(val)
            .unwrap_or_else(|| "Font::default()".to_string()),
        Value::Picture(p) => {
            if p.is_empty() {
                if let Some(src) = &p.source {
                    format!("Picture::empty().with_source({})", crate::literal(src))
                } else {
                    "Picture::empty()".to_string()
                }
            } else {
                crate::targets::shared::literal_da_peca(val)
                    .unwrap_or_else(|| "Picture::empty()".to_string())
            }
        }
        _ => "Default::default()".to_string(),
    }
}

fn written_by_hand_or_not_emitted(prop: &PropDesc, by_hand: &[&str]) -> bool {
    matches!(prop.key, "name" | "rect" | "action")
        || by_hand.contains(&prop.key)
        || crate::emission::why_not_set(prop.key, prop.category).is_some()
}

pub fn catalogue_fields(desc: &ComponentDesc, by_hand: &[&str]) -> Vec<String> {
    let mut fields = Vec::new();
    for prop in &desc.props {
        if written_by_hand_or_not_emitted(prop, by_hand) {
            continue;
        }
        let ty = crate::targets::makepad::state::rust_type_of(&prop.default);
        fields.push(format_field(prop.key, ty));
    }
    fields
}

pub fn catalogue_inits(node: &Node, desc: &ComponentDesc, by_hand: &[&str]) -> Vec<String> {
    let mut inits = Vec::new();
    for prop in &desc.props {
        if written_by_hand_or_not_emitted(prop, by_hand) {
            continue;
        }
        let val_str = match node.props.get(prop.key) {
            Some(v) => value_to_rust_literal(v),
            None => value_to_rust_literal(&prop.default),
        };
        inits.push(format!("{}: {val_str}", prop.key));
    }
    inits
}

/// Helper that extracts caption layout properties.
pub fn layout_props_caption(
    node: &Node,
    x: f32,
    y: f32,
) -> Result<(String, String, String), crate::Erro> {
    let name = crate::identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = escape_slint_string(&caption);
    let pos = layout::position(x, y);
    Ok((name, caption_esc, pos))
}

/// Helper that builds struct fields for standard text controls.
pub fn struct_fields_with_text(desc: &ComponentDesc) -> String {
    let mut fields = common_control_fields();
    fields.push(FIELD_TEXT.to_string());
    fields.extend(catalogue_fields(desc, &["visible", "enabled", "text"]));
    fields.join("\n")
}

/// Helper that builds default struct inits for standard text controls.
pub fn struct_inits_with_text(node: &Node, desc: &ComponentDesc) -> String {
    let mut inits = common_control_inits(node);
    inits.push(init_text(node));
    inits.extend(catalogue_inits(node, desc, &["visible", "enabled", "text"]));
    inits.join(", ")
}

/// Helper for standard text control struct definition.
pub fn standard_text_control_struct(class_name: &str, desc: &ComponentDesc) -> String {
    let body = struct_fields_with_text(desc);
    format!("#[derive(Debug, Clone, Default)]\npub struct {class_name} {{\n{body}\n}}\n")
}

/// Helper for standard text control default initialization.
pub fn standard_text_control_default(
    class_name: &str,
    node: &Node,
    desc: &ComponentDesc,
) -> String {
    let body = struct_inits_with_text(node, desc);
    format!("{class_name} {{ {body} }}")
}

/// Helper that emits window click handler wiring.
pub fn emit_click_action(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    if let Some(h) = layout::event_handler(node, "on_click") {
        let h_name = crate::identificador(&h)?;
        code.push_str(&format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_clicked(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{h_name}();\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ));
    }
    Ok(code)
}

/// Helper that emits window click callback definition.
pub fn click_callbacks(node: &Node) -> Vec<String> {
    let name = crate::identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
    if layout::event_handler(node, "on_click").is_some() {
        vec![format!("callback {name}_clicked();")]
    } else {
        Vec::new()
    }
}

/// Helper that builds standard visual component structs with common fields.
pub fn standard_control_struct(class_name: &str, desc: &ComponentDesc) -> String {
    let mut fields = common_control_fields();
    fields.extend(catalogue_fields(desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct {class_name} {{\n{body}\n}}\n")
}

/// Helper that builds standard visual component default inits with common fields.
pub fn standard_control_default(class_name: &str, node: &Node, desc: &ComponentDesc) -> String {
    let mut inits = common_control_inits(node);
    inits.extend(catalogue_inits(node, desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("{class_name} {{ {body} }}")
}

/// Helper for list and item-index extraction.
pub fn extract_items_and_selected(node: &Node) -> (String, String, Vec<String>) {
    let items = match node.props.get("items") {
        Some(v) => extract_string_items(v),
        None => Vec::new(),
    };
    let items_quoted: Vec<String> = items.iter().map(|it| crate::literal(it)).collect();
    let items_slice = items_quoted.join(", ");
    let selected = match node.props.get("item_index") {
        Some(Value::Int(i)) if *i >= 0 => format!("Some({i})"),
        _ => "None".to_string(),
    };
    (items_slice, selected, items)
}

/// Helper for checked-control structs.
pub fn checked_control_struct(class_name: &str, desc: &ComponentDesc) -> String {
    let mut fields = common_control_fields();
    fields.push(FIELD_CHECKED.to_string());
    fields.push(FIELD_TEXT.to_string());
    fields.extend(catalogue_fields(
        desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct {class_name} {{\n{body}\n}}\n")
}

/// Helper for checked-control defaults.
pub fn checked_control_default(class_name: &str, node: &Node, desc: &ComponentDesc) -> String {
    let mut inits = common_control_inits(node);
    inits.push(init_checked(node));
    inits.push(init_text(node));
    inits.extend(catalogue_inits(
        node,
        desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = inits.join(", ");
    format!("{class_name} {{ {body} }}")
}

/// Helper for checked layout properties extraction.
pub fn layout_props_checked(
    node: &Node,
    x: f32,
    y: f32,
) -> Result<(String, String, bool, bool, String), crate::Erro> {
    let name = crate::identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = escape_slint_string(&caption);
    let checked = layout::is_checked(node);
    let en = layout::is_enabled(node);
    let pos = layout::position(x, y);
    Ok((name, caption_esc, checked, en, pos))
}

/// Helper for standard rectangle container slint emission.
pub fn emit_simple_rectangle(
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    style: &str,
) -> Result<String, crate::Erro> {
    let name = crate::identificador(node.name())?;
    let pos = layout::position(x, y);
    Ok(format!(
        "        {name} := Rectangle {{\n            {pos}\n            width: {w:.1}px;\n            height: {h:.1}px;\n{style}        }}\n"
    ))
}

/// Helper for toggled/change event action emission.
pub fn emit_toggle_action(
    node: &Node,
    field: &str,
    cb_name: &str,
    state_field: &str,
) -> Result<String, crate::Erro> {
    let mut calls = String::new();
    for ev in ["on_click", "on_change"] {
        if let Some(h) = layout::event_handler(node, ev) {
            let h_name = crate::identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
    }
    Ok(format!(
        "        {{\n            let form = app.clone();\n            window.on_{field}_{cb_name}(move |val| {{\n                if let Ok(mut f) = form.lock() {{\n                    f.form.{field}.{state_field} = val;\n{calls}                }}\n            }});\n        }}\n"
    ))
}
