use super::{
    DateField, DateTimeKind, TimeField, box_text, clamp_iso, date_field_at, days_in_month,
    format_iso_date, format_iso_time, format_shown_date, format_shown_time, is_leap_year,
    iso_to_shown, parse_iso_date, parse_iso_time, parse_shown_date, parse_shown_time, shown_to_iso,
    step_date_field, step_iso, step_time_field, time_field_at, validate_date, validate_time,
};

#[test]
fn leap_years_follow_gregorian_rules() {
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2024));
    assert!(is_leap_year(2028));
    assert!(!is_leap_year(1900));
    assert!(!is_leap_year(2100));
    assert!(!is_leap_year(2023));
}

#[test]
fn days_in_month_counts_correctly() {
    assert_eq!(days_in_month(2024, 1), 31);
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2023, 2), 28);
    assert_eq!(days_in_month(2024, 4), 30);
    assert_eq!(days_in_month(2024, 12), 31);
    assert_eq!(days_in_month(2024, 0), 0);
    assert_eq!(days_in_month(2024, 13), 0);
}

#[test]
fn date_validation_rejects_impossible_dates() {
    assert!(validate_date(2024, 2, 29));
    assert!(!validate_date(2023, 2, 29));
    assert!(!validate_date(2024, 4, 31));
    assert!(validate_date(2024, 12, 31));
    assert!(!validate_date(2024, 0, 10));
    assert!(!validate_date(2024, 13, 1));
    assert!(!validate_date(0, 1, 1));
}

#[test]
fn time_validation_rejects_out_of_range_units() {
    assert!(validate_time(0, 0, 0));
    assert!(validate_time(23, 59, 59));
    assert!(!validate_time(24, 0, 0));
    assert!(!validate_time(12, 60, 0));
    assert!(!validate_time(12, 0, 60));
}

#[test]
fn iso_date_and_time_round_trip() {
    assert_eq!(parse_iso_date("2024-02-29"), Some((2024, 2, 29)));
    assert_eq!(parse_iso_date("invalid"), None);
    assert_eq!(parse_iso_date("2023-02-29"), None);

    assert_eq!(parse_iso_time("23:59:58"), Some((23, 59, 58)));
    assert_eq!(parse_iso_time("24:00:00"), None);

    assert_eq!(format_iso_date(2024, 2, 9), "2024-02-09");
    assert_eq!(format_iso_time(9, 5, 1), "09:05:01");
}

#[test]
fn shown_date_and_time_round_trip() {
    assert_eq!(parse_shown_date("29/02/2024"), Some((2024, 2, 29)));
    assert_eq!(parse_shown_date("31/04/2024"), None);
    assert_eq!(parse_shown_date("1/2/2024"), None);

    assert_eq!(parse_shown_time("14:30:00"), Some((14, 30, 0)));
    assert_eq!(parse_shown_time("14:60:00"), None);

    assert_eq!(format_shown_date(2024, 2, 9), "09/02/2024");
    assert_eq!(format_shown_time(9, 5, 1), "09:05:01");
}

#[test]
fn conversion_between_iso_and_shown() {
    assert_eq!(iso_to_shown(DateTimeKind::Date, "2024-02-29"), "29/02/2024");
    assert_eq!(iso_to_shown(DateTimeKind::Time, "15:45:00"), "15:45:00");
    assert_eq!(
        shown_to_iso(DateTimeKind::Date, "29/02/2024"),
        Some("2024-02-29".to_string())
    );
    assert_eq!(
        shown_to_iso(DateTimeKind::Time, "15:45:00"),
        Some("15:45:00".to_string())
    );
    assert_eq!(shown_to_iso(DateTimeKind::Date, "31/02/2024"), None);
    assert_eq!(shown_to_iso(DateTimeKind::Time, "25:00:00"), None);
}

#[test]
fn clamping_respects_bounds() {
    assert_eq!(
        clamp_iso("2024-06-15", "2024-01-01", "2024-12-31"),
        "2024-06-15"
    );
    assert_eq!(
        clamp_iso("2023-12-31", "2024-01-01", "2024-12-31"),
        "2024-01-01"
    );
    assert_eq!(
        clamp_iso("2025-01-01", "2024-01-01", "2024-12-31"),
        "2024-12-31"
    );
    assert_eq!(clamp_iso("2025-01-01", "", ""), "2025-01-01");
}

#[test]
fn field_detection_by_caret_index() {
    assert_eq!(date_field_at(0), DateField::Day);
    assert_eq!(date_field_at(2), DateField::Day);
    assert_eq!(date_field_at(3), DateField::Month);
    assert_eq!(date_field_at(5), DateField::Month);
    assert_eq!(date_field_at(6), DateField::Year);
    assert_eq!(date_field_at(10), DateField::Year);

    assert_eq!(time_field_at(0), TimeField::Hour);
    assert_eq!(time_field_at(2), TimeField::Hour);
    assert_eq!(time_field_at(3), TimeField::Minute);
    assert_eq!(time_field_at(5), TimeField::Minute);
    assert_eq!(time_field_at(6), TimeField::Second);
    assert_eq!(time_field_at(8), TimeField::Second);
}

#[test]
fn date_field_stepping_wraps_and_clamps() {
    // Day stepping wraps within month
    assert_eq!(
        step_date_field(2024, 1, 31, DateField::Day, true),
        (2024, 1, 1)
    );
    assert_eq!(
        step_date_field(2024, 1, 1, DateField::Day, false),
        (2024, 1, 31)
    );

    // Month stepping wraps 1..12 and clamps day
    assert_eq!(
        step_date_field(2024, 12, 15, DateField::Month, true),
        (2024, 1, 15)
    );
    assert_eq!(
        step_date_field(2024, 1, 15, DateField::Month, false),
        (2024, 12, 15)
    );
    // 31 March stepping down to February clamps to 29 (2024 is leap)
    assert_eq!(
        step_date_field(2024, 3, 31, DateField::Month, false),
        (2024, 2, 29)
    );
    // 31 March 2023 stepping down to February clamps to 28
    assert_eq!(
        step_date_field(2023, 3, 31, DateField::Month, false),
        (2023, 2, 28)
    );

    // Year stepping: 29 Feb 2024 stepped to 2025 clamps to 28 Feb
    assert_eq!(
        step_date_field(2024, 2, 29, DateField::Year, true),
        (2025, 2, 28)
    );
}

#[test]
fn time_field_stepping_wraps() {
    assert_eq!(
        step_time_field(23, 30, 0, TimeField::Hour, true),
        (0, 30, 0)
    );
    assert_eq!(
        step_time_field(0, 30, 0, TimeField::Hour, false),
        (23, 30, 0)
    );
    assert_eq!(
        step_time_field(12, 59, 0, TimeField::Minute, true),
        (12, 0, 0)
    );
    assert_eq!(
        step_time_field(12, 0, 0, TimeField::Minute, false),
        (12, 59, 0)
    );
    assert_eq!(
        step_time_field(12, 0, 59, TimeField::Second, true),
        (12, 0, 0)
    );
    assert_eq!(
        step_time_field(12, 0, 0, TimeField::Second, false),
        (12, 0, 59)
    );
}

#[test]
fn step_iso_with_caret_and_limits() {
    // Step day up at caret 0
    let next = step_iso(DateTimeKind::Date, "2024-01-10", 0, true, "", "");
    assert_eq!(next, Some("2024-01-11".to_string()));

    // Step with min clamp
    let clamped_min = step_iso(
        DateTimeKind::Date,
        "2024-01-05",
        0,
        false,
        "2024-01-05",
        "2024-01-10",
    );
    assert_eq!(clamped_min, Some("2024-01-05".to_string()));

    // Step time hour down at caret 1
    let next_time = step_iso(DateTimeKind::Time, "00:15:30", 1, false, "", "");
    assert_eq!(next_time, Some("23:15:30".to_string()));
}

#[test]
fn box_text_preserves_typing_until_valid_and_differs() {
    // Date: half-typed returns None
    assert_eq!(
        box_text(DateTimeKind::Date, "12/0_/____", "2000-01-01"),
        None
    );

    // Date: invalid complete date returns None
    assert_eq!(
        box_text(DateTimeKind::Date, "31/02/2024", "2024-02-01"),
        None
    );

    // Date: valid date matching value returns None
    assert_eq!(
        box_text(DateTimeKind::Date, "01/01/2000", "2000-01-01"),
        None
    );

    // Date: valid date differing from value returns Some(iso_to_shown(kind, value))
    assert_eq!(
        box_text(DateTimeKind::Date, "15/06/2024", "2000-01-01"),
        Some("01/01/2000".to_string())
    );

    // Time: half-typed returns None
    assert_eq!(box_text(DateTimeKind::Time, "12:__:__", "00:00:00"), None);

    // Time: invalid complete time returns None
    assert_eq!(box_text(DateTimeKind::Time, "25:00:00", "12:00:00"), None);

    // Time: valid time matching value returns None
    assert_eq!(box_text(DateTimeKind::Time, "12:00:00", "12:00:00"), None);

    // Time: valid time differing from value returns Some(iso_to_shown(kind, value))
    assert_eq!(
        box_text(DateTimeKind::Time, "15:30:00", "12:00:00"),
        Some("12:00:00".to_string())
    );
}

#[test]
fn no_value_shows_an_empty_box() {
    assert_eq!(iso_to_shown(DateTimeKind::Date, ""), "");
    assert_eq!(iso_to_shown(DateTimeKind::Time, "  "), "");
}
