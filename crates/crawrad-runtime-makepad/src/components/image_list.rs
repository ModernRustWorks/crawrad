//! The `ImageList` — the form's collection of icons, as in a classic RAD IDE.
//!
//! Non-visual: in the designer it is the badge from `super::nao_visual`, and in
//! the application no widget exists.

use crate::components::non_visual::Badge;
use crawrad_note::note;

#[note("components-image-list-desenho")]
pub static DESENHO: Badge = Badge::for_class("ImageList");
