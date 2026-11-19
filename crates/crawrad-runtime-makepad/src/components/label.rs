//! Makepad Label component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene, text_color};
use crawrad_note::note;

#[note("components-label-classe")]
pub const CLASSE: &str = "Label";

#[note("components-label-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // The label has no box: it is loose text over the form paper, and
    // therefore writes across the entire rectangle, without indent.
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.write_caption(r, &c.text, text_color(c), false);
    }
}
