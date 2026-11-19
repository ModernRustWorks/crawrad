//! Makepad StatusBar component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

pub const CLASSE: &str = "StatusBar";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        p.rounded_box(r, MAKEPAD_PANEL, MAKEPAD_PANEL_BORDER, 0.0);
        // Off SimplePanel the bar shows its panels, as the generated
        // app does — each its own width and alignment, a line between them
        let simple = !matches!(c.props.get("simple_panel"), Some(Value::Bool(false)));
        let panels: Vec<&MakepadCtl> = scene.children_of_class(c, "StatusPanel").collect();
        if simple || panels.is_empty() {
            let dentro = p.inset(r);
            p.write_caption(dentro, &c.text, text_color(c), false);
            return;
        }
        let right = r.pos.x + r.size.x;
        let mut x = r.pos.x;
        for panel in panels {
            let width = match panel.props.get("width") {
                Some(Value::Int(w)) => (*w).max(0) as f32,
                _ => 50.0,
            };
            let w = p.medida(width).min(right - x).max(0.0);
            let cell = Rect {
                pos: dvec2(x, r.pos.y),
                size: dvec2(w, r.size.y),
            };
            let inner = p.inset(cell);
            let text = match panel.props.get("text") {
                Some(Value::Str(s)) => s.as_str(),
                _ => "",
            };
            let tw = p.measure(text).x.min(inner.size.x);
            let start = match panel.props.get("alignment") {
                Some(Value::Enum(a)) if a == "taCenter" => inner.pos.x + (inner.size.x - tw) / 2.0,
                Some(Value::Enum(a)) if a == "taRightJustify" => inner.pos.x + inner.size.x - tw,
                _ => inner.pos.x,
            };
            let area = Rect {
                pos: dvec2(start, inner.pos.y),
                size: dvec2(inner.pos.x + inner.size.x - start, inner.size.y),
            };
            p.write(area, text, text_color(c), false);
            x += w;
            p.stroke(
                Rect {
                    pos: dvec2(x, r.pos.y),
                    size: dvec2(1.0, r.size.y),
                },
                MAKEPAD_PANEL_BORDER,
            );
        }
    }
}
