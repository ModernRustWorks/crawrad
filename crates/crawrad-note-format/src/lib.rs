//! The CrawRAD note file: YAML front matter + markdown body.
//!
//! Shared by the IDE (`crawrad-project`), the `#[note]` macro (`crawrad-note`), and the CLI (`crawrad-cli`).

#![forbid(unsafe_code)]

pub mod rustdoc;

use std::fmt;
use std::path::{Path, PathBuf};

/// The kind of note: documentation note, internal note, or comment note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteKind {
    /// Public rustdoc of the item.
    Doc,
    /// Private development context ("note"), never rustdoc.
    #[default]
    Internal,
    /// A short remark tied to a line, like a review comment, never rustdoc.
    Comment,
}

impl NoteKind {
    /// Returns the string representation of the note kind.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Doc => "doc",
            Self::Internal => "internal",
            Self::Comment => "comment",
        }
    }

    /// Returns the UI label for the note kind.
    pub fn label(self) -> &'static str {
        match self {
            Self::Doc => "Documentation",
            Self::Internal => "Internal",
            Self::Comment => "Comment",
        }
    }
}

impl fmt::Display for NoteKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A parsed CrawRAD note file containing front matter metadata and a markdown body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteFile {
    /// Unique identifier of the note (`[A-Za-z0-9_-]+`).
    pub id: String,
    /// Note kind (Doc or Internal).
    pub kind: NoteKind,
    /// `false` when the front matter had no `kind` key (notes written earlier).
    pub kind_declared: bool,
    /// A comment marked resolved: hidden from the gutter, kept in the Notes panel. Always false for other kinds.
    pub resolved: bool,
    /// Descriptive title of the note.
    pub title: String,
    /// Target file relative path or name.
    pub target_file: String,
    /// Approximate line hint where the anchor was inserted.
    pub line_hint: usize,
    /// Author of the note.
    pub author: String,
    /// Creation timestamp (ISO 8601).
    pub created: String,
    /// Associated thematic tags.
    pub tags: Vec<String>,
    /// The item's signature when the note was saved; empty in notes
    /// written before it, which are never reported stale.
    pub signature: String,
    /// Explanatory Markdown content (trimmed).
    pub body: String,
}

impl Default for NoteFile {
    fn default() -> Self {
        Self {
            id: String::new(),
            kind: NoteKind::Internal,
            kind_declared: false,
            resolved: false,
            title: String::new(),
            target_file: String::new(),
            line_hint: 1,
            author: String::new(),
            created: String::new(),
            tags: Vec::new(),
            signature: String::new(),
            body: String::new(),
        }
    }
}

impl NoteFile {
    /// Parses note file text into a `NoteFile` structure.
    pub fn parse(text: &str) -> Result<NoteFile, NoteError> {
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let trimmed = text.trim_start();
        if !trimmed.starts_with("---") {
            return Err(NoteError::MissingFrontMatter);
        }

        let after_first = &trimmed[3..];
        let first_line_end = after_first
            .find('\n')
            .ok_or(NoteError::UnterminatedFrontMatter)?;

        if !after_first[..first_line_end].trim().is_empty() {
            return Err(NoteError::MissingFrontMatter);
        }

        let rest = &after_first[first_line_end + 1..];

        let mut header_end = None;
        let mut body_start = None;
        let mut curr_offset = 0;

        for line in rest.split_inclusive('\n') {
            if line.trim() == "---" {
                header_end = Some(curr_offset);
                body_start = Some(curr_offset + line.len());
                break;
            }
            curr_offset += line.len();
        }

        let header_end = header_end.ok_or(NoteError::UnterminatedFrontMatter)?;
        let body_start = body_start.unwrap_or(curr_offset);

        let header = &rest[..header_end];
        let body = rest[body_start..].trim().to_string();

        let mut id = None;
        let mut kind = None;
        let mut status: Option<bool> = None;
        let mut title = String::new();
        let mut target_file = String::new();
        let mut line_hint = 1usize;
        let mut author = String::new();
        let mut created = String::new();
        let mut tags = Vec::new();
        let mut signature = String::new();

        for line in header.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                let key = k.trim();
                let val = v.trim();
                match key {
                    "id" => {
                        let parsed_id = parse_value(val);
                        if !parsed_id.is_empty() {
                            id = Some(parsed_id);
                        }
                    }
                    "kind" => {
                        let val_clean = parse_value(val);
                        match val_clean.as_str() {
                            "doc" => kind = Some(NoteKind::Doc),
                            "internal" => kind = Some(NoteKind::Internal),
                            "comment" => kind = Some(NoteKind::Comment),
                            other => return Err(NoteError::UnknownKind(other.to_string())),
                        }
                    }
                    "status" => {
                        let val_clean = parse_value(val);
                        match val_clean.as_str() {
                            "open" => status = Some(false),
                            "resolved" => status = Some(true),
                            other => return Err(NoteError::UnknownStatus(other.to_string())),
                        }
                    }
                    "title" => title = parse_value(val),
                    "target_file" => target_file = parse_value(val),
                    "line_hint" => {
                        let parsed = parse_value(val);
                        line_hint = parsed.parse::<usize>().unwrap_or(1);
                    }
                    "author" => author = parse_value(val),
                    "created" => created = parse_value(val),
                    "tags" => tags = parse_tags(val),
                    "signature" => signature = parse_value(val),
                    _ => {}
                }
            }
        }

        let id = id.ok_or(NoteError::MissingId)?;
        let (kind, kind_declared) = match kind {
            Some(k) => (k, true),
            None => (NoteKind::Internal, false),
        };
        let resolved = match kind {
            NoteKind::Comment => status.unwrap_or(false),
            _ => false,
        };

        Ok(NoteFile {
            id,
            kind,
            kind_declared,
            resolved,
            title,
            target_file,
            line_hint,
            author,
            created,
            tags,
            signature,
            body,
        })
    }

    /// Serializes the note file to Markdown text with YAML front matter.
    pub fn to_markdown(&self) -> String {
        let tags_str = if self.tags.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}]", self.tags.join(", "))
        };
        // The header is one value per line, so a signature spanning lines (a
        // `const` string continued with `\`) keeps its line breaks escaped.
        let quoted = |s: &str| {
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\r', "\\r")
                .replace('\n', "\\n");
            format!("\"{escaped}\"")
        };
        let status_line = match self.kind {
            NoteKind::Comment => {
                if self.resolved {
                    "status: resolved\n"
                } else {
                    "status: open\n"
                }
            }
            _ => "",
        };
        let signature = match self.signature.is_empty() {
            true => String::new(),
            false => format!("signature: {}\n", quoted(&self.signature)),
        };
        format!(
            "---\nid: {}\nkind: {}\n{status_line}target_file: {}\nline_hint: {}\nauthor: {}\ncreated: {}\ntitle: {}\ntags: {}\n{signature}---\n\n{}\n",
            self.id,
            self.kind.as_str(),
            self.target_file,
            self.line_hint,
            self.author,
            self.created,
            quoted(&self.title),
            tags_str,
            self.body.trim()
        )
    }
}

/// A signature as compared: without whitespace or a trailing comma, so
/// reformatting the item (rustfmt breaking the parameters into lines) does
/// not count as a change, and the IDE's source text and the macro's tokens
/// agree.
pub fn signature_key(signature: &str) -> String {
    let bare: String = signature.chars().filter(|c| !c.is_whitespace()).collect();
    bare.replace(",)", ")").replace(",>", ">")
}

/// The item's signature as written: its tokens up to the body or the `;`,
/// every leading outer attribute left out (the `#[note]` itself, doc
/// comments, derives). The macro and the `crawrad note` CLI both call this,
/// so they can never disagree on what "the signature changed" means.
pub fn item_signature(item: &proc_macro2::TokenStream) -> String {
    use proc_macro2::{Delimiter, TokenStream, TokenTree};
    let mut signature = TokenStream::new();
    let mut tokens = item.clone().into_iter();
    while let Some(token) = tokens.next() {
        match &token {
            TokenTree::Punct(p) if p.as_char() == '#' && signature.is_empty() => {
                tokens.next(); // the attribute's `[...]`
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => break,
            TokenTree::Punct(p) if p.as_char() == ';' => break,
            _ => signature.extend([token]),
        }
    }
    signature.to_string()
}

/// The warning for a note whose item changed since it was saved, or `None`
/// while it still matches (or the note recorded no signature).
pub fn stale_signature(id: &str, saved: &str, now: &str) -> Option<String> {
    if saved.is_empty() || signature_key(saved) == signature_key(now) {
        return None;
    }
    Some(format!(
        "note \"{id}\" was written for `{saved}` and the item's signature \
         changed since: review the note and save it again"
    ))
}

/// Errors encountered while parsing note files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteError {
    /// Opening `---` delimiter was missing.
    MissingFrontMatter,
    /// Closing `---` delimiter was missing.
    UnterminatedFrontMatter,
    /// The `id` key was missing or empty in the front matter.
    MissingId,
    /// An unknown note kind was specified.
    UnknownKind(String),
    /// An unknown note status was specified.
    UnknownStatus(String),
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFrontMatter => write!(f, "front matter missing: expected opening \"---\""),
            Self::UnterminatedFrontMatter => {
                write!(f, "front matter unterminated: missing closing \"---\"")
            }
            Self::MissingId => write!(f, "front matter missing \"id\""),
            Self::UnknownKind(kind) => {
                write!(
                    f,
                    "unknown note kind \"{kind}\": expected \"doc\" or \"internal\""
                )
            }
            Self::UnknownStatus(status) => {
                write!(
                    f,
                    "unknown note status \"{status}\": expected \"open\" or \"resolved\""
                )
            }
        }
    }
}

impl std::error::Error for NoteError {}

/// Validates that a note identifier contains only ASCII alphanumeric characters,
/// underscores, or hyphens (`[A-Za-z0-9_-]+`).
pub fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("note id cannot be empty".to_string());
    }
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(format!(
            "invalid note id \"{id}\": id must match [A-Za-z0-9_-]+"
        ));
    }
    Ok(())
}

/// Resolves the file path where a note should be located.
pub fn note_path(notes_dir_parent: &Path, id: &str) -> PathBuf {
    notes_dir_parent
        .join(".crawrad")
        .join("notes")
        .join(format!("{id}.md"))
}

fn parse_value(raw: &str) -> String {
    let s = raw.trim();
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        let inner = &s[1..s.len() - 1];
        let mut out = String::with_capacity(inner.len());
        let mut chars = inner.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.peek() {
                    Some('"') => {
                        out.push('"');
                        chars.next();
                    }
                    Some('\\') => {
                        out.push('\\');
                        chars.next();
                    }
                    Some('n') => {
                        out.push('\n');
                        chars.next();
                    }
                    Some('r') => {
                        out.push('\r');
                        chars.next();
                    }
                    _ => out.push('\\'),
                }
            } else {
                out.push(c);
            }
        }
        out
    } else if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn parse_tags(raw: &str) -> Vec<String> {
    let s = raw.trim();
    let inner = s
        .strip_prefix('[')
        .and_then(|v| v.strip_suffix(']'))
        .unwrap_or(s);
    inner
        .split(',')
        .map(|item| parse_value(item.trim()))
        .filter(|item| !item.is_empty())
        .collect()
}

#[cfg(test)]
#[path = "test/lib/test.rs"]
mod test;
