//! The fraction arithmetic, and the edges where it tends to go wrong.

use super::fraction;

/// It follows the value, and is **clamped** on both sides.
///
/// Clamped, not refused: a component does not paint outside its own box, and a
/// reading above the top of the scale is still a full bar.
#[test]
fn the_fraction_follows_the_value_and_is_clamped() {
    assert_eq!(fraction(0.0, 0.0, 100.0), 0.0);
    assert_eq!(fraction(20.0, 0.0, 100.0), 0.2);
    assert_eq!(fraction(100.0, 0.0, 100.0), 1.0);
    assert_eq!(
        fraction(-30.0, 0.0, 100.0),
        0.0,
        "below the range stays empty"
    );
    assert_eq!(
        fraction(500.0, 0.0, 100.0),
        1.0,
        "above the range stays full"
    );
}

/// **A zero-width range gives zero, not `NaN`.**
///
/// A manifest with `min == max` is wrong. An empty bar says so; a strange
/// symbol painted in the middle of the designer's form does not.
#[test]
fn a_zero_width_range_does_not_divide_by_zero() {
    let f = fraction(50.0, 10.0, 10.0);
    assert_eq!(f, 0.0);
    assert!(!f.is_nan());
}

/// The range need not start at zero, nor be increasing by chance.
#[test]
fn the_range_can_start_anywhere() {
    assert_eq!(
        fraction(0.0, -50.0, 50.0),
        0.5,
        "the middle of -50..50 is zero"
    );
    assert_eq!(fraction(-50.0, -50.0, 50.0), 0.0);
    assert_eq!(fraction(50.0, -50.0, 50.0), 1.0);
}

/// **An inverted range does not blow up.**
///
/// `min > max` is a wrong manifest just the same, and Rust's `clamp` panics
/// when the floor is above the ceiling. Here the divisor goes negative and the
/// result falls outside `0..=1` before the `clamp`, so the `clamp` still
/// receives `0.0, 1.0`, which is what it expects.
#[test]
fn an_inverted_range_does_not_panic() {
    let f = fraction(30.0, 100.0, 0.0);
    assert!((0.0..=1.0).contains(&f), "left the normalized range: {f}");
}
