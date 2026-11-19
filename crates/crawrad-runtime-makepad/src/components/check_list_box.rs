//! A `CheckListBox` component on the designer canvas.

use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-check-list-box-class")]
pub const CLASS: &str = "CheckListBox";

#[crawrad_note::note("components-check-list-box-row-height")]
pub const ROW_HEIGHT: f32 = 22.0;

#[crawrad_note::note("components-check-list-box-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASS
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        let alpha = if c.enabled { 1.0 } else { 0.5 };
        let mut bg = MAKEPAD_WIDGET_INPUT;
        let mut border = MAKEPAD_BORDER;
        let mut text_col = MAKEPAD_TEXT;
        bg[3] *= alpha;
        border[3] *= alpha;
        text_col[3] *= alpha;

        // Container background and border
        p.rounded_box(r, bg, border, 2.0);

        let items: Vec<String> = if !c.itens.is_empty() {
            c.itens.iter().map(|it| it.text.clone()).collect()
        } else {
            match c.props.get("items") {
                Some(v) => crawrad_ir::Item::list_from(v)
                    .into_iter()
                    .map(|it| it.text)
                    .collect(),
                None => Vec::new(),
            }
        };

        let checked: Vec<String> = match c.props.get("checked_items") {
            Some(v) => crawrad_ir::Item::list_from(v)
                .into_iter()
                .map(|it| it.text)
                .collect(),
            None => Vec::new(),
        };

        let row_h = p.medida(ROW_HEIGHT);
        let box_size = p.medida(14.0);
        let pad_x = p.medida(6.0);
        let gap_x = p.medida(6.0);

        let mut y = r.pos.y;
        for it in &items {
            if y + row_h > r.pos.y + r.size.y {
                break;
            }

            let box_y = y + (row_h - box_size) * 0.5;
            let check_rect = Rect {
                pos: dvec2(r.pos.x + pad_x, box_y),
                size: dvec2(box_size, box_size),
            };

            p.rounded_box(check_rect, bg, border, 2.0);

            if checked.iter().any(|c| c == it) {
                p.check(check_rect, text_col);
            }

            let text_x = check_rect.pos.x + box_size + gap_x;
            let text_w = (r.pos.x + r.size.x - text_x - pad_x).max(0.0);
            let text_rect = Rect {
                pos: dvec2(text_x, y),
                size: dvec2(text_w, row_h),
            };
            p.write(text_rect, it, text_col, false);

            y += row_h;
        }
    }
}
