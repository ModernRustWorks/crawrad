//! A `Drawer` on the designer canvas: side panel with header,
//! optional close button, child controls, and shadow.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-drawer-class")]
pub const CLASS: &str = "Drawer";

#[crawrad_note::note("components-drawer-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let side = match c.props.get("side") {
            Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
            _ => "dsLeft",
        };
        let opened = matches!(c.props.get("opened"), Some(Value::Bool(true)));
        let show_header = !matches!(c.props.get("show_header"), Some(Value::Bool(false)));

        // Shadow opposite to Side (dsLeft -> shadow on right edge; dsRight -> shadow on left edge)
        let is_right = side == "dsRight";
        for i in 1..=3 {
            let offset_x = p.medida(i as f32 * 2.0);
            let shadow_rect = if is_right {
                Rect {
                    pos: dvec2(r.pos.x - offset_x, r.pos.y),
                    size: dvec2(offset_x, r.size.y),
                }
            } else {
                Rect {
                    pos: dvec2(r.pos.x + r.size.x, r.pos.y),
                    size: dvec2(offset_x, r.size.y),
                }
            };
            let alpha = (0.05 * (4 - i) as f32).min(0.2);
            p.rounded_box(
                shadow_rect,
                [0.0, 0.0, 0.0, alpha],
                [0.0, 0.0, 0.0, 0.0],
                0.0,
            );
        }

        // Drawer panel surface and border
        p.rounded_box(r, MAKEPAD_PANEL, MAKEPAD_BORDER, 0.0);

        if !opened {
            p.dashed(r, MAKEPAD_BORDER);
        }

        if !show_header {
            return;
        }

        let header_h = p.medida(32.0);
        let header = Rect {
            pos: r.pos,
            size: dvec2(r.size.x, header_h.min(r.size.y)),
        };

        let ink = if c.enabled {
            MAKEPAD_TEXT
        } else {
            MAKEPAD_TEXT_DIM
        };

        let close_size = header_h.min(header.size.x);
        let (close_rect, title_rect) = if is_right {
            let cr = Rect {
                pos: header.pos,
                size: dvec2(close_size, close_size),
            };
            let tr = Rect {
                pos: dvec2(header.pos.x + close_size + p.medida(8.0), header.pos.y),
                size: dvec2(
                    (header.size.x - close_size - p.medida(16.0)).max(0.0),
                    header.size.y,
                ),
            };
            (cr, tr)
        } else {
            let cr = Rect {
                pos: dvec2(header.pos.x + header.size.x - close_size, header.pos.y),
                size: dvec2(close_size, close_size),
            };
            let tr = Rect {
                pos: dvec2(header.pos.x + p.medida(12.0), header.pos.y),
                size: dvec2(
                    (header.size.x - close_size - p.medida(16.0)).max(0.0),
                    header.size.y,
                ),
            };
            (cr, tr)
        };

        let caption = if !opened {
            format!("{} (closed)", c.text)
        } else {
            c.text.clone()
        };

        p.write_caption(title_rect, &caption, ink, false);
        p.write(close_rect, "×", MAKEPAD_TEXT_DIM, true);

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
