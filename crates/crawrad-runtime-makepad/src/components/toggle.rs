//! A `Toggle` on the designer canvas: the switch's track, its knob at
//! the left (off) or the right (on), and the caption beside it.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-toggle-class")]
pub const CLASS: &str = "Toggle";

#[crawrad_note::note("components-toggle-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let (tw, th) = (p.medida(32.0), p.medida(16.0));
        let track = Rect {
            pos: dvec2(r.pos.x + p.medida(2.0), r.pos.y + (r.size.y - th) / 2.0),
            size: dvec2(tw, th),
        };
        let fill = if c.marcado {
            MAKEPAD_ACCENT
        } else {
            MAKEPAD_WIDGET
        };
        p.rounded_box(track, fill, MAKEPAD_BORDER, th / 2.0);
        let k = th - p.medida(4.0);
        let x = if c.marcado {
            track.pos.x + tw - k - p.medida(2.0)
        } else {
            track.pos.x + p.medida(2.0)
        };
        let knob = Rect {
            pos: dvec2(x, track.pos.y + p.medida(2.0)),
            size: dvec2(k, k),
        };
        p.rounded_box(knob, MAKEPAD_TEXT, MAKEPAD_TEXT, k / 2.0);
        let caption = Rect {
            pos: dvec2(track.pos.x + tw + p.medida(6.0), r.pos.y),
            size: dvec2(
                (r.pos.x + r.size.x - track.pos.x - tw - p.medida(6.0)).max(0.0),
                r.size.y,
            ),
        };
        p.write_caption(caption, &c.text, text_color(c), false);
    }
}
