//! **Where the completion list opens**: its left edge at the start of
//! the word being completed — where the caret stood when it opened, before
//! any prefix — and its top under the caret's line. The editor reports the
//! caret in its text area, which starts after the gutter; the list is placed
//! in the container that holds the gutter too, so both come in window
//! coordinates and the container's origin is taken off here.

use crawrad_note::note;
#[note("anchor-anchor")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor {
    /// The list's margin inside its container.
    pub left: f64,
    pub top: f64,
    /// The start of the word in the window, which the list is aligned to.
    pub word: [f64; 2],
}

#[note("anchor-place")]
pub fn place(
    caret: [f64; 2],
    typed: usize,
    cell: [f64; 2],
    container: [f64; 4],
    size: [f64; 2],
) -> Anchor {
    let word = [caret[0] - typed as f64 * cell[0], caret[1]];
    let [cx, cy, cw, ch] = container;
    let left = (word[0] - cx).min(cw - size[0]).max(0.0);
    let below = word[1] + cell[1] - cy;
    let above = word[1] - cy - size[1];
    let top = if below + size[1] > ch && above >= 0.0 {
        above
    } else {
        below
    };
    Anchor { left, top, word }
}

#[cfg(test)]
#[path = "test/anchor/test.rs"]
mod tests;
