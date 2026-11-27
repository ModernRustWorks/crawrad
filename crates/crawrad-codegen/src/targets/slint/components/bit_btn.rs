//! Slint component implementation for `BitBtn`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct BitBtnCodegen;

impl SlintComponentCodegen for BitBtnCodegen {
    fn class(&self) -> &'static str {
        "BitBtn"
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
        if layout::event_handler(node, "on_click").is_some() {
            vec![format!("callback {name}_clicked();")]
        } else {
            Vec::new()
        }
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::bit_btn();
    let body = util::struct_fields_with_text(&desc);
    format!("#[derive(Debug, Clone, Default)]\npub struct BitBtn {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let desc = crawrad_registry::components::bit_btn();
    let body = util::struct_inits_with_text(node, &desc);
    format!("BitBtn {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    if let Some(h) = layout::event_handler(node, "on_click") {
        let h_name = identificador(&h)?;
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

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = util::escape_slint_string(&caption);
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := Button {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            text: \"{caption_esc}\";\n"));
    if layout::event_handler(node, "on_click").is_some() {
        out.push_str(&format!(
            "            clicked => {{ root.{name}_clicked(); }}\n"
        ));
    }
    out.push_str("        }\n");
    Ok(out)
}
