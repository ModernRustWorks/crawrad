//! Agnostic utility helpers for Makepad component property formatting.
//!
//! This module contains NO knowledge of specific component classes.
//! It only provides pure, general formatting helpers for IR values, types,
//! and reusable common control fields (visible, enabled, text, etc.).

use crate::targets::makepad::layout;
use crawrad_ir::{Node, Value};
use crawrad_registry::{ComponentDesc, PropDesc};

pub const FIELD_VISIBLE: &str = "    pub visible: bool,";
pub const FIELD_ENABLED: &str = "    pub enabled: bool,";
pub const FIELD_TEXT: &str = "    pub text: String,";
pub const FIELD_CHECKED: &str = "    pub checked: bool,";
pub const FIELD_SELECTED: &str = "    pub selected: Option<usize>,";
#[crawrad_note::note("targets-makepad-components-util-field-selection")]
pub const FIELD_SELECTION: &str = "    pub selection: Vec<usize>,";
pub const FIELD_ITEMS: &str = "    pub items: Strings,";
pub const FIELD_LINES: &str = "    pub lines: Strings,";

#[crawrad_note::note("targets-makepad-components-util-common-control-fields")]
pub fn common_control_fields() -> Vec<String> {
    vec![FIELD_VISIBLE.to_string(), FIELD_ENABLED.to_string()]
}

#[crawrad_note::note("targets-makepad-components-util-common-control-inits")]
pub fn common_control_inits(node: &Node) -> Vec<String> {
    vec![init_visible(node), init_enabled(node)]
}

#[crawrad_note::note("targets-makepad-components-util-init-visible")]
pub fn init_visible(node: &Node) -> String {
    let vis = layout::is_visible(node);
    format!("visible: {vis}")
}

#[crawrad_note::note("targets-makepad-components-util-init-enabled")]
pub fn init_enabled(node: &Node) -> String {
    let en = layout::is_enabled(node);
    format!("enabled: {en}")
}

#[crawrad_note::note("targets-makepad-components-util-init-text")]
pub fn init_text(node: &Node) -> String {
    let txt = layout::text(node);
    let txt_lit = crate::literal(&txt);
    format!("text: {txt_lit}.to_string()")
}

#[crawrad_note::note("targets-makepad-components-util-init-checked")]
pub fn init_checked(node: &Node) -> String {
    let chk = layout::is_checked(node);
    format!("checked: {chk}")
}

#[crawrad_note::note("targets-makepad-components-util-format-field")]
pub fn format_field(name: &str, ty: &str) -> String {
    format!("    pub {name}: {ty},")
}

#[crawrad_note::note("targets-makepad-components-util-value-to-rust-literal")]
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
                // the key by its full path: a form with key events names a
                // `Key` of its own
                format!(
                    "Shortcut {{ key: crawrad_runtime::prelude::Key::{:?}, ctrl: {}, alt: {}, shift: {}, meta: {} }}",
                    sc.key, sc.ctrl, sc.alt, sc.shift, sc.meta
                )
            }
        }
        Value::Color(c) => format!("Color::rgba({}, {}, {}, {})", c.r, c.g, c.b, c.a),
        Value::Font(_) => {
            crate::targets::compartilhado::literal_da_peca(val).expect("the box writes every Font")
        }
        Value::Picture(p) => {
            if p.is_empty() {
                if let Some(src) = &p.source {
                    format!("Picture::empty().with_source({})", crate::literal(src))
                } else {
                    "Picture::empty()".to_string()
                }
            } else {
                // as the EMITTED `Picture` takes it, the same literal the
                // Image writes (a Glyph went out in the IR's own form,
                // which the generated project does not have)
                crate::targets::compartilhado::literal_da_peca(val)
                    .unwrap_or_else(|| "Picture::empty()".to_string())
            }
        }
        _ => "Default::default()".to_string(),
    }
}

#[crawrad_note::note("targets-makepad-components-util-prop-default-literal")]
pub fn prop_default_literal(prop: &PropDesc) -> String {
    value_to_rust_literal(&prop.default)
}

#[crawrad_note::note("targets-makepad-components-util-text-input-flags")]
pub fn text_input_flags(node: &Node) -> String {
    let masked = matches!(node.props.get("password_char"), Some(Value::Str(s)) if !s.is_empty());
    let read_only = matches!(node.props.get("readonly"), Some(Value::Bool(true)));
    let mut lines = String::new();
    if masked {
        lines.push_str("                is_password: true\n");
    }
    if read_only {
        lines.push_str("                is_read_only: true\n");
    }
    lines
}

#[crawrad_note::note("targets-makepad-components-util-disabled-mark-sync")]
pub fn disabled_mark_sync(field: &str, widget: &str) -> String {
    format!(
        "        if !self.form.{field}.enabled {{\n\
         \x20           crawrad_runtime_makepad::enabled::disabled_mark(cx, &self.ui, id!({widget}), self.form.{field}.checked);\n\
         \x20       }}\n"
    )
}

#[crawrad_note::note("targets-makepad-components-util-text-input-flags-sync")]
pub fn text_input_flags_sync(field: &str, password: bool) -> String {
    let mut code = String::new();
    if password {
        code.push_str(&format!(
            "        let masked = !self.form.{field}.password_char.is_empty();\n\
             \x20       if ti.is_password() != masked {{\n\
             \x20           ti.set_is_password(cx, masked);\n\
             \x20       }}\n"
        ));
    }
    // a disabled one takes no typing either: Makepad only dims it
    code.push_str(&format!(
        "        let read_only = self.form.{field}.readonly || !self.form.{field}.enabled;\n\
         \x20       if ti.is_read_only() != read_only {{\n\
         \x20           ti.set_is_read_only(cx, read_only);\n\
         \x20       }}\n"
    ));
    code
}

fn written_by_hand_or_not_emitted(prop: &PropDesc, by_hand: &[&str]) -> bool {
    matches!(prop.key, "name" | "rect" | "action")
        || by_hand.contains(&prop.key)
        || crate::emission::why_not_set(prop.key, prop.category).is_some()
}

#[crawrad_note::note("targets-makepad-components-util-catalogue-fields")]
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

#[crawrad_note::note("targets-makepad-components-util-catalogue-inits")]
pub fn catalogue_inits(node: &Node, desc: &ComponentDesc, by_hand: &[&str]) -> Vec<String> {
    let mut inits = Vec::new();
    for prop in &desc.props {
        if written_by_hand_or_not_emitted(prop, by_hand) {
            continue;
        }
        let val_str = match node.props.get(prop.key) {
            Some(v) => value_to_rust_literal(v),
            None => prop_default_literal(prop),
        };
        inits.push(format!("{}: {val_str}", prop.key));
    }
    inits
}

#[cfg(test)]
#[path = "../../../test/makepad_catalogue_fields/test.rs"]
mod tests;
