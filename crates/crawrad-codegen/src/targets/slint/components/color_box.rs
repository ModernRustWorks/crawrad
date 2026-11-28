//! Slint component implementation for `ColorBox`.

use super::{codegen::SlintComponentCodegen, util};
use crate::{identificador, targets::slint::layout};
use crawrad_ir::{Document, Node, Value};

pub struct ColorBoxCodegen;

impl SlintComponentCodegen for ColorBoxCodegen {
    fn class(&self) -> &'static str {
        "ColorBox"
    }

    fn struct_definition(&self) -> String {
        let desc = crawrad_registry::components::color_box::color_box();
        let mut fields = util::common_control_fields();
        fields.push("    pub selected: crawrad_ir::Color,".to_string());
        fields.extend(util::catalogue_fields(
            &desc,
            &["visible", "enabled", "selected"],
        ));
        let body = fields.join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\npub struct ColorBox {{\n{body}\n}}\n\n{}",
            crate::targets::shared::standard_colors_const()
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);
        let selected = match node.props.get("selected") {
            Some(Value::Color(c)) => format!("crawrad_ir::Color::rgb({}, {}, {})", c.r, c.g, c.b),
            _ => "crawrad_ir::Color::rgb(0, 0, 0)".to_string(),
        };
        let mut inits = vec![
            format!("visible: {vis}"),
            format!("enabled: {en}"),
            format!("selected: {selected}"),
        ];
        let desc = crawrad_registry::components::color_box::color_box();
        inits.extend(util::catalogue_inits(
            node,
            &desc,
            &["visible", "enabled", "selected"],
        ));
        let body = inits.join(", ");
        format!("ColorBox {{ {body} }}")
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
        let on_change_call = match layout::event_handler(node, "on_change") {
            Some(h) => {
                let h_name = identificador(&h)?;
                format!("                    f.form.{h_name}();\n")
            }
            None => String::new(),
        };
        Ok(format!(
            "        {{\n\
             \x20           let form = app.clone();\n\
             \x20           window.on_{field}_changed(move |val| {{\n\
             \x20               if let Ok(mut f) = form.lock() {{\n\
             \x20                   if let Some((_, c)) = crawrad_ir::Color::STANDARD.iter().find(|(n, _)| *n == val.as_str()) {{\n\
             \x20                       f.form.{field}.selected = *c;\n\
             \x20                   }}\n\
             {on_change_call}\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        ))
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
        let name = identificador(node.name())?;
        let selected_color = match node.props.get("selected") {
            Some(Value::Color(c)) => *c,
            _ => crawrad_ir::Color::rgb(0, 0, 0),
        };
        let hex = selected_color.to_hex();
        let selected_name = crawrad_ir::Color::STANDARD
            .iter()
            .find(|(_, c)| *c == selected_color)
            .map(|(n, _)| *n)
            .unwrap_or("Custom");
        let labels: Vec<String> = crawrad_ir::Color::STANDARD
            .iter()
            .map(|(n, _)| format!("\"{}\"", n))
            .collect();
        let labels_slice = labels.join(", ");
        let pos = layout::position(x, y);
        let vis = layout::is_visible(node);
        let en = layout::is_enabled(node);

        let mut out = String::new();
        out.push_str(&format!("        {name} := Rectangle {{\n"));
        out.push_str(&format!("            {pos}\n"));
        out.push_str(&format!("            width: {w:.1}px;\n"));
        out.push_str(&format!("            height: {h:.1}px;\n"));
        out.push_str("            border-width: 1px;\n");
        out.push_str("            border-color: #cccccc;\n");
        out.push_str("            border-radius: 4px;\n");
        out.push_str("            background: #ffffff;\n");
        if !vis {
            out.push_str("            visible: false;\n");
        }
        out.push_str("            HorizontalLayout {\n");
        out.push_str("                padding-left: 4px;\n");
        out.push_str("                padding-right: 4px;\n");
        out.push_str("                spacing: 6px;\n");
        out.push_str("                alignment: center;\n");
        out.push_str(&format!("                {name}_swatch := Rectangle {{\n"));
        out.push_str("                    width: 16px;\n");
        out.push_str("                    height: 16px;\n");
        out.push_str(&format!("                    background: {hex};\n"));
        out.push_str("                    border-width: 1px;\n");
        out.push_str("                    border-color: #888888;\n");
        out.push_str("                    border-radius: 2px;\n");
        out.push_str("                }\n");
        out.push_str(&format!("                {name}_combo := ComboBox {{\n"));
        out.push_str("                    horizontal-stretch: 1;\n");
        out.push_str(&format!("                    model: [{labels_slice}];\n"));
        out.push_str(&format!(
            "                    current-value: \"{selected_name}\";\n"
        ));
        if !en {
            out.push_str("                    enabled: false;\n");
        }
        out.push_str(&format!(
            "                    selected(val) => {{ root.{name}_changed(val); }}\n"
        ));
        out.push_str("                }\n");
        out.push_str("            }\n");
        out.push_str("        }\n");
        Ok(out)
    }

    fn slint_callbacks(&self, node: &Node) -> Vec<String> {
        let name = identificador(node.name()).unwrap_or_else(|_| node.name().to_string());
        vec![format!("callback {name}_changed(string);")]
    }
}
