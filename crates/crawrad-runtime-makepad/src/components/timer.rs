//! The `Timer` — periodic firing, as in a classic RAD IDE.
//!
//! Non-visual: in the designer it is the badge from `super::nao_visual`, and in
//! the application it is code, never screen.

use crate::components::non_visual::Badge;
use crawrad_note::note;

#[note("components-timer-desenho")]
pub static DESENHO: Badge = Badge::for_class("Timer");
