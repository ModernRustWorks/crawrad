//! A `Pagination` component on the designer canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};
use crate::pagination::{PAGINATION_CELL, PAGINATION_GAP, PageSlot, pagination_slots};
use crawrad_note::note;

#[note("components-pagination-class")]
pub const CLASS: &str = "Pagination";

#[note("components-pagination-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let page_count = match c.props.get("page_count") {
            Some(Value::Int(i)) => (*i).max(1) as usize,
            _ => 10,
        };
        let page_index = match c.props.get("page_index") {
            Some(Value::Int(i)) => (*i).max(0) as usize,
            _ => 0,
        };
        let max_buttons = match c.props.get("max_buttons") {
            Some(Value::Int(i)) => (*i).max(1) as usize,
            _ => 7,
        };
        let show_arrows = match c.props.get("show_arrows") {
            Some(Value::Bool(b)) => *b,
            _ => true,
        };

        let alpha = if c.enabled { 1.0 } else { 0.5 };
        let slots = pagination_slots(page_count, page_index, max_buttons, show_arrows);
        let clamped_index = page_index.min(page_count.saturating_sub(1));

        let current_bg = [0.204, 0.353, 0.584, alpha]; // #345a95 selection fill
        let current_fg = [0.933, 0.945, 0.965, alpha]; // #eef1f6
        let outline_col = [0.259, 0.278, 0.353, alpha]; // #42475a
        let text_fg = [0.804, 0.839, 0.957, alpha]; // #cdd6f4
        let disabled_arrow_fg = [0.424, 0.439, 0.525, alpha]; // #6c7086

        let cell_h = PAGINATION_CELL.min(r.size.y as f32);
        let mut x = 0.0f32;

        p.with_clip(r, |p| {
            for slot in &slots {
                let (text, is_current, is_disabled_arrow) = match slot {
                    PageSlot::Prev => ("‹".to_string(), false, clamped_index == 0),
                    PageSlot::Next => (
                        "›".to_string(),
                        false,
                        clamped_index >= page_count.saturating_sub(1),
                    ),
                    PageSlot::Gap => ("…".to_string(), false, false),
                    PageSlot::Page(idx) => ((idx + 1).to_string(), *idx == clamped_index, false),
                };

                let measured = p.measure(&text).x as f32;
                let cell_w = PAGINATION_CELL.max(measured + 8.0);

                if (x + cell_w) as f64 > r.size.x {
                    break;
                }

                let cell_r = Rect {
                    pos: dvec2(
                        r.pos.x + x as f64,
                        r.pos.y + ((r.size.y as f32 - cell_h) * 0.5) as f64,
                    ),
                    size: dvec2(cell_w as f64, cell_h as f64),
                };

                if is_current {
                    p.rounded_box(cell_r, current_bg, current_bg, 3.0);
                    p.write(cell_r, &text, current_fg, true);
                } else if matches!(slot, PageSlot::Gap) {
                    p.write(cell_r, &text, text_fg, true);
                } else {
                    let border_col = outline_col;
                    let empty_bg = [0.0, 0.0, 0.0, 0.0];
                    p.rounded_box(cell_r, empty_bg, border_col, 3.0);
                    let fg = if is_disabled_arrow {
                        disabled_arrow_fg
                    } else {
                        text_fg
                    };
                    p.write(cell_r, &text, fg, true);
                }

                x += cell_w + PAGINATION_GAP;
            }
        });
    }
}
