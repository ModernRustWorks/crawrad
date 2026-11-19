//! Makepad PaintBox component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

pub const CLASSE: &str = "PaintBox";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.dashed(r, MAKEPAD_BORDER);
        let dentro = p.inset(r);
        p.write_caption(dentro, &c.name, text_color(c), true);
    }
}
