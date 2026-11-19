//! **The menus of a generated form**: the MainMenu's bar and
//! each drop-down list are plain views the generator writes into the form;
//! this places a list where it opens and words an item's caption.
//!
//! A list is a hidden `View` in the form's body, over every control. Opening
//! it is moving it to the point (relative to the body) and showing it; a
//! press anywhere outside the open lists and the bar closes them.

use crawrad_note::note;
use makepad_widgets::*;

#[note("menu-caption")]
pub fn caption(text: &str, checked: bool, shortcut: &str) -> String {
    let mark = if checked { "\u{2713} " } else { "    " };
    let text = without_accelerator(text);
    if shortcut.is_empty() {
        format!("{mark}{text}")
    } else {
        format!("{mark}{text}    {shortcut}")
    }
}

#[note("menu-entry")]
pub fn entry(text: &str, checked: bool, shortcut: &str) -> String {
    let text = without_accelerator(text);
    let text = if checked {
        format!("\u{2713} {text}")
    } else {
        text
    };
    if shortcut.is_empty() {
        text
    } else {
        format!("{text}    {shortcut}")
    }
}

#[note("menu-without-accelerator")]
pub fn without_accelerator(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                out.push('&');
                chars.next();
            }
            continue;
        }
        out.push(c);
    }
    out
}

#[note("menu-open")]
pub fn open(cx: &mut Cx, list: &WidgetRef, body: &WidgetRef, at: DVec2) {
    let origin = body.area().rect(cx).pos;
    let x = (at.x - origin.x).max(0.0);
    let y = (at.y - origin.y).max(0.0);
    list.apply_over(cx, live! { margin: { left: (x), top: (y) } });
    list.set_visible(cx, true);
    list.redraw(cx);
}

#[note("menu-below")]
pub fn below(cx: &mut Cx, widget: &WidgetRef) -> DVec2 {
    let r = widget.area().rect(cx);
    dvec2(r.pos.x, r.pos.y + r.size.y)
}

#[note("menu-beside")]
pub fn beside(cx: &mut Cx, widget: &WidgetRef) -> DVec2 {
    let r = widget.area().rect(cx);
    dvec2(r.pos.x + r.size.x, r.pos.y)
}

#[note("menu-covers")]
pub fn covers(cx: &mut Cx, widget: &WidgetRef, at: DVec2) -> bool {
    widget.visible() && widget.area().rect(cx).contains(at)
}

#[note("menu-close")]
pub fn close(cx: &mut Cx, list: &WidgetRef) {
    if list.visible() {
        list.set_visible(cx, false);
        list.redraw(cx);
    }
}

#[cfg(test)]
#[path = "test/menu/test.rs"]
mod tests;
