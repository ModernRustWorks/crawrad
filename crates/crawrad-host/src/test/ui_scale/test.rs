//! Unit tests for pure UI scale and zoom helpers.

use super::{next_step, zoom_label};

#[test]
fn next_step_walks_ladder() {
    assert_eq!(next_step(1.0, true), 1.1);
    assert_eq!(next_step(1.0, false), 0.9);
}

#[test]
fn zoom_label_names_both_at_one() {
    assert_eq!(zoom_label(1.0, 1.0), "Form 100%  ·  IDE 100%");
    assert_eq!(zoom_label(1.5, 1.0), "Form 150%  ·  IDE 100%");
}

#[test]
fn zoom_label_shows_both_when_ui_different() {
    assert_eq!(zoom_label(1.0, 1.25), "Form 100%  ·  IDE 125%");
}
