//! A `MenuItem` — one item of a [`super::main_menu`] or a `PopupMenu`.
//!
//! Non-visual and inside a non-visual, so `crawrad_runtime::na_superficie`
//! keeps it off the canvas, as an `Action` inside its `ActionList`; the
//! MainMenu's bar shows the top-level ones. The badge is for an item found
//! loose.

use crate::components::non_visual::Badge;
use crawrad_note::note;

#[note("components-menu-item-drawing")]
pub static DRAWING: Badge = Badge::for_class("MenuItem");
