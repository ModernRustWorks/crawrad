//! **The pointer over a control, and over the form**.
//!
//! The generated app carries its own `Cursor` (the piece of `crawrad-component`),
//! which this crate does not know: it hands over the name (`Cursor::as_str`)
//! and gets Makepad's pointer back. `default` is no pointer of ours — the
//! widget keeps its own (the text beam of an edit), as the classic
//! `crDefault` does.

use crawrad_note::note;
use makepad_widgets::*;

#[note("cursor-mouse")]
pub fn mouse(name: &str) -> Option<MouseCursor> {
    Some(match name {
        "arrow" => MouseCursor::Arrow,
        "hand" => MouseCursor::Hand,
        "text" => MouseCursor::Text,
        "crosshair" => MouseCursor::Crosshair,
        "move" => MouseCursor::Move,
        "wait" => MouseCursor::Wait,
        "not_allowed" => MouseCursor::NotAllowed,
        "resize_ns" => MouseCursor::NsResize,
        "resize_ew" => MouseCursor::EwResize,
        "resize_nesw" => MouseCursor::NeswResize,
        "resize_nwse" => MouseCursor::NwseResize,
        _ => return None,
    })
}

#[note("cursor-show")]
pub fn show(cx: &mut Cx, name: &str) {
    if let Some(c) = mouse(name) {
        cx.set_cursor(c);
    }
}

#[cfg(test)]
#[path = "test/cursor/test.rs"]
mod tests;
