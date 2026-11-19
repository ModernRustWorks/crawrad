//! **THE FRACTION ARITHMETIC**: how much of a range a value represents.
//!
//! It is the box's arithmetic piece, and it exists because the same sum had
//! **three** owners: the designer's canvas, the text emitted into the
//! application, and the component's own `Gauge::fraction()`, which
//! reimplemented it by hand.
//!
//! Three sums that should agree are three chances to disagree, and the
//! divergence would show up as a gauge reading 65% in the designer and
//! something else on the user's screen.
//!
//! # Why it lives in the BOX, and not in the runtime
//!
//! Because whoever needs it most is whoever writes components. The canvas and
//! the generator are in-house and reach any file of ours; the author of a
//! `gauge-crawrad` published on crates.io reaches **only what the box offers**.
//!
//! The `[CRAWRAD:INICIO fraction]` / `[CRAWRAD:FIM fraction]` markers are the cut that
//! `crawrad-codegen` emits into the user's project: the same mechanism as
//! `desenho_comum.rs`. The generator searches for the marker text in existing
//! projects, so it migrates in a PR of its own (ADR 0047, rule 7).

// [CRAWRAD:INICIO fraction]

/// How far `value` sits along `min..=max`, as `0.0..=1.0`.
///
/// This is what a gauge, a progress bar and a scroll thumb all need, and it is
/// the arithmetic behind a declared drawing's fractional area.
///
/// Out of range is CLAMPED, not refused: a component does not paint outside its
/// own box, and a reading above the top of the scale is still a full bar.
///
/// A zero-width range gives `0.0`, not `NaN`: a manifest with `min == max` is
/// wrong, and an empty bar says so while a strange glyph painted across the
/// form does not.
pub fn fraction(value: f32, min: f32, max: f32) -> f32 {
    let range = max - min;
    if range.abs() < f32::EPSILON {
        0.0
    } else {
        ((value - min) / range).clamp(0.0, 1.0)
    }
}
// [CRAWRAD:FIM fraction]

#[cfg(test)]
#[path = "../test/fraction/test.rs"]
mod test;
