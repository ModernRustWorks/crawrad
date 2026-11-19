//! Tests for ActionBulb colors and pulse animation logic.

use super::*;

#[test]
fn test_bulb_colors_all_four_combinations() {
    // Actions only (preferred == false), dark theme
    let (fill, outline, r) = bulb_colors(false, true);
    assert_eq!(fill, "#3a2a10");
    assert_eq!(outline, "#ffb547");
    assert_eq!(r, "#fff4dc");

    // Actions only (preferred == false), light theme
    let (fill, outline, r) = bulb_colors(false, false);
    assert_eq!(fill, "#ffd27a");
    assert_eq!(outline, "#b86e00");
    assert_eq!(r, "#6b3d00");

    // Quick fix (preferred == true), dark theme
    let (fill, outline, r) = bulb_colors(true, true);
    assert_eq!(fill, "#5a3d0c");
    assert_eq!(outline, "#ffcf5c");
    assert_eq!(r, "#fffaf0");

    // Quick fix (preferred == true), light theme
    let (fill, outline, r) = bulb_colors(true, false);
    assert_eq!(fill, "#ffc24d");
    assert_eq!(outline, "#a35f00");
    assert_eq!(r, "#5a3300");
}

#[test]
fn test_pulse_alpha_period_and_values() {
    let eps = 1e-5;

    // t = 0: minimum opacity (0.55)
    let a0 = pulse_alpha(0.0);
    assert!((a0 - 0.55).abs() < eps, "expected 0.55 at t=0, got {a0}");

    // t = 0.8: peak opacity (1.0)
    let a_mid = pulse_alpha(0.8);
    assert!(
        (a_mid - 1.0).abs() < eps,
        "expected 1.0 at t=0.8, got {a_mid}"
    );

    // t = 1.6: full cycle back to minimum opacity (0.55)
    let a_cycle = pulse_alpha(1.6);
    assert!(
        (a_cycle - 0.55).abs() < eps,
        "expected 0.55 at t=1.6, got {a_cycle}"
    );

    // Next period peak at t = 2.4 (1.6 + 0.8)
    let a_next_peak = pulse_alpha(2.4);
    assert!(
        (a_next_peak - 1.0).abs() < eps,
        "expected 1.0 at t=2.4, got {a_next_peak}"
    );

    // Next period minimum at t = 3.2 (1.6 * 2)
    let a_next_min = pulse_alpha(3.2);
    assert!(
        (a_next_min - 0.55).abs() < eps,
        "expected 0.55 at t=3.2, got {a_next_min}"
    );
}
