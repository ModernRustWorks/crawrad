// This file is EMITTED VERBATIM into the user's project. Everything in it --
// comments included -- ends up in front of whoever opens the generated code, so
// it is English from the first line (ADR 0030). The house rationale, in
// Portuguese, lives in `pieces/mod.rs`, which is never emitted.
//
// Authoring rules:
// 1. No `//!` and no `#![...]`.
// 2. No `use`, and no type of ours. Only `std`.
//
// FONT -- typography description: typeface name, size and styling flags.

use std::fmt;
use std::str::FromStr;

/// Typographical font attributes: typeface family name, point size, and styling flags.
#[derive(Clone, PartialEq, Debug)]
pub struct Font {
    /// The typeface family name (e.g., "Segoe UI", "Tahoma", "Arial", "Roboto").
    pub name: String,
    /// The font size in typography points or design units (default: 9.0).
    pub size: f32,
    /// Whether bold weight is active.
    pub bold: bool,
    /// Whether italic slant is active.
    pub italic: bool,
    /// Whether text underline is active.
    pub underline: bool,
    /// Whether strikethrough is active.
    pub strike_out: bool,
    /// The colour of the text, as red, green, blue and alpha bytes. Alpha 0
    /// means the theme's text colour.
    pub color: [u8; 4],
}

impl Default for Font {
    fn default() -> Self {
        Self {
            name: "Segoe UI".to_string(),
            size: 9.0,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            color: [0, 0, 0, 0],
        }
    }
}

impl Font {
    /// Creates a new font with the specified family name and size.
    pub fn new(name: impl Into<String>, size: f32) -> Self {
        Self {
            name: name.into(),
            size,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            color: [0, 0, 0, 0],
        }
    }

    /// Creates a standard default font.
    pub fn standard() -> Self {
        Self::default()
    }

    /// Sets the bold style flag.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Sets the italic style flag.
    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Sets the underline style flag.
    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    /// Sets the strike-out style flag.
    pub fn strike_out(mut self, strike_out: bool) -> Self {
        self.strike_out = strike_out;
        self
    }

    /// Sets the text colour (red, green, blue, alpha).
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }

    /// Sets the font size.
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Formats font attributes as a readable descriptor (e.g. "Segoe UI, 9pt, Bold").
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
        let [r, g, b, a] = self.color;
        if a == 255 {
            parts.push(format!("#{r:02X}{g:02X}{b:02X}"));
        } else if a > 0 {
            parts.push(format!("#{r:02X}{g:02X}{b:02X}{a:02X}"));
        }

        parts.join(", ")
    }

    /// Parses a font descriptor from a formatted string (e.g. "Segoe UI, 9pt, Bold").
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
        let mut color = [0u8; 4];

        for part in &parts[1..] {
            let p_lower = part.to_ascii_lowercase();
            if let Some(hex) = part.strip_prefix('#') {
                let byte = |i: usize| u8::from_str_radix(hex.get(i..i + 2)?, 16).ok();
                color = match hex.len() {
                    6 => [byte(0)?, byte(2)?, byte(4)?, 255],
                    8 => [byte(0)?, byte(2)?, byte(4)?, byte(6)?],
                    _ => return None,
                };
            } else if p_lower == "bold" {
                bold = true;
            } else if p_lower == "italic" {
                italic = true;
            } else if p_lower == "underline" {
                underline = true;
            } else if p_lower == "strikeout" || p_lower == "strike_out" {
                strike_out = true;
            } else {
                // Try parsing size (e.g., "9pt", "10.5", "12px")
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

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_formatted())
    }
}

impl FromStr for Font {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_formatted(s).ok_or(())
    }
}
