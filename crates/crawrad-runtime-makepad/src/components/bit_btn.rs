//! Makepad BitBtn component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::canvas::atlas::fit_rect;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};
use crawrad_note::note;

pub const CLASSE: &str = "BitBtn";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET, MAKEPAD_BORDER, 3.0);
        let dentro = glyph_room(p, c, p.inset(r));
        p.write_caption(dentro, &c.text, text_color(c), true);
    }
}

#[note("components-bit-btn-glyph-room")]
pub(crate) fn glyph_room(p: &mut Brush, c: &MakepadCtl, inner: Rect) -> Rect {
    let pic = match c.props.get("glyph") {
        Some(Value::Picture(pic)) if !pic.is_empty() => Some(pic),
        _ => None,
    };
    let Some(pic) = pic else {
        return inner;
    };
    let side = p.medida(16.0).min(inner.size.y);
    let slot = Rect {
        pos: makepad_widgets::dvec2(inner.pos.x, inner.pos.y + (inner.size.y - side) / 2.0),
        size: makepad_widgets::dvec2(side, side),
    };
    let num_glyphs = match c.props.get("num_glyphs") {
        Some(Value::Int(n)) => (*n).max(1) as usize,
        _ => 1,
    };
    let bytes = pic.bytes();
    let mut painted = false;
    if let Some((orig_w, orig_h)) = p.picture_size(bytes) {
        let single_w = (orig_w as f64 / num_glyphs as f64).max(1.0);
        let single_h = orig_h as f64;
        let target = fit_rect(slot, single_w, single_h, true, true, true);
        let part = [0.0, 0.0, 1.0 / num_glyphs as f32, 1.0];
        painted = p.picture_part(target, bytes, part);
    }
    if !painted {
        p.rounded_box(slot, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 1.0);
    }
    let gap = side + p.medida(4.0);
    Rect {
        pos: makepad_widgets::dvec2(inner.pos.x + gap, inner.pos.y),
        size: makepad_widgets::dvec2((inner.size.x - gap).max(0.0), inner.size.y),
    }
}
