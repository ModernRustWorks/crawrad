//! The Card's header and shadow arithmetic.

use super::{header_height, shadow};

#[test]
fn the_header_grows_with_a_subtitle() {
    assert_eq!(header_height(""), 28.0);
    assert_eq!(header_height("Details"), 42.0);
}

#[test]
fn elevation_zero_casts_no_shadow() {
    assert_eq!(shadow(0), (0.0, 0.0));
    assert_eq!(shadow(-1), (0.0, 0.0));
    let (a1, o1) = shadow(1);
    let (a3, o3) = shadow(9);
    assert!((a1 - 0.2).abs() < 1e-6 && o1 == 2.0);
    assert!((a3 - 0.36).abs() < 1e-6 && o3 == 6.0, "clamped to 3");
}
