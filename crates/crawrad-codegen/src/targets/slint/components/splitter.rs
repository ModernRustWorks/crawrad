//! Slint component implementation for `Splitter`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct SplitterCodegen;

impl SlintComponentCodegen for SplitterCodegen {
    fn class(&self) -> &'static str {
        "Splitter"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::splitter::splitter();
        let mut fields = util::common_control_fields();
        fields.push("    pub min_size: i64,".to_string());
        fields.push("    pub auto_snap: bool,".to_string());
        fields.push("    pub beveled: bool,".to_string());
        fields.push("    pub offset: f32,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "min_size", "auto_snap", "beveled"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Splitter {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::splitter::splitter();
        let min_size = match node.props.get("min_size") {
            Some(Value::Int(i)) => *i,
            _ => 30,
        };
        let auto_snap = !matches!(node.props.get("auto_snap"), Some(Value::Bool(false)));
        let beveled = !matches!(node.props.get("beveled"), Some(Value::Bool(false)));
        let mut inits = util::common_control_inits(node);
        inits.push(format!("min_size: {min_size}"));
        inits.push(format!("auto_snap: {auto_snap}"));
        inits.push(format!("beveled: {beveled}"));
        inits.push("offset: 0.0".to_string());
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "min_size", "auto_snap", "beveled"],
        ));
        let body = inits.join(", ");
        format!("Splitter {{ {body} }}")
    }

    fn emit_actions(
        &self,
        // splitter action doc
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        // splitter action call
        let mut calls = String::new();
        if let Some(h) = layout::event_handler(node, "on_moved") {
            let h_name = identificador(&h)?;
            calls.push_str(&format!("                    f.form.{h_name}();\n"));
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_moved(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             {calls}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // splitter doc
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // splitter bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // splitter id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let cursor = if h >= w { "col-resize" } else { "row-resize" };

        let mut out = util::emit_rectangle_header(node, x, y, w, h)?;
        out.push_str("            background: #252538;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str("            TouchArea {\n");
        if !en {
            out.push_str("                enabled: false;\n");
        }
        out.push_str(&format!("                mouse-cursor: {cursor};\n"));
        out.push_str(&format!(
            "                moved => {{ root.{name}_moved(); }}\n"
        ));
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_moved();")]
    }
}
