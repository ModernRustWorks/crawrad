//! What can be measured of the SCROLL BAR **without a window**.
//!
//! The scroll bar fix under proof. What is demanded here is what the previous
//! drawing got wrong, and every test names the mistake it locks out:
//!
//! - the thumb **reads `Position`** (the `_c: &MakepadCtl` of `desenhar` threw
//!   the control away: the RXP read `posicao_scroll` in four places and this
//!   runtime in none);
//! - the thumb **does not leave the component** (`(r.size.x * 0.3).max(8.0)` has
//!   no ceiling, and on a narrow bar it painted outside);
//! - the bar **withstands a degenerate rectangle** (the RXP's
//!   `clamp(min > max)` closed the IDE, and this file is born with the guard).

use super::*;

use makepad_widgets::dvec2;

fn r(x: f64, y: f64, w: f64, h: f64) -> Rect {
    Rect {
        pos: dvec2(x, y),
        size: dvec2(w, h),
    }
}

/// Does rectangle `a` fit inside `b`?
fn fits(a: Rect, b: Rect) -> bool {
    a.pos.x >= b.pos.x - 0.001
        && a.pos.y >= b.pos.y - 0.001
        && a.pos.x + a.size.x <= b.pos.x + b.size.x + 0.001
        && a.pos.y + a.size.y <= b.pos.y + b.size.y + 0.001
}

// ---------------------------------------------------------------------------
// The scroll bar fix: the thumb reads `Position`
// ---------------------------------------------------------------------------

#[test]
fn the_thumb_moves_with_the_position() {
    let frame = r(0.0, 0.0, 200.0, 16.0);
    let at_start = scroll_bar_geometry(frame, 0.0, 1.0);
    let at_middle = scroll_bar_geometry(frame, 50.0, 1.0);
    let at_end = scroll_bar_geometry(frame, 100.0, 1.0);

    assert!(
        at_start.thumb.pos.x < at_middle.thumb.pos.x,
        "a `Position` of 50 has to advance the thumb"
    );
    assert!(at_middle.thumb.pos.x < at_end.thumb.pos.x);

    // At position 0 the thumb touches the starting button; at 100, the ending
    // one.
    assert!((at_start.thumb.pos.x - at_start.start.size.x).abs() < 0.001);
    let right = at_end.thumb.pos.x + at_end.thumb.size.x;
    assert!((right - at_end.end.pos.x).abs() < 0.001);
}

#[test]
fn a_position_out_of_range_does_not_escape_the_track() {
    // The document is a text file: `Position: -30` and `Position: 400` reach
    // here, and they may not become a thumb outside the component.
    let frame = r(10.0, 20.0, 16.0, 120.0);
    for position in [-300.0, -1.0, 0.0, 50.0, 100.0, 101.0, 9999.0] {
        let b = scroll_bar_geometry(frame, position, 1.0);
        assert!(
            fits(b.thumb, frame),
            "position {position}: the thumb left the component ({:?})",
            b.thumb
        );
    }
}

// ---------------------------------------------------------------------------
// What the old drawing painted outside
// ---------------------------------------------------------------------------

#[test]
fn nothing_of_the_bar_paints_outside_the_component() {
    // The previous version did `(r.size.x * 0.3).max(8.0)` — a floor with NO
    // ceiling. On a bar 10 points wide the thumb came out at 8 and the component
    // had 6 of track: two points of ink on the neighbour.
    for (w, h) in [
        (200.0, 16.0),
        (16.0, 200.0),
        (10.0, 6.0),
        (6.0, 10.0),
        (3.0, 3.0),
        (1.0, 40.0),
        (40.0, 1.0),
        (0.0, 0.0),
    ] {
        let frame = r(5.0, 7.0, w, h);
        let b = scroll_bar_geometry(frame, 60.0, 1.0);
        for (name, part) in [("start", b.start), ("end", b.end), ("thumb", b.thumb)] {
            assert!(
                fits(part, frame),
                "{w}x{h}: the {name} left the component ({part:?})"
            );
        }
    }
}

#[test]
fn a_degenerate_rectangle_survives_the_drawing() {
    // *"The floor is a wish, the ceiling is a fact."* Shrinking a component
    // until the bar had 4 points of track already killed the RXP's IDE with a
    // `clamp(min > max)` (ADR 0027, Amendment 1). Here nothing may panic and
    // nothing may come out negative.
    for (w, h) in [(0.0, 0.0), (1.0, 1.0), (7.0, 3.0), (3.0, 7.0), (32.0, 5.0)] {
        let b = scroll_bar_geometry(r(0.0, 0.0, w, h), 50.0, 1.0);
        for part in [b.start, b.end, b.thumb] {
            assert!(
                part.size.x >= 0.0 && part.size.y >= 0.0,
                "{w}x{h}: {part:?}"
            );
            assert!(part.size.x.is_finite() && part.size.y.is_finite());
        }
    }
}

// ---------------------------------------------------------------------------
// The shape of the bar — what the classic RAD print shows
// ---------------------------------------------------------------------------

#[test]
fn the_thumb_runs_along_the_long_side() {
    // A horizontal bar has a horizontal thumb, a vertical bar a vertical one —
    // that is what the classic RAD print shows, and it is what the RXP does.
    let across = scroll_bar_geometry(r(0.0, 0.0, 200.0, 16.0), 0.0, 1.0);
    assert!(across.horizontal);
    assert!(across.thumb.size.x > across.thumb.size.y);
    // And the two buttons sit at the ENDS of the long side.
    assert!(across.start.pos.x < across.end.pos.x);
    assert_eq!(across.start.size.y, 16.0);

    let upright = scroll_bar_geometry(r(0.0, 0.0, 16.0, 200.0), 0.0, 1.0);
    assert!(!upright.horizontal);
    assert!(upright.thumb.size.y > upright.thumb.size.x);
    assert!(upright.start.pos.y < upright.end.pos.y);
}

#[test]
fn the_thumb_is_proportional_and_not_of_a_fixed_size() {
    // The classic RAD print the owner sent: *"in the `ListBox` the thumb takes up a
    // fraction of the track. It is not a fixed size."*
    let short = scroll_bar_geometry(r(0.0, 0.0, 16.0, 120.0), 0.0, 1.0);
    let long = scroll_bar_geometry(r(0.0, 0.0, 16.0, 400.0), 0.0, 1.0);
    assert!(
        long.thumb.size.y > short.thumb.size.y,
        "a longer track, a longer thumb"
    );
    // The fraction is the RXP's: 30% of the track.
    let track = 400.0 - 16.0 * 2.0;
    assert!((long.thumb.size.y - track * THUMB_FRACTION).abs() < 0.001);
}

#[test]
fn the_bar_scales_with_the_zoom() {
    // ADR 0019: every geometry goes through the `Transformacao`. A magnified bar
    // has a magnified button — it is the same component, bigger, and not a
    // component with wrong-sized parts. **Here the RXP diverges on purpose**:
    // there the bar measures are fixed screen points.
    let one = scroll_bar_geometry(r(0.0, 0.0, 200.0, 16.0), 0.0, 1.0);
    let two = scroll_bar_geometry(r(0.0, 0.0, 400.0, 32.0), 0.0, 2.0);
    assert!((two.start.size.x - one.start.size.x * 2.0).abs() < 0.001);
    assert!((two.thumb.size.y - one.thumb.size.y * 2.0).abs() < 0.001);
}

// ---------------------------------------------------------------------------
// Which bars show up
// ---------------------------------------------------------------------------

#[test]
fn the_documents_scroll_bars_property_rules() {
    // The same reading as the RXP's (`crawrad-runtime-egui/src/lib.rs:1244-1245`).
    assert_eq!(bars_from_model("ssNone", true, true), Bars::NONE);
    assert!(bars_from_model("ssVertical", true, true).vertical.visible());
    assert!(
        !bars_from_model("ssVertical", true, true)
            .horizontal
            .visible()
    );
    assert!(
        bars_from_model("ssHorizontal", true, true)
            .horizontal
            .visible()
    );
    let both = bars_from_model("ssBoth", true, true);
    assert!(both.vertical.visible() && both.horizontal.visible());
}

#[test]
fn with_no_property_the_bar_shows_up_when_the_content_overflows() {
    // `ListBox` and `TreeView` have no `ScrollBars` in the catalogue, and the
    // live RXP draws them with egui's `VisibleWhenNeeded`. The bar here REPORTS
    // that there is more content — when there is none, it has nothing to report.
    assert_eq!(bars_from_model("", false, false), Bars::NONE);
    assert_eq!(
        bars_from_model("", true, false).vertical,
        ScrollBarState::Active
    );
}

// ---------------------------------------------------------------------------
// The earlier fix: the bar the DEV turned on does not vanish — it goes grey
// ---------------------------------------------------------------------------

/// With `ScrollBars` in the document and content that fits, the bar stays on
/// screen and goes GREY.
///
/// It is the rule the owner measured on a classic RAD IDE, with a print: *"instead of
/// disabling, since the dev is the one who enables it, it deactivates"*. Before this round a
/// `Memo` with `ssBoth` and three lines drew two LIVE bars, thumb and all — a
/// promise of a drag that does not exist.
///
/// The mutation that breaks it: swapping `asked_for` for `when_needed` in
/// `bars_from_model` — then the bar vanishes, which is the `ListBox` behaviour,
/// and the three `visible()` assertions fall.
#[test]
fn the_bar_the_dev_turned_on_goes_grey_when_the_content_fits() {
    for (prop, v, h) in [
        ("ssVertical", true, false),
        ("ssHorizontal", false, true),
        ("ssBoth", true, true),
    ] {
        let b = bars_from_model(prop, false, false);
        let expected_v = if v {
            ScrollBarState::Disabled
        } else {
            ScrollBarState::Hidden
        };
        let expected_h = if h {
            ScrollBarState::Disabled
        } else {
            ScrollBarState::Hidden
        };
        assert_eq!(b.vertical, expected_v, "{prop}: the vertical bar");
        assert_eq!(b.horizontal, expected_h, "{prop}: the horizontal bar");
    }
}

/// And when the content overflows, the SAME property gives a live bar.
///
/// The two halves of the contract: without this one, greying the bar out always
/// would pass.
#[test]
fn the_bar_the_dev_turned_on_goes_live_when_the_content_overflows() {
    let b = bars_from_model("ssBoth", true, true);
    assert_eq!(b.vertical, ScrollBarState::Active);
    assert_eq!(b.horizontal, ScrollBarState::Active);
}

/// The automatic regime does **not** grey out: it hides.
///
/// `ListBox`, `TreeView` and `ScrollBox` have no `ScrollBars` in the catalogue,
/// and a classic RAD IDE makes their bar disappear when the content fits. A greyed-out bar
/// where a classic RAD IDE draws nothing would be invented fidelity.
#[test]
fn the_automatic_regime_hides_instead_of_greying_out() {
    assert_eq!(
        bars_from_model("", false, false).vertical,
        ScrollBarState::Hidden
    );
    assert_eq!(ScrollBarState::when_needed(false), ScrollBarState::Hidden);
    assert_eq!(ScrollBarState::asked_for(false), ScrollBarState::Disabled);
}

/// A hidden bar is not drawn — no track, no arrow, no thumb.
///
/// The guard lives in [`paint_bar`], and not in each caller: as long as `Hidden`
/// was just a `bool` read on the outside, one caller forgetting to look at it
/// was enough for an `ssNone` bar to show up.
#[test]
fn the_hidden_state_is_refused_at_the_drawing() {
    // The refusal belongs to `paint_bar`, which is where the ink comes out. What
    // is demanded here is that the state reaches it at all: `paint_inner_bars`
    // passes the bar's state, and not a `bool`.
    assert!(!ScrollBarState::Hidden.visible());
    assert!(ScrollBarState::Active.visible());
    assert!(ScrollBarState::Disabled.visible());
}

/// **A greyed-out bar may not look clickable.**
///
/// It is the house's honesty rule, and both halves of it: the thumb — which is
/// the promise of a drag — only exists on a live bar, and the arrow of a
/// greyed-out bar has to come out in a DIFFERENT colour from the live one.
///
/// Both answers live on [`ScrollBarState`], and not inside [`paint_bar`], for a
/// reason of measurement: painting needs a `Cx2d`, and a `Cx2d` needs an open
/// window. This is what the test reaches; that `paint_bar` actually OBEYS is
/// still screen proof — it was measured pixel by pixel in the
/// `target/prova-111-122` print.
///
/// The mutation that breaks it: `live()` returning `self.visible()` — then the
/// bar of a `Memo` whose content fits shows a thumb again, which is exactly the
/// defect of the earlier report.
#[test]
fn a_greyed_out_bar_may_not_look_clickable() {
    assert!(
        ScrollBarState::Active.live(),
        "a live bar has something to drag"
    );
    assert!(
        !ScrollBarState::Disabled.live(),
        "a greyed-out bar draws no thumb: it would promise a drag that does not exist"
    );
    assert!(!ScrollBarState::Hidden.live());

    assert_eq!(ScrollBarState::Active.arrow(), MAKEPAD_TEXT);
    assert_eq!(ScrollBarState::Disabled.arrow(), MAKEPAD_TEXT_DIM);
    assert_ne!(
        ScrollBarState::Active.arrow(),
        ScrollBarState::Disabled.arrow(),
        "the greyed-out arrow has to be TOLD APART from the live one, or greyed out does not read"
    );
}
