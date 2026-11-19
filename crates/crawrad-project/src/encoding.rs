//! **HOW A FILE IS WRITTEN** — the byte order mark and the line ending.
//!
//! # Why it left the egui editor
//!
//! It lived inside `editor.rs`, which is built on `eframe::egui`, so the
//! Makepad shell could not read one line of it. On 17/09/2026 the owner
//! pointed at the egui editor's footer — `1: 1   Insert   UTF-8 LF` — next to
//! the Makepad one, which had an empty band where that line belongs, and asked
//! for it.
//!
//! The rule is three lines of arithmetic and it is the kind that quietly
//! diverges: which end-of-line wins when a file mixes them, whether a BOM was
//! there, and how the two are spelled on screen. Copying it would be the
//! seventh divergence.
//!
//! # What it is NOT
//!
//! **It does not read files and it does not write them.** It describes what
//! was found ([`Encoding::of`]) and builds the bytes that put it back
//! ([`Encoding::to_bytes`]) — and stops there. Who opens and who saves is
//! whoever owns the buffer; a second place deciding what goes to disk is
//! exactly what the egui `Buffer::salvar` doc warns against.

use crawrad_note::note;
#[note("encoding-bom")]
pub const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

#[note("encoding-lineending")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEnding {
    /// `\n` — what the generator writes.
    #[default]
    Lf,
    /// `\r\n`.
    CrLf,
}

impl LineEnding {
    #[note("encoding-lineending-detect")]
    pub fn detect(raw: &str) -> Self {
        match raw.find('\n') {
            Some(0) | None => Self::Lf,
            Some(p) if raw.as_bytes()[p - 1] == b'\r' => Self::CrLf,
            Some(_) => Self::Lf,
        }
    }

    #[note("encoding-lineending-apply")]
    pub fn apply(self, text: &str) -> std::borrow::Cow<'_, str> {
        match self {
            Self::Lf => std::borrow::Cow::Borrowed(text),
            Self::CrLf => std::borrow::Cow::Owned(text.replace('\n', "\r\n")),
        }
    }

    #[note("encoding-lineending-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Lf => "LF",
            Self::CrLf => "CRLF",
        }
    }
}

#[note("encoding-encoding")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Encoding {
    /// Did the file carry a UTF-8 byte order mark?
    pub bom: bool,
    /// What it used to end a line.
    pub line_ending: LineEnding,
}

impl Encoding {
    #[note("encoding-encoding-of")]
    pub fn of(bytes: &[u8]) -> Self {
        let (body, bom) = match bytes.strip_prefix(BOM) {
            Some(rest) => (rest, true),
            None => (bytes, false),
        };
        Self {
            bom,
            line_ending: LineEnding::detect(&String::from_utf8_lossy(body)),
        }
    }

    #[note("encoding-encoding-to-bytes")]
    pub fn to_bytes(&self, text: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        if self.bom {
            bytes.extend_from_slice(BOM);
        }
        bytes.extend_from_slice(self.line_ending.apply(text).as_bytes());
        bytes
    }

    #[note("encoding-encoding-label")]
    pub fn label(&self) -> &'static str {
        match (self.bom, self.line_ending) {
            (true, LineEnding::CrLf) => "UTF-8 BOM  CRLF",
            (true, LineEnding::Lf) => "UTF-8 BOM  LF",
            (false, LineEnding::CrLf) => "UTF-8  CRLF",
            (false, LineEnding::Lf) => "UTF-8  LF",
        }
    }
}

#[note("encoding-file-encoding")]
pub fn file_encoding(path: Option<&std::path::Path>) -> Encoding {
    match path.and_then(|p| std::fs::read(p).ok()) {
        Some(bytes) => Encoding::of(&bytes),
        None => Encoding::default(),
    }
}

#[cfg(test)]
#[path = "test/encoding/test.rs"]
mod test;
