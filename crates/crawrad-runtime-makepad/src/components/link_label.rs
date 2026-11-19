//! A `LinkLabel` on the designer canvas: the caption in the link
//! colour, underlined as far as the text runs.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-link-label-class")]
pub const CLASS: &str = "LinkLabel";

#[crawrad_note::note("components-link-label-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let color = if c.enabled {
            MAKEPAD_ACCENT
        } else {
            MAKEPAD_TEXT_DIM
        };
        p.write(r, &c.text, color, false);
        let size = p.measure(&c.text);
        let thickness = p.medida(1.0).max(1.0);
        let y = r.pos.y + ((r.size.y - size.y) * 0.5).max(0.0) + size.y - thickness;
        if size.x > 0.0 && y < r.pos.y + r.size.y {
            p.stroke(
                Rect {
                    pos: dvec2(r.pos.x, y),
                    size: dvec2(size.x.min(r.size.x), thickness),
                },
                color,
            );
        }
    }
}
