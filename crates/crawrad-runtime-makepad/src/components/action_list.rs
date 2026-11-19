//! `ActionList` — the form's action list, as in a classic RAD IDE.
//!
//! Non-visual: it has no appearance in the application, and in the designer it appears as the
//! badge from `super::nao_visual`. This file exists to hold the class name
//! **once**, which is what the catalogue dispatch compares.

use crate::components::non_visual::Badge;
use crawrad_note::note;

#[note("components-action-list-desenho")]
pub static DESENHO: Badge = Badge::for_class("ActionList");
