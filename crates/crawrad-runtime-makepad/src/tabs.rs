//! **A page control's tab strip**: which tab is the open one, and
//! whether the strip sits above or below the pages.
//!
//! The generated page control is an overlay of two views, the strip and the
//! pages, each placed by its top margin. Each tab is a cell with its button
//! and a thin mark under it, shown on the open tab only. Before this the open
//! tab looked like any other: switching pages changed nothing in the strip.
//! (The mark, and not the button's own colour: Makepad 1.0.0 took no
//! `draw_bg` colour on a button inside the strip, measured with a pure red.)

use crawrad_note::note;
use makepad_widgets::*;

#[note("tabs-strip")]
pub const STRIP: f64 = 24.0;

#[note("tabs-look")]
pub fn look(cx: &mut Cx, mark: &WidgetRef, ring: &WidgetRef, open: bool, focused: bool) {
    if mark.visible() != open {
        mark.set_visible(cx, open);
    }
    let ringed = open && focused;
    if ring.visible() != ringed {
        ring.set_visible(cx, ringed);
    }
}

#[note("tabs-focused")]
pub fn focused(cx: &Cx, tab: &WidgetRef) -> bool {
    let area = tab.area();
    !area.is_empty() && cx.has_key_focus(area)
}

#[note("tabs-prev-page")]
pub fn prev_page(current: usize, eligible: &[bool]) -> Option<usize> {
    (0..current).rfind(|&idx| eligible.get(idx).copied().unwrap_or(false))
}

#[note("tabs-next-page")]
pub fn next_page(current: usize, eligible: &[bool]) -> Option<usize> {
    ((current + 1)..eligible.len()).find(|&idx| eligible.get(idx).copied().unwrap_or(false))
}

#[note("tabs-place")]
pub fn place(cx: &mut Cx, strip: &WidgetRef, pages: &WidgetRef, height: f64, bottom: bool) {
    let (strip_top, pages_top) = if bottom {
        ((height - STRIP).max(0.0), 0.0)
    } else {
        (0.0, STRIP)
    };
    strip.apply_over(cx, live! { margin: { top: (strip_top) } });
    pages.apply_over(cx, live! { margin: { top: (pages_top) } });
}

#[note("tabs-window")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    indices: Vec<usize>,
    pub arrows: bool,
}

impl Window {
    pub fn contains(&self, i: usize) -> bool {
        self.indices.contains(&i)
    }
}

pub fn window(width: f32, shown: &[bool], active: usize) -> Window {
    let list: Vec<usize> = shown
        .iter()
        .enumerate()
        .filter_map(|(i, &vis)| if vis { Some(i) } else { None })
        .collect();
    if list.is_empty() {
        return Window {
            indices: Vec::new(),
            arrows: false,
        };
    }
    let pos = list.iter().position(|&idx| idx == active).unwrap_or(0);
    let (first, count, arrows) = crate::shared_geometry::tab_window(width, list.len(), pos, 0.0);
    Window {
        indices: list[first..first + count].to_vec(),
        arrows,
    }
}

#[cfg(test)]
#[path = "test/tabs/test.rs"]
mod tests;
