//! A `GridColumn` — one typed column of a [`super::grid`].
//!
//! Non-visual, and drawn by its owner: the `Grid` paints its columns, so the
//! column itself puts nothing on the canvas while it lives in one.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-grid-column-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        "GridColumn"
    }

    fn visual(&self) -> bool {
        false
    }

    fn drawn_by_owner(&self) -> Option<&'static str> {
        Some("Grid")
    }

    fn draw(&self, _p: &mut Brush, _c: &MakepadCtl, _r: Rect, _scene: &Scene) {}
}
