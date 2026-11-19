//! **The user's code templates**: `templates.toml` in the per-user
//! folder, beside `preferences.txt`. What a template is and the shipped set
//! live in `crawrad_insight::templates`; this reads and writes the file.
//!
//! ```toml
//! [[template]]
//! trigger = "evlog"
//! description = "Event with log line"
//! body = """
//! pub fn ${1:btn_ok}_click(&mut self) {
//!     $0
//! }"""
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One entry: (trigger, description, body).
pub type TemplateEntry = (String, String, String);

#[derive(Debug, Default, Deserialize, Serialize)]
struct File {
    #[serde(default)]
    template: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Entry {
    #[serde(default)]
    trigger: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    body: String,
}

/// `templates.toml` in the per-user folder.
pub fn templates_file() -> Option<PathBuf> {
    Some(crate::workspace::config_folder()?.join("templates.toml"))
}

/// The entries of `text`; a bad file reads as none, an entry without a
/// trigger or a body is skipped.
pub fn parse(text: &str) -> Vec<TemplateEntry> {
    toml::from_str::<File>(text)
        .map(|f| f.template)
        .unwrap_or_default()
        .into_iter()
        .filter(|e| !e.trigger.trim().is_empty() && !e.body.is_empty())
        .map(|e| (e.trigger.trim().to_string(), e.description, e.body))
        .collect()
}

/// The file's text for `entries` (round-trips with [`parse`]).
pub fn to_text(entries: &[TemplateEntry]) -> String {
    let file = File {
        template: entries
            .iter()
            .map(|(t, d, b)| Entry {
                trigger: t.clone(),
                description: d.clone(),
                body: b.clone(),
            })
            .collect(),
    };
    toml::to_string(&file).unwrap_or_default()
}

/// The user's templates; no file reads as none.
pub fn read_user() -> Vec<TemplateEntry> {
    templates_file()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| parse(&t))
        .unwrap_or_default()
}

/// Reads templates from an arbitrary path (e.g. for import).
pub fn read_file(path: impl AsRef<std::path::Path>) -> Result<Vec<TemplateEntry>, String> {
    let content = std::fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    Ok(parse(&content))
}

/// Writes templates to an arbitrary path (e.g. for export).
pub fn write_file(
    path: impl AsRef<std::path::Path>,
    entries: &[TemplateEntry],
) -> Result<(), String> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, to_text(entries)).map_err(|e| e.to_string())
}

/// Writes the user's templates to `templates_file()`.
pub fn write_user(entries: &[TemplateEntry]) -> Result<(), String> {
    let path = templates_file().ok_or_else(|| "Could not determine config folder".to_string())?;
    write_file(&path, entries)
}

#[cfg(test)]
#[path = "test/code_templates/test.rs"]
mod tests;
