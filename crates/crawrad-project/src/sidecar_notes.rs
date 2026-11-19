//! Anchor and Decoupled Code Notes System in the Editor (Sidecar Code Notes).
//!
//! As per allows attaching architectural context, history, and
//! explanations without cluttering executable source code with huge comment
//! blocks. The code retains only a compact anchor `// [crawrad:note#id]`,
//! while the full content is persisted in `.crawrad/notes/{id}.md`.

use std::fs;
use std::path::{Path, PathBuf};

use crawrad_note::note;
use crawrad_note_format::NoteFile;
pub use crawrad_note_format::NoteKind;

#[note("sidecar-notes-sidecarnote")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarNote {
    /// The unique identifier of the note (e.g. `main_20260923_01`).
    pub id: String,
    /// Note kind: documentation or internal note.
    pub kind: NoteKind,
    /// A comment marked resolved: hidden from the gutter, kept in the Notes panel. Always false for other kinds.
    pub resolved: bool,
    /// Descriptive title of the note.
    pub title: String,
    /// Target file relative path or name.
    pub target_file: String,
    /// Approximate line hint where the anchor was inserted.
    pub line_hint: usize,
    /// Author of the note (e.g. `user`, `claude`, `gemini`, `agent`).
    pub author: String,
    /// Creation timestamp (ISO 8601).
    pub created: String,
    /// Associated thematic tags (e.g. `["architecture", "maintenance"]`).
    pub tags: Vec<String>,
    /// A documentation note's item signature when it was saved.
    pub signature: String,
    /// Explanatory Markdown content.
    pub content: String,
}

impl Default for SidecarNote {
    fn default() -> Self {
        Self {
            id: String::new(),
            kind: NoteKind::Internal,
            resolved: false,
            title: String::new(),
            target_file: String::new(),
            line_hint: 1,
            author: "user".to_string(),
            created: Self::current_timestamp(),
            tags: Vec::new(),
            signature: String::new(),
            content: String::new(),
        }
    }
}

/// `unix_secs` as an RFC 3339 local time with its UTC offset
/// (`2026-11-16T12:03:14-03:00`).
pub(crate) fn rfc3339_local(unix_secs: i64, offset_secs: i64) -> String {
    let local = unix_secs + offset_secs;
    let (y, m, d) = crawrad_os::clock::civil_from_days(local.div_euclid(86_400));
    let day = local.rem_euclid(86_400);
    let sign = if offset_secs < 0 { '-' } else { '+' };
    let off = offset_secs.abs() / 60;
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}{sign}{:02}:{:02}",
        day / 3600,
        day % 3600 / 60,
        day % 60,
        off / 60,
        off % 60
    )
}

impl SidecarNote {
    #[note("sidecar-notes-sidecarnote-new")]
    pub fn new(id: String, target_file: &str, line_hint: usize) -> Self {
        Self {
            id,
            kind: NoteKind::Internal,
            resolved: false,
            title: "New Note".to_string(),
            target_file: target_file.to_string(),
            line_hint,
            author: "user".to_string(),
            created: Self::current_timestamp(),
            tags: Vec::new(),
            signature: String::new(),
            content: String::new(),
        }
    }

    #[note("sidecar-notes-sidecarnote-current-timestamp")]
    pub fn current_timestamp() -> String {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or_default();
        rfc3339_local(secs, crawrad_os::clock::local_offset_seconds())
    }

    #[note("sidecar-notes-sidecarnote-to-markdown")]
    pub fn to_markdown(&self) -> String {
        let file = NoteFile {
            id: self.id.clone(),
            kind: self.kind,
            kind_declared: true,
            resolved: self.resolved,
            title: self.title.clone(),
            target_file: self.target_file.clone(),
            line_hint: self.line_hint,
            author: self.author.clone(),
            created: self.created.clone(),
            tags: self.tags.clone(),
            signature: self.signature.clone(),
            body: self.content.clone(),
        };
        file.to_markdown()
    }

    #[note("sidecar-notes-sidecarnote-from-markdown")]
    pub fn from_markdown(text: &str) -> Result<Self, String> {
        let file = NoteFile::parse(text).map_err(|e| e.to_string())?;
        Ok(Self {
            id: file.id,
            kind: file.kind,
            resolved: file.resolved,
            title: file.title,
            target_file: file.target_file,
            line_hint: file.line_hint,
            author: file.author,
            created: file.created,
            tags: file.tags,
            signature: file.signature,
            content: file.body,
        })
    }
}

#[note("sidecar-notes-anchormatch")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorMatch {
    /// Note identifier pointed to by the anchor.
    pub note_id: String,
    /// 1-based line number where the anchor was found.
    pub line_number: usize,
    /// Whether the anchor marks the beginning of a region (`[crawrad:note:begin#id]`).
    pub is_region_begin: bool,
    /// Whether the anchor marks the end of a region (`[crawrad:note:end#id]`).
    pub is_region_end: bool,
    /// Whether the anchor is a comment note (`[crawrad:comment#id]`).
    pub is_comment: bool,
}

#[note("sidecar-notes-extract-anchors")]
pub fn extract_anchors(code: &str) -> Vec<AnchorMatch> {
    let mut matches = Vec::new();

    for (idx, line) in code.lines().enumerate() {
        let line_num = idx + 1;
        // Accepted patterns:
        // // [crawrad:note#id]
        // /* [crawrad:note#id] */
        // // [crawrad:note:begin#id]
        // // [crawrad:note:end#id]
        // // [crawrad:comment#id]
        let scan = if let Some(pos) = line.find("[crawrad:note") {
            Some((pos, false))
        } else {
            line.find("[crawrad:comment#").map(|pos| (pos, true))
        };
        if let Some((pos, is_comment)) = scan
            && let slice = &line[pos..]
            && let Some(end_bracket) = slice.find(']')
        {
            let tag_content = &slice[..end_bracket];
            let is_begin = tag_content.starts_with("[crawrad:note:begin#");
            let is_end = tag_content.starts_with("[crawrad:note:end#");

            if let Some(hash_pos) = tag_content.find('#') {
                let note_id = tag_content[hash_pos + 1..].trim().to_string();
                if !note_id.is_empty() {
                    matches.push(AnchorMatch {
                        note_id,
                        line_number: line_num,
                        is_region_begin: is_begin,
                        is_region_end: is_end,
                        is_comment,
                    });
                }
            }
        }
    }

    matches
}

#[note("sidecar-notes-generate-note-id")]
pub fn generate_note_id(file_path: &str) -> String {
    let path = Path::new(file_path);
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("note")
        .to_ascii_lowercase()
        .replace(['.', '-', ' '], "_");

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    format!("{stem}_{nanos}")
}

#[note("sidecar-notes-notes-directory")]
pub fn notes_directory(project_root: &Path) -> PathBuf {
    project_root.join(".crawrad").join("notes")
}

#[note("sidecar-notes-save-note")]
pub fn save_note(project_root: &Path, note: &SidecarNote) -> Result<PathBuf, String> {
    if note.id.trim().is_empty() {
        return Err("Note ID cannot be empty".to_string());
    }
    let dir = notes_directory(project_root);
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create .crawrad/notes directory: {e}"))?;
    }
    let filename = format!("{}.md", note.id);
    let file_path = dir.join(&filename);
    let content = note.to_markdown();
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write note '{}': {e}", file_path.display()))?;
    Ok(file_path)
}

#[note("sidecar-notes-load-note")]
pub fn load_note(project_root: &Path, id: &str) -> Result<SidecarNote, String> {
    let file_path = notes_directory(project_root).join(format!("{id}.md"));
    if !file_path.exists() {
        return Err(format!("Note file not found: {}", file_path.display()));
    }
    let text = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read note file '{}': {e}", file_path.display()))?;
    SidecarNote::from_markdown(&text)
}

#[note("sidecar-notes-list-notes")]
pub fn list_notes(project_root: &Path) -> Vec<SidecarNote> {
    let dir = notes_directory(project_root);
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut notes = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && let Ok(text) = fs::read_to_string(&path)
                && let Ok(note) = SidecarNote::from_markdown(&text)
            {
                notes.push(note);
            }
        }
    }
    notes.sort_by(|a, b| a.id.cmp(&b.id));
    notes
}

#[cfg(test)]
#[path = "test/sidecar_notes/test.rs"]
mod tests;
