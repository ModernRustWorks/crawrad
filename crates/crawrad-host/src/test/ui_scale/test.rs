//! Unit tests for pure UI scale and zoom helpers.

use super::{next_step, zoom_label};

#[test]
fn next_step_walks_ladder() {
    assert_eq!(next_step(1.0, true), 1.1);
    assert_eq!(next_step(1.0, false), 0.9);
}

#[test]
fn zoom_label_names_both_at_one() {
    // The canvas at real size is not announced; only the IDE scale is.
    assert_eq!(zoom_label(1.0, 1.0), "IDE 100%");
    assert_eq!(zoom_label(1.5, 1.0), "Form 150%  ·  IDE 100%");
}

#[test]
fn zoom_label_shows_both_when_ui_different() {
    assert_eq!(zoom_label(1.0, 1.25), "IDE 125%");
}

/// The canvas readout earns its place only when the canvas is not at 100%.
///
/// The owner (23/11/2026): at 100% the Form readout says nothing, and two
/// percentages side by side made him read the wrong one.
#[test]
fn the_form_readout_shows_only_when_the_canvas_is_not_real_size() {
    assert_eq!(zoom_label(1.0, 1.2), "IDE 120%");
    assert_eq!(zoom_label(1.0, 1.0), "IDE 100%");
    assert_eq!(zoom_label(1.5, 1.2), "Form 150%  ·  IDE 120%");
    assert_eq!(zoom_label(0.75, 1.0), "Form 75%  ·  IDE 100%");
}
