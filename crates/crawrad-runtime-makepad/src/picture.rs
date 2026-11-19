//! **An Image draws its Picture**.
//!
//! The Makepad target emitted a blank `View` for an `Image`. Now the control
//! holds a makepad `<Image>` (`img`) and the sync hands it the picture's bytes
//! — PNG or JPEG, told apart by their signature —
//! through `ImageRef::load_png_from_data` / `load_jpg_from_data`
//! (`makepad-widgets-1.0.0/src/image.rs:433`, `:424`).
//!
//! Decoding is not cheap and the sync runs after every event (a mouse move is
//! one), so a [`Shown`] in the generated `App` remembers what is on screen and
//! the bytes are only decoded again when the picture changes.

use crawrad_note::note;
use makepad_widgets::*;

#[note("picture-shown")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Shown {
    fingerprint: Option<u64>,
}

#[note("picture-kind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Png,
    Jpeg,
}

#[note("picture-kind-of")]
pub fn kind_of(bytes: &[u8]) -> Option<Kind> {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G']) {
        Some(Kind::Png)
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some(Kind::Jpeg)
    } else {
        None
    }
}

#[note("picture-fingerprint")]
pub fn fingerprint(bytes: &[u8]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    bytes.len().hash(&mut h);
    let step = (bytes.len() / 64).max(1);
    for b in bytes.iter().step_by(step).take(64) {
        b.hash(&mut h);
    }
    h.finish()
}

#[note("picture-show")]
pub fn show(cx: &mut Cx, ui: &WidgetRef, path: &[LiveId], bytes: &[u8], shown: &mut Shown) {
    let print = fingerprint(bytes);
    if shown.fingerprint == Some(print) {
        return;
    }
    shown.fingerprint = Some(print);
    let img = ui.widget(path).image(id!(img));
    let _ = match kind_of(bytes) {
        Some(Kind::Png) => img.load_png_from_data(cx, bytes),
        Some(Kind::Jpeg) => img.load_jpg_from_data(cx, bytes),
        None => Ok(()),
    };
    ui.widget(path).redraw(cx);
}

#[cfg(test)]
#[path = "test/picture/test.rs"]
mod tests;
