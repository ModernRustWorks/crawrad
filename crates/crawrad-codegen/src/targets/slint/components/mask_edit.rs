//! Slint component implementation for `MaskEdit`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct MaskEditCodegen;

impl SlintComponentCodegen for MaskEditCodegen {
    fn class(&self) -> &'static str {
        "MaskEdit"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::mask_edit::mask_edit();
        util::standard_text_control_struct("MaskEdit", &desc)
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::mask_edit::mask_edit();
        let mask = match node.props.get("edit_mask") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let mut inits = util::common_control_inits(node);
        let text = layout::text(node);
        inits.push(format!("text: {}.to_string()", crate::literal(&text)));
        inits.push(format!("edit_mask: {}.to_string()", crate::literal(mask)));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "text", "edit_mask"],
        ));
        let body = inits.join(", ");
        format!("MaskEdit {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        // mask_edit action target
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut code = util::emit_text_edited_action(node, field)?;
        code.push_str(&util::emit_return_action(node, field)?);
        Ok(code)
    }

    fn emit_slint(
        &self,
        // mask_edit document reference
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        // mask_edit layout bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // mask_edit component name
        let name = identificador(node.name())?;
        let text = layout::text(node);
        let text_esc = util::escape_slint_string(&text);
        let mask = match node.props.get("edit_mask") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        let hint = layout::str_val(node, "text_hint").unwrap_or(mask);
        let hint_esc = util::escape_slint_string(hint);
        let en = layout::is_enabled(node);
        let vis = layout::is_visible(node);
        let read_only = matches!(node.props.get("read_only"), Some(Value::Bool(true)));
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
        if !vis {
            out.push_str("            visible: false;\n");
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

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        let mut cbs = vec![format!("callback {name}_edited(string);")];
        if layout::event_handler(node, "on_return").is_some() {
            cbs.push(format!("callback {name}_accepted();"));
        }
        cbs
    }

    fn std_widgets(&self) -> &'static [&'static str] {
        &["LineEdit"]
    }
}
