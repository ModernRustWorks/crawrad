//! Slint component implementation for `PageControl` and `TabSheet`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node, Value};

pub struct PageControlCodegen;

impl SlintComponentCodegen for PageControlCodegen {
    fn class(&self) -> &'static str {
        "PageControl"
    }

    fn struct_definition(&self) -> String {
        page_control_struct()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_page_control_default(node)
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
        vec![format!("callback {name}_tab_changed(int);")]
    }
}

pub struct TabSheetCodegen;

impl SlintComponentCodegen for TabSheetCodegen {
    fn class(&self) -> &'static str {
        "TabSheet"
    }

    fn struct_definition(&self) -> String {
        tab_sheet_struct()
    }

    fn emit_default(&self, node: &Node) -> String {
        emit_tab_sheet_default(node)
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        _node: &Node,
        _field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }

    fn emit_slint(
        &self,
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        emit_tab_sheet_slint(doc, reg, node)
    }
}

pub fn page_control_struct() -> String {
    let desc = crawrad_registry::components::page_control();
    let mut fields = vec![
        util::FIELD_VISIBLE.to_string(),
        util::FIELD_ENABLED.to_string(),
        "    pub active_page: usize,".to_string(),
        "    pub active_tab: usize,".to_string(),
    ];
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "active_page"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct PageControl {{\n{body}\n}}\n")
}

pub fn tab_sheet_struct() -> String {
    let desc = crawrad_registry::components::tab_sheet();
    let mut fields = util::common_control_fields();
    fields.push(util::FIELD_TEXT.to_string());
    fields.extend(util::catalogue_fields(
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct TabSheet {{\n{body}\n}}\n")
}

pub fn emit_page_control_default(node: &Node) -> String {
    let vis = layout::is_visible(node);
    let en = layout::is_enabled(node);
    let active_page = match node.props.get("active_page") {
        Some(Value::Int(i)) if *i >= 0 => *i as usize,
        _ => 0,
    };
    let mut inits = vec![
        format!("visible: {vis}"),
        format!("enabled: {en}"),
        format!("active_page: {active_page}"),
        format!("active_tab: {active_page}"),
    ];
    let desc = crawrad_registry::components::page_control();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "active_page"],
    ));
    let body = inits.join(", ");
    format!("PageControl {{ {body} }}")
}

pub fn emit_tab_sheet_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    inits.push(util::init_text(node));
    let desc = crawrad_registry::components::tab_sheet();
    inits.extend(util::catalogue_inits(
        node,
        &desc,
        &["visible", "enabled", "text"],
    ));
    let body = inits.join(", ");
    format!("TabSheet {{ {body} }}")
}

pub fn emit_actions(node: &Node, field: &str) -> Result<String, crate::Erro> {
    let mut code = String::new();
    let mut calls = String::new();
    if let Some(h) = layout::event_handler(node, "on_change") {
        let h_name = identificador(&h)?;
        calls.push_str(&format!("                    f.form.{h_name}();\n"));
    }
    code.push_str(&format!(
        "        {{\n\
         \x20           let form = app.clone();\n\
         \x20           window.on_{field}_tab_changed(move |idx| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.active_page = idx as usize;\n\
         \x20                   f.form.{field}.active_tab = idx as usize;\n\
         {calls}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
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
    out.push_str("            background: #f8f8f8;\n");
    out.push_str("            border-width: 1px;\n");
    out.push_str("            border-color: #d0d0d0;\n");
    out.push_str("            clip: true;\n");
    if !children.is_empty() {
        out.push_str(&children);
    }
    out.push_str("        }\n");
    Ok(out)
}

pub fn emit_tab_sheet_slint(
    doc: &Document,
    reg: &crawrad_registry::Registry,
    node: &Node,
) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let children = super::emit_children(doc, reg, node, "    ")?;

    let mut out = String::new();
    out.push_str(&format!("        {name} := Rectangle {{\n"));
    out.push_str("            x: 0px;\n");
    out.push_str("            y: 0px;\n");
    out.push_str("            width: parent.width;\n");
    out.push_str("            height: parent.height;\n");
    if !children.is_empty() {
        out.push_str(&children);
    }
    out.push_str("        }\n");
    Ok(out)
}
