// This file is EMITTED VERBATIM into the user's project. Everything in it --
// comments included -- ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules, the same ones `strings.rs` carries:
//
// 1. No `//!` and no `#![...]`. This text lands in the MIDDLE of a file, after
//    other items, and an inner attribute after an item is `error[E0753]`.
// 2. No `use`, and no type of ours. Only `std`. On the other side none of our
//    crates exist -- the generated project depends on none of them, and that
//    is the proof this is generation and not an interpreter in disguise.
//    Do not NAME one either, not even in a comment: the gate that proves it
//    reads the whole emitted file, and it is right to.

use std::fmt;

/// A date and time: year, month, day, hour, minute, second.
///
/// Pure data representation with zero external dependencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    /// Full date and time.
    pub const fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Date with time set to midnight (00:00:00).
    pub const fn from_date(year: i32, month: u8, day: u8) -> Self {
        Self::new(year, month, day, 0, 0, 0)
    }

    /// Time on Unix epoch date (1970-01-01).
    pub const fn from_time(hour: u8, minute: u8, second: u8) -> Self {
        Self::new(1970, 1, 1, hour, minute, second)
    }

    /// Unix epoch: 1970-01-01 00:00:00.
    pub const fn epoch() -> Self {
        Self::new(1970, 1, 1, 0, 0, 0)
    }

    /// Format as ISO-8601 string: `YYYY-MM-DDTHH:MM:SS`.
    pub fn to_iso8601(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// Format as date string: `YYYY-MM-DD`.
    pub fn format_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Format as time string: `HH:MM:SS`.
    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// Parse from an ISO-8601 string (`YYYY-MM-DDTHH:MM:SS` or `YYYY-MM-DD HH:MM:SS` or `YYYY-MM-DD`).
    pub fn parse(text: &str) -> Result<Self, &'static str> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err("empty date string");
        }

        let (date_part, time_part) = if let Some(idx) = trimmed.find(['T', ' ']) {
            (&trimmed[..idx], Some(&trimmed[idx + 1..]))
        } else {
            (trimmed, None)
        };

        let mut date_segs = date_part.split('-');
        let year_str = date_segs.next().ok_or("missing year")?;
        let month_str = date_segs.next().ok_or("missing month")?;
        let day_str = date_segs.next().ok_or("missing day")?;
        if date_segs.next().is_some() {
            return Err("too many date segments");
        }

        let year: i32 = year_str.parse().map_err(|_| "invalid year")?;
        let month: u8 = month_str.parse().map_err(|_| "invalid month")?;
        let day: u8 = day_str.parse().map_err(|_| "invalid day")?;

        if !matches!(month, 1..=12) {
            return Err("month out of bounds (1..=12)");
        }
        if !matches!(day, 1..=31) {
            return Err("day out of bounds (1..=31)");
        }

        let (hour, minute, second) = if let Some(t) = time_part {
            let mut time_segs = t.split(':');
            let h_str = time_segs.next().ok_or("missing hour")?;
            let m_str = time_segs.next().ok_or("missing minute")?;
            let s_str = time_segs.next().unwrap_or("00");

            let h: u8 = h_str.parse().map_err(|_| "invalid hour")?;
            let m: u8 = m_str.parse().map_err(|_| "invalid minute")?;
            let s: u8 = s_str.parse().map_err(|_| "invalid second")?;

            if h > 23 {
                return Err("hour out of bounds (0..=23)");
            }
            if m > 59 {
                return Err("minute out of bounds (0..=59)");
            }
            if s > 59 {
                return Err("second out of bounds (0..=59)");
            }
            (h, m, s)
        } else {
            (0, 0, 0)
        };

        Ok(Self::new(year, month, day, hour, minute, second))
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::epoch()
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
