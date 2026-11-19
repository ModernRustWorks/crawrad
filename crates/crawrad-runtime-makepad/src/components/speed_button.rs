//! Makepad SpeedButton component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

pub const CLASSE: &str = "SpeedButton";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
        let dentro = super::bit_btn::glyph_room(p, c, p.inset(r));
        p.write_caption(dentro, &c.text, text_color(c), true);
    }
}
