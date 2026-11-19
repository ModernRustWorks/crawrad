//! Code sample preview for the Color Scheme page.
//!
//! Provides the six-line syntax and diagnostic preview lines, token roles,
//! and color resolution methods based on user preferences.

use crawrad_note::note;
#[note("scheme-preview-role")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Language keywords (e.g. `fn`, `let`).
    Keyword,
    /// Type names (e.g. `u32`, `Order`).
    Type,
    /// Function names (e.g. `load`).
    Function,
    /// Identifiers (e.g. `id`, `name`).
    Identifier,
    /// Numeric literals (e.g. `42`, `7`).
    Number,
    /// String literals (e.g. `"Order"`).
    Str,
    /// Line and block comments.
    Comment,
    /// Delimiters and operators (e.g. `(`, `)`, `;`).
    Delimiter,
    /// Plain whitespace and unstyled punctuation.
    Plain,
    /// Diagnostic error message text.
    Error,
    /// Diagnostic warning message text.
    Warning,
}

#[note("scheme-preview-linekind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    /// Normal source line without diagnostic highlight.
    Normal,
    /// Line carrying a diagnostic error.
    Error,
    /// Line carrying a diagnostic warning.
    Warning,
}

#[note("scheme-preview-previewline")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewLine {
    /// 1-based line number.
    pub number: u32,
    /// Diagnostic classification of the line.
    pub kind: LineKind,
    /// Token spans that make up the line.
    pub spans: Vec<(&'static str, Role)>,
}

#[note("scheme-preview-lines")]
pub fn lines() -> Vec<PreviewLine> {
    vec![
        PreviewLine {
            number: 1,
            kind: LineKind::Normal,
            spans: vec![("// load the order and its lines", Role::Comment)],
        },
        PreviewLine {
            number: 2,
            kind: LineKind::Normal,
            spans: vec![
                ("fn", Role::Keyword),
                (" ", Role::Plain),
                ("load", Role::Function),
                ("(", Role::Delimiter),
                ("id", Role::Identifier),
                (":", Role::Delimiter),
                (" ", Role::Plain),
                ("u32", Role::Type),
                (")", Role::Delimiter),
                (" ", Role::Plain),
                ("->", Role::Delimiter),
                (" ", Role::Plain),
                ("Order", Role::Type),
                (" ", Role::Plain),
                ("{", Role::Delimiter),
            ],
        },
        PreviewLine {
            number: 3,
            kind: LineKind::Normal,
            spans: vec![
                ("    ", Role::Plain),
                ("let", Role::Keyword),
                (" ", Role::Plain),
                ("name", Role::Identifier),
                (" ", Role::Plain),
                ("=", Role::Delimiter),
                (" ", Role::Plain),
                ("\"Order\"", Role::Str),
                (";", Role::Delimiter),
            ],
        },
        PreviewLine {
            number: 4,
            kind: LineKind::Error,
            spans: vec![
                ("    ", Role::Plain),
                ("let", Role::Keyword),
                (" ", Role::Plain),
                ("total", Role::Identifier),
                (" ", Role::Plain),
                ("=", Role::Delimiter),
                (" ", Role::Plain),
                ("42", Role::Number),
                (" ", Role::Plain),
                ("+", Role::Delimiter),
                (" ", Role::Plain),
                ("\"x\"", Role::Str),
                (";", Role::Delimiter),
                ("  ", Role::Plain),
                ("cannot add &str to i32", Role::Error),
            ],
        },
        PreviewLine {
            number: 5,
            kind: LineKind::Warning,
            spans: vec![
                ("    ", Role::Plain),
                ("let", Role::Keyword),
                (" ", Role::Plain),
                ("unused", Role::Identifier),
                (" ", Role::Plain),
                ("=", Role::Delimiter),
                (" ", Role::Plain),
                ("7", Role::Number),
                (";", Role::Delimiter),
                ("  ", Role::Plain),
                ("unused variable", Role::Warning),
            ],
        },
        PreviewLine {
            number: 6,
            kind: LineKind::Normal,
            spans: vec![("}", Role::Delimiter)],
        },
    ]
}

#[note("scheme-preview-color")]
pub fn color(prefs: &crate::workspace::Preferences, role: Role) -> [f32; 4] {
    let colors = prefs.editor_colors();
    let diag = prefs.editor_diagnostic_colors.rgba();
    match role {
        Role::Keyword => colors.other_keyword,
        Role::Type => colors.typename,
        Role::Function => colors.function,
        Role::Identifier => colors.identifier,
        Role::Number => colors.number,
        Role::Str => colors.string,
        Role::Comment => colors.comment,
        Role::Delimiter => colors.delimiter,
        Role::Plain => colors.identifier,
        Role::Error => diag[0],
        Role::Warning => diag[1],
    }
}

#[note("scheme-preview-gutter")]
pub const GUTTER: [f32; 4] = [92.0 / 255.0, 99.0 / 255.0, 112.0 / 255.0, 1.0];

#[note("scheme-preview-band")]
pub fn band(prefs: &crate::workspace::Preferences, kind: LineKind) -> Option<[f32; 4]> {
    let diag = prefs.editor_diagnostic_colors.rgba();
    match kind {
        LineKind::Normal => None,
        LineKind::Error => Some(diag[3]),
        LineKind::Warning => Some(diag[4]),
    }
}

#[note("scheme-preview-background")]
pub fn background(prefs: &crate::workspace::Preferences) -> [f32; 4] {
    prefs.editor_colors().bg
}

#[note("scheme-preview-paintedline")]
#[derive(Debug, Clone, PartialEq)]
pub struct PaintedLine {
    /// The line number, as drawn in the gutter.
    pub number: String,
    /// The gutter colour: the diagnostic one on an error or warning line.
    pub number_color: [f32; 4],
    /// The band behind the line, end to end; `None` on a normal line.
    pub band: Option<[f32; 4]>,
    /// The text runs, each in its colour.
    pub spans: Vec<(String, [f32; 4])>,
}

#[note("scheme-preview-painted")]
pub fn painted(prefs: &crate::workspace::Preferences) -> Vec<PaintedLine> {
    lines()
        .into_iter()
        .map(|line| {
            let num_text = line.number.to_string();
            let num_color = match line.kind {
                LineKind::Normal => GUTTER,
                LineKind::Error => color(prefs, Role::Error),
                LineKind::Warning => color(prefs, Role::Warning),
            };
            let line_band = band(prefs, line.kind);
            let spans = line
                .spans
                .into_iter()
                .map(|(text, role)| (text.to_string(), color(prefs, role)))
                .collect();
            PaintedLine {
                number: num_text,
                number_color: num_color,
                band: line_band,
                spans,
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "test/scheme_preview/test.rs"]
mod tests;
