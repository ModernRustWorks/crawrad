//! A `SpinEdit` on the designer canvas: the field with its `Value`,
//! and the up and down buttons at its right, where the generated
//! applications put them.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-spin-edit-class")]
pub const CLASS: &str = "SpinEdit";

#[crawrad_note::note("components-spin-edit-steps")]
const STEPS: f32 = 16.0;

#[crawrad_note::note("components-spin-edit-inset")]
const INSET: f32 = 2.0;

#[crawrad_note::note("components-spin-edit-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let steps = p.medida(STEPS).min(r.size.x);
        let field = Rect {
            pos: r.pos,
            size: dvec2((r.size.x - steps).max(0.0), r.size.y),
        };
        let value = match c.props.get("value") {
            Some(Value::Int(v)) => v.to_string(),
            _ => "0".to_string(),
        };
        p.write(p.inset(field), &value, text_color(c), false);
        let half = r.size.y / 2.0;
        let inset = p.medida(INSET);
        for (i, dir) in [dvec2(0.0, -1.0), dvec2(0.0, 1.0)].into_iter().enumerate() {
            let button = Rect {
                pos: dvec2(
                    r.pos.x + r.size.x - steps - inset,
                    r.pos.y + inset + (half - inset) * i as f64,
                ),
                size: dvec2(steps, half - inset - 0.5),
            };
            p.rounded_box(button, MAKEPAD_WIDGET, MAKEPAD_WIDGET, 0.0);
            p.arrow(
                crate::canvas::shrink(button, half * 0.25),
                dir,
                text_color(c),
            );
        }
    }
}
