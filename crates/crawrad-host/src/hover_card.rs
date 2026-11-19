//! **The hover card's state and place**: which request it waits for,
//! and where it goes on screen — pure, so it is tested without a window.

use crawrad_note::note;
#[note("hover-card-anchor")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor {
    pub x: f64,
    pub y: f64,
    pub line_height: f64,
}

#[note("hover-card-hovercard")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HoverCard {
    tickets: u64,
    /// The newest request: its ticket, its anchor, and whether the user
    /// asked by key (then "nothing" and "no answer" are said).
    pending: Option<(u64, Anchor, bool)>,
    /// The card up, and the name it points at.
    shown: Option<Anchor>,
    /// The caret (line, byte) the last [`HoverCard::caret_moved`] saw.
    caret: Option<(usize, usize)>,
    /// The card up is a noted item's: the note's id, and the file and line
    /// the item is defined at.
    note: Option<(String, std::path::PathBuf, usize)>,
}

impl HoverCard {
    #[note("hover-card-hovercard-ask")]
    pub fn ask(&mut self, anchor: Anchor, by_key: bool) -> u64 {
        self.tickets += 1;
        self.pending = Some((self.tickets, anchor, by_key));
        self.tickets
    }

    pub fn pending_ticket(&self) -> Option<u64> {
        self.pending.map(|(t, _, _)| t)
    }

    #[note("hover-card-hovercard-answered")]
    pub fn answered(&mut self, ticket: u64) -> Option<(Anchor, bool)> {
        match self.pending.take() {
            Some((t, a, k)) if t == ticket => Some((a, k)),
            other => {
                self.pending = other;
                None
            }
        }
    }

    #[note("hover-card-hovercard-shown-at")]
    pub fn shown_at(&mut self, anchor: Anchor) {
        self.shown = Some(anchor);
    }

    #[note("hover-card-hovercard-set-note")]
    pub fn set_note(&mut self, note: Option<(String, std::path::PathBuf, usize)>) {
        self.note = note;
    }

    #[note("hover-card-hovercard-note")]
    pub fn note(&self) -> Option<&(String, std::path::PathBuf, usize)> {
        self.note.as_ref().filter(|_| self.shown.is_some())
    }

    #[note("hover-card-hovercard-shown")]
    pub fn shown(&self) -> Option<Anchor> {
        self.shown
    }

    #[note("hover-card-hovercard-caret-moved")]
    pub fn caret_moved(&mut self, caret: (usize, usize)) -> bool {
        let moved = self.shown.is_some() && self.caret.is_some_and(|c| c != caret);
        self.caret = Some(caret);
        moved
    }

    #[note("hover-card-hovercard-close")]
    pub fn close(&mut self) -> bool {
        self.pending = None;
        self.note = None;
        self.shown.take().is_some()
    }
}

#[note("hover-card-gap")]
pub const GAP: f64 = 4.0;

#[note("hover-card-place")]
pub fn place(anchor: Anchor, size: (f64, f64), window: (f64, f64, f64, f64)) -> (f64, f64) {
    let (wx, wy, ww, wh) = window;
    let above = anchor.y - GAP - size.1;
    let y = if above >= wy {
        above
    } else {
        anchor.y + anchor.line_height + GAP
    };
    let y = y.min(wy + wh - size.1).max(wy);
    let x = anchor.x.min(wx + ww - size.0).max(wx);
    (x, y)
}

#[note("hover-card-markdown-height")]
pub fn markdown_height(markdown: &str, line_height: f64, cols: usize, max: f64) -> f64 {
    let rows: usize = markdown
        .lines()
        .map(|l| {
            let extra = usize::from(l.starts_with('#') || l.starts_with("```"));
            extra + 1 + l.chars().count() / cols.max(1)
        })
        .sum();
    (rows as f64 * line_height).min(max)
}

#[note("hover-card-plain-height")]
pub fn plain_height(text: &str, line_height: f64, cols: usize) -> f64 {
    let rows: usize = text
        .lines()
        .map(|l| 1 + l.chars().count() / cols.max(1))
        .sum();
    rows as f64 * line_height + 16.0
}

#[note("hover-card-clip")]
pub fn clip(text: &str, max: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() <= max {
        return text.to_string();
    }
    let mut out = lines[..max].join("\n");
    out.push_str("\n\u{2026}");
    out
}

#[cfg(test)]
#[path = "test/hover_card/test.rs"]
mod tests;
