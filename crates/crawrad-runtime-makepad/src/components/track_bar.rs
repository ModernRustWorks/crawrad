//! Makepad TrackBar component, drawn on the canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crawrad_ir::Value;
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("components-track-bar-parts")]
pub fn parts(r: Rect, props: &BTreeMap<String, Value>) -> (Rect, Rect, i64) {
    let int = |k: &str, d: i64| match props.get(k) {
        Some(Value::Int(i)) => *i,
        _ => d,
    };
    let position = int("position", 0);
    let f = crate::progress::fraction(position, int("min", 0), int("max", 10));
    let vertical =
        matches!(props.get("orientation"), Some(Value::Enum(o) | Value::Str(o)) if o == "Vertical");
    if vertical {
        let x = r.pos.x + r.size.x * 0.5 - 1.0;
        let track = Rect {
            pos: dvec2(x, r.pos.y),
            size: dvec2(2.0, r.size.y),
        };
        let h = r.size.y * f;
        let fill = Rect {
            pos: dvec2(x, r.pos.y + r.size.y - h),
            size: dvec2(2.0, h),
        };
        (track, fill, position)
    } else {
        let y = r.pos.y + r.size.y - 3.0;
        let track = Rect {
            pos: dvec2(r.pos.x, y),
            size: dvec2(r.size.x, 2.0),
        };
        let fill = Rect {
            pos: dvec2(r.pos.x, y),
            size: dvec2(r.size.x * f, 2.0),
        };
        (track, fill, position)
    }
}

#[note("components-track-bar-look-parts")]
pub fn look_parts(r: Rect, props: &BTreeMap<String, Value>) -> Option<(Rect, Rect, f64)> {
    let look = match props.get("makepad.look") {
        Some(Value::Enum(l) | Value::Str(l)) => l.as_str(),
        _ => return None,
    };
    let (track, fill, _) = parts(r, props);
    if track.size.y > track.size.x {
        return None;
    }
    let f = fill.size.x / track.size.x.max(1.0);
    match look {
        "Box" => {
            let bar = Rect {
                pos: dvec2(r.pos.x, r.pos.y + r.size.y - 11.0),
                size: dvec2(r.size.x, 11.0),
            };
            let line = Rect {
                pos: dvec2(bar.pos.x + 3.0, bar.pos.y + 4.5),
                size: dvec2((bar.size.x - 6.0) * f, 2.0),
            };
            Some((bar, line, 5.5))
        }
        "Round" => {
            let pill = Rect {
                pos: r.pos,
                size: dvec2(r.size.x, r.size.y - 1.0),
            };
            let inner = pill.size.y - 5.0;
            let filled = Rect {
                pos: dvec2(pill.pos.x + 2.5, pill.pos.y + 2.5),
                size: dvec2(inner + (pill.size.x - 5.0 - inner) * f, inner),
            };
            Some((pill, filled, pill.size.y * 0.5))
        }
        _ => None,
    }
}

pub const CLASSE: &str = "TrackBar";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let (track, fill, value) = parts(r, &c.props);
        match look_parts(r, &c.props) {
            Some((outer, filled, radius)) => {
                p.rounded_box(outer, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, radius);
                let inner = (radius - 2.5).max(0.0).min(filled.size.y * 0.5);
                p.rounded_box(filled, MAKEPAD_TEXT_DIM, MAKEPAD_TEXT_DIM, inner);
            }
            None => {
                p.stroke(track, MAKEPAD_WIDGET);
                p.stroke(fill, MAKEPAD_TEXT_DIM);
            }
        }
        let text = value.to_string();
        let w = p.measure(&text).x + 2.0;
        let at = Rect {
            pos: dvec2(r.pos.x + r.size.x - w, r.pos.y),
            size: dvec2(w, r.size.y - 4.0),
        };
        p.write_caption(at, &text, MAKEPAD_TEXT_DIM, false);
    }
}

#[cfg(test)]
#[path = "../test/track_bar/test.rs"]
mod tests;
