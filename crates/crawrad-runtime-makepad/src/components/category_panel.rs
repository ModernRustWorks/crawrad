//! A `CategoryPanel` on the designer canvas: the header with its
//! chevron and caption, and the body's frame when open. Folded, its children
//! leave the surface (`crawrad_runtime::in_collapsed_category`) and only the
//! header is drawn.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-category-panel-class")]
pub const CLASS: &str = "CategoryPanel";

#[crawrad_note::note("components-category-panel-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let collapsed = matches!(c.props.get("collapsed"), Some(Value::Bool(true)));
        let header = Rect {
            pos: r.pos,
            size: dvec2(r.size.x, p.medida(24.0).min(r.size.y)),
        };
        if !collapsed {
            p.frame(r, MAKEPAD_BORDER, p.medida(1.0).max(1.0));
        }
        p.rounded_box(header, MAKEPAD_PANEL, MAKEPAD_BORDER, 0.0);
        let side = header.size.y * 0.5;
        let mark = Rect {
            pos: dvec2(header.pos.x + side * 0.5, header.pos.y + side * 0.5),
            size: dvec2(side, side),
        };
        let direction = if collapsed {
            dvec2(1.0, 0.0)
        } else {
            dvec2(0.0, 1.0)
        };
        let ink = if c.enabled {
            MAKEPAD_TEXT
        } else {
            MAKEPAD_TEXT_DIM
        };
        p.arrow(mark, direction, ink);
        let text = Rect {
            pos: dvec2(header.pos.x + side * 2.0, header.pos.y),
            size: dvec2((header.size.x - side * 2.0).max(0.0), header.size.y),
        };
        p.write(text, &c.text, ink, false);
    }
}
