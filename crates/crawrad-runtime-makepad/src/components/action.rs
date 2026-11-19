//! The `Action` — an action inside a [`super::action_list`].
//!
//! Non-visual, and doubly invisible on the canvas: besides having no appearance of its own,
//! it lives INSIDE a non-visual, and `crawrad_runtime::na_superficie` removes from the
//! screen everything that lives inside there (it is the `ActionList` editor that shows it, as
//! in a classic RAD IDE). Its badge exists in case it appears loose.

use crate::components::non_visual::Badge;
use crawrad_note::note;

#[note("components-action-desenho")]
pub static DESENHO: Badge = Badge::for_class("Action");
