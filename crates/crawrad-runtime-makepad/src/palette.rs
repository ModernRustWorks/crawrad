//! **The glue every palette widget shares, written once**.
//!
//! `Chips`, `Breadcrumb`, `MultiSelect`, `Pagination`, `ListBox` and
//! `ListView` each carried their own copy of `set_look` and (four of them)
//! `set_enabled` — and a defect copied with them: on 17/10 four were
//! born disabled, the same line wrong in four places. `palette_glue!` gives
//! a widget's `Ref` both setters; the widget keeps only what is its own, its
//! [`TakesLook::take_look`] (which of its draw fields take the font).

use crate::list::ListLook;
use makepad_widgets::Cx;

#[note("palette-takeslook")]
pub trait TakesLook {
    fn take_look(&mut self, cx: &mut Cx, look: ListLook);
}

/// `palette_glue!(CrawradChipsRef)` gives the widget's `Ref` a `set_look` that
/// stores the look, lets the widget take it and redraws — only when it
/// changed (the sync runs after every event). `palette_glue!(CrawradChipsRef,
/// enabled)` adds `set_enabled`, for a widget with an `enabled` field (born
/// `true`, see `palette_widgets_are_born_enabled`).
macro_rules! palette_glue {
    ($widget_ref:ident) => {
        impl $widget_ref {
            /// The font and colours the form gave it; the same look
            /// again does nothing.
            pub fn set_look(&self, cx: &mut Cx, look: $crate::list::ListLook) {
                if let Some(mut inner) = self.borrow_mut() {
                    if inner.look != look {
                        inner.look = look;
                        $crate::palette::TakesLook::take_look(&mut *inner, cx, look);
                        inner.redraw(cx);
                    }
                }
            }
        }
    };
    ($widget_ref:ident, enabled) => {
        $crate::palette::palette_glue!($widget_ref);
        impl $widget_ref {
            /// Enabled or not (`Enabled` in the Inspector); disabled, the
            /// widget ignores the mouse and draws faded.
            pub fn set_enabled(&self, cx: &mut Cx, enabled: bool) {
                if let Some(mut inner) = self.borrow_mut() {
                    if inner.enabled != enabled {
                        inner.enabled = enabled;
                        inner.redraw(cx);
                    }
                }
            }
        }
    };
}
use crawrad_note::note;
pub(crate) use palette_glue;
