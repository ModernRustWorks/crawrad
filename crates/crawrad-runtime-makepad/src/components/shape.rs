//! Makepad Shape component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

pub const CLASSE: &str = "Shape";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[note("components-shape-desenho-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let colour = |key: &str, default: [f32; 4]| match c.props.get(key) {
            Some(Value::Color(k)) => [k.r, k.g, k.b, k.a].map(|v| f32::from(v) / 255.0),
            _ => default,
        };
        let fill = colour("brush_color", [1.0, 1.0, 1.0, 1.0]);
        let pen = colour("pen_color", [0.0, 0.0, 0.0, 1.0]);
        let kind = match c.props.get("shape") {
            Some(Value::Enum(k)) | Some(Value::Str(k)) => k.as_str(),
            _ => "Rectangle",
        };
        let side = r.size.x.min(r.size.y);
        let square = Rect {
            pos: dvec2(
                r.pos.x + (r.size.x - side) / 2.0,
                r.pos.y + (r.size.y - side) / 2.0,
            ),
            size: dvec2(side, side),
        };
        let (area, radius) = match kind {
            "Square" => (square, 0.0),
            "RoundRect" => (r, side * 0.25),
            "RoundSquare" => (square, side * 0.25),
            "Ellipse" => (r, side * 0.5),
            "Circle" => (square, side * 0.5),
            _ => (r, 0.0),
        };
        p.rounded_box(area, fill, pen, radius);
    }
}
