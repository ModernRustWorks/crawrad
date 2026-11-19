//! Makepad RadioButton component, drawn on the canvas.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-radio-button-classe")]
pub const CLASSE: &str = "RadioButton";

#[crawrad_note::note("components-radio-button-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // A mesma figura do `CheckBox`, com a marca redonda.
    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        super::check_box::marca_com_rotulo(p, c, r, true);
    }
}
