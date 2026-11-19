//! A `StatusPanel` — one panel of a [`super::status_bar`].
//!
//! Non-visual, and drawn by its owner: the `StatusBar` paints its panels side
//! by side, so the panel itself puts nothing on the canvas (not even a badge)
//! while it lives in one.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-status-panel-drawing")]
pub struct Drawing;

impl ComponentDrawing for Drawing {
    fn class_name(&self) -> &'static str {
        "StatusPanel"
    }

    fn visual(&self) -> bool {
        false
    }

    fn drawn_by_owner(&self) -> Option<&'static str> {
        Some("StatusBar")
    }

    fn draw(&self, _p: &mut Brush, _c: &MakepadCtl, _r: Rect, _scene: &Scene) {}
}
