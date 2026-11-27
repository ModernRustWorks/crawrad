//! Slint component implementation for `Panel`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node};

pub struct PanelCodegen;

impl SlintComponentCodegen for PanelCodegen {
    fn class(&self) -> &'static str {
        "Panel"
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
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        emit_slint(doc, reg, node, x, y, w, h)
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
    let desc = crawrad_registry::components::panel();
    let mut fields = util::common_control_fields();
    fields.push(util::FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct Panel {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));
    let desc = crawrad_registry::components::panel();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("Panel {{ {body} }}")
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

pub fn emit_slint(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos = layout::position(x, y);
    let children = super::emit_children(doc, reg, node, "    ")?;

    let mut out = String::new();
    out.push_str(&format!("        {name} := Rectangle {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            background: #f0f0f0;\n");
    out.push_str("            border-width: 1px;\n");
    out.push_str("            border-color: #c0c0c0;\n");
    out.push_str("            clip: true;\n");
    if layout::event_handler(node, "on_click").is_some() {
        out.push_str(&format!(
            "            TouchArea {{\n\
             \x20               clicked => {{ root.{name}_clicked(); }}\n\
             \x20           }}\n"
        ));
    }
    if !children.is_empty() {
        out.push_str(&children);
    }
    out.push_str("        }\n");
    Ok(out)
}
