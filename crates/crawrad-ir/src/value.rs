//! Typed property values.
//!
//! Closed and serializable on purpose. It was the `Box<dyn Any>` of the old core
//! that prevented generic undo, metadata-driven inspector, and crossing the
//! plugin boundary — see §3.7 of the study.

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "v")]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    /// Value from a closed set declared by the component (alignment, cursor…).
    Enum(String),
    /// **Reference to another component of the same form**, by id: it is the
    /// `Action1` that a button points to, the `ImageList1` that an action list
    /// uses. Empty string = none, the `nil` of the classic RAD IDE. What ensures that the target
    /// exists is `validate` — dangling references are silent garbage.
    Ref(String),
    Color(Color),
    Font(Font),
    Rect(Rect),
    Edges(Edges),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
    Null,
    DateTime(DateTime),
    FilePath(FilePath),
    Cursor(Cursor),
    ModalResult(ModalResult),
    Shortcut(Shortcut),
    Picture(Picture),
}

#[note("value-no-reference")]
pub const NO_REFERENCE: &str = "(none)";

impl Value {
    #[note("value-value-describe")]
    pub fn describe(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => format!("{f:.2}"),
            Value::Bool(b) => b.to_string(),
            Value::Rect(r) => format!("{}, {}, {} \u{d7} {}", r.x, r.y, r.w, r.h),
            Value::Edges(e) => format!("{}, {}, {}, {}", e.left, e.top, e.right, e.bottom),
            Value::Enum(e) => e.clone(),
            Value::Ref(r) if r.is_empty() => NO_REFERENCE.to_string(),
            Value::Ref(r) => r.clone(),
            Value::Color(c) if c.a == 255 => format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b),
            Value::Color(c) => format!("#{:02X}{:02X}{:02X}{:02X}", c.r, c.g, c.b, c.a),
            // Nameless and sizeless is the catalogue's default: the parent's font.
            Value::Font(f) if f.name.trim().is_empty() && f.size <= 0.0 => "(default)".to_string(),
            Value::Font(f) => f.to_string_formatted(),
            Value::Map(m) => {
                let on: Vec<&str> = ["left", "top", "right", "bottom"]
                    .into_iter()
                    .filter(|k| matches!(m.get(*k), Some(Value::Bool(true))))
                    .collect();
                format!("[{}]", on.join(","))
            }
            Value::List(items) => {
                let n = crate::Item::total(&crate::Item::list_from(self)).max(items.len());
                match n {
                    0 => "(empty)".to_string(),
                    1 => "1 item".to_string(),
                    n => format!("{n} items"),
                }
            }
            other => other.kind().to_string(),
        }
    }

    #[note("value-value-kind")]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Str(_) => "str",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
            Self::Enum(_) => "enum",
            Self::Ref(_) => "ref",
            Self::Color(_) => "color",
            Self::Font(_) => "font",
            Self::Rect(_) => "rect",
            Self::Edges(_) => "edges",
            Self::List(_) => "list",
            Self::Map(_) => "map",
            Self::Null => "null",
            Self::DateTime(_) => "datetime",
            Self::FilePath(_) => "file_path",
            Self::Cursor(_) => "cursor",
            Self::ModalResult(_) => "modal_result",
            Self::Shortcut(_) => "shortcut",
            Self::Picture(_) => "picture",
        }
    }

    pub fn as_picture(&self) -> Option<&Picture> {
        match self {
            Self::Picture(p) => Some(p),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_file_path(&self) -> Option<&str> {
        match self {
            Self::FilePath(fp) => Some(fp.as_str()),
            _ => None,
        }
    }

    pub fn as_rect(&self) -> Option<Rect> {
        match self {
            Self::Rect(r) => Some(*r),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_datetime(&self) -> Option<DateTime> {
        match self {
            Self::DateTime(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_cursor(&self) -> Option<Cursor> {
        match self {
            Self::Cursor(c) => Some(*c),
            _ => None,
        }
    }

    pub fn as_modal_result(&self) -> Option<ModalResult> {
        match self {
            Self::ModalResult(m) => Some(*m),
            _ => None,
        }
    }

    pub fn as_shortcut(&self) -> Option<Shortcut> {
        match self {
            Self::Shortcut(s) => Some(*s),
            _ => None,
        }
    }

    pub fn as_font(&self) -> Option<&Font> {
        match self {
            Self::Font(f) => Some(f),
            _ => None,
        }
    }
}

#[note("value-modalresult")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModalResult {
    #[default]
    None,
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
    Close,
}

impl ModalResult {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Ok => "ok",
            Self::Cancel => "cancel",
            Self::Abort => "abort",
            Self::Retry => "retry",
            Self::Ignore => "ignore",
            Self::Yes => "yes",
            Self::No => "no",
            Self::Close => "close",
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "none" | "mrnone" => Some(Self::None),
            "ok" | "mrok" => Some(Self::Ok),
            "cancel" | "mrcancel" => Some(Self::Cancel),
            "abort" | "mrabort" => Some(Self::Abort),
            "retry" | "mrretry" => Some(Self::Retry),
            "ignore" | "mrignore" => Some(Self::Ignore),
            "yes" | "mryes" => Some(Self::Yes),
            "no" | "mrno" => Some(Self::No),
            "close" | "mrclose" => Some(Self::Close),
            _ => None,
        }
    }

    pub const fn as_i32(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Ok => 1,
            Self::Cancel => 2,
            Self::Abort => 3,
            Self::Retry => 4,
            Self::Ignore => 5,
            Self::Yes => 6,
            Self::No => 7,
            Self::Close => 8,
        }
    }

    pub const fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(Self::None),
            1 => Some(Self::Ok),
            2 => Some(Self::Cancel),
            3 => Some(Self::Abort),
            4 => Some(Self::Retry),
            5 => Some(Self::Ignore),
            6 => Some(Self::Yes),
            7 => Some(Self::No),
            8 => Some(Self::Close),
            _ => None,
        }
    }

    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_affirmative(&self) -> bool {
        matches!(self, Self::Ok | Self::Yes)
    }

    pub const fn is_dismissive(&self) -> bool {
        matches!(self, Self::Cancel | Self::Abort | Self::Close)
    }
}

impl std::fmt::Display for ModalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for ModalResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}

#[note("value-cursor")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cursor {
    #[default]
    Default,
    Arrow,
    Hand,
    Text,
    Crosshair,
    Move,
    Wait,
    NotAllowed,
    ResizeNs,
    ResizeEw,
    ResizeNesw,
    ResizeNwse,
}

impl Cursor {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Arrow => "arrow",
            Self::Hand => "hand",
            Self::Text => "text",
            Self::Crosshair => "crosshair",
            Self::Move => "move",
            Self::Wait => "wait",
            Self::NotAllowed => "not_allowed",
            Self::ResizeNs => "resize_ns",
            Self::ResizeEw => "resize_ew",
            Self::ResizeNesw => "resize_nesw",
            Self::ResizeNwse => "resize_nwse",
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "default" => Some(Self::Default),
            "arrow" => Some(Self::Arrow),
            "hand" | "pointer" => Some(Self::Hand),
            "text" | "ibeam" => Some(Self::Text),
            "crosshair" | "cross" => Some(Self::Crosshair),
            "move" | "size_all" | "sizeall" => Some(Self::Move),
            "wait" | "busy" | "hourglass" => Some(Self::Wait),
            "not_allowed" | "nodrop" | "forbidden" => Some(Self::NotAllowed),
            "resize_ns" | "sizens" | "row_resize" => Some(Self::ResizeNs),
            "resize_ew" | "sizewe" | "col_resize" => Some(Self::ResizeEw),
            "resize_nesw" | "sizenesw" => Some(Self::ResizeNesw),
            "resize_nwse" | "sizenwse" => Some(Self::ResizeNwse),
            _ => None,
        }
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Cursor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}

#[note("value-filepath")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FilePath {
    pub path: String,
}

impl FilePath {
    pub const fn empty() -> Self {
        Self {
            path: String::new(),
        }
    }

    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl From<&str> for FilePath {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for FilePath {
    fn from(s: String) -> Self {
        Self { path: s }
    }
}

#[note("value-datetime")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
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

    pub const fn epoch() -> Self {
        Self::new(1970, 1, 1, 0, 0, 0)
    }

    pub fn to_iso8601(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    pub fn format_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

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

        if !(1..=12).contains(&month) {
            return Err("month out of bounds (1..=12)");
        }
        if !(1..=31).contains(&day) {
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

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[note("value-color-standard")]
    pub const STANDARD: [(&'static str, Color); 16] = [
        ("Black", Color::rgb(0x00, 0x00, 0x00)),
        ("Maroon", Color::rgb(0x80, 0x00, 0x00)),
        ("Green", Color::rgb(0x00, 0x80, 0x00)),
        ("Olive", Color::rgb(0x80, 0x80, 0x00)),
        ("Navy", Color::rgb(0x00, 0x00, 0x80)),
        ("Purple", Color::rgb(0x80, 0x00, 0x80)),
        ("Teal", Color::rgb(0x00, 0x80, 0x80)),
        ("Gray", Color::rgb(0x80, 0x80, 0x80)),
        ("Silver", Color::rgb(0xC0, 0xC0, 0xC0)),
        ("Red", Color::rgb(0xFF, 0x00, 0x00)),
        ("Lime", Color::rgb(0x00, 0xFF, 0x00)),
        ("Yellow", Color::rgb(0xFF, 0xFF, 0x00)),
        ("Blue", Color::rgb(0x00, 0x00, 0xFF)),
        ("Fuchsia", Color::rgb(0xFF, 0x00, 0xFF)),
        ("Aqua", Color::rgb(0x00, 0xFF, 0xFF)),
        ("White", Color::rgb(0xFF, 0xFF, 0xFF)),
    ];

    #[note("value-color-transparent")]
    pub const fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let h = hex.trim_start_matches('#');
        let p = |i: usize| u8::from_str_radix(h.get(i..i + 2)?, 16).ok();
        match h.len() {
            6 => Some(Self {
                r: p(0)?,
                g: p(2)?,
                b: p(4)?,
                a: 255,
            }),
            8 => Some(Self {
                r: p(0)?,
                g: p(2)?,
                b: p(4)?,
                a: p(6)?,
            }),
            _ => None,
        }
    }

    pub fn to_hex(self) -> String {
        if self.a == 255 {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
        }
    }

    #[note("value-color-to-hsv")]
    pub fn to_hsv(self) -> (f32, f32, f32) {
        if self.r == self.g && self.g == self.b {
            return (0.0, 0.0, self.r as f32 / 255.0);
        }
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        let v = max;
        let s = if max > 0.0 { delta / max } else { 0.0 };
        let mut h = if (max - r).abs() < f32::EPSILON {
            60.0 * ((g - b) / delta)
        } else if (max - g).abs() < f32::EPSILON {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };
        if h < 0.0 {
            h += 360.0;
        } else if h >= 360.0 {
            h -= 360.0;
        }
        (h, s, v)
    }

    #[note("value-color-from-hsv")]
    pub fn from_hsv(h: f32, s: f32, v: f32, a: u8) -> Self {
        let s = s.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);
        if s <= 0.0 {
            let val = (v * 255.0).round().clamp(0.0, 255.0) as u8;
            return Self {
                r: val,
                g: val,
                b: val,
                a,
            };
        }
        let h = if h.is_nan() { 0.0 } else { h.rem_euclid(360.0) };
        let c = v * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
        let (r1, g1, b1) = if h_prime < 1.0 {
            (c, x, 0.0)
        } else if h_prime < 2.0 {
            (x, c, 0.0)
        } else if h_prime < 3.0 {
            (0.0, c, x)
        } else if h_prime < 4.0 {
            (0.0, x, c)
        } else if h_prime < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        let m = v - c;
        let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        Self { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Font {
    #[serde(alias = "family")]
    pub name: String,
    pub size: f32,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub strike_out: bool,
    /// The colour of the text. Alpha 0 is the theme's text colour,
    /// which is what a font with no colour said before the field existed.
    #[serde(default = "Color::transparent")]
    pub color: Color,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            name: "Segoe UI".into(),
            size: 9.0,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            color: Color::transparent(),
        }
    }
}

impl Font {
    pub fn new(name: impl Into<String>, size: f32) -> Self {
        Self {
            name: name.into(),
            size,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            color: Color::transparent(),
        }
    }

    pub fn family(&self) -> &str {
        &self.name
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn strike_out(mut self, strike_out: bool) -> Self {
        self.strike_out = strike_out;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn to_string_formatted(&self) -> String {
        let mut parts = Vec::new();
        let family = if self.name.trim().is_empty() {
            "Segoe UI"
        } else {
            self.name.as_str()
        };
        parts.push(family.to_string());
        parts.push(format!("{:.1}pt", self.size));

        if self.bold {
            parts.push("Bold".to_string());
        }
        if self.italic {
            parts.push("Italic".to_string());
        }
        if self.underline {
            parts.push("Underline".to_string());
        }
        if self.strike_out {
            parts.push("StrikeOut".to_string());
        }
        if self.color.a > 0 {
            parts.push(self.color.to_hex());
        }

        parts.join(", ")
    }

    pub fn from_str_formatted(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Some(Self::default());
        }

        let parts: Vec<&str> = trimmed.split(',').map(str::trim).collect();
        if parts.is_empty() {
            return Some(Self::default());
        }

        let name = parts[0].to_string();
        let mut size = 9.0f32;
        let mut bold = false;
        let mut italic = false;
        let mut underline = false;
        let mut strike_out = false;
        let mut color = Color::transparent();

        for part in &parts[1..] {
            let p_lower = part.to_ascii_lowercase();
            if part.starts_with('#') {
                color = Color::from_hex(part)?;
            } else if p_lower == "bold" {
                bold = true;
            } else if p_lower == "italic" {
                italic = true;
            } else if p_lower == "underline" {
                underline = true;
            } else if p_lower == "strikeout" || p_lower == "strike_out" {
                strike_out = true;
            } else {
                let clean_size = part.trim_end_matches("pt").trim_end_matches("px").trim();
                if let Ok(parsed_size) = clean_size.parse::<f32>() {
                    size = parsed_size;
                }
            }
        }

        Some(Self {
            name,
            size,
            bold,
            italic,
            underline,
            strike_out,
            color,
        })
    }
}

impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_formatted())
    }
}

impl std::str::FromStr for Font {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_formatted(s).ok_or(())
    }
}

#[note("value-rect")]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    #[note("value-rect-from-points")]
    pub fn from_points(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            x: x0.min(x1),
            y: y0.min(y1),
            w: (x1 - x0).abs(),
            h: (y1 - y0).abs(),
        }
    }

    #[note("value-rect-intersects")]
    pub fn intersects(&self, o: &Rect) -> bool {
        self.x < o.x + o.w && o.x < self.x + self.w && self.y < o.y + o.h && o.y < self.y + self.h
    }
}

#[note("value-edges")]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Edges {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Edges {
    pub const fn uniform(v: f32) -> Self {
        Self {
            left: v,
            top: v,
            right: v,
            bottom: v,
        }
    }
}

#[note("value-key")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    Escape,
    Tab,
    Backspace,
    Enter,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Copy,
    Cut,
    Paste,
    Colon,
    Comma,
    Backslash,
    Slash,
    Pipe,
    Questionmark,
    Exclamationmark,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Backtick,
    Minus,
    Period,
    Plus,
    Equals,
    Semicolon,
    Quote,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    BrowserBack,
    IntlBackslash,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,
    #[default]
    Unknown,
}

impl Key {
    pub const NONE: Self = Self::Unknown;

    pub const fn is_none(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "",
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::ArrowUp => "Up",
            Self::Escape => "Esc",
            Self::Tab => "Tab",
            Self::Backspace => "BkSp",
            Self::Enter => "Enter",
            Self::Space => "Space",
            Self::Insert => "Ins",
            Self::Delete => "Del",
            Self::Home => "Home",
            Self::End => "End",
            Self::PageUp => "PgUp",
            Self::PageDown => "PgDn",
            Self::Num0 => "0",
            Self::Num1 => "1",
            Self::Num2 => "2",
            Self::Num3 => "3",
            Self::Num4 => "4",
            Self::Num5 => "5",
            Self::Num6 => "6",
            Self::Num7 => "7",
            Self::Num8 => "8",
            Self::Num9 => "9",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            _ => "",
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty()
            || trimmed.eq_ignore_ascii_case("none")
            || trimmed.eq_ignore_ascii_case("unknown")
        {
            return Some(Self::Unknown);
        }

        if trimmed.len() == 1 {
            let c = trimmed.chars().next().unwrap();
            return match c.to_ascii_uppercase() {
                'A' => Some(Self::A),
                'B' => Some(Self::B),
                'C' => Some(Self::C),
                'D' => Some(Self::D),
                'E' => Some(Self::E),
                'F' => Some(Self::F),
                'G' => Some(Self::G),
                'H' => Some(Self::H),
                'I' => Some(Self::I),
                'J' => Some(Self::J),
                'K' => Some(Self::K),
                'L' => Some(Self::L),
                'M' => Some(Self::M),
                'N' => Some(Self::N),
                'O' => Some(Self::O),
                'P' => Some(Self::P),
                'Q' => Some(Self::Q),
                'R' => Some(Self::R),
                'S' => Some(Self::S),
                'T' => Some(Self::T),
                'U' => Some(Self::U),
                'V' => Some(Self::V),
                'W' => Some(Self::W),
                'X' => Some(Self::X),
                'Y' => Some(Self::Y),
                'Z' => Some(Self::Z),
                '0' => Some(Self::Num0),
                '1' => Some(Self::Num1),
                '2' => Some(Self::Num2),
                '3' => Some(Self::Num3),
                '4' => Some(Self::Num4),
                '5' => Some(Self::Num5),
                '6' => Some(Self::Num6),
                '7' => Some(Self::Num7),
                '8' => Some(Self::Num8),
                '9' => Some(Self::Num9),
                _ => None,
            };
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "f1" => Some(Self::F1),
            "f2" => Some(Self::F2),
            "f3" => Some(Self::F3),
            "f4" => Some(Self::F4),
            "f5" => Some(Self::F5),
            "f6" => Some(Self::F6),
            "f7" => Some(Self::F7),
            "f8" => Some(Self::F8),
            "f9" => Some(Self::F9),
            "f10" => Some(Self::F10),
            "f11" => Some(Self::F11),
            "f12" => Some(Self::F12),
            "enter" | "return" => Some(Self::Enter),
            "esc" | "escape" => Some(Self::Escape),
            "space" => Some(Self::Space),
            "tab" => Some(Self::Tab),
            "backspace" | "bksp" => Some(Self::Backspace),
            "del" | "delete" => Some(Self::Delete),
            "ins" | "insert" => Some(Self::Insert),
            "home" => Some(Self::Home),
            "end" => Some(Self::End),
            "pgup" | "pageup" => Some(Self::PageUp),
            "pgdn" | "pagedown" => Some(Self::PageDown),
            "up" | "arrowup" => Some(Self::ArrowUp),
            "down" | "arrowdown" => Some(Self::ArrowDown),
            "left" | "arrowleft" => Some(Self::ArrowLeft),
            "right" | "arrowright" => Some(Self::ArrowRight),
            _ => None,
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_name(s).ok_or(())
    }
}

#[note("value-shortcut")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Shortcut {
    pub key: Key,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

impl Shortcut {
    pub const NONE: Self = Self {
        key: Key::Unknown,
        ctrl: false,
        alt: false,
        shift: false,
        meta: false,
    };

    pub const fn none() -> Self {
        Self::NONE
    }

    pub const fn is_none(&self) -> bool {
        self.key.is_none()
    }

    pub const fn new(key: Key, ctrl: bool, alt: bool, shift: bool) -> Self {
        Self {
            key,
            ctrl,
            alt,
            shift,
            meta: false,
        }
    }

    pub const fn ctrl(key: Key) -> Self {
        Self {
            key,
            ctrl: true,
            alt: false,
            shift: false,
            meta: false,
        }
    }

    pub const fn alt(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: true,
            shift: false,
            meta: false,
        }
    }

    pub const fn shift(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: false,
            shift: true,
            meta: false,
        }
    }

    pub const fn ctrl_shift(key: Key) -> Self {
        Self {
            key,
            ctrl: true,
            alt: false,
            shift: true,
            meta: false,
        }
    }

    pub const fn ctrl_alt(key: Key) -> Self {
        Self {
            key,
            ctrl: true,
            alt: true,
            shift: false,
            meta: false,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
            return Some(Self::NONE);
        }

        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut meta = false;
        let mut primary_key: Option<Key> = None;

        for part in trimmed.split('+') {
            let p = part.trim();
            if p.is_empty() {
                continue;
            }
            match p.to_ascii_lowercase().as_str() {
                "ctrl" | "control" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "meta" | "cmd" | "command" | "win" => meta = true,
                other => {
                    if let Some(k) = Key::from_str_name(other) {
                        primary_key = Some(k);
                    } else {
                        return None;
                    }
                }
            }
        }

        let key = primary_key.unwrap_or(Key::Unknown);
        Some(Self {
            key,
            ctrl,
            alt,
            shift,
            meta,
        })
    }
}

impl std::fmt::Display for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_none() {
            return write!(f, "");
        }
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.meta {
            parts.push("Meta");
        }
        let key_str = self.key.as_str();
        if !key_str.is_empty() {
            parts.push(key_str);
        }
        write!(f, "{}", parts.join("+"))
    }
}

impl std::str::FromStr for Shortcut {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or(())
    }
}

impl From<Key> for Shortcut {
    fn from(key: Key) -> Self {
        Self::new(key, false, false, false)
    }
}

impl From<&str> for Shortcut {
    fn from(s: &str) -> Self {
        Self::parse(s).unwrap_or_default()
    }
}

impl Serialize for Shortcut {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Shortcut {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ShortcutVisitor;

        impl<'de> serde::de::Visitor<'de> for ShortcutVisitor {
            type Value = Shortcut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a shortcut string like 'Ctrl+S' or a map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Shortcut, E>
            where
                E: serde::de::Error,
            {
                Shortcut::parse(value)
                    .ok_or_else(|| E::custom(format!("invalid shortcut: '{value}'")))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Shortcut, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut key = Key::NONE;
                let mut ctrl = false;
                let mut alt = false;
                let mut shift = false;
                let mut meta = false;

                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "key" => key = map.next_value()?,
                        "ctrl" => ctrl = map.next_value()?,
                        "alt" => alt = map.next_value()?,
                        "shift" => shift = map.next_value()?,
                        "meta" => meta = map.next_value()?,
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Shortcut {
                    key,
                    ctrl,
                    alt,
                    shift,
                    meta,
                })
            }
        }

        deserializer.deserialize_any(ShortcutVisitor)
    }
}

#[note("value-imageformat")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageFormat {
    #[default]
    Unknown,
    Png,
    Jpeg,
    Svg,
    Ico,
    Bmp,
}

impl ImageFormat {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Svg => "svg",
            Self::Ico => "ico",
            Self::Bmp => "bmp",
        }
    }

    pub fn from_mime_or_ext(s: &str) -> Self {
        let clean = s.trim().trim_start_matches('.').to_ascii_lowercase();
        match clean.as_str() {
            "png" | "image/png" => Self::Png,
            "jpg" | "jpeg" | "image/jpeg" => Self::Jpeg,
            "svg" | "image/svg+xml" => Self::Svg,
            "ico" | "image/x-icon" | "image/vnd.microsoft.icon" => Self::Ico,
            "bmp" | "image/bmp" => Self::Bmp,
            _ => Self::Unknown,
        }
    }

    pub fn detect(bytes: &[u8]) -> Self {
        if bytes.len() >= 8 && bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A])
        {
            return Self::Png;
        }
        if bytes.len() >= 3 && bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Self::Jpeg;
        }
        if bytes.len() >= 4 && bytes.starts_with(&[0x00, 0x00, 0x01, 0x00]) {
            return Self::Ico;
        }
        if bytes.len() >= 2 && bytes.starts_with(b"BM") {
            return Self::Bmp;
        }
        let check_len = bytes.len().min(256);
        if let Ok(text) = std::str::from_utf8(&bytes[..check_len]) {
            let lower = text.to_ascii_lowercase();
            if lower.contains("<svg") || (lower.contains("<?xml") && lower.contains("svg")) {
                return Self::Svg;
            }
        }
        Self::Unknown
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[note("value-picture")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Picture {
    pub bytes: Vec<u8>,
    pub format: ImageFormat,
    pub source: Option<String>,
}

impl Picture {
    pub const fn empty() -> Self {
        Self {
            bytes: Vec::new(),
            format: ImageFormat::Unknown,
            source: None,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let format = ImageFormat::detect(&bytes);
        Self {
            bytes,
            format,
            source: None,
        }
    }

    pub fn from_bytes_with_format(bytes: Vec<u8>, format: ImageFormat) -> Self {
        Self {
            bytes,
            format,
            source: None,
        }
    }

    pub fn from_svg_str(svg: &str) -> Self {
        Self {
            bytes: svg.as_bytes().to_vec(),
            format: ImageFormat::Svg,
            source: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
        self.format = ImageFormat::Unknown;
        self.source = None;
    }

    pub fn to_base64(&self) -> String {
        encode_b64(&self.bytes)
    }

    pub fn from_base64(input: &str) -> Option<Self> {
        let clean = input.trim();
        let payload = if let Some(idx) = clean.find(";base64,") {
            &clean[idx + 8..]
        } else {
            clean
        };
        let bytes = decode_b64(payload)?;
        Some(Self::from_bytes(bytes))
    }
}

impl std::fmt::Display for Picture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "(None)")
        } else if let Some(src) = &self.source {
            write!(f, "(Picture: {})", src)
        } else {
            write!(
                f,
                "(Picture: {}, {} bytes)",
                self.format.as_str(),
                self.len()
            )
        }
    }
}

impl std::str::FromStr for Picture {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() || trimmed == "(None)" || trimmed == "(none)" {
            return Ok(Self::empty());
        }
        if let Some(pic) = Self::from_base64(trimmed) {
            return Ok(pic);
        }
        Ok(Self::empty().with_source(trimmed))
    }
}

impl Serialize for Picture {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.is_empty() && self.source.is_none() {
            serializer.serialize_str("")
        } else if let Some(src) = &self.source {
            serializer.serialize_str(src)
        } else {
            let b64 = self.to_base64();
            serializer.serialize_str(&format!(
                "data:image/{};base64,{}",
                self.format.as_str(),
                b64
            ))
        }
    }
}

impl<'de> Deserialize<'de> for Picture {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PictureVisitor;

        impl<'de> serde::de::Visitor<'de> for PictureVisitor {
            type Value = Picture;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a picture string (path or base64) or a map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Picture, E>
            where
                E: serde::de::Error,
            {
                let trimmed = value.trim();
                if trimmed.is_empty() || trimmed == "(None)" || trimmed == "(none)" {
                    return Ok(Picture::empty());
                }
                if let Some(pic) = Picture::from_base64(trimmed) {
                    return Ok(pic);
                }
                Ok(Picture::empty().with_source(trimmed))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Picture, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut bytes = Vec::new();
                let mut format = ImageFormat::Unknown;
                let mut source = None;

                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bytes" => bytes = map.next_value()?,
                        "format" => format = map.next_value()?,
                        "source" => source = map.next_value()?,
                        "data" => {
                            let data_str: String = map.next_value()?;
                            if let Some(pic) = Picture::from_base64(&data_str) {
                                bytes = pic.bytes;
                                format = pic.format;
                            }
                        }
                        _ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                if format == ImageFormat::Unknown && !bytes.is_empty() {
                    format = ImageFormat::detect(&bytes);
                }
                Ok(Picture {
                    bytes,
                    format,
                    source,
                })
            }
        }

        deserializer.deserialize_any(PictureVisitor)
    }
}

const B64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_b64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };

        out.push(B64_ALPHABET[(b0 >> 2) as usize] as char);
        out.push(B64_ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64_ALPHABET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64_ALPHABET[(b2 & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn decode_b64(input: &str) -> Option<Vec<u8>> {
    let mut clean = Vec::with_capacity(input.len());
    for b in input.bytes() {
        if !b.is_ascii_whitespace() {
            clean.push(b);
        }
    }
    if clean.is_empty() || clean.len() % 4 != 0 {
        return None;
    }

    fn val(b: u8) -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }

    let mut out = Vec::with_capacity(clean.len() / 4 * 3);
    for chunk in clean.chunks(4) {
        let c0 = val(chunk[0])?;
        let c1 = val(chunk[1])?;
        out.push((c0 << 2) | (c1 >> 4));

        if chunk[2] != b'=' {
            let c2 = val(chunk[2])?;
            out.push(((c1 & 0x0F) << 4) | (c2 >> 2));
            if chunk[3] != b'=' {
                let c3 = val(chunk[3])?;
                out.push(((c2 & 0x03) << 6) | c3);
            }
        }
    }
    Some(out)
}

#[cfg(test)]
#[path = "test/color_hsv/test.rs"]
mod tests;
