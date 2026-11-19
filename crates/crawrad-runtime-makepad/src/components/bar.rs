//! The scroll bar — **one computation, many readers**.
//!
//! This file answers for no catalogue class at all: it is the geometry and the
//! painting of a bar, and its callers are the `ScrollBar` (which IS a bar) and
//! every scrolling container — `ListBox`, `Memo`, `TreeView` and `ScrollBox` —
//! that draws its own.
//!
//! The rule comes from the RXP, which already paid the price of not having it:
//! its `barra_de_rolagem` (`crawrad-runtime-egui/src/lib.rs:2950`) *"was born of
//! four copies — design horizontal, design vertical, live horizontal and live
//! vertical — and all four carried the same `clamp(8.0, track)` that killed the
//! IDE when the track got smaller than 8"*. Here there is only one, and it is
//! measured without a window.
//!
//! **What changes against the RXP, and it is on purpose:** there the bar
//! measures are fixed SCREEN points, so the button stays 16 px at any zoom; here
//! they are DESIGN units multiplied by the scale, because in this house every
//! geometry goes through [`crate::Transformacao`] (ADR 0019). A magnified bar has
//! a magnified button — it is the same component, bigger, and not a component
//! with wrong-sized parts.

use makepad_widgets::{Rect, dvec2};

use crate::canvas::Brush;
use crate::components::comum::*;

/// The side of one of a bar's two buttons, in design units.
///
/// Square: a bar's button is as wide as the bar is tall. The ceiling is what
/// keeps a thick bar from becoming two buttons and no track at all. Mirrors the
/// RXP's `BOTAO_DA_BARRA` (`crawrad-runtime-egui/src/lib.rs:2916`).
///
/// **It is no longer declared here.** It comes from
/// [`crate::shared_geometry`], the one file that both this canvas and the code
/// generator read — the generator used to carry the same number as a bare
/// literal in another crate, eight times over, with nothing tying the two.
pub use crate::shared_geometry::BAR_BUTTON_SIDE;
use crawrad_note::note;

#[note("components-bar-thumb-fraction")]
pub const THUMB_FRACTION: f64 = 0.3;

#[note("components-bar-min-thumb")]
pub const MIN_THUMB: f32 = 8.0;

#[note("components-bar-thumb-inset")]
pub const THUMB_INSET: f32 = 2.0;

#[note("components-bar-inner-bar-thickness")]
pub const INNER_BAR_THICKNESS: f32 = 14.0;

#[note("components-bar-scrollbargeometry")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollBarGeometry {
    /// The starting button: on the left when horizontal, on top when vertical.
    pub start: Rect,
    /// The ending button.
    pub end: Rect,
    /// The thumb, already placed at the component's `position`.
    pub thumb: Rect,
    /// Is the bar wider than it is tall?
    pub horizontal: bool,
}

#[note("components-bar-scroll-bar-geometry")]
pub fn scroll_bar_geometry(r: Rect, position: f32, scale: f64) -> ScrollBarGeometry {
    let horizontal = r.size.x > r.size.y;
    let button_ceiling = BAR_BUTTON_SIDE as f64 * scale;
    // The short side rules the button, both ways round.
    let button = if horizontal {
        r.size.y.min(button_ceiling)
    } else {
        r.size.x.min(button_ceiling)
    };
    let along = if horizontal { r.size.x } else { r.size.y };
    // The track is what is LEFT between the two buttons — and when nothing is
    // left, it measures zero. A made-up floor here would be the panic's
    // `min > max`, written two lines before it happens.
    let track = (along - button * 2.0).max(0.0);
    let thumb = entre(track * THUMB_FRACTION, MIN_THUMB as f64 * scale, track);
    let fraction = (position / 100.0).clamp(0.0, 1.0) as f64;
    let advance = button + fraction * (track - thumb);
    // The thumb thickness is the short side minus the gap on both sides.
    //
    // **The gap has a ceiling, and the RXP never gave it one.** There the
    // computation is `(thick - GAP*2).max(1.0)` with the thumb placed at
    // `min + GAP` (`crawrad-runtime-egui/src/lib.rs:2989-2991`): on a bar 1 point
    // thick that puts a 1-point thumb 2 points away from the wall, that is
    // **entirely outside the component**. Measured here by
    // `nothing_of_the_bar_paints_outside_the_component` — the `1x40` case put the thumb
    // at x=7 in a component that ends at x=6. With the ceiling, a thin bar
    // simply has no gap and the thumb takes up whatever there is.
    let thick = if horizontal { r.size.y } else { r.size.x };
    let gap = (THUMB_INSET as f64 * scale).min((thick * 0.5 - 0.5).max(0.0));
    let thumb_thickness = (thick - gap * 2.0).max(0.0);
    if horizontal {
        ScrollBarGeometry {
            start: Rect {
                pos: r.pos,
                size: dvec2(button, r.size.y),
            },
            end: Rect {
                pos: dvec2(r.pos.x + r.size.x - button, r.pos.y),
                size: dvec2(button, r.size.y),
            },
            thumb: Rect {
                pos: dvec2(r.pos.x + advance, r.pos.y + gap),
                size: dvec2(thumb, thumb_thickness),
            },
            horizontal,
        }
    } else {
        ScrollBarGeometry {
            start: Rect {
                pos: r.pos,
                size: dvec2(r.size.x, button),
            },
            end: Rect {
                pos: dvec2(r.pos.x, r.pos.y + r.size.y - button),
                size: dvec2(r.size.x, button),
            },
            thumb: Rect {
                pos: dvec2(r.pos.x + gap, r.pos.y + advance),
                size: dvec2(thumb_thickness, thumb),
            },
            horizontal,
        }
    }
}

#[note("components-bar-scrollbarstate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBarState {
    /// Not drawn at all.
    #[default]
    Hidden,
    /// Drawn whole, with the thumb: there is content beyond the window.
    Active,
    /// Drawn without the thumb and with grey arrows: the developer asked for
    /// the bar, and everything already fits.
    Disabled,
}

impl ScrollBarState {
    #[note("components-bar-scrollbarstate-when-needed")]
    pub fn when_needed(needed: bool) -> Self {
        if needed { Self::Active } else { Self::Hidden }
    }

    #[note("components-bar-scrollbarstate-asked-for")]
    pub fn asked_for(needed: bool) -> Self {
        if needed { Self::Active } else { Self::Disabled }
    }

    #[note("components-bar-scrollbarstate-visible")]
    pub fn visible(self) -> bool {
        self != Self::Hidden
    }

    #[note("components-bar-scrollbarstate-live")]
    pub fn live(self) -> bool {
        self == Self::Active
    }

    #[note("components-bar-scrollbarstate-arrow")]
    pub fn arrow(self) -> [f32; 4] {
        if self.live() {
            MAKEPAD_TEXT
        } else {
            MAKEPAD_TEXT_DIM
        }
    }
}

#[note("components-bar-paint-bar")]
pub fn paint_bar(p: &mut Brush, r: Rect, position: f32, state: ScrollBarState) {
    if r.size.x < 1.0 || r.size.y < 1.0 || !state.visible() {
        return;
    }
    // The track SINKS: it is the same paper as a text field's.
    p.rounded_box(r, MAKEPAD_WIDGET_INPUT, MAKEPAD_BORDER, 2.0);
    let b = scroll_bar_geometry(r, position, p.medida(1.0));
    // The buttons RISE, like the `Button`: they are buttons.
    p.rounded_box(b.start, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
    p.rounded_box(b.end, MAKEPAD_WIDGET, MAKEPAD_BORDER, 2.0);
    let (backwards, forwards) = if b.horizontal {
        (dvec2(-1.0, 0.0), dvec2(1.0, 0.0))
    } else {
        (dvec2(0.0, -1.0), dvec2(0.0, 1.0))
    };
    p.arrow(centered_square(b.start), backwards, state.arrow());
    p.arrow(centered_square(b.end), forwards, state.arrow());
    if state.live() {
        p.rounded_box(b.thumb, MAKEPAD_PANEL_BORDER, MAKEPAD_BORDER, 2.0);
    }
}

#[note("components-bar-centered-square")]
pub fn centered_square(r: Rect) -> Rect {
    let side = r.size.x.min(r.size.y);
    Rect {
        pos: dvec2(
            r.pos.x + (r.size.x - side) * 0.5,
            r.pos.y + (r.size.y - side) * 0.5,
        ),
        size: dvec2(side, side),
    }
}

#[note("components-bar-bars")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bars {
    pub vertical: ScrollBarState,
    pub horizontal: ScrollBarState,
}

impl Bars {
    #[note("components-bar-bars-none")]
    pub const NONE: Bars = Bars {
        vertical: ScrollBarState::Hidden,
        horizontal: ScrollBarState::Hidden,
    };
}

#[note("components-bar-bars-from-model")]
pub fn bars_from_model(scroll_bars: &str, overflow_y: bool, overflow_x: bool) -> Bars {
    let asked_y = ScrollBarState::asked_for(overflow_y);
    let asked_x = ScrollBarState::asked_for(overflow_x);
    match scroll_bars {
        "ssNone" => Bars::NONE,
        "ssHorizontal" => Bars {
            vertical: ScrollBarState::Hidden,
            horizontal: asked_x,
        },
        "ssVertical" => Bars {
            vertical: asked_y,
            horizontal: ScrollBarState::Hidden,
        },
        "ssBoth" => Bars {
            vertical: asked_y,
            horizontal: asked_x,
        },
        _ => Bars {
            vertical: ScrollBarState::when_needed(overflow_y),
            horizontal: ScrollBarState::Hidden,
        },
    }
}

#[note("components-bar-paint-inner-bars")]
pub fn paint_inner_bars(p: &mut Brush, r: Rect, bars: Bars, position: f32) -> Rect {
    let thickness = p.medida(INNER_BAR_THICKNESS).max(6.0);
    // A bar that does not fit is not drawn: a `ListBox` shrunk to 10 px wide
    // must not become a bar and no list.
    let fits_v = bars.vertical.visible() && r.size.x > thickness * 2.0;
    let fits_h = bars.horizontal.visible() && r.size.y > thickness * 2.0;
    let width = if fits_v { thickness } else { 0.0 };
    let height = if fits_h { thickness } else { 0.0 };

    if fits_v {
        paint_bar(
            p,
            Rect {
                pos: dvec2(r.pos.x + r.size.x - width, r.pos.y),
                size: dvec2(width, r.size.y - height),
            },
            position,
            bars.vertical,
        );
    }
    if fits_h {
        paint_bar(
            p,
            Rect {
                pos: dvec2(r.pos.x, r.pos.y + r.size.y - height),
                size: dvec2(r.size.x - width, height),
            },
            0.0,
            bars.horizontal,
        );
    }
    Rect {
        pos: r.pos,
        size: dvec2((r.size.x - width).max(0.0), (r.size.y - height).max(0.0)),
    }
}

#[cfg(test)]
#[path = "../test/bar/test.rs"]
mod tests;
