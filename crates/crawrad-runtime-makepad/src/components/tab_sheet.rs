//! A `PageControl` page — the surface where its children live.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-tab-sheet-classe")]
pub const CLASSE: &str = "TabSheet";

#[crawrad_note::note("components-tab-sheet-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    fn class_name(&self) -> &'static str {
        CLASSE
    }

    // The page caption is drawn by the `PageControl`, on the tab strip — the
    // page itself is just the bottom surface.
    fn draw(&self, p: &mut Brush, _c: &MakepadCtl, r: Rect, _scene: &Scene) {
        super::panel::caixa_de_superficie(p, r);
    }
}
