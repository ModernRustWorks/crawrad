//! Slint component implementation for `Toggle`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct ToggleCodegen;

impl SlintComponentCodegen for ToggleCodegen {
    fn class(&self) -> &'static str {
        "Toggle"
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
        vec![format!("callback {name}_toggled(bool);")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::toggle();
    let mut fields = util::common_control_fields();
    fields.push(util::FIELD_CHECKED.to_string());
    fields.push(util::FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Toggle {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_checked(node));
    inits.push(util::init_text(node));
    let desc = crawrad_registry::components::toggle();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = inits.join(", ");
    format!("Toggle {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let mut calls = String::new();
    for ev in ["on_click", "on_change"] {
        if let Some(h) = layout::event_handler(node, ev) {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
    }
    code.push_str(&format!(
        "        {{\n\
         \x20           let form = app.clone();\n\
         \x20           window.on_{field}_toggled(move |val| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.checked = val;\n\
         {calls}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let caption = layout::text(node);
    let caption_esc = util::escape_slint_string(&caption);
    let checked = layout::is_checked(node);
    let en = layout::is_enabled(node);
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := Switch {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            text: \"{caption_esc}\";\n"));
    if checked {
        out.push_str("            checked: true;\n");
    }
    if !en {
        out.push_str("            enabled: false;\n");
    }
    out.push_str(&format!(
        "            toggled => {{ root.{name}_toggled(self.checked); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
