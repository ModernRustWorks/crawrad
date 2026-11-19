//! A `ListColumn` — one column of a [`super::list_view`].
//!
//! Non-visual, and drawn by its owner: the `ListView` paints its columns,
//! so the column itself puts nothing on the canvas while it lives in one.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-list-column-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        "ListColumn"
    }

    fn visual(&self) -> bool {
        false
    }

    fn drawn_by_owner(&self) -> Option<&'static str> {
        Some("ListView")
    }

    fn draw(&self, _p: &mut Brush, _c: &MakepadCtl, _r: Rect, _scene: &Scene) {}
}
