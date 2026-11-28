//! Slint component implementation for `CategoryPanel`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct CategoryPanelCodegen;

impl SlintComponentCodegen for CategoryPanelCodegen {
    fn class(&self) -> &'static str {
        "CategoryPanel"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::category_panel::category_panel();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.push("    pub collapsed: bool,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "text", "caption", "collapsed"],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct CategoryPanel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::category_panel::category_panel();
        let collapsed = matches!(node.props.get("collapsed"), Some(Value::Bool(true)));
        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.push(format!("collapsed: {collapsed}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "text", "caption", "collapsed"],
        ));
        let body = inits.join(", ");
        format!("CategoryPanel {{ {body} }}")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut on_collapsed_call = String::new();
        if let Some(h) = layout::event_handler(node, "on_collapsed") {
            let h_name = identificador(&h)?;
            on_collapsed_call = format!("                        f.form.{h_name}();\n");
        }
        let mut on_expanded_call = String::new();
        if let Some(h) = layout::event_handler(node, "on_expanded") {
            let h_name = identificador(&h)?;
            on_expanded_call = format!("                        f.form.{h_name}();\n");
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_header_clicked(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.collapsed = !f.form.{field}.collapsed;\n\
             \x20                   if f.form.{field}.collapsed {{\n\
             {on_collapsed_call}\
             \x20                   }} else {{\n\
             {on_expanded_call}\
             \x20                   }}\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
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
        let name = identificador(node.name())?;
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let caption = layout::text(node);
        let caption_esc = util::escape_slint_string(&caption);
        let collapsed = matches!(node.props.get("collapsed"), Some(Value::Bool(true)));
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        if collapsed {
            out.push_str("            height: 24.0px;\n");
        } else {
            out.push_str(&format!("            height: {h:.1}px;\n"));
        }
        out.push_str("            background: #1e1e2e;\n");
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #3e3e56;\n");
        out.push_str("            clip: true;\n");
        out.push_str(&format!("            visible: {vis};\n"));
        out.push_str(&format!("            enabled: {en};\n"));
        out.push_str("            Rectangle {\n");
        out.push_str("                x: 0px;\n");
        out.push_str("                y: 0px;\n");
        out.push_str("                width: parent.width;\n");
        out.push_str("                height: 24px;\n");
        out.push_str("                background: #2d2d42;\n");
        out.push_str("                Text {\n");
        out.push_str("                    x: 8px;\n");
        out.push_str("                    y: 4px;\n");
        let prefix = if collapsed { "+" } else { "-" };
        out.push_str(&format!(
            "                    text: \"{prefix} {caption_esc}\";\n"
        ));
        out.push_str("                    color: #f2f2fa;\n");
        out.push_str("                    font-size: 11px;\n");
        out.push_str("                }\n");
        out.push_str("                TouchArea {\n");
        out.push_str(&format!(
            "                    clicked => {{ root.{name}_header_clicked(); }}\n"
        ));
        out.push_str("                }\n");
        out.push_str("            }\n");
        if !children.is_empty() {
            out.push_str(&children);
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_header_clicked();")]
    }
}
