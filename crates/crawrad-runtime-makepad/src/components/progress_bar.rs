//! Makepad ProgressBar component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crawrad_ir::Value;
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("components-progress-bar-fill")]
pub fn fill(r: Rect, props: &BTreeMap<String, Value>) -> Rect {
    let int = |k: &str, d: i64| match props.get(k) {
        Some(Value::Int(i)) => *i,
        _ => d,
    };
    let f = crate::progress::fraction(int("position", 0), int("min", 0), int("max", 100));
    let inner = Rect {
        pos: r.pos + dvec2(1.0, 1.0),
        size: dvec2((r.size.x - 2.0).max(1.0), (r.size.y - 2.0).max(1.0)),
    };
    if matches!(props.get("orientation"), Some(Value::Enum(o) | Value::Str(o)) if o == "Vertical") {
        let h = inner.size.y * f;
        Rect {
            pos: dvec2(inner.pos.x, inner.pos.y + inner.size.y - h),
            size: dvec2(inner.size.x, h),
        }
    } else {
        Rect {
            pos: inner.pos,
            size: dvec2(inner.size.x * f, inner.size.y),
        }
    }
}

pub const CLASSE: &str = "ProgressBar";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        p.stroke(fill(r, &c.props), MAKEPAD_ACCENT);
    }
}
