//! Slint component implementation for `ScrollBar`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ScrollBarCodegen;

impl SlintComponentCodegen for ScrollBarCodegen {
    fn class(&self) -> &'static str {
        "ScrollBar"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::scroll_bar::scroll_bar();
        let mut fields = util::common_control_fields();
        fields.push("    pub position: f32,".to_string());
        fields.push("    pub min: f32,".to_string());
        fields.push("    pub max: f32,".to_string());
        fields.push("    pub small_change: f32,".to_string());
        fields.push("    pub large_change: f32,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &[
                "visible",
                "enabled",
                "position",
                // range limits
                "min",
                "max",
                "small_change",
                "large_change",
                "kind",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct ScrollBar {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::scroll_bar::scroll_bar();
        let pos = match node.props.get("position") {
            Some(Value::Int(i)) => *i as f32,
            Some(Value::Float(f)) => *f as f32,
            _ => 0.0,
        };
        // resolve scroll range min and max
        let (min, max) = util::extract_min_max_f32(node, 0.0, 100.0);
        let sc = match node.props.get("small_change") {
            Some(Value::Int(i)) => *i as f32,
            Some(Value::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let lc = match node.props.get("large_change") {
            Some(Value::Int(i)) => *i as f32,
            Some(Value::Float(f)) => *f as f32,
            _ => 10.0,
        };
        let mut inits = util::common_control_inits(node);
        inits.push(format!("position: {pos:.1}"));
        inits.push(format!("min: {min:.1}"));
        inits.push(format!("max: {max:.1}"));
        inits.push(format!("small_change: {sc:.1}"));
        inits.push(format!("large_change: {lc:.1}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            // scroll_bar inits
            &[
                "visible",
                "enabled",
                "position",
                // range limits
                "min",
                "max",
                "small_change",
                "large_change",
                "kind",
            ],
        ));
        let body = inits.join(", ");
        format!("ScrollBar {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // scroll_bar action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // scroll_bar action call
        let mut calls = String::new();
        for ev in ["on_change", "on_scroll"] {
            if let Some(h) = layout::event_handler(node, ev) {
                let h_name = identificador(&h)?;
                calls.push_str(&format!("                    f.form.{h_name}();\n"));
            }
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_changed(move |val| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.position = val;\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // scroll_bar doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // scroll_bar bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // scroll_bar id
        let name = identificador(node.name())?;
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        // resolve scroll range min and max
        let (min, max) = util::extract_min_max_f32(node, 0.0, 100.0);
        let cur_pos = match node.props.get("position") {
            Some(Value::Int(i)) => *i as f32,
            Some(Value::Float(f)) => *f as f32,
            _ => 0.0,
        };
        let is_horizontal = w >= h;
        let orientation = if is_horizontal {
            "horizontal"
        } else {
            "vertical"
        };

        let mut out = String::new();
        out.push_str(&format!("        {name} := Slider {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str(&format!("            minimum: {min:.1};\n"));
        out.push_str(&format!("            maximum: {max:.1};\n"));
        out.push_str(&format!("            value: {cur_pos:.1};\n"));
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        out.push_str(&format!(
            "            changed(val) => {{ root.{name}_changed(val); }}\n"
        ));
        out.push_str("        }\n");
        let _ = orientation;
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_changed(float);")]
    }
}
