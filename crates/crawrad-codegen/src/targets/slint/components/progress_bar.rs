//! Slint component implementation for `ProgressBar`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ProgressBarCodegen;

impl SlintComponentCodegen for ProgressBarCodegen {
    fn class(&self) -> &'static str {
        "ProgressBar"
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
        _node: &Node,
        // field name
        _field: &str,
        // emitted nodes
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
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
}

pub fn struct_definition() -> String {
    let desc = crawrad_registry::components::progress_bar();
    let mut fields = util::common_control_fields();
    fields.extend(util::catalogue_fields(&desc, &["visible", "enabled"]));
    let body = fields.join("\n");
    format!(
        "#[derive(Debug, Clone, Default)]\npub struct ProgressBar {{\n{body}\n}}\n\n\
         impl ProgressBar {{\n\
         \x20   pub fn step_it(&mut self) {{\n\
         \x20       self.position += self.step;\n\
         \x20       if self.position > self.max {{\n\
         \x20           self.position = self.min;\n\
         \x20       }}\n\
         \x20   }}\n\
         }}\n"
    )
}

pub fn emit_default(node: &Node) -> String {
    let mut inits = util::common_control_inits(node);
    let desc = crawrad_registry::components::progress_bar();
    inits.extend(util::catalogue_inits(node, &desc, &["visible", "enabled"]));
    let body = inits.join(", ");
    format!("ProgressBar {{ {body} }}")
}

pub fn emit_slint(node: &Node, x: f32, y: f32, w: f32, h: f32) -> Result<String, crate::Erro> {
    let name = identificador(node.name())?;
    let pos_val = match node.props.get("position") {
        Some(Value::Int(i)) => *i,
        _ => 0,
    };
    let min_val = match node.props.get("min") {
        Some(Value::Int(i)) => *i,
        _ => 0,
    };
    let max_val = match node.props.get("max") {
        Some(Value::Int(i)) => *i,
        _ => 100,
    };
    let fraction = if max_val > min_val {
        ((pos_val - min_val) as f32) / ((max_val - min_val) as f32)
    } else {
        0.0
    }
    .clamp(0.0, 1.0);

    let pos = layout::position(x, y);

    // ProgressBar gauge container
    let mut out = String::new();
    out.push_str(&format!("        {name} := Rectangle {{\n"));
    out.push_str(&format!("            {pos}\n"));
    out.push_str(&format!("            width: {w:.1}px;\n"));
    out.push_str(&format!("            height: {h:.1}px;\n"));
    out.push_str("            background: #e0e0e0;\n");
    out.push_str("            border-radius: 3px;\n");
    out.push_str("            clip: true;\n");
    out.push_str("            Rectangle {\n");
    out.push_str("                x: 0px;\n");
    out.push_str("                y: 0px;\n");
    out.push_str(&format!(
        "                width: parent.width * {fraction:.2};\n"
    ));
    out.push_str("                height: parent.height;\n");
    out.push_str("                background: #2a82da;\n");
    out.push_str("            }\n");
    out.push_str("        }\n");
    Ok(out)
}
