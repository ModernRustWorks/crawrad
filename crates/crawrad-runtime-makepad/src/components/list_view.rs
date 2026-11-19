//! Makepad ListView component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::{Rect, dvec2};

use crate::MakepadCtl;
use crate::canvas::{Brush, Content, ContentLine};
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

pub const CLASSE: &str = "ListView";

pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
        let show_headers = !matches!(c.props.get("show_column_headers"), Some(Value::Bool(false)));
        let columns: Vec<&MakepadCtl> = scene.children_of_class(c, "ListColumn").collect();
        let header_h = 24.0;
        let mut top_y = r.pos.y;

        if show_headers && !columns.is_empty() {
            let mut col_x = r.pos.x;
            for col in &columns {
                let col_w_prop = match col.props.get("width") {
                    Some(Value::Int(w)) => (*w).max(0) as f32,
                    _ => 80.0,
                };
                let w = p
                    .medida(col_w_prop)
                    .min((r.pos.x + r.size.x - col_x).max(0.0));
                let header_cell = Rect {
                    pos: dvec2(col_x, top_y),
                    size: dvec2(w, p.medida(header_h)),
                };
                p.rounded_box(header_cell, MAKEPAD_PANEL, MAKEPAD_BORDER, 0.0);
                let col_caption = match col.props.get("text") {
                    Some(Value::Str(s)) => s.as_str(),
                    _ => "",
                };
                let inner = p.inset(header_cell);
                p.write_caption(inner, col_caption, MAKEPAD_TEXT, false);
                col_x += w;
            }
            top_y += p.medida(header_h);
        }

        let body_rect = Rect {
            pos: dvec2(r.pos.x, top_y),
            size: dvec2(r.size.x, (r.pos.y + r.size.y - top_y).max(0.0)),
        };

        let lines: Vec<ContentLine> = c
            .itens
            .iter()
            .map(|it| {
                let text = if it.children.is_empty() {
                    it.text.clone()
                } else {
                    let sub: Vec<String> = it.children.iter().map(|ch| ch.text.clone()).collect();
                    format!("{}  |  {}", it.text, sub.join("  |  "))
                };
                ContentLine::leaf(0, text)
            })
            .collect();

        let row_h = 20.0;
        let dentro = p.inset(body_rect);
        p.content(
            dentro,
            &Content {
                lines: &lines,
                line_height: row_h,
                color: text_color(c),
                selected: c.escolhido,
            },
        );
    }
}
