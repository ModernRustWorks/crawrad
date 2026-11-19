//! A `StyleRule` — one conditional style of a [`super::grid_column`].
//!
//! Non-visual, and drawn by its owner: the `Grid` paints the cells the rule
//! styles, so the rule itself puts nothing on the canvas while it lives in a column.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-style-rule-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        "StyleRule"
    }

    fn visual(&self) -> bool {
        false
    }

    fn drawn_by_owner(&self) -> Option<&'static str> {
        Some("GridColumn")
    }

    fn draw(&self, _p: &mut Brush, _c: &MakepadCtl, _r: Rect, _scene: &Scene) {}
}
