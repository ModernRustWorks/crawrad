//! Pure date and time parsing, formatting, validation and caret-field stepping
//! for `DateTimePicker`.
//!
//! This module has no external dependencies and uses ISO-8601 strings
//! (`YYYY-MM-DD` and `HH:MM:SS`) as canonical runtime values, matching the
//! classic control's behavior while keeping the representation portable.

use crate::mask::EditMask;
use crawrad_note::note;

#[note("date-time-date-mask")]
pub const DATE_MASK: &str = "00/00/0000;1;_";

#[note("date-time-time-mask")]
pub const TIME_MASK: &str = "00:00:00;1;_";

#[note("date-time-default-date")]
pub const DEFAULT_DATE: &str = "2000-01-01";

#[note("date-time-default-time")]
pub const DEFAULT_TIME: &str = "00:00:00";

#[note("date-time-datetimekind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DateTimeKind {
    /// Date mode: `YYYY-MM-DD` canonical value, displayed as `dd/mm/yyyy`.
    #[default]
    Date,
    /// Time mode: `HH:MM:SS` canonical value, displayed as `HH:MM:SS`.
    Time,
}

impl DateTimeKind {
    #[note("date-time-datetimekind-parse-kind")]
    pub fn parse_kind(s: &str) -> Self {
        if s.eq_ignore_ascii_case("dtkTime") {
            Self::Time
        } else {
            Self::Date
        }
    }

    #[note("date-time-datetimekind-as-str")]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Date => "dtkDate",
            Self::Time => "dtkTime",
        }
    }

    #[note("date-time-datetimekind-mask")]
    pub fn mask(&self) -> &'static str {
        match self {
            Self::Date => DATE_MASK,
            Self::Time => TIME_MASK,
        }
    }

    #[note("date-time-datetimekind-default-value")]
    pub fn default_value(&self) -> &'static str {
        match self {
            Self::Date => DEFAULT_DATE,
            Self::Time => DEFAULT_TIME,
        }
    }
}

#[note("date-time-datefield")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateField {
    Day,
    Month,
    Year,
}

#[note("date-time-timefield")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeField {
    Hour,
    Minute,
    Second,
}

#[note("date-time-is-leap-year")]
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[note("date-time-days-in-month")]
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

#[note("date-time-validate-date")]
pub fn validate_date(year: i32, month: u32, day: u32) -> bool {
    (1..=9999).contains(&year)
        && (1..=12).contains(&month)
        && day >= 1
        && day <= days_in_month(year, month)
}

#[note("date-time-validate-time")]
pub fn validate_time(hour: u32, min: u32, sec: u32) -> bool {
    hour <= 23 && min <= 59 && sec <= 59
}

#[note("date-time-parse-iso-date")]
pub fn parse_iso_date(iso: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;
    if validate_date(year, month, day) {
        Some((year, month, day))
    } else {
        None
    }
}

#[note("date-time-parse-iso-time")]
pub fn parse_iso_time(iso: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = iso.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hour: u32 = parts[0].parse().ok()?;
    let min: u32 = parts[1].parse().ok()?;
    let sec: u32 = parts[2].parse().ok()?;
    if validate_time(hour, min, sec) {
        Some((hour, min, sec))
    } else {
        None
    }
}

#[note("date-time-parse-shown-date")]
pub fn parse_shown_date(shown: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = shown.split('/').collect();
    if parts.len() != 3 {
        return None;
    }
    if parts[0].len() != 2 || parts[1].len() != 2 || parts[2].len() != 4 {
        return None;
    }
    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let year: i32 = parts[2].parse().ok()?;
    if validate_date(year, month, day) {
        Some((year, month, day))
    } else {
        None
    }
}

#[note("date-time-parse-shown-time")]
pub fn parse_shown_time(shown: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = shown.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    if parts[0].len() != 2 || parts[1].len() != 2 || parts[2].len() != 2 {
        return None;
    }
    let hour: u32 = parts[0].parse().ok()?;
    let min: u32 = parts[1].parse().ok()?;
    let sec: u32 = parts[2].parse().ok()?;
    if validate_time(hour, min, sec) {
        Some((hour, min, sec))
    } else {
        None
    }
}

#[note("date-time-format-iso-date")]
pub fn format_iso_date(year: i32, month: u32, day: u32) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

#[note("date-time-format-iso-time")]
pub fn format_iso_time(hour: u32, min: u32, sec: u32) -> String {
    format!("{hour:02}:{min:02}:{sec:02}")
}

#[note("date-time-format-shown-date")]
pub fn format_shown_date(year: i32, month: u32, day: u32) -> String {
    format!("{day:02}/{month:02}/{year:04}")
}

#[note("date-time-format-shown-time")]
pub fn format_shown_time(hour: u32, min: u32, sec: u32) -> String {
    format!("{hour:02}:{min:02}:{sec:02}")
}

#[note("date-time-iso-to-shown")]
pub fn iso_to_shown(kind: DateTimeKind, iso: &str) -> String {
    // No value, no text: the box is empty until something is typed.
    if iso.trim().is_empty() {
        return String::new();
    }
    match kind {
        DateTimeKind::Date => {
            if let Some((y, m, d)) = parse_iso_date(iso.trim()) {
                format_shown_date(y, m, d)
            } else {
                EditMask::parse(DATE_MASK).format(iso)
            }
        }
        DateTimeKind::Time => {
            if let Some((h, m, s)) = parse_iso_time(iso.trim()) {
                format_shown_time(h, m, s)
            } else {
                EditMask::parse(TIME_MASK).format(iso)
            }
        }
    }
}

#[note("date-time-shown-to-iso")]
pub fn shown_to_iso(kind: DateTimeKind, shown: &str) -> Option<String> {
    match kind {
        DateTimeKind::Date => {
            let (y, m, d) = parse_shown_date(shown.trim())?;
            Some(format_iso_date(y, m, d))
        }
        DateTimeKind::Time => {
            let (h, m, s) = parse_shown_time(shown.trim())?;
            Some(format_iso_time(h, m, s))
        }
    }
}

#[note("date-time-box-text")]
pub fn box_text(kind: DateTimeKind, typed: &str, value: &str) -> Option<String> {
    match shown_to_iso(kind, typed) {
        None => None,
        Some(ref iso) if iso == value => None,
        Some(_) => Some(iso_to_shown(kind, value)),
    }
}

#[note("date-time-clamp-iso")]
pub fn clamp_iso(iso: &str, min: &str, max: &str) -> String {
    let mut val = iso;
    let min = min.trim();
    let max = max.trim();
    if !min.is_empty() && val < min {
        val = min;
    }
    if !max.is_empty() && val > max {
        val = max;
    }
    val.to_string()
}

#[note("date-time-date-field-at")]
pub fn date_field_at(caret: usize) -> DateField {
    if caret <= 2 {
        DateField::Day
    } else if caret <= 5 {
        DateField::Month
    } else {
        DateField::Year
    }
}

#[note("date-time-time-field-at")]
pub fn time_field_at(caret: usize) -> TimeField {
    if caret <= 2 {
        TimeField::Hour
    } else if caret <= 5 {
        TimeField::Minute
    } else {
        TimeField::Second
    }
}

#[note("date-time-step-date-field")]
pub fn step_date_field(
    mut year: i32,
    mut month: u32,
    mut day: u32,
    field: DateField,
    up: bool,
) -> (i32, u32, u32) {
    match field {
        DateField::Day => {
            let max_days = days_in_month(year, month);
            if up {
                day = if day >= max_days { 1 } else { day + 1 };
            } else {
                day = if day <= 1 { max_days } else { day - 1 };
            }
        }
        DateField::Month => {
            if up {
                month = if month >= 12 { 1 } else { month + 1 };
            } else {
                month = if month <= 1 { 12 } else { month - 1 };
            }
            let max_days = days_in_month(year, month);
            if day > max_days {
                day = max_days;
            }
        }
        DateField::Year => {
            if up {
                year = if year >= 9999 { 1 } else { year + 1 };
            } else {
                year = if year <= 1 { 9999 } else { year - 1 };
            }
            let max_days = days_in_month(year, month);
            if day > max_days {
                day = max_days;
            }
        }
    }
    (year, month, day)
}

#[note("date-time-step-time-field")]
pub fn step_time_field(
    mut hour: u32,
    mut min: u32,
    mut sec: u32,
    field: TimeField,
    up: bool,
) -> (u32, u32, u32) {
    match field {
        TimeField::Hour => {
            if up {
                hour = (hour + 1) % 24;
            } else {
                hour = if hour == 0 { 23 } else { hour - 1 };
            }
        }
        TimeField::Minute => {
            if up {
                min = (min + 1) % 60;
            } else {
                min = if min == 0 { 59 } else { min - 1 };
            }
        }
        TimeField::Second => {
            if up {
                sec = (sec + 1) % 60;
            } else {
                sec = if sec == 0 { 59 } else { sec - 1 };
            }
        }
    }
    (hour, min, sec)
}

#[note("date-time-step-iso")]
pub fn step_iso(
    kind: DateTimeKind,
    iso: &str,
    caret: usize,
    up: bool,
    min: &str,
    max: &str,
) -> Option<String> {
    match kind {
        DateTimeKind::Date => {
            let (y, m, d) = parse_iso_date(iso.trim())?;
            let field = date_field_at(caret);
            let (ny, nm, nd) = step_date_field(y, m, d, field, up);
            let next_iso = format_iso_date(ny, nm, nd);
            Some(clamp_iso(&next_iso, min, max))
        }
        DateTimeKind::Time => {
            let (h, m, s) = parse_iso_time(iso.trim())?;
            let field = time_field_at(caret);
            let (nh, nm, ns) = step_time_field(h, m, s, field, up);
            let next_iso = format_iso_time(nh, nm, ns);
            Some(clamp_iso(&next_iso, min, max))
        }
    }
}

#[note("date-time-shown-of")]
pub fn shown_of(node: &crate::Node) -> String {
    let text = |key: &str| match node.props.get(key) {
        Some(crate::Value::Enum(s) | crate::Value::Str(s)) => Some(s.as_str()),
        _ => None,
    };
    let kind = DateTimeKind::parse_kind(text("kind").unwrap_or("dtkDate"));
    iso_to_shown(kind, text("date_time").unwrap_or(kind.default_value()))
}

#[cfg(test)]
#[path = "test/date_time/test.rs"]
mod test;
