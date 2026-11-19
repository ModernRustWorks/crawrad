use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct PaintBoxCodegen;

impl EguiComponentCodegen for PaintBoxCodegen {
    fn class(&self) -> &'static str {
        "PaintBox"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_PAINTBOX)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let desc = ctx.reg.get("PaintBox").expect("PaintBox descriptor");

        // OnPaint draws on a Canvas of the box's size, and the
        // commands it recorded are painted in the box, clipped to it
        let paint = match crate::targets::egui::layout::handler(ctx.node, "on_paint") {
            Some(h) => {
                let h = crate::identificador(&h)?;
                format!(
                    "            let mut canvas = Canvas::new(r.width(), r.height());\n\
                     \x20           self.{h}(&mut canvas);\n\
                     {}",
                    replay("canvas", "r")
                )
            }
            None => String::new(),
        };
        let body = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             {paint}\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               let _ = ui.allocate_rect(r, egui::Sense::click());\n\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: PaintBox,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(ctx.node, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: body,
            tipo_usado: "PaintBox",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

#[crawrad_note::note("targets-egui-components-paint-box-replay")]
pub fn replay(canvas: &str, rect: &str) -> String {
    format!(
        "            let painter = ui.painter_at({rect});\n\
         \x20           let at = |x: f32, y: f32| {rect}.min + egui::vec2(x, y);\n\
         \x20           let rgba = |c: [u8; 4]| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]);\n\
         \x20           let pen = |s: Option<([u8; 4], f32)>| s.map_or(egui::Stroke::NONE, |(c, w)| egui::Stroke::new(w, rgba(c)));\n\
         \x20           for c in {canvas}.commands() {{\n\
         \x20               match c {{\n\
         \x20                   PaintCommand::Rect {{ x, y, w, h, radius, fill, stroke }} => {{\n\
         \x20                       let rect = egui::Rect::from_min_size(at(*x, *y), egui::vec2(*w, *h));\n\
         \x20                       painter.rect(rect, *radius, fill.map_or(egui::Color32::TRANSPARENT, rgba), pen(*stroke), egui::StrokeKind::Inside);\n\
         \x20                   }}\n\
         \x20                   PaintCommand::Circle {{ x, y, r: radius, fill, stroke }} => {{\n\
         \x20                       painter.circle(at(*x, *y), *radius, fill.map_or(egui::Color32::TRANSPARENT, rgba), pen(*stroke));\n\
         \x20                   }}\n\
         \x20                   PaintCommand::Line {{ x0, y0, x1, y1, color, width }} => {{\n\
         \x20                       painter.line_segment([at(*x0, *y0), at(*x1, *y1)], egui::Stroke::new(*width, rgba(*color)));\n\
         \x20                   }}\n\
         \x20                   PaintCommand::Text {{ text, x, y, color, size }} => {{\n\
         \x20                       painter.text(at(*x, *y), egui::Align2::LEFT_TOP, text, egui::FontId::proportional(*size), rgba(*color));\n\
         \x20                   }}\n\
         \x20               }}\n\
         \x20           }}\n"
    )
}

const ESTADO_PAINTBOX: StateText = StateText {
    head: r#"
/// A `PaintBox` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
