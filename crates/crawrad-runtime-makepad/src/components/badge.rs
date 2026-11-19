//! A `Badge` on the designer canvas: pill label with kind tones.

use crawrad_ir::Value;
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-badge-class")]
pub const CLASS: &str = "Badge";

#[note("components-badge-drawing")]
pub struct Drawing;

fn kind_colors(kind: &str) -> ([f32; 4], [f32; 4]) {
    match kind {
        "bkDefault" => ([0.22, 0.22, 0.28, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkPrimary" => ([0.23, 0.35, 0.55, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkInfo" => ([0.18, 0.50, 0.72, 1.0], [0.95, 0.95, 0.98, 1.0]),
        "bkWarning" => ([0.82, 0.55, 0.15, 1.0], [0.10, 0.10, 0.15, 1.0]),
        "bkDanger" => ([0.80, 0.22, 0.22, 1.0], [0.95, 0.95, 0.98, 1.0]),
        _ => ([0.23, 0.35, 0.55, 1.0], [0.95, 0.95, 0.98, 1.0]),
    }
}

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        let count = match c.props.get("count") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        };
        let max_count = match c.props.get("max_count") {
            Some(Value::Int(i)) => *i,
            _ => 99,
        };
        let Some(text) = crate::badge::badge_text(&c.text, count, max_count) else {
            return;
        };

        let kind = match c.props.get("kind") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "bkPrimary",
        };
        let effective_kind = if count > 0 && (kind == "bkPrimary" || kind.is_empty()) {
            "bkDanger"
        } else {
            kind
        };

        let outline = matches!(c.props.get("outline"), Some(Value::Bool(true)));
        let (bg, fg) = kind_colors(effective_kind);
        let alpha = if c.enabled { 1.0 } else { 0.5 };

        let h = r.size.y;
        let w = if text.chars().count() == 1 {
            h
        } else {
            r.size.x.max(h)
        };
        let radius = (h * 0.5).max(0.0);

        let associate = match c.props.get("associate") {
            Some(Value::Ref(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };

        let badge_rect = if !associate.is_empty() {
            if let Some(target) = scene.find(associate) {
                let tr = p.na_tela(target.rect);
                Rect {
                    pos: makepad_widgets::dvec2(
                        tr.pos.x + tr.size.x - (w * 0.5),
                        tr.pos.y - (h * 0.5),
                    ),
                    size: makepad_widgets::dvec2(w, h),
                }
            } else {
                Rect {
                    pos: r.pos,
                    size: makepad_widgets::dvec2(w, h),
                }
            }
        } else {
            Rect {
                pos: r.pos,
                size: makepad_widgets::dvec2(w, h),
            }
        };

        if outline {
            let stroke_col = [bg[0], bg[1], bg[2], bg[3] * alpha];
            let fill_col = [0.0, 0.0, 0.0, 0.0];
            p.rounded_box(badge_rect, fill_col, stroke_col, radius);
            p.write(badge_rect, &text, stroke_col, true);
        } else {
            let fill_col = [bg[0], bg[1], bg[2], bg[3] * alpha];
            let border_col = [0.0, 0.0, 0.0, 0.0];
            let text_col = [fg[0], fg[1], fg[2], fg[3] * alpha];
            p.rounded_box(badge_rect, fill_col, border_col, radius);
            p.write(badge_rect, &text, text_col, true);
        }
    }
}
