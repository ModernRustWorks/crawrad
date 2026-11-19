//! A `MultiSelect` component on the designer canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};
use crate::multi_select::multi_select_summary;
use crawrad_note::note;

#[note("components-multi-select-class")]
pub const CLASS: &str = "MultiSelect";

#[note("components-multi-select-drawing")]
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

        // 1. Field background and border
        p.rounded_box(r, bg, border, 2.0);

        // 2. Chevron indicator at the right
        let chevron_w = 20.0f64;
        let chevron_rect = Rect {
            pos: dvec2(r.pos.x + r.size.x - chevron_w, r.pos.y),
            size: dvec2(chevron_w, r.size.y),
        };
        p.arrow(
            crate::components::bar::centered_square(chevron_rect),
            dvec2(0.0, 1.0),
            text_col,
        );

        // 3. Summary text
        let items: Vec<String> = c.itens.iter().map(|it| it.text.clone()).collect();
        let selected: Vec<String> = match c.props.get("selected_items") {
            Some(v) => crawrad_ir::Item::list_from(v)
                .into_iter()
                .map(|it| it.text)
                .collect(),
            None => Vec::new(),
        };
        let hint = match c.props.get("text_hint") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "Select…",
        };
        let max_text_w = (r.size.x - chevron_w - 8.0).max(0.0) as f32;
        let summary = multi_select_summary(&items, &selected, hint, |s| {
            (p.measure(s).x as f32) <= max_text_w
        });

        let text_area = Rect {
            pos: dvec2(r.pos.x + 8.0, r.pos.y),
            size: dvec2(max_text_w as f64, r.size.y),
        };
        p.write(text_area, &summary, text_col, false);
    }
}
