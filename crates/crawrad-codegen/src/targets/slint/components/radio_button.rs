//! Slint component implementation for `RadioButton`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct RadioButtonCodegen;

impl SlintComponentCodegen for RadioButtonCodegen {
    fn class(&self) -> &'static str {
        "RadioButton"
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
        vec![format!("callback {name}_clicked();")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::radio_button();
    let mut fields = util::common_control_fields();
    fields.push(util::FIELD_CHECKED.to_string());
    fields.push(util::FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct RadioButton {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_checked(node));
    inits.push(util::init_text(node));
    let desc = crawrad_registry::components::radio_button();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "checked", "text"],
    ));
    let body = inits.join(", ");
    format!("RadioButton {{ {body} }}")
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
         \x20           window.on_{field}_clicked(move || {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.checked = true;\n\
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
    out.push_str(&format!("        {name} := HorizontalLayout {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            spacing: 6px;\n");
    out.push_str("            Rectangle {\n");
    out.push_str("                width: 16px;\n");
    out.push_str("                height: 16px;\n");
    out.push_str("                border-radius: 8px;\n");
    out.push_str("                border-width: 1px;\n");
    out.push_str("                border-color: #606060;\n");
    out.push_str("                background: transparent;\n");
    if checked {
        out.push_str("                Rectangle {\n");
        out.push_str("                    width: 8px;\n");
        out.push_str("                    height: 8px;\n");
        out.push_str("                    border-radius: 4px;\n");
        out.push_str("                    background: #2a82da;\n");
        out.push_str("                }\n");
    }
    out.push_str("            }\n");
    out.push_str("            Text {\n");
    out.push_str(&format!("                text: \"{caption_esc}\";\n"));
    out.push_str("                vertical-alignment: center;\n");
    if !en {
        out.push_str("                color: #888888;\n");
    }
    out.push_str("            }\n");
    out.push_str("            TouchArea {\n");
    out.push_str(&format!(
        "                clicked => {{ root.{name}_clicked(); }}\n"
    ));
    out.push_str("            }\n");
    out.push_str("        }\n");
    Ok(out)
}
