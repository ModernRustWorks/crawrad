//! Slint component implementation for `TrackBar`.

use super::codegen::SlintComponentCodegen;
use super::util;
use crate::identificador;
use crate::targets::slint::layout;
use crawrad_ir::{Document, Node, Value};

pub struct TrackBarCodegen;

impl SlintComponentCodegen for TrackBarCodegen {
    fn class(&self) -> &'static str {
        "TrackBar"
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
        vec![format!("callback {name}_changed(float);")]
    }
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::track_bar();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!("#[derive(Debug, Clone, Default)]\npub struct TrackBar {{\n{body}\n}}\n")
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    let desc = crawrad_registry::components::track_bar();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("TrackBar {{ {body} }}")
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
         \x20           window.on_{field}_changed(move |val| {{\n\
         \x20               if let Ok(mut f) = form.lock() {{\n\
         \x20                   f.form.{field}.position = val as i64;\n\
         {calls}\
         \x20               }}\n\
         \x20           }});\n\
         \x20       }}\n"
    ));
    Ok(code)
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos_val = match node.props.get("position") {
        Some(Value::Int(i)) => *i as f32,
        _ => 0.0,
    };
    let min_val = match node.props.get("min") {
        Some(Value::Int(i)) => *i as f32,
        _ => 0.0,
    };
    let max_val = match node.props.get("max") {
        Some(Value::Int(i)) => *i as f32,
        _ => 10.0,
    };
    let en = layout::is_enabled(node);
    let pos = layout::position(x, y);

    let mut out = String::new();
    out.push_str(&format!("        {name} := Slider {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str(&format!("            minimum: {min_val:.1};\n"));
    out.push_str(&format!("            maximum: {max_val:.1};\n"));
    out.push_str(&format!("            value: {pos_val:.1};\n"));
    if !en {
        out.push_str("            enabled: false;\n");
    }
    out.push_str(&format!(
        "            changed(val) => {{ root.{name}_changed(val); }}\n"
    ));
    out.push_str("        }\n");
    Ok(out)
}
