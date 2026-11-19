//! Makepad component implementation for `Shape`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::layout;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct ShapeCodegen;

impl MakepadComponentCodegen for ShapeCodegen {
    fn class(&self) -> &'static str {
        "Shape"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Shape").expect("Shape desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("#[derive(Debug, Clone, Default)]\npub struct Shape {{\n{body}\n}}\n")
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Shape").expect("Shape desc");
        let inits = state::inits_of(node, desc);
        let body = inits
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Shape {{ {body} }}")
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, field: &str) -> String {
        // The kind, the pen and the brush reach the shader for real
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("Shape").expect("Shape desc");
        let kind = state::fields_of(desc)
            .into_iter()
            .map(|(f, _)| f)
            .find(|f| f.starts_with("shape"))
            .unwrap_or_else(|| "shape_kind".to_string());
        format!(
            "        crawrad_runtime_makepad::shape::apply(cx, &self.ui.widget(id!({field})), &self.form.{field}.{kind}, self.form.{field}.pen_width, self.form.{field}.brush_color.to_f32(), self.form.{field}.pen_color.to_f32());\n"
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
        // One Sdf2d shader for the six classic kinds; the sync hands it
        // the kind (0 Rectangle .. 5 Circle), the pen width and both colours.
        // Makepad 1.0.0's Sdf2d has no ellipse: it is the circle's distance
        // over the box's two radii.
        // Its `box` rounds by 2r (`shader/std.rs:394`), so a quarter of the
        // short side is `s * 0.125` here, the same corner egui draws.
        Ok(format!(
            "            {name} = <View> {{\n\
             \x20               {pos}\n\
             \x20               width: {w}, height: {h},\n\
             \x20               show_bg: true,\n\
             \x20               draw_bg: {{\n\
             \x20                   instance kind: 0.0\n\
             \x20                   instance pen: 1.0\n\
             \x20                   instance brush: #ffffff\n\
             \x20                   instance pen_color: #000000\n\
             \x20                   fn pixel(self) -> vec4 {{\n\
             \x20                       let sdf = Sdf2d::viewport(self.pos * self.rect_size);\n\
             \x20                       let w = self.rect_size.x;\n\
             \x20                       let h = self.rect_size.y;\n\
             \x20                       let p = self.pen;\n\
             \x20                       let s = min(w, h);\n\
             \x20                       let sx = (w - s) * 0.5 + p * 0.5;\n\
             \x20                       let sy = (h - s) * 0.5 + p * 0.5;\n\
             \x20                       if self.kind < 0.5 {{\n\
             \x20                           sdf.rect(p * 0.5, p * 0.5, w - p, h - p);\n\
             \x20                       }} else if self.kind < 1.5 {{\n\
             \x20                           sdf.rect(sx, sy, s - p, s - p);\n\
             \x20                       }} else if self.kind < 2.5 {{\n\
             \x20                           sdf.box(p * 0.5, p * 0.5, w - p, h - p, s * 0.125);\n\
             \x20                       }} else if self.kind < 3.5 {{\n\
             \x20                           sdf.box(sx, sy, s - p, s - p, s * 0.125);\n\
             \x20                       }} else if self.kind < 4.5 {{\n\
             \x20                           let c = sdf.pos - vec2(w * 0.5, h * 0.5);\n\
             \x20                           let rad = vec2(max((w - p) * 0.5, 0.5), max((h - p) * 0.5, 0.5));\n\
             \x20                           sdf.dist = (length(c / rad) - 1.0) * min(rad.x, rad.y) / sdf.scale_factor;\n\
             \x20                           sdf.old_shape = sdf.shape;\n\
             \x20                           sdf.shape = min(sdf.shape, sdf.dist);\n\
             \x20                       }} else {{\n\
             \x20                           sdf.circle(w * 0.5, h * 0.5, (s - p) * 0.5);\n\
             \x20                       }}\n\
             \x20                       sdf.fill_keep(self.brush);\n\
             \x20                       if p > 0.0 {{\n\
             \x20                           sdf.stroke(self.pen_color, p);\n\
             \x20                       }}\n\
             \x20                       return sdf.result;\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20           }}\n"
        ))
    }
}
