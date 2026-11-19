use crate::DateTime;

#[test]
fn default_is_epoch() {
    let dt = DateTime::default();
    assert_eq!(dt.year, 1970);
    assert_eq!(dt.month, 1);
    assert_eq!(dt.day, 1);
    assert_eq!(dt.hour, 0);
    assert_eq!(dt.minute, 0);
    assert_eq!(dt.second, 0);
}

#[test]
fn constructors_and_accessors() {
    let dt = DateTime::new(2026, 9, 19, 14, 30, 45);
    assert_eq!(dt.year, 2026);
    assert_eq!(dt.month, 9);
    assert_eq!(dt.day, 19);
    assert_eq!(dt.hour, 14);
    assert_eq!(dt.minute, 30);
    assert_eq!(dt.second, 45);

    let from_d = DateTime::from_date(2026, 12, 25);
    assert_eq!(from_d.year, 2026);
    assert_eq!(from_d.month, 12);
    assert_eq!(from_d.day, 25);
    assert_eq!(from_d.hour, 0);
    assert_eq!(from_d.minute, 0);
    assert_eq!(from_d.second, 0);

    let from_t = DateTime::from_time(10, 15, 20);
    assert_eq!(from_t.year, 1970);
    assert_eq!(from_t.month, 1);
    assert_eq!(from_t.day, 1);
    assert_eq!(from_t.hour, 10);
    assert_eq!(from_t.minute, 15);
    assert_eq!(from_t.second, 20);
}

#[test]
fn formatting_iso_date_time() {
    let dt = DateTime::new(2026, 5, 4, 8, 5, 9);
    assert_eq!(dt.to_iso8601(), "2026-05-04T08:05:09");
    assert_eq!(dt.format_date(), "2026-05-04");
    assert_eq!(dt.format_time(), "08:05:09");
    assert_eq!(format!("{dt}"), "2026-05-04 08:05:09");
}

#[test]
fn parse_iso8601_and_variations() {
    let parsed = DateTime::parse("2026-09-19T14:30:45").unwrap();
    assert_eq!(parsed, DateTime::new(2026, 9, 19, 14, 30, 45));

    let parsed_space = DateTime::parse("2026-09-19 14:30:45").unwrap();
    assert_eq!(parsed_space, DateTime::new(2026, 9, 19, 14, 30, 45));

    let parsed_date_only = DateTime::parse("2026-09-19").unwrap();
    assert_eq!(parsed_date_only, DateTime::new(2026, 9, 19, 0, 0, 0));

    let parsed_padded = DateTime::parse("  2026-01-02T03:04:05  ").unwrap();
    assert_eq!(parsed_padded, DateTime::new(2026, 1, 2, 3, 4, 5));
}

#[test]
fn parse_rejects_invalid_values() {
    assert!(DateTime::parse("").is_err());
    assert!(DateTime::parse("not-a-date").is_err());
    assert!(DateTime::parse("2026-13-01T00:00:00").is_err()); // month 13
    assert!(DateTime::parse("2026-00-01T00:00:00").is_err()); // month 0
    assert!(DateTime::parse("2026-01-32T00:00:00").is_err()); // day 32
    assert!(DateTime::parse("2026-01-01T24:00:00").is_err()); // hour 24
    assert!(DateTime::parse("2026-01-01T12:60:00").is_err()); // minute 60
    assert!(DateTime::parse("2026-01-01T12:00:60").is_err()); // second 60
}
