//! The canvas ruler — **drawing, not caption**.
//!
//! The ruler here used to be a pair of text labels in the shell DSL
//! (`ruler_marks` and `ruler_marks_v`, with the numbers separated by hand-typed
//! spaces). The owner compared the two IDEs and said: *"this ruler is still the
//! ugly ruler, right? The other one was much prettier."* — and the difference
//! was not taste, it was **structural**: a hand-written label does not follow
//! the form origin, does not fall every 50 units and **does not react to zoom**.
//! With zoom bound to a shortcut, it stays put while the form grows.
//!
//! This module is the part of the ruler that can be measured **without a
//! window**: how often to tick, and where each tick lands. Painting is the
//! canvas's job.
//!
//! # ADR 0019, and it is law here
//!
//! *"The ruler reads the same transform. It does not recompute 'how much is a
//! point worth': it asks where the coordinate is and draws the tick there."*
//! That is why [`ticks`] takes the frame's [`Transformacao`] and calls
//! `screen_point` — there is no second coordinate computation in this file,
//! and there must never be one.

use crate::Transformacao;
use crawrad_note::note;

#[note("designer-ruler-ruler-thickness")]
pub const RULER_THICKNESS: f32 = 18.0;

#[note("designer-ruler-ruler-label-gap")]
pub const RULER_LABEL_GAP: f32 = 40.0;

#[note("designer-ruler-steps")]
pub const STEPS: [f32; 8] = [5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0];

#[note("designer-ruler-ruler-step")]
pub fn ruler_step(scale: f32) -> f32 {
    STEPS
        .iter()
        .copied()
        .find(|step| step * scale >= RULER_LABEL_GAP)
        .unwrap_or(STEPS[STEPS.len() - 1])
}

#[note("designer-ruler-tick")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tick {
    /// The value in DESIGN units — what gets written when the tick is major.
    pub value: f32,
    /// Where it lands, in screen points, along the ruler axis.
    pub on_screen: f64,
    /// Numbered tick (long stroke) or minor one (short stroke)?
    pub major: bool,
}

#[note("designer-ruler-minor-ticks-per-step")]
const MINOR_TICKS_PER_STEP: u32 = 5;

#[note("designer-ruler-max-ticks")]
const MAX_TICKS: usize = 8_000;

#[note("designer-ruler-min-minor-tick-gap")]
const MIN_MINOR_TICK_GAP: f64 = 5.0;

#[note("designer-ruler-ticks")]
pub fn ticks(t: &Transformacao, vertical: bool, from: f64, to: f64) -> Vec<Tick> {
    let step = ruler_step(t.escala);
    let minor = step / MINOR_TICKS_PER_STEP as f32;
    let minor_width = t.medida(minor) as f64;
    // A degenerate scale (zero zoom, a window mid-resize) must not become an
    // infinite loop: with no advance there is no ruler to draw.
    // The guard is deliberately written without `!(a > b)`, and handles `NaN`
    // explicitly. **This is not fussiness: it was MEASURED.** With the scale at
    // `NaN` — which is what a window mid-resize produces — the comparison
    // `on_screen > to` is always false, the loop never ends and `n += 1`
    // overflows the `u32`: `attempt to add with overflow`, after four billion
    // turns. The test `a_degenerate_scale_or_range_yields_an_empty_ruler`
    // caught this.
    if !minor_width.is_finite() || minor_width <= 0.01 || from.is_nan() || to.is_nan() || to <= from
    {
        return Vec::new();
    }
    let with_minor = minor_width >= MIN_MINOR_TICK_GAP;
    let mut out = Vec::new();
    let mut n: u32 = 0;
    loop {
        let value = n as f32 * minor;
        let (x, y) = t.screen_point(
            if vertical { 0.0 } else { value },
            if vertical { value } else { 0.0 },
        );
        let on_screen = if vertical { y as f64 } else { x as f64 };
        // Belt on the belt: any coordinate that is not finite ends the ruler
        // instead of running until the counter overflows.
        if !on_screen.is_finite() || on_screen > to || out.len() >= MAX_TICKS {
            break;
        }
        let major = n.is_multiple_of(MINOR_TICKS_PER_STEP);
        if on_screen >= from && (major || with_minor) {
            out.push(Tick {
                value,
                on_screen,
                major,
            });
        }
        n += 1;
    }
    out
}

#[cfg(test)]
#[path = "../test/ruler/test.rs"]
mod tests;
