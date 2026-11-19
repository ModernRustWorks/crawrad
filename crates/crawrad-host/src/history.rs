//! Local file history engine for CrawRAD Studio.
//!
//! Stores snapshots of saved or externally reloaded files under `.crawrad/history/`
//! in the workspace root, keeping a configurable number of revisions per file.

use std::path::{Component, Path, PathBuf};

use crawrad_note::note;
use sha2::{Digest, Sha256};

#[note("history-origin")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// Revision created when saving from the IDE.
    Save,
    /// Revision created when external changes are reloaded by the IDE.
    External,
}

impl Origin {
    #[note("history-origin-as-str")]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Save => "save",
            Self::External => "external",
        }
    }

    #[note("history-origin-parse")]
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "save" => Some(Self::Save),
            "external" => Some(Self::External),
            _ => None,
        }
    }
}

#[note("history-entry")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Unix timestamp in seconds when the entry was recorded.
    pub time: u64,
    /// SHA-256 hex digest of the file content.
    pub hash: String,
    /// Whether the revision came from a save or an external reload.
    pub origin: Origin,
}

#[note("history-historysettings")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HistorySettings {
    pub enabled: bool,
    pub limit: usize,
    pub merge_seconds: u64,
    pub max_file_kb: u64,
}

impl Default for HistorySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            limit: LocalHistory::DEFAULT_LIMIT,
            merge_seconds: 10,
            max_file_kb: 5120,
        }
    }
}

#[note("history-localhistory")]
#[derive(Debug, Clone)]
pub struct LocalHistory {
    root: PathBuf,
    settings: HistorySettings,
}

impl LocalHistory {
    #[note("history-localhistory-default-limit")]
    pub const DEFAULT_LIMIT: usize = 10;

    #[note("history-localhistory-new")]
    pub fn new(root: PathBuf, mut settings: HistorySettings) -> Self {
        settings.limit = settings.limit.clamp(1, 100);
        Self { root, settings }
    }

    #[note("history-localhistory-record")]
    pub fn record(
        &self,
        file: &Path,
        text: &str,
        origin: Origin,
        time: u64,
    ) -> std::io::Result<bool> {
        if !self.settings.enabled {
            return Ok(false);
        }
        if (text.len() as u64) > self.settings.max_file_kb.saturating_mul(1024) {
            return Ok(false);
        }
        let Some((rel_path, dir)) = self.file_dir(file) else {
            return Ok(false);
        };
        let history_root = self.root.join(".crawrad").join("history");
        let entries_file = dir.join("entries.json");

        let mut entries = read_entries_file(&entries_file);
        let content_hash = sha256_hex(text.as_bytes());

        if let Some(last) = entries.last()
            && last.hash == content_hash
        {
            return Ok(false);
        }

        let can_merge = self.settings.merge_seconds > 0
            && entries.last().is_some_and(|last| {
                last.origin == origin
                    && time >= last.time
                    && time < last.time.saturating_add(self.settings.merge_seconds)
            });

        if !history_root.exists() {
            ensure_gitignore(&self.root);
            std::fs::create_dir_all(&history_root)?;
        }
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        let blob_file = dir.join(format!("{content_hash}.txt"));
        if !blob_file.exists() {
            std::fs::write(&blob_file, text)?;
        }

        if can_merge {
            if let Some(old_entry) = entries.pop() {
                entries.push(Entry {
                    time,
                    hash: content_hash,
                    origin,
                });
                let still_referenced = entries.iter().any(|e| e.hash == old_entry.hash);
                if !still_referenced {
                    let old_blob = dir.join(format!("{}.txt", old_entry.hash));
                    std::fs::remove_file(&old_blob).ok();
                }
            }
        } else {
            entries.push(Entry {
                time,
                hash: content_hash,
                origin,
            });
        }

        if entries.len() > self.settings.limit {
            let excess = entries.len() - self.settings.limit;
            let pruned: Vec<Entry> = entries.drain(..excess).collect();
            for p in &pruned {
                let still_referenced = entries.iter().any(|e| e.hash == p.hash);
                if !still_referenced {
                    let pruned_blob = dir.join(format!("{}.txt", p.hash));
                    std::fs::remove_file(&pruned_blob).ok();
                }
            }
        }

        write_entries_file(&entries_file, &rel_path, &entries)?;
        Ok(true)
    }

    #[note("history-localhistory-entries")]
    pub fn entries(&self, file: &Path) -> Vec<Entry> {
        self.file_dir(file)
            .map(|(_, dir)| read_entries_file(&dir.join("entries.json")))
            .unwrap_or_default()
    }

    #[note("history-localhistory-content")]
    pub fn content(&self, file: &Path, entry: &Entry) -> Option<String> {
        let (_, dir) = self.file_dir(file)?;
        std::fs::read_to_string(dir.join(format!("{}.txt", entry.hash))).ok()
    }

    #[note("history-localhistory-file-dir")]
    fn file_dir(&self, file: &Path) -> Option<(String, PathBuf)> {
        let rel_path = relative_slash_path(&self.root, file)?;
        let dir = self
            .root
            .join(".crawrad")
            .join("history")
            .join(sha256_hex(rel_path.as_bytes()));
        Some((rel_path, dir))
    }
}

#[note("history-now")]
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn strip_verbatim(path: &Path) -> &Path {
    let Some(s) = path.to_str() else {
        return path;
    };
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        Path::new(stripped)
    } else {
        path
    }
}

fn to_slash_relative<'a>(components: impl Iterator<Item = Component<'a>>) -> Option<String> {
    let mut parts = Vec::new();
    for component in components {
        match component {
            Component::Normal(part) => parts.push(part.to_str()?),
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

fn relative_slash_path(root: &Path, file: &Path) -> Option<String> {
    let root: Vec<_> = strip_verbatim(root).components().collect();
    let file: Vec<_> = strip_verbatim(file).components().collect();
    let inside = file.len() > root.len()
        && root
            .iter()
            .zip(&file)
            .all(|(r, f)| r.as_os_str().eq_ignore_ascii_case(f.as_os_str()));
    if !inside {
        return None;
    }
    to_slash_relative(file[root.len()..].iter().copied())
}

fn read_entries_file(path: &Path) -> Vec<Entry> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) else {
        return Vec::new();
    };
    let Some(arr) = val.get("entries").and_then(|e| e.as_array()) else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for item in arr {
        let time = item.get("time").and_then(|t| t.as_u64());
        let hash = item.get("hash").and_then(|h| h.as_str());
        let origin = item
            .get("origin")
            .and_then(|o| o.as_str())
            .and_then(Origin::parse);
        if let (Some(time), Some(hash), Some(origin)) = (time, hash, origin) {
            entries.push(Entry {
                time,
                hash: hash.to_string(),
                origin,
            });
        }
    }
    entries
}

fn write_entries_file(path: &Path, rel_path: &str, entries: &[Entry]) -> std::io::Result<()> {
    let entries_val: Vec<serde_json::Value> = entries
        .iter()
        .map(|e| {
            serde_json::json!({
                "time": e.time,
                "hash": e.hash,
                "origin": e.origin.as_str(),
            })
        })
        .collect();
    let root_val = serde_json::json!({
        "path": rel_path,
        "entries": entries_val,
    });
    let json_bytes = serde_json::to_vec_pretty(&root_val).map_err(std::io::Error::other)?;
    std::fs::write(path, json_bytes)
}

fn ensure_gitignore(root: &Path) {
    let gitignore_path = root.join(".gitignore");
    if gitignore_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
            let already_ignored = content.lines().any(|line| line.trim() == ".crawrad/");
            if !already_ignored {
                let mut updated = content;
                if !updated.is_empty() && !updated.ends_with('\n') {
                    updated.push('\n');
                }
                updated.push_str(".crawrad/\n");
                std::fs::write(&gitignore_path, updated).ok();
            }
        }
    } else {
        let git_path = root.join(".git");
        if git_path.exists() {
            std::fs::write(&gitignore_path, ".crawrad/\n").ok();
        }
    }
}

#[cfg(test)]
#[path = "test/history/test.rs"]
mod tests;
