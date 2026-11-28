//! Slint component implementation for `Drawer`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct DrawerCodegen;

impl SlintComponentCodegen for DrawerCodegen {
    fn class(&self) -> &'static str {
        "Drawer"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::drawer::drawer();
        let mut fields = util::common_control_fields();
        fields.push(util::FIELD_TEXT.to_string());
        fields.push("    pub side: String,".to_string());
        fields.push("    pub opened: bool,".to_string());
        fields.push("    pub show_header: bool,".to_string());
        fields.push("    pub overlay: bool,".to_string());
        // drawer text catalogue fields
        fields.extend(util::catalogue_fields(
            &desc,
            // drawer fields
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "side",
                "opened",
                "show_header",
                "overlay",
            ],
        ));
        let body = fields.join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Drawer {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let desc = crawrad_registry::components::drawer::drawer();
        let side = match node.props.get("side") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "dsLeft",
        };
        let opened = matches!(node.props.get("opened"), Some(Value::Bool(true)));
        let show_header = !matches!(node.props.get("show_header"), Some(Value::Bool(false)));
        let overlay = !matches!(node.props.get("overlay"), Some(Value::Bool(false)));

        let mut inits = util::common_control_inits(node);
        inits.push(util::init_text(node));
        inits.push(format!("side: {}.to_string()", crate::literal(side)));
        inits.push(format!("opened: {opened}"));
        inits.push(format!("show_header: {show_header}"));
        inits.push(format!("overlay: {overlay}"));
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            // drawer inits
            &[
                "visible",
                "enabled",
                "text",
                "caption",
                "side",
                "opened",
                "show_header",
                "overlay",
            ],
        ));
        let body = inits.join(", ");
        format!("Drawer {{ {body} }}")
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let mut on_close_call = String::new();
        if let Some(h) = layout::event_handler(node, "on_close") {
            let h_name = identificador(&h)?;
            on_close_call = format!("                    f.form.{h_name}();\n");
        }
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_close_clicked(move || {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   f.form.{field}.opened = false;\n\
             {on_close_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
    }

    fn emit_slint(
        &self,
        // drawer doc
        doc: &Document,
        reg: &crawrad_registry::Registry,
        node: &Node,
        // drawer bounds
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        // drawer id
        let name = identificador(node.name())?;
        let vis = layout::is_visible(node);
        let _en = layout::is_enabled(node);
        // drawer title text
        let caption = layout::text(node);
        let caption_esc = util::escape_slint_string(&caption);
        let opened = matches!(node.props.get("opened"), Some(Value::Bool(true)));
        let show_header = !matches!(node.props.get("show_header"), Some(Value::Bool(false)));
        // drawer child nodes
        let children = super::emit_children(doc, reg, node, "    ")?;

        let mut out = util::emit_rectangle_header(node, x, y, w, h)?;
        out.push_str("            background: #2f2f45;\n");
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #6e7290;\n");
        out.push_str("            clip: true;\n");
        out.push_str(&format!("            visible: {vis} && {opened};\n"));
        if show_header {
            out.push_str("            Rectangle {\n");
            out.push_str("                x: 0px;\n");
            out.push_str("                y: 0px;\n");
            out.push_str("                width: parent.width;\n");
            out.push_str("                height: 32px;\n");
            out.push_str("                background: #252538;\n");
            out.push_str("                border-width: 1px;\n");
            out.push_str("                border-color: #6e7290;\n");
            out.push_str("                Text {\n");
            out.push_str("                    x: 10px;\n");
            out.push_str("                    y: 8px;\n");
            out.push_str(&format!("                    text: \"{caption_esc}\";\n"));
            out.push_str("                    color: #f2f2fa;\n");
            out.push_str("                    font-size: 11px;\n");
            out.push_str("                }\n");
            out.push_str("                Text {\n");
            out.push_str("                    x: parent.width - 24px;\n");
            out.push_str("                    y: 6px;\n");
            out.push_str("                    text: \"×\";\n");
            out.push_str("                    color: #a6adc7;\n");
            out.push_str("                    font-size: 14px;\n");
            out.push_str("                    TouchArea {\n");
            out.push_str(&format!(
                "                        clicked => {{ root.{name}_close_clicked(); }}\n"
            ));
            out.push_str("                    }\n");
            out.push_str("                }\n");
            out.push_str("            }\n");
        }
        if !children.is_empty() {
            out.push_str(&children);
        }
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_close_clicked();")]
    }
}
