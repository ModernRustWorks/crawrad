//! Slint component implementation for `Edit`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node};

pub struct EditCodegen;

impl SlintComponentCodegen for EditCodegen {
    fn class(&self) -> &'static str {
        "Edit"
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
        let mut cbs = vec![format!("callback {name}_edited(string);")];
        if layout::event_handler(node, "on_return").is_some() {
            cbs.push(format!("callback {name}_accepted();"));
        }
        cbs
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::edit();
    util::standard_text_control_struct("Edit", &desc)
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::edit();
    util::standard_text_control_default("Edit", node, &desc)
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = util::emit_text_edited_action(node, field)?;
    code.push_str(&util::emit_return_action(node, field)?);
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let text = layout::text(node);
    let text_esc = util::escape_slint_string(&text);
    let hint = layout::str_val(node, "text_hint").unwrap_or("");
    let hint_esc = util::escape_slint_string(hint);
    let en = layout::is_enabled(node);
    let read_only = matches!(
        node.props.get("read_only"),
        Some(crawrad_ir::Value::Bool(true))
    );
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := LineEdit {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            text: \"{text_esc}\";\n"));
    if !hint.is_empty() {
        out.push_str(&format!("            placeholder-text: \"{hint_esc}\";\n"));
    }
    if !en {
        out.push_str("            enabled: false;\n");
    }
    if read_only {
        out.push_str("            read-only: true;\n");
    }
    out.push_str(&format!(
        "            edited(val) => {{ root.{name}_edited(val); }}\n"
    ));
    if layout::event_handler(node, "on_return").is_some() {
        out.push_str(&format!(
            "            accepted => {{ root.{name}_accepted(); }}\n"
        ));
    }
    out.push_str("        }\n");
    Ok(out)
}
