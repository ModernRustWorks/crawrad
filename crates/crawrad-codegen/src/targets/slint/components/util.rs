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
