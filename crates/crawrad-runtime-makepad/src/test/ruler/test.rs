//! What can be measured about the RULER **without a window**.
//!
//! The ruler here used to be a pair of hand-written labels in the shell DSL,
//! and the owner said: *"this ruler is still the ugly ruler, right? The other
//! one was much prettier."* What these tests demand is exactly what a
//! hand-written label **cannot** do: follow the form origin and react to zoom.

use super::*;

use crate::Transformacao;

fn t(origin: f32, scale: f32) -> Transformacao {
    Transformacao::new(origin, origin, scale)
}

// ---------------------------------------------------------------------------
// The step — the rule that keeps the ruler readable at any zoom
// ---------------------------------------------------------------------------

#[test]
fn the_step_mirrors_the_rxp_rule() {
    // The steps and the gap are the RXP's (`crawrad-runtime-egui/src/lib.rs:3310`
    // and `:3324`). An adapter that picked a different step would show the same
    // coordinate in two different places of the same ruler.
    assert_eq!(STEPS, [5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0]);
    assert_eq!(RULER_LABEL_GAP, 40.0);

    // At 100% the step is 50 — the same every-50 the old ruler had written by
    // hand, and the only zoom where it was right.
    assert_eq!(ruler_step(1.0), 50.0);
    // At 25% those same 50 units would fall within 12 screen points and the
    // ruler would become a wall: the step goes up.
    assert_eq!(ruler_step(0.25), 250.0);
    // At 400% it goes down, because many more numbers fit.
    assert_eq!(ruler_step(4.0), 10.0);
}

#[test]
fn the_step_never_drops_when_the_zoom_shrinks() {
    // Monotonicity is what makes the ruler behave: zooming out can only remove
    // numbers, never add them.
    let mut previous = f32::INFINITY;
    for scale in [4.0, 2.0, 1.5, 1.0, 0.5, 0.25, 0.1] {
        let step = ruler_step(scale);
        assert!(
            step >= previous || previous.is_infinite(),
            "at {scale}x the step dropped to {step} (was {previous})"
        );
        previous = step;
    }
}

#[test]
fn two_numbers_never_touch_at_any_zoom() {
    // **This is why the step exists.** The ruler promises
    // [`RULER_LABEL_GAP`] screen points between two numbers, and that promise
    // is what a hand-written ruler has no way of keeping.
    for scale in [0.1, 0.25, 0.5, 1.0, 1.5, 2.0, 4.0, 8.0] {
        let view = t(0.0, scale);
        let ts = ticks(&view, false, 0.0, 4000.0);
        let majors: Vec<f64> = ts.iter().filter(|m| m.major).map(|m| m.on_screen).collect();
        assert!(majors.len() >= 2, "at {scale}x the ruler numbered nothing");
        for pair in majors.windows(2) {
            let d = pair[1] - pair[0];
            assert!(
                d >= RULER_LABEL_GAP as f64,
                "at {scale}x two numbers landed {d:.1} points apart — the floor is {RULER_LABEL_GAP}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The origin — the ruler's zero is the FORM's zero
// ---------------------------------------------------------------------------

#[test]
fn the_rulers_zero_is_the_forms_zero() {
    // In a classic RAD IDE the coordinate the Object Inspector shows is relative to the
    // form, and the ruler has to say the SAME number. A label written in the
    // shell does not know where the form landed; this function asks the
    // frame's [`Transformacao`] (ADR 0019).
    for origin in [0.0, 37.0, 240.0] {
        let view = t(origin, 1.0);
        let first = ticks(&view, false, origin as f64 - 100.0, 1000.0)
            .into_iter()
            .find(|m| m.major)
            .expect("there is a numbered tick");
        assert_eq!(first.value, 0.0);
        assert!((first.on_screen - origin as f64).abs() < 0.001);
    }
}

#[test]
fn the_tick_lands_where_the_transform_says_and_not_on_a_second_computation() {
    // If the ruler recomputed the coordinate on its own, it would disagree with
    // the drawing the moment someone zoomed — that is the defect ADR 0019
    // exists to prevent. Here the result is checked against the SAME
    // `screen_point` the canvas uses to paint.
    let view = t(50.0, 2.5);
    for m in ticks(&view, true, 0.0, 2000.0) {
        let (_, y) = view.screen_point(0.0, m.value);
        assert!((m.on_screen - y as f64).abs() < 0.001);
    }
}

#[test]
fn the_ruler_only_ticks_what_fits_the_range() {
    let view = t(0.0, 1.0);
    let (from, to) = (120.0, 600.0);
    let ts = ticks(&view, false, from, to);
    assert!(!ts.is_empty());
    for m in &ts {
        assert!(
            m.on_screen >= from && m.on_screen <= to,
            "tick at {} outside [{from}, {to}]",
            m.on_screen
        );
        // And never a negative coordinate: whatever sits before the form has no
        // number worth anything.
        assert!(m.value >= 0.0);
    }
}

// ---------------------------------------------------------------------------
// The degenerate case — a window mid-resize must not freeze the IDE
// ---------------------------------------------------------------------------

#[test]
fn a_degenerate_scale_or_range_yields_an_empty_ruler() {
    // A zero scale would make the loop advance zero per step and never reach
    // the end; `NaN` would make every comparison false, with the same result.
    // Both really do get here — zoom at the limit, and a window in the middle
    // of a resize.
    assert!(ticks(&t(0.0, 0.0), false, 0.0, 800.0).is_empty());
    assert!(ticks(&t(0.0, 1.0), false, 800.0, 100.0).is_empty());
    assert!(ticks(&t(0.0, 1.0), false, 0.0, 0.0).is_empty());
    assert!(ticks(&t(0.0, 1.0), false, f64::NAN, 800.0).is_empty());
    assert!(ticks(&t(0.0, 1.0), false, 0.0, f64::NAN).is_empty());
    assert!(ticks(&t(0.0, f32::NAN), false, 0.0, 800.0).is_empty());
}

#[test]
fn the_minor_stroke_disappears_before_becoming_a_solid_line() {
    // Five strokes across two screen points become a wall that measures nothing
    // and hides the numbered one — this is the RXP's note (`lib.rs:1985-1987`).
    // At 100% the minor step is 10 units = 10 points: it shows.
    let with = ticks(&t(0.0, 1.0), false, 0.0, 600.0);
    assert!(
        with.iter().any(|m| !m.major),
        "at 100% the minor stroke has to show"
    );
    // Zoomed far out, the step has already hit the ceiling and the minor
    // stroke shrinks until it disappears.
    let without = ticks(&t(0.0, 0.02), false, 0.0, 600.0);
    assert!(
        without.iter().all(|m| m.major),
        "with the minor stroke under 5 points it must not be drawn"
    );
}
