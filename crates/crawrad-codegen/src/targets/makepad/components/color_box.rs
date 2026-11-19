//! Makepad component implementation for `ColorBox`: a row with the
//! swatch of the chosen colour (`<field>_swatch`) and a `DropDown` of the
//! standard colours (`<field>_list`).

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct ColorBoxCodegen;

impl MakepadComponentCodegen for ColorBoxCodegen {
    fn class(&self) -> &'static str {
        "ColorBox"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ColorBox").expect("ColorBox desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        // the standard colours go out once, with the type
        format!(
            "#[derive(Debug, Clone, Default)]\npub struct ColorBox {{\n{body}\n}}\n\n{}",
            crate::targets::shared::standard_colors_const()
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("ColorBox").expect("ColorBox desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("ColorBox {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        // the swatch shows the chosen colour, the list marks it\n\
             \x20       {{\n\
             \x20           let c = self.form.{field}.selected.to_f32();\n\
             \x20           let rgba = vec4(c[0], c[1], c[2], c[3]);\n\
             \x20           self.ui.view(id!({field}_swatch)).apply_over(cx, live! {{ draw_bg: {{ color: (rgba) }} }});\n\
             \x20           if let Some(i) = CRAWRAD_STANDARD_COLORS.iter().position(|(_, s)| *s == self.form.{field}.selected)\n\
             \x20               && self.ui.drop_down(id!({field}_list)).selected_item() != i\n\
             \x20           {{\n\
             \x20               self.ui.drop_down(id!({field}_list)).set_selected_item(cx, i);\n\
             \x20           }}\n\
             \x20       }}\n"
        )
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let change = match layout::handler(node, "on_change") {
            Some(h) => format!(
                "            if self.form.{field}.selected != before {{\n\
                 \x20               self.form.{}();\n\
                 \x20           }}\n",
                identificador(&h)?
            ),
            None => "            let _ = before;\n".to_string(),
        };
        Ok(format!(
            "        if let Some(i) = self.ui.drop_down(id!({field}_list)).selected(actions) {{\n\
             \x20           let before = self.form.{field}.selected;\n\
             \x20           if let Some((_, c)) = CRAWRAD_STANDARD_COLORS.get(i) {{\n\
             \x20               self.form.{field}.selected = *c;\n\
             \x20           }}\n\
             {change}\
             \x20       }}\n"
        ))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        let pos = layout::posicao(x, y);
        let name = identificador(node.name())?;
        let labels = crawrad_ir::Color::STANDARD
            .iter()
            .map(|(n, _)| crate::literal(n))
            .collect::<Vec<_>>()
            .join(", ");
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h}\n\
             \x20               flow: Right, align: {{ y: 0.5 }}, spacing: 4.0\n\
             \x20               {name}_swatch = <RoundedView> {{ width: 14.0, height: 14.0, show_bg: true, draw_bg: {{ color: #000, border_radius: 2.0 }} }}\n\
             \x20               {name}_list = <DropDown> {{ width: Fill, height: Fill, labels: [{labels}] }}\n\
             \x20           }}\n"
        ))
    }
}
