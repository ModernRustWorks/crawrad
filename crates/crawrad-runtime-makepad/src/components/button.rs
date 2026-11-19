//! Makepad Button component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene, text_color};

#[crawrad_note::note("components-button-classe")]
pub const CLASSE: &str = "Button";

#[crawrad_note::note("components-button-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // The button RISES: a face lighter than the paper and an outline around it. It is
    // the value hierarchy required by the owner — without it the button and the text
    // field were the same dark rectangle.
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.rounded_box(r, MAKEPAD_WIDGET, MAKEPAD_BORDER, 3.0);
        let dentro = p.inset(r);
        p.write_caption(dentro, &c.text, text_color(c), true);
    }
}
