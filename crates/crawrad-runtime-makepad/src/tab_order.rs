//! **Tab walks the form's controls in `TabOrder`**, as in classic RAD.
//!
//! makepad 1.0 has no Tab traversal at all: a `TextInput` reports the key as
//! unhandled and the focus stays put. The generated `handle_event` hands
//! every key to [`tab_key`] before the widgets see it, with the form's stops
//! already in Tab order (the generator sorts them: the `TabOrder` of each
//! container down to the control, then its own). Tab focuses the next stop
//! that is visible and enabled, Shift+Tab the previous one, wrapping round.
//! A `Memo` with `WantTabs` keeps its Tab and types it.

use crawrad_note::note;
use makepad_widgets::*;

#[note("tab-order-tabstop")]
pub type TabStop = (bool, bool, WidgetRef);

#[note("tab-order-tab-key")]
pub fn tab_key(cx: &mut Cx, stops: &[TabStop], event: &Event) -> bool {
    let Event::KeyDown(key) = event else {
        return false;
    };
    let m = key.modifiers;
    if key.key_code != KeyCode::Tab || m.control || m.alt || m.logo {
        return false;
    }
    let at = stops.iter().position(|(_, _, w)| w.key_focus(cx));
    if at.is_some_and(|i| stops[i].1) {
        return false;
    }
    let active: Vec<bool> = stops.iter().map(|(on, _, _)| *on).collect();
    let Some(next) = next(at, &active, !m.shift) else {
        return false;
    };
    stops[next].2.set_key_focus(cx);
    true
}

#[note("tab-order-next")]
pub fn next(at: Option<usize>, active: &[bool], forward: bool) -> Option<usize> {
    let count = active.len();
    let start = match (at, forward) {
        (Some(i), _) => i,
        (None, true) => count.checked_sub(1)?,
        (None, false) => 0,
    };
    (1..=count)
        .map(|step| {
            if forward {
                (start + step) % count
            } else {
                (start + count - step % count) % count
            }
        })
        .find(|i| active[*i])
}

#[cfg(test)]
#[path = "test/tab_order/test.rs"]
mod tests;
