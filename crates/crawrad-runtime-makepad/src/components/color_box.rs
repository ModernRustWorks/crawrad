//! A `ColorBox` on the designer canvas: the swatch of `Selected`,
//! its name from the standard colours, and the list's drop arrow.

use crawrad_ir::{Color, Value};
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-color-box-class")]
pub const CLASS: &str = "ColorBox";

#[crawrad_note::note("components-color-box-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let chosen = match c.props.get("selected") {
            Some(Value::Color(k)) => *k,
            _ => Color::STANDARD[0].1,
        };
        let s = p.medida(14.0).min(r.size.y);
        let swatch = Rect {
            pos: dvec2(r.pos.x + p.medida(4.0), r.pos.y + (r.size.y - s) / 2.0),
            size: dvec2(s, s),
        };
        let fill = [chosen.r, chosen.g, chosen.b, chosen.a].map(|v| f32::from(v) / 255.0);
        p.rounded_box(swatch, fill, MAKEPAD_BORDER, 2.0);
        let name = Color::STANDARD
            .iter()
            .find(|(_, k)| *k == chosen)
            .map_or("Custom", |(n, _)| n);
        let arrow = p.medida(16.0).min(r.size.x);
        let caption = Rect {
            pos: dvec2(swatch.pos.x + s + p.medida(4.0), r.pos.y),
            size: dvec2(
                (r.pos.x + r.size.x - arrow - swatch.pos.x - s - p.medida(4.0)).max(0.0),
                r.size.y,
            ),
        };
        p.write(caption, name, text_color(c), false);
        let drop = Rect {
            pos: dvec2(r.pos.x + r.size.x - arrow, r.pos.y),
            size: dvec2(arrow, r.size.y),
        };
        p.arrow(
            crate::canvas::shrink(drop, r.size.y * 0.3),
            dvec2(0.0, 1.0),
            text_color(c),
        );
    }
}
