//! Makepad component implementation for `Bevel`: one shader for the
//! seven classic shapes, with the shape and the style as instances that the
//! sync hands it (`crawrad_runtime_makepad::bevel`).

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::{layout, state};
use crawrad_ir::{Document, Node};

pub struct BevelCodegen;

impl MakepadComponentCodegen for BevelCodegen {
    fn class(&self) -> &'static str {
        "Bevel"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Bevel").expect("Bevel desc");
        let body = state::fields_of(desc)
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Bevel {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Bevel").expect("Bevel desc");
        let body = state::inits_of(node, desc)
            .into_iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Bevel {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        format!(
            "        crawrad_runtime_makepad::bevel::apply(cx, &self.ui.widget(id!({field})), &self.form.{field}.bevel_shape, &self.form.{field}.bevel_style);\n"
        )
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
        // the kind is 0 Box .. 5 RightLine, 6 Spacer; shadow and light are
        // translucent (premultiplied), so the bevel reads on any background.
        // One comparison per `if`: the shader has no `||`.
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{\n\
             \x20                   instance kind: 0.0\n\
             \x20                   instance raised: 0.0\n\
             \x20                   fn pixel(self) -> vec4 {{\n\
             \x20                       let p = self.pos * self.rect_size;\n\
             \x20                       let w = self.rect_size.x;\n\
             \x20                       let h = self.rect_size.y;\n\
             \x20                       let shadow = vec4(0.0, 0.0, 0.0, 0.45);\n\
             \x20                       let light = vec4(0.25, 0.25, 0.25, 0.25);\n\
             \x20                       let first = mix(shadow, light, self.raised);\n\
             \x20                       let second = mix(light, shadow, self.raised);\n\
             \x20                       if self.kind < 0.5 {{\n\
             \x20                           if p.y < 1.0 {{ return first; }}\n\
             \x20                           if p.x < 1.0 {{ return first; }}\n\
             \x20                           if p.y > h - 1.0 {{ return second; }}\n\
             \x20                           if p.x > w - 1.0 {{ return second; }}\n\
             \x20                       }} else if self.kind < 1.5 {{\n\
             \x20                           if p.y > h - 1.0 {{ return second; }}\n\
             \x20                           if p.x > w - 1.0 {{ return second; }}\n\
             \x20                           if p.y < 1.0 {{ return first; }}\n\
             \x20                           if p.x < 1.0 {{ return first; }}\n\
             \x20                           if p.y > h - 2.0 {{ return first; }}\n\
             \x20                           if p.x > w - 2.0 {{ return first; }}\n\
             \x20                           if p.y < 2.0 {{ return second; }}\n\
             \x20                           if p.x < 2.0 {{ return second; }}\n\
             \x20                       }} else if self.kind < 2.5 {{\n\
             \x20                           if p.y < 1.0 {{ return first; }}\n\
             \x20                           if p.y < 2.0 {{ return second; }}\n\
             \x20                       }} else if self.kind < 3.5 {{\n\
             \x20                           if p.y > h - 1.0 {{ return second; }}\n\
             \x20                           if p.y > h - 2.0 {{ return first; }}\n\
             \x20                       }} else if self.kind < 4.5 {{\n\
             \x20                           if p.x < 1.0 {{ return first; }}\n\
             \x20                           if p.x < 2.0 {{ return second; }}\n\
             \x20                       }} else if self.kind < 5.5 {{\n\
             \x20                           if p.x > w - 1.0 {{ return second; }}\n\
             \x20                           if p.x > w - 2.0 {{ return first; }}\n\
             \x20                       }}\n\
             \x20                       return vec4(0.0, 0.0, 0.0, 0.0);\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20           }}\n"
        ))
    }
}
