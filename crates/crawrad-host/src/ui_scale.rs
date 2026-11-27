//! The interface zoom's ladder: the steps `Ctrl +` / `Ctrl -` walk. Pure,
//! so the walk is tested without a window.

use crawrad_note::note;
// Ten by ten (owner, 13/11): 110 -> 120 -> 130, not 110 -> 125 -> 150.
#[note("ui-scale-steps")]
pub const STEPS: &[f64] = &[
    0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0,
];

#[note("ui-scale-one")]
pub const ONE: f64 = 1.0;

#[note("ui-scale-next-step")]
pub fn next_step(current: f64, inward: bool) -> f64 {
    // The NEAREST declared step, so a factor that came from somewhere else — a
    // restored session, a Settings slider one day — still walks the list
    // instead of falling off it.
    let i = STEPS
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            (*a - current)
                .abs()
                .partial_cmp(&(*b - current).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
        .unwrap_or(0);
    let next = if inward { i + 1 } else { i.saturating_sub(1) };
    STEPS[next.min(STEPS.len() - 1)]
}

/// The zoom cell: the IDE scale always, and the canvas zoom only when the
/// canvas is NOT at 100%.
///
/// At 100% the canvas readout says nothing and competes with the IDE one —
/// side by side, two bare percentages read as a single confused number. When
/// it differs it is worth saying, because then the form on screen is not at
/// its real size. Both readouts are named, never bare.
pub fn zoom_label(canvas_zoom: f64, ui_zoom: f64) -> String {
    let ide = format!("IDE {:.0}%", ui_zoom * 100.0);
    if (canvas_zoom - ONE).abs() < 0.005 {
        return ide;
    }
    format!("Form {:.0}%  ·  {ide}", canvas_zoom * 100.0)
}

#[cfg(test)]
#[path = "test/ui_scale/test.rs"]
mod ui_scale_test;
