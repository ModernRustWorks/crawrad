//! A `RadioGroup` on the designer canvas: the group box's frame and
//! caption, and a ring per item where the generated application puts it —
//! the cells come from [`crate::radio_group::cells`], the same layout the
//! running widget uses — with the dot on `ItemIndex`.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-radio-group-class")]
pub const CLASS: &str = "RadioGroup";

#[crawrad_note::note("components-radio-group-mark")]
const MARK: f32 = 12.0;

#[crawrad_note::note("components-radio-group-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        // the frame and the caption are the group box's
        super::group_box::Desenho.draw(p, c, r, scene);
        let columns = match c.props.get("columns") {
            Some(Value::Int(n)) => *n,
            _ => 1,
        };
        let items = &c.itens;
        for (i, (x, y, _, ch)) in
            crate::radio_group::cells(c.rect.w, c.rect.h, items.len(), columns)
                .into_iter()
                .enumerate()
        {
            let side = p.medida(MARK);
            let ring = Rect {
                pos: dvec2(
                    r.pos.x + p.medida(x),
                    r.pos.y + p.medida(y + (ch - MARK) / 2.0),
                ),
                size: dvec2(side, side),
            };
            p.rounded_box(ring, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, side * 0.5);
            if c.escolhido == Some(i) {
                let dot = crate::canvas::shrink(ring, side * 0.28);
                p.rounded_box(dot, MAKEPAD_TEXT, MAKEPAD_TEXT, dot.size.x * 0.5);
            }
            let text_at = Rect {
                pos: dvec2(ring.pos.x + side + p.medida(6.0), r.pos.y + p.medida(y)),
                size: dvec2(
                    (r.pos.x + r.size.x - ring.pos.x - side - p.medida(6.0)).max(0.0),
                    p.medida(ch),
                ),
            };
            p.write_caption(text_at, &items[i].text, text_color(c), false);
        }
    }
}
