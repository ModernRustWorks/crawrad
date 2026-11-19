//! Makepad CheckBox component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-check-box-classe")]
pub const CLASSE: &str = "CheckBox";

#[crawrad_note::note("components-check-box-lado-da-marca")]
pub const LADO_DA_MARCA: f32 = 14.0;

#[crawrad_note::note("components-check-box-vao-do-rotulo")]
pub const VAO_DO_ROTULO: f32 = 6.0;

#[crawrad_note::note("components-check-box-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        marca_com_rotulo(p, c, r, false);
    }
}

#[crawrad_note::note("components-check-box-marca-com-rotulo")]
pub(crate) fn marca_com_rotulo(p: &mut Brush, c: &MakepadCtl, r: Rect, redonda: bool) {
    let lado = p.medida(LADO_DA_MARCA);
    // `Alignment` left puts the caption at the left edge and the box at
    // the right one, as the generated applications draw it
    let left = matches!(
        c.props.get("alignment"),
        Some(crawrad_ir::Value::Enum(a)) if a == "taLeftJustify"
    );
    let box_x = if left {
        r.pos.x + (r.size.x - lado).max(0.0)
    } else {
        r.pos.x
    };
    let marca = Rect {
        pos: dvec2(box_x, r.pos.y + (r.size.y - lado) * 0.5),
        size: dvec2(lado, lado),
    };
    // Round means radius = half the side. `Brush::rounded_box` already corrects the
    // doubled radius of `Sdf2d`, so here the number is the actual radius.
    let raio = if redonda { lado * 0.5 } else { 2.0 };
    p.rounded_box(marca, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, raio);
    if c.marcado {
        if redonda {
            // The radio dot: a filled circle with half the side, centered.
            let ponto = crate::canvas::shrink(marca, lado * 0.28);
            p.rounded_box(ponto, MAKEPAD_TEXT, MAKEPAD_TEXT, ponto.size.x * 0.5);
        } else {
            p.check(marca, MAKEPAD_TEXT);
        }
    }
    let vao = p.medida(VAO_DO_ROTULO);
    let caption_x = if left {
        r.pos.x
    } else {
        marca.pos.x + lado + vao
    };
    let rotulo = Rect {
        pos: dvec2(caption_x, r.pos.y),
        size: dvec2((r.size.x - lado - vao).max(0.0), r.size.y),
    };
    p.write_caption(rotulo, &c.text, text_color(c), false);
}
