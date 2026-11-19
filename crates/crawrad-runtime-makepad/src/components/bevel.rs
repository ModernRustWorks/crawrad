//! A `Bevel` on the designer canvas: the same shadow-and-light lines
//! the generated app's shader draws; a spacer shows only a dashed outline,
//! so it can still be found and picked.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-bevel-class")]
pub const CLASS: &str = "Bevel";

#[crawrad_note::note("components-bevel-drawing")]
pub struct Drawing;

#[crawrad_note::note("components-bevel-shadow")]
const SHADOW: [f32; 4] = [0.0, 0.0, 0.0, 0.45];
const LIGHT: [f32; 4] = [1.0, 1.0, 1.0, 0.25];

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let text = |key: &str, default: &'static str| match c.props.get(key) {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.clone(),
            _ => default.to_string(),
        };
        let shape = text("bevel_shape", "Box");
        let (first, second) = if text("bevel_style", "Lowered") == "Raised" {
            (LIGHT, SHADOW)
        } else {
            (SHADOW, LIGHT)
        };
        if shape == "Spacer" {
            p.dashed(r, MAKEPAD_TEXT_DIM);
            return;
        }
        let t = p.medida(1.0).max(1.0);
        let (x, y, w, h) = (r.pos.x, r.pos.y, r.size.x, r.size.y);
        let mut line = |x: f64, y: f64, w: f64, h: f64, colour: [f32; 4]| {
            p.stroke(
                Rect {
                    pos: dvec2(x, y),
                    size: dvec2(w, h),
                },
                colour,
            );
        };
        match shape.as_str() {
            "Box" => {
                line(x, y, w, t, first);
                line(x, y, t, h, first);
                line(x, y + h - t, w, t, second);
                line(x + w - t, y, t, h, second);
            }
            "Frame" => {
                // etched: one outline, and the other one line in
                for (d, colour) in [(0.0, first), (t, second)] {
                    let (fx, fy, fw, fh) = (x + d, y + d, w - t, h - t);
                    line(fx, fy, fw, t, colour);
                    line(fx, fy, t, fh, colour);
                    line(fx, fy + fh - t, fw, t, colour);
                    line(fx + fw - t, fy, t, fh, colour);
                }
            }
            "TopLine" => {
                line(x, y, w, t, first);
                line(x, y + t, w, t, second);
            }
            "BottomLine" => {
                line(x, y + h - 2.0 * t, w, t, first);
                line(x, y + h - t, w, t, second);
            }
            "LeftLine" => {
                line(x, y, t, h, first);
                line(x + t, y, t, h, second);
            }
            "RightLine" => {
                line(x + w - 2.0 * t, y, t, h, first);
                line(x + w - t, y, t, h, second);
            }
            _ => {}
        }
    }
}
