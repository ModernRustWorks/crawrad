use super::codegen::{EguiComponentCodegen, StateText};
use super::{Compartilhado, ContextoEmissao, ResultadoComponente};
use crate::Erro;

pub struct ShapeCodegen;

impl EguiComponentCodegen for ShapeCodegen {
    fn class(&self) -> &'static str {
        "Shape"
    }

    fn state_text(&self) -> Option<StateText> {
        Some(ESTADO_SHAPE)
    }

    fn emit(&self, ctx: &ContextoEmissao) -> Result<ResultadoComponente, Erro> {
        let nome = ctx.nome;
        let n = ctx.node;
        let desc = ctx.reg.get("Shape").expect("Shape descriptor");
        // The six classic kinds, filled with the brush and outlined
        // with the pen; a square kind is the largest square centred in the box
        let body = format!(
            "        if self.{nome}.visible {{\n\
             \x20           let is_enabled = self.{nome}.enabled;\n\
             \x20           ui.add_enabled_ui(is_enabled, |ui| {{\n\
             \x20               let b = self.{nome}.brush_color;\n\
             \x20               let p = self.{nome}.pen_color;\n\
             \x20               let fill = egui::Color32::from_rgba_unmultiplied(b.r, b.g, b.b, b.a);\n\
             \x20               let width = self.{nome}.pen_width.max(0) as f32;\n\
             \x20               let stroke = egui::Stroke::new(width, egui::Color32::from_rgba_unmultiplied(p.r, p.g, p.b, p.a));\n\
             \x20               let side = r.width().min(r.height());\n\
             \x20               let square = egui::Rect::from_center_size(r.center(), egui::vec2(side, side));\n\
             \x20               let painter = ui.painter();\n\
             \x20               match self.{nome}.shape_kind.as_str() {{\n\
             \x20                   \"Square\" => {{\n\
             \x20                       painter.rect(square, 0.0, fill, stroke, egui::StrokeKind::Inside);\n\
             \x20                   }}\n\
             \x20                   \"RoundRect\" => {{\n\
             \x20                       painter.rect(r, side * 0.25, fill, stroke, egui::StrokeKind::Inside);\n\
             \x20                   }}\n\
             \x20                   \"RoundSquare\" => {{\n\
             \x20                       painter.rect(square, side * 0.25, fill, stroke, egui::StrokeKind::Inside);\n\
             \x20                   }}\n\
             \x20                   \"Ellipse\" => {{\n\
             \x20                       let radius = (r.size() - egui::vec2(width, width)) * 0.5;\n\
             \x20                       painter.add(egui::Shape::ellipse_filled(r.center(), radius, fill));\n\
             \x20                       painter.add(egui::Shape::ellipse_stroke(r.center(), radius, stroke));\n\
             \x20                   }}\n\
             \x20                   \"Circle\" => {{\n\
             \x20                       painter.circle(r.center(), (side - width) * 0.5, fill, stroke);\n\
             \x20                   }}\n\
             \x20                   _ => {{\n\
             \x20                       painter.rect(r, 0.0, fill, stroke, egui::StrokeKind::Inside);\n\
             \x20                   }}\n\
             \x20               }}\n\
             \x20           }});\n\
             \x20       }}\n"
        );

        Ok(ResultadoComponente {
            campo: format!("    pub {nome}: Shape,\n"),
            padrao: format!(
                "            {nome}: {},\n",
                crate::targets::egui::state::init_of(n, desc, ctx.doc)
            ),
            constante: String::new(),
            corpo: body,
            tipo_usado: "Shape",
            precisa_timer: false,
            precisa_item: false,
            compartilhado: Compartilhado::default(),
        })
    }
}

const ESTADO_SHAPE: StateText = StateText {
    head: r#"
/// A `Shape` on the form.
#[derive(Clone, Debug)]
"#,
    tail: "",
};
