//! **The third state of a check box**: `cbGrayed`, "neither".
//!
//! Makepad 1.0.0's `CheckBox` has two states: its `active` animator drives
//! the tick from 0 (off) to 1 (on) (`check_box.rs:1526-1549`). The grayed
//! mark is that same tick held halfway, which reads as "partly", and not a
//! drawing of our own: the theme still paints it.

use crawrad_note::note;
use makepad_widgets::*;

#[note("check-align")]
pub fn align(cx: &mut Cx, lead: &LabelRef, check: &CheckBoxRef, text: &str, alignment: &str) {
    let left = alignment == "taLeftJustify";
    let (lead_text, box_text) = if left { (text, "") } else { ("", text) };
    if lead.text() == lead_text && check.text() == box_text {
        return;
    }
    lead.set_text(cx, lead_text);
    check.set_text(box_text);
    if left {
        lead.apply_over(cx, live! { width: Fill });
        check.apply_over(cx, live! { width: Fit });
    } else {
        lead.apply_over(cx, live! { width: 0.0 });
        check.apply_over(cx, live! { width: Fill });
    }
}

#[note("check-grayed")]
pub fn grayed(cx: &mut Cx, widget: &WidgetRef, grayed: bool) {
    if grayed {
        widget.apply_over(cx, live! { draw_bg: { active: 0.5 } });
    }
}
