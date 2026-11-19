//! Makepad ScrollBar component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::bar::{ScrollBarState, paint_bar};
use crate::components::{ComponentDrawing, Scene};
use crawrad_note::note;

#[note("components-scroll-bar-classe")]
pub const CLASSE: &str = "ScrollBar";

#[note("components-scroll-bar-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    #[note("components-scroll-bar-desenho-draw")]
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        paint_bar(p, r, c.posicao_scroll, ScrollBarState::asked_for(c.enabled));
    }
}
