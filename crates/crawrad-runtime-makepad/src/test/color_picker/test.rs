//! Tests for the color picker geometry helpers.

use super::{square_fractions, square_marker, strip_fraction};

#[test]
fn corners_of_square() {
    let pos = (100.0, 50.0);
    let size = (200.0, 200.0);

    // Top-left: saturation 0.0, value 1.0 (white)
    let (s, v) = square_fractions(pos, size, (100.0, 50.0));
    assert_eq!((s, v), (0.0, 1.0));

    // Top-right: saturation 1.0, value 1.0 (pure hue)
    let (s, v) = square_fractions(pos, size, (300.0, 50.0));
    assert_eq!((s, v), (1.0, 1.0));

    // Bottom-left: saturation 0.0, value 0.0 (black)
    let (s, v) = square_fractions(pos, size, (100.0, 250.0));
    assert_eq!((s, v), (0.0, 0.0));

    // Bottom-right: saturation 1.0, value 0.0 (black)
    let (s, v) = square_fractions(pos, size, (300.0, 250.0));
    assert_eq!((s, v), (1.0, 0.0));
}

#[test]
fn clamping_outside_square() {
    let pos = (50.0, 50.0);
    let size = (100.0, 100.0);

    // Far above and to the left
    let (s, v) = square_fractions(pos, size, (-100.0, -100.0));
    assert_eq!((s, v), (0.0, 1.0));

    // Far below and to the right
    let (s, v) = square_fractions(pos, size, (500.0, 500.0));
    assert_eq!((s, v), (1.0, 0.0));
}

#[test]
fn centre_of_square() {
    let pos = (20.0, 40.0);
    let size = (100.0, 100.0);

    let (s, v) = square_fractions(pos, size, (70.0, 90.0));
    assert!((s - 0.5).abs() < 1e-6);
    assert!((v - 0.5).abs() < 1e-6);
}

#[test]
fn round_trip_fractions_to_marker() {
    let pos = (10.0, 20.0);
    let size = (150.0, 150.0);

    let samples = [
        (0.0, 0.0),
        (0.0, 1.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.5, 0.5),
        (0.25, 0.75),
        (0.8, 0.2),
    ];

    for &(s, v) in &samples {
        let marker = square_marker(pos, size, s, v);
        let (calc_s, calc_v) = square_fractions(pos, size, marker);
        assert!(
            (calc_s - s).abs() < 1e-5,
            "s failed round trip: expected {s}, got {calc_s}"
        );
        assert!(
            (calc_v - v).abs() < 1e-5,
            "v failed round trip: expected {v}, got {calc_v}"
        );
    }
}

#[test]
fn strip_fraction_extremes_and_centre() {
    let top = 40.0;
    let height = 200.0;

    assert_eq!(strip_fraction(top, height, 40.0), 0.0);
    assert_eq!(strip_fraction(top, height, 240.0), 1.0);
    assert_eq!(strip_fraction(top, height, 140.0), 0.5);
    assert_eq!(strip_fraction(top, height, 90.0), 0.25);
}

#[test]
fn strip_fraction_clamping() {
    let top = 10.0;
    let height = 100.0;

    assert_eq!(strip_fraction(top, height, -50.0), 0.0);
    assert_eq!(strip_fraction(top, height, 200.0), 1.0);
}

#[test]
fn zero_or_negative_dimensions_safe() {
    assert_eq!(
        square_fractions((0.0, 0.0), (0.0, 0.0), (10.0, 10.0)),
        (0.0, 0.0)
    );
    assert_eq!(strip_fraction(10.0, 0.0, 20.0), 0.0);
    assert_eq!(strip_fraction(10.0, -10.0, 20.0), 0.0);
}
