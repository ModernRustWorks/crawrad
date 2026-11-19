//! **Classic RAD `Enabled` on any control**.
//!
//! A generated form stores `enabled` in every control's struct, and before
//! this the Makepad target never handed it to a widget: a disabled control
//! stayed fully interactive. Makepad 1.0.0 does not make it one call:
//!
//! - every widget dims itself with the `disabled` animator
//!   (`WidgetRef::set_disabled`, `makepad-widgets-1.0.0/src/widget.rs:669`);
//! - only `Button` also stops taking clicks, on its own `enabled` field
//!   (`src/button.rs:604`, the gate on `FingerDown` / `FingerUp`; setter
//!   `ButtonRef::set_enabled`, `:860`);
//! - `CheckBox`, `RadioButton` and `TextInput` keep taking input while dimmed,
//!   so the generated `handle_actions` drops a disabled control's events, and a
//!   disabled `Edit` / `Memo` is read-only.
//!
//! [`apply`] is the one call the generated sync makes per control, so the
//! behaviour lives here and not in a hand-written line per class (ADR 0036).

use crawrad_note::note;
use makepad_widgets::*;

use crate::combo_box::CrawradComboBoxWidgetRefExt;

#[note("enabled-mark-disabled")]
fn mark_disabled() -> Vec4 {
    vec4(1.0, 1.0, 1.0, 0.15)
}

#[note("enabled-disabled-mark")]
pub fn disabled_mark(cx: &mut Cx, ui: &WidgetRef, path: &[LiveId], checked: bool) {
    let mark = if checked {
        mark_disabled()
    } else {
        vec4(0.0, 0.0, 0.0, 0.0)
    };
    ui.widget(path)
        .apply_over(cx, live! { draw_bg: { mark_color_disabled: (mark) } });
}

#[note("enabled-apply")]
pub fn apply(cx: &mut Cx, ui: &WidgetRef, path: &[LiveId], enabled: bool) {
    let widget = ui.widget(path);
    if widget.disabled(cx) != enabled {
        return;
    }
    widget.set_disabled(cx, !enabled);
    let button = ui.button(path);
    if button.borrow().is_some() {
        button.set_enabled(cx, enabled);
    }
    let combo_box = ui.crawrad_combo_box(path);
    if combo_box.borrow().is_some() {
        combo_box.set_enabled(cx, enabled);
    }
}
