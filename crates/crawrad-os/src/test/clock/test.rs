use super::*;

#[test]
fn hh_mm_formatting() {
    assert_eq!(hh_mm(8, 5), "08:05");
    assert_eq!(hh_mm(23, 59), "23:59");
}

#[test]
fn local_hh_mm_shape() {
    let now = local_hh_mm();
    assert_eq!(now.len(), 5, "local_hh_mm must have length 5");
    assert_eq!(&now[2..3], ":", "local_hh_mm must have ':' at index 2");
}

#[test]
fn local_stamp_shape() {
    let s = local_stamp();
    assert_eq!(s.len(), 16, "{s}");
    assert_eq!(
        (&s[4..5], &s[7..8], &s[10..11], &s[13..14]),
        ("-", "-", " ", ":")
    );
}

#[test]
fn civil_days_round_trip_across_leap_years_and_eras() {
    assert_eq!(civil_from_days(0), (1970, 1, 1));
    assert_eq!(days_from_civil(2026, 10, 7), 20_733);
    assert_eq!(
        civil_from_days(days_from_civil(2024, 2, 29) + 1),
        (2024, 3, 1)
    );
    assert_eq!(
        civil_from_days(days_from_civil(2026, 1, 1) - 1),
        (2025, 12, 31)
    );
    for z in (-800_000..800_000).step_by(997) {
        let (y, m, d) = civil_from_days(z);
        assert_eq!(days_from_civil(y, m, d), z);
    }
}

#[test]
fn local_offset_seconds_bounds_and_alignment() {
    let offset = local_offset_seconds();
    assert!(
        (-14 * 3600..=14 * 3600).contains(&offset),
        "local offset {offset} must be within ±14 hours"
    );
    assert_eq!(
        offset % 60,
        0,
        "local offset {offset} must be a whole number of minutes"
    );
}
