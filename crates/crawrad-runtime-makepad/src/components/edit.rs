//! Makepad Edit / TextInput component, drawn on the canvas.

use crawrad_ir::Value;
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-edit-classe")]
pub const CLASSE: &str = "Edit";

#[crawrad_note::note("components-edit-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // The field SINKS: background darker than the paper, with an outline. And its
    // text does NOT pass through `sem_acelerador` — typed content is content, and
    // an `&` there is an `&`, as in the RXP (`crawrad-runtime-egui/src/lib.rs:2492-2498`).
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        draw_text_box(p, c, r, &c.text);
    }
}

#[crawrad_note::note("components-edit-draw-text-box")]
pub fn draw_text_box(p: &mut Brush, c: &MakepadCtl, r: Rect, text: &str) {
    p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
    let dentro = p.inset(r);

    let alignment = match c.props.get("alignment") {
        Some(Value::Enum(s)) | Some(Value::Str(s)) => s.as_str(),
        _ => "taLeftJustify",
    };

    let pwd_char = match c.props.get("password_char") {
        Some(Value::Str(s)) if !s.is_empty() => s.chars().next(),
        _ => None,
    };

    if !text.is_empty() {
        let display_text = if let Some(ch) = pwd_char {
            ch.to_string().repeat(text.chars().count())
        } else {
            text.to_string()
        };
        p.write_aligned(dentro, &display_text, text_color(c), alignment);
    } else {
        let text_hint = match c.props.get("text_hint") {
            Some(Value::Str(s)) => s.as_str(),
            _ => "",
        };
        if !text_hint.is_empty() {
            p.write_aligned(dentro, text_hint, MAKEPAD_TEXT_DIM, alignment);
        }
    }
}
