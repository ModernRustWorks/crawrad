//! The drawing of whoever does not have a file of their own yet.
//!
//! A class that entered the catalogue (`crawrad-registry`) and has not earned its
//! painting here **shows up anyway**: a dashed rectangle with the class name
//! inside. That is what the runtime trait promises, and it is the visual sign
//! that a file is missing — not a silent hole on the screen.
//!
//! This is the only drawing that is NOT in the `super::DESENHOS` catalogue: it
//! is the bottom of the dispatch well, and that is why [`crate::CLASSES`]
//! (derived from the catalogue) does not include it.

use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crate::components::{ComponentDrawing, Scene};

#[crawrad_note::note("components-generic-desenho")]
pub struct Desenho;

impl ComponentDrawing for Desenho {
    #[crawrad_note::note("components-generic-desenho-class-name")]
    fn class_name(&self) -> &'static str {
        ""
    }

    fn draw(&self, p: &mut Brush, c: &MakepadCtl, r: Rect, _scene: &Scene) {
        p.dashed(r, MAKEPAD_TEXT_DIM);
        let inside = p.inset(r);
        p.write(inside, &c.class, MAKEPAD_TEXT_DIM, true);
    }
}
