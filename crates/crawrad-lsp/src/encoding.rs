//! **The unit of a column** (LSP 3.17, `PositionEncodingKind`).
//!
//! An LSP position is a line and a column; what the column counts is
//! negotiated: the client offers `general.positionEncodings` and the server
//! answers `capabilities.positionEncoding` — absent means UTF-16, the
//! protocol's default. The editor works in UTF-8 bytes, so with `utf-8` a
//! column IS the byte in the line and nothing is converted; with `utf-16`
//! (or `utf-32`) every position crosses here, and only here.

use serde_json::Value;

use crate::protocol::Position;
use crawrad_note::note;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PositionEncoding {
    /// Bytes — the editor's own unit.
    Utf8,
    /// UTF-16 code units: the protocol's default.
    #[default]
    Utf16,
    /// Unicode scalar values (chars).
    Utf32,
}

impl PositionEncoding {
    #[note("encoding-positionencoding-offered")]
    pub const OFFERED: [&'static str; 2] = ["utf-8", "utf-16"];

    #[note("encoding-positionencoding-from-capabilities")]
    pub fn from_capabilities(capabilities: Option<&Value>) -> Self {
        match capabilities
            .and_then(|c| c.get("positionEncoding"))
            .and_then(Value::as_str)
        {
            Some("utf-8") => Self::Utf8,
            Some("utf-32") => Self::Utf32,
            _ => Self::Utf16,
        }
    }

    #[note("encoding-positionencoding-column")]
    pub fn column(self, line: &str, byte: usize) -> u32 {
        let mut byte = byte.min(line.len());
        while !line.is_char_boundary(byte) {
            byte -= 1;
        }
        let upto = &line[..byte];
        let n = match self {
            Self::Utf8 => upto.len(),
            Self::Utf16 => upto.encode_utf16().count(),
            Self::Utf32 => upto.chars().count(),
        };
        u32::try_from(n).unwrap_or(u32::MAX)
    }

    #[note("encoding-positionencoding-byte")]
    pub fn byte(self, line: &str, column: u32) -> usize {
        let column = column as usize;
        if self == Self::Utf8 {
            let mut b = column.min(line.len());
            while !line.is_char_boundary(b) {
                b -= 1;
            }
            return b;
        }
        let mut units = 0usize;
        for (byte, c) in line.char_indices() {
            if units >= column {
                return byte;
            }
            units += if self == Self::Utf16 {
                c.len_utf16()
            } else {
                1
            };
        }
        line.len()
    }

    #[note("encoding-positionencoding-place")]
    pub fn place(self, lines: &[String], at: Position) -> (usize, usize) {
        let line = at.line as usize;
        let byte = lines.get(line).map_or(0, |l| self.byte(l, at.character));
        (line, byte)
    }
}

#[cfg(test)]
#[path = "test/encoding/test.rs"]
mod tests;
