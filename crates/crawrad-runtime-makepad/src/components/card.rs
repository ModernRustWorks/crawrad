//! A `Card` on the designer canvas: rounded container surface with
//! elevation shadow and optional header.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-card-class")]
pub const CLASS: &str = "Card";

#[crawrad_note::note("components-card-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let corner_radius = match c.props.get("corner_radius") {
            Some(Value::Int(i)) => (*i).max(0) as f64,
            _ => 8.0,
        };
        let elevation = match c.props.get("elevation") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        };
        let show_header = !matches!(c.props.get("show_header"), Some(Value::Bool(false)));
        let subtitle = match c.props.get("subtitle") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };

        let radius = p.medida(corner_radius as f32);

        // Shadow layers offset down
        if elevation > 0 {
            for i in 1..=elevation.min(3) {
                let offset_y = p.medida(i as f32 * 1.5);
                let shadow_rect = Rect {
                    pos: dvec2(r.pos.x, r.pos.y + offset_y),
                    size: r.size,
                };
                let alpha = (0.05 * (4 - i) as f32).min(0.2);
                p.rounded_box(
                    shadow_rect,
                    [0.0, 0.0, 0.0, alpha],
                    [0.0, 0.0, 0.0, 0.0],
                    radius,
                );
            }
        }

        // Card surface and outline
        p.rounded_box(r, MAKEPAD_PANEL, MAKEPAD_BORDER, radius);

        if !show_header {
            return;
        }

        let has_sub = !subtitle.is_empty();
        let header_h = if has_sub { 42.0 } else { 28.0 };
        let header = Rect {
            pos: r.pos,
            size: dvec2(r.size.x, p.medida(header_h).min(r.size.y)),
        };

        let ink = if c.enabled {
            MAKEPAD_TEXT
        } else {
            MAKEPAD_TEXT_DIM
        };

        let pad_x = p.medida(12.0);
        if has_sub {
            let title_rect = Rect {
                pos: dvec2(header.pos.x + pad_x, header.pos.y + p.medida(6.0)),
                size: dvec2((header.size.x - pad_x * 2.0).max(0.0), p.medida(16.0)),
            };
            p.write_caption(title_rect, &c.text, ink, false);

            let sub_rect = Rect {
                pos: dvec2(header.pos.x + pad_x, header.pos.y + p.medida(22.0)),
                size: dvec2((header.size.x - pad_x * 2.0).max(0.0), p.medida(14.0)),
            };
            p.write(sub_rect, subtitle, MAKEPAD_TEXT_DIM, false);
        } else {
            let title_rect = Rect {
                pos: dvec2(
                    header.pos.x + pad_x,
                    header.pos.y + (header.size.y - p.medida(16.0)) * 0.5,
                ),
                size: dvec2((header.size.x - pad_x * 2.0).max(0.0), p.medida(16.0)),
            };
            p.write_caption(title_rect, &c.text, ink, false);
        }

        // Thin divider under the header
        let divider_y = header.pos.y + header.size.y;
        p.stroke(
            Rect {
                pos: dvec2(r.pos.x, divider_y),
                size: dvec2(r.size.x, p.medida(1.0).max(1.0)),
            },
            MAKEPAD_BORDER,
        );
    }
}
