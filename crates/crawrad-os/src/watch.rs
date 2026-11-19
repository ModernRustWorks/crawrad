//! Project file watcher waking the UI on external filesystem changes.

use crawrad_note::note;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;

#[note("watch-strip-verbatim")]
fn strip_verbatim(p: &Path) -> &Path {
    let s = match p.to_str() {
        Some(s) => s,
        None => return p,
    };
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        Path::new(stripped)
    } else {
        p
    }
}

#[note("watch-ignored")]
pub fn ignored(root: &Path, path: &Path) -> bool {
    let norm_root = strip_verbatim(root);
    let norm_path = strip_verbatim(path);
    if let Ok(rel) = norm_path.strip_prefix(norm_root) {
        return rel.components().any(|c| {
            c.as_os_str() == "target" || c.as_os_str() == ".git" || c.as_os_str() == ".crawrad"
        });
    }
    let root_comps: Vec<_> = norm_root.components().collect();
    let path_comps: Vec<_> = norm_path.components().collect();
    if path_comps.len() >= root_comps.len() {
        let prefix_matches = root_comps
            .iter()
            .zip(&path_comps)
            .all(|(rc, pc)| rc.as_os_str().eq_ignore_ascii_case(pc.as_os_str()));
        if prefix_matches {
            return path_comps[root_comps.len()..].iter().any(|c| {
                c.as_os_str() == "target" || c.as_os_str() == ".git" || c.as_os_str() == ".crawrad"
            });
        }
    }
    false
}

#[note("watch-projectwatcher")]
pub struct ProjectWatcher {
    _watcher: RecommendedWatcher,
    rx: Receiver<PathBuf>,
    root: PathBuf,
}

impl ProjectWatcher {
    #[note("watch-projectwatcher-start")]
    pub fn start(root: &Path) -> Result<Self, String> {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res
                    && !matches!(event.kind, EventKind::Access(_))
                    && !event.paths.is_empty()
                {
                    let mut sent = false;
                    for path in event.paths {
                        if tx.send(path).is_ok() {
                            sent = true;
                        }
                    }
                    if sent {
                        crate::tasks::wake();
                    }
                }
            },
            Config::default(),
        )
        .map_err(|e| e.to_string())?;

        watcher
            .watch(root, RecursiveMode::Recursive)
            .map_err(|e| e.to_string())?;

        Ok(Self {
            _watcher: watcher,
            rx,
            root: root.to_path_buf(),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    #[note("watch-projectwatcher-drain")]
    pub fn drain(&mut self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        while let Ok(path) = self.rx.try_recv() {
            if !ignored(&self.root, &path) {
                paths.push(path);
            }
        }
        paths.sort();
        paths.dedup();
        paths
    }
}

#[cfg(test)]
#[path = "test/watch/test.rs"]
mod tests;
