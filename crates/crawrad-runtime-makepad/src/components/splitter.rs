//! Makepad Splitter component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::panel::caixa_de_superficie;
use crate::components::{ComponentDrawing, Scene};

pub const CLASSE: &str = "Splitter";

pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    fn draw(&self, p: &mut Brush, _c: &MakepadCtl, r: Rect, _scene: &Scene) {
        caixa_de_superficie(p, r);
    }
}
