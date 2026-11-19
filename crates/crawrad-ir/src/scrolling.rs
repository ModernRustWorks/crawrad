//! **When a container shows a scroll bar** — the rule, in one place.
//!
//! # The defect this module closes
//!
//! Measured on 17/09/2026, with the owner looking at both sides on screen:
//! *"now we need to make everything that appears in the designer appear built, as seen
//! here with the scrolls"*.
//!
//! The designer's `ScrollBox` drew both bars **always**, and the generated
//! application — on BOTH targets — drew none. It was not one side that was
//! wrong, and that is the expensive part: **the user places children looking
//! at one box and the program ships another.**
//!
//! # The DESIGNER was the wrong one, and the classic component library is the proof
//!
//! The temptation was to make the application draw the bar always. The earlier check
//! measured it, in the classic library's source and in the two-minute check the owner
//! described — a 300x150 `TScrollBox` with a `TButton` at `Top := 200`:
//!
//! > *"The bar appears when the content overflows."*
//!
//! With no child overflowing there is **no bar** — in the classic RAD designer
//! either. So the side to give way was our drawing and not the generated code:
//! both frameworks hand out the automatic regime for free
//! (`ScrollBarVisibility::VisibleWhenNeeded` in egui, `scroll_area.rs:1203`,
//! and `self.visible = view_total > view_rect` in Makepad,
//! `makepad-widgets-1.0.0/src/scroll_bar.rs:520`).
//!
//! # Why the axes are INDEPENDENT here
//!
//! In the classic library, turning the vertical bar on can make the horizontal one appear,
//! because there the bar **eats client area** and the content then has less
//! room. egui does the same, by frame feedback (`limit_rect` is `inner_rect`
//! when the bar is not floating).
//!
//! **Our bar eats no area**, and that was already decided and written into the
//! `ScrollBox` design: children are placed by the document and drawing may not
//! change where they land (ADR 0012). It is what Makepad does too —
//! `ScrollBars::begin` reserves no gutter
//! (`makepad-widgets-1.0.0/src/scroll_bars.rs:245`).
//!
//! With the bar on top, turning one on does **not** shrink the other axis, and
//! coupling them would be a rule with no cause. The declared limit: for content
//! that fits by less than the thickness of a bar, egui shows the second bar and
//! we do not. That is twelve points on one edge, and it is the price of the bar
//! not pushing a child.
//!
//! # And why the rule lives in the NEUTRAL crate
//!
//! Because it belongs to the model, not to the drawing. It was once about to be
//! written inside the Makepad adapter, and the comment left there said why it
//! could not stay:
//!
//! > *"saying WHEN it fits requires a property that crawrad-registry does not
//! > yet have. Inventing it here would mean deciding the model inside the drawing."*
//!
//! It was not a missing property: it was a **computation** — and a computation
//! each adapter does separately: *the design written twice has
//! already diverged in seven places*.

use crawrad_note::note;
#[note("scrolling-bars")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bars {
    /// The standing bar, on the right.
    pub vertical: bool,
    /// The lying bar, at the bottom.
    pub horizontal: bool,
}

#[note("scrolling-bars-needed")]
pub fn bars_needed(client: (f32, f32), content: (f32, f32)) -> Bars {
    Bars {
        vertical: content.1 > client.1,
        horizontal: content.0 > client.0,
    }
}

#[note("scrolling-content-extent")]
pub fn content_extent(
    box_origin: (f32, f32),
    children: impl Iterator<Item = (f32, f32, f32, f32)>,
) -> (f32, f32) {
    let mut extent = (0.0f32, 0.0f32);
    for (x, y, w, h) in children {
        extent.0 = extent.0.max(x + w - box_origin.0);
        extent.1 = extent.1.max(y + h - box_origin.1);
    }
    extent
}

#[note("scrolling-scroll-box-axes")]
pub fn scroll_box_axes(auto_scroll: bool, horz_scroll_bar: bool, vert_scroll_bar: bool) -> Bars {
    Bars {
        vertical: auto_scroll && vert_scroll_bar,
        horizontal: auto_scroll && horz_scroll_bar,
    }
}

#[note("scrolling-scroll-bar-box-for-kind")]
pub fn scroll_bar_box_for_kind(rect: crate::Rect, kind: &str) -> Option<crate::Rect> {
    let horizontal = kind == "sbHorizontal";
    ((rect.w >= rect.h) != horizontal).then(|| crate::Rect::new(rect.x, rect.y, rect.h, rect.w))
}

#[cfg(test)]
#[path = "test/scrolling/test.rs"]
mod test;
