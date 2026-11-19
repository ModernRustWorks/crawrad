//! External file change detection and watcher management (ADR 0044).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crawrad_os::watch::ProjectWatcher;

use crate::history::{HistorySettings, LocalHistory, Origin, now};
use crawrad_note::note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskAction {
    Deleted,
    Same,
    Reload,
    Conflict,
}

#[note("disk-watch-disk-change-action")]
pub fn disk_change_action(
    disk: Option<&str>,
    ide_text: &str,
    has_buffer: bool,
    base: Option<&str>,
) -> DiskAction {
    let Some(disk) = disk else {
        return DiskAction::Deleted;
    };
    if disk == ide_text {
        return DiskAction::Same;
    }
    if !has_buffer {
        return DiskAction::Reload;
    }
    if base == Some(disk) {
        return DiskAction::Same;
    }
    DiskAction::Conflict
}

#[note("disk-watch-diskreaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskReaction {
    pub path: PathBuf,
    pub name: String,
    pub action: DiskAction,
    pub active: bool,
}

#[derive(Default)]
pub struct DiskWatch {
    watcher: Option<ProjectWatcher>,
    watched_root: Option<PathBuf>,
    bases: BTreeMap<PathBuf, String>,
    conflict: Option<PathBuf>,
    history_settings: HistorySettings,
}

impl DiskWatch {
    #[note("disk-watch-diskwatch-set-history-settings")]
    pub fn set_history_settings(&mut self, settings: HistorySettings) {
        self.history_settings = settings;
    }

    #[note("disk-watch-diskwatch-history")]
    pub fn history(&self) -> Option<LocalHistory> {
        if !self.history_settings.enabled {
            return None;
        }
        let root = self.watched_root.clone()?;
        Some(LocalHistory::new(root, self.history_settings))
    }

    pub fn ensure(&mut self, root: Option<PathBuf>) {
        if root == self.watched_root {
            return;
        }
        self.watcher = root.as_deref().and_then(|r| ProjectWatcher::start(r).ok());
        self.watched_root = root;
    }

    #[note("disk-watch-diskwatch-drain")]
    pub fn drain(
        &mut self,
        open: &[PathBuf],
        active: Option<&Path>,
        buffer: &dyn Fn(&Path) -> Option<String>,
        view_text: &str,
    ) -> (bool, Vec<DiskReaction>) {
        let Some(watcher) = self.watcher.as_mut() else {
            return (false, Vec::new());
        };
        let changed = watcher.drain();
        if changed.is_empty() {
            return (false, Vec::new());
        }
        let mut reactions = Vec::new();
        for path in changed {
            if !open.iter().any(|f| f == &path) {
                continue;
            }
            let name = file_name(&path);
            let is_active = active == Some(&path);
            let disk_content = match std::fs::read_to_string(&path) {
                Ok(content) => content.replace("\r\n", "\n"),
                Err(_) => {
                    reactions.push(DiskReaction {
                        path,
                        name,
                        action: DiskAction::Deleted,
                        active: is_active,
                    });
                    continue;
                }
            };
            let buf = buffer(&path);
            let has_buffer = buf.is_some();
            let ide_text = if let Some(ref b) = buf {
                b.as_str()
            } else if is_active {
                view_text
            } else {
                self.bases.get(&path).map(String::as_str).unwrap_or("")
            };
            let base = self.bases.get(&path).map(String::as_str);
            let action = disk_change_action(Some(&disk_content), ide_text, has_buffer, base);
            match action {
                DiskAction::Deleted => {}
                DiskAction::Same => {
                    self.bases.insert(path.clone(), disk_content);
                }
                DiskAction::Reload => {
                    if let Some(history) = self.history() {
                        history
                            .record(&path, &disk_content, Origin::External, now())
                            .ok();
                    }
                    self.bases.insert(path.clone(), disk_content);
                }
                DiskAction::Conflict => {
                    self.conflict = Some(path.clone());
                }
            }
            reactions.push(DiskReaction {
                path,
                name,
                action,
                active: is_active,
            });
        }
        (true, reactions)
    }

    pub fn saved(&mut self, path: &Path, text: String) {
        self.bases.insert(path.to_path_buf(), text);
        if self.conflict.as_deref() == Some(path) {
            self.conflict = None;
        }
        if let Some(history) = self.history()
            && let Some(saved_text) = self.bases.get(path)
        {
            history.record(path, saved_text, Origin::Save, now()).ok();
        }
    }

    pub fn shown(&mut self, path: &Path, disk_text: String) {
        self.bases.insert(path.to_path_buf(), disk_text);
    }

    pub fn keep_mine(&mut self, path: &Path) {
        if let Ok(c) = std::fs::read_to_string(path) {
            self.bases
                .insert(path.to_path_buf(), c.replace("\r\n", "\n"));
        }
        self.conflict = None;
    }

    pub fn clear_conflict(&mut self) {
        self.conflict = None;
    }

    pub fn conflict(&self) -> Option<&Path> {
        self.conflict.as_deref()
    }
}

pub fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    )
}

#[cfg(test)]
#[path = "test/disk_watch/test.rs"]
mod tests;
