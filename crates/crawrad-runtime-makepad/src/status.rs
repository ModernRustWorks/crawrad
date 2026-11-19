//! **One panel of a StatusBar**: its text, its width and where the
//! text sits, read at run time so `self.status_panel1.text = ...` shows.

use crawrad_note::note;
use makepad_widgets::*;

#[note("status-panel")]
pub fn panel(
    cx: &mut Cx,
    cell: &WidgetRef,
    label: &LabelRef,
    text: &str,
    width: i64,
    alignment: &str,
) {
    if label.text() != text {
        label.set_text(cx, text);
    }
    let w = width.max(0) as f64;
    let x = match alignment {
        "taCenter" => 0.5,
        "taRightJustify" => 1.0,
        _ => 0.0,
    };
    cell.apply_over(cx, live! { width: (w), align: { x: (x), y: 0.5 } });
}
