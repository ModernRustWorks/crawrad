//! Autorecover: recovery copies of the files with unsaved edits, kept
//! in `<project>/.crawrad/recover` at the same relative path — never over the
//! file itself — so a crash loses at most one interval of typing.
//!
//! The folder mirrors the unsaved edits ([`sync`]): a file saved or closed
//! loses its copy on the next pass, and a clean exit empties it (`clear`).
//! What is left after a crash and is newer than the file on disk is what the
//! IDE offers back ([`pending`]), and [`Mirror`] keeps those until the user
//! recovers or discards them: the next pass must not wipe them.

use crawrad_note::note;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

#[note("recover-minutes")]
pub const MINUTES: [u32; 4] = [1, 5, 10, 30];

#[note("recover-folder")]
pub fn folder(root: &Path) -> PathBuf {
    root.join(".crawrad").join("recover")
}

#[note("recover-pending")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pending {
    /// The file, relative to the project.
    pub file: PathBuf,
    /// When the copy was written.
    pub copy: SystemTime,
    /// When the file on disk was last written; `None` when it is gone.
    pub disk: Option<SystemTime>,
}

#[note("recover-mirror")]
#[derive(Debug, Default)]
pub struct Mirror {
    root: Option<PathBuf>,
    kept: Vec<PathBuf>,
}

impl Mirror {
    #[note("recover-mirror-follow")]
    pub fn follow(&mut self, root: Option<&Path>) -> Option<Vec<Pending>> {
        if self.root.as_deref() == root {
            return None;
        }
        self.close();
        self.root = root.map(Path::to_path_buf);
        let found = root.map(pending).unwrap_or_default();
        self.kept = found.iter().map(|p| p.file.clone()).collect();
        Some(found)
    }

    #[note("recover-mirror-root")]
    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    #[note("recover-mirror-kept")]
    pub fn kept(&self) -> &[PathBuf] {
        &self.kept
    }

    #[note("recover-mirror-sync")]
    pub fn sync<'a>(
        &self,
        edits: impl IntoIterator<Item = (&'a Path, &'a str)>,
    ) -> std::io::Result<usize> {
        self.root
            .as_deref()
            .map_or(Ok(0), |r| sync(r, edits, &self.kept))
    }

    #[note("recover-mirror-settle")]
    pub fn settle(&mut self, files: &[PathBuf]) {
        self.kept.retain(|k| !files.contains(k));
    }

    #[note("recover-mirror-answer")]
    pub fn answer(&mut self, files: Vec<PathBuf>, discard_them: bool) -> Vec<(PathBuf, String)> {
        let Some(root) = self.root.clone() else {
            return Vec::new();
        };
        let files = if files.is_empty() {
            self.kept.clone()
        } else {
            files
        };
        let found: Vec<_> = files
            .into_iter()
            .filter_map(|f| read(&root, &f).map(|text| (f, text)))
            .collect();
        let names: Vec<_> = found.iter().map(|(f, _)| f.clone()).collect();
        if discard_them {
            discard(&root, &names);
        }
        self.settle(&names);
        found
            .into_iter()
            .map(|(f, text)| (root.join(f), text))
            .collect()
    }

    #[note("recover-mirror-close")]
    pub fn close(&mut self) {
        if let Some(r) = &self.root {
            let _ = sync(r, [], &self.kept);
        }
    }
}

#[note("recover-sync")]
pub fn sync<'a>(
    root: &Path,
    edits: impl IntoIterator<Item = (&'a Path, &'a str)>,
    keep: &[PathBuf],
) -> std::io::Result<usize> {
    let dir = folder(root);
    let mut kept = Vec::new();
    for (file, text) in edits {
        let Ok(rel) = file.strip_prefix(root) else {
            continue;
        };
        let copy = dir.join(rel);
        if std::fs::read_to_string(&copy).ok().as_deref() != Some(text) {
            std::fs::create_dir_all(copy.parent().unwrap_or(&dir))?;
            std::fs::write(&copy, text)?;
        }
        kept.push(rel.to_path_buf());
    }
    let mut left = 0;
    for old in copies(&dir) {
        if kept.contains(&old) || keep.contains(&old) {
            left += 1;
        } else {
            let _ = std::fs::remove_file(dir.join(&old));
        }
    }
    Ok(left)
}

#[note("recover-pending-2")]
pub fn pending(root: &Path) -> Vec<Pending> {
    let dir = folder(root);
    let modified = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    let mut out: Vec<Pending> = copies(&dir)
        .into_iter()
        .filter_map(|file| {
            let copy = modified(&dir.join(&file))?;
            let disk = modified(&root.join(&file));
            disk.is_none_or(|d| copy > d)
                .then_some(Pending { file, copy, disk })
        })
        .collect();
    out.sort_by(|a, b| a.file.cmp(&b.file));
    out
}

#[note("recover-read")]
pub fn read(root: &Path, file: &Path) -> Option<String> {
    inside(file).then(|| std::fs::read_to_string(folder(root).join(file)).ok())?
}

#[note("recover-discard")]
pub fn discard(root: &Path, files: &[PathBuf]) {
    for f in files.iter().filter(|f| inside(f)) {
        let _ = std::fs::remove_file(folder(root).join(f));
    }
}

#[note("recover-inside")]
fn inside(file: &Path) -> bool {
    file.components().all(|c| matches!(c, Component::Normal(_)))
}

#[note("recover-offer")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Offer {
    pub rows: Vec<OfferRow>,
}

#[note("recover-offerrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfferRow {
    pub file: PathBuf,
    /// When the copy was written, as [`when`] says it.
    pub copy: String,
    /// When the file on disk was, or `missing` when it is gone.
    pub disk: String,
    pub checked: bool,
}

impl Offer {
    #[note("recover-offer-new")]
    pub fn new(found: &[Pending], now: SystemTime, offset: i64) -> Self {
        let rows = found.iter().map(|p| OfferRow {
            file: p.file.clone(),
            copy: when(p.copy, now, offset),
            disk: p
                .disk
                .map_or_else(|| "missing".into(), |d| when(d, now, offset)),
            checked: true,
        });
        Self {
            rows: rows.collect(),
        }
    }

    #[note("recover-offer-check")]
    pub fn check(&mut self, i: usize, on: bool) {
        if let Some(r) = self.rows.get_mut(i) {
            r.checked = on;
        }
    }

    #[note("recover-offer-checked")]
    pub fn checked(&self) -> Vec<PathBuf> {
        let rows = self.rows.iter().filter(|r| r.checked);
        rows.map(|r| r.file.clone()).collect()
    }

    #[note("recover-offer-files")]
    pub fn files(&self) -> Vec<PathBuf> {
        self.rows.iter().map(|r| r.file.clone()).collect()
    }
}

#[note("recover-when")]
pub fn when(then: SystemTime, now: SystemTime, offset: i64) -> String {
    let local = |t: SystemTime| {
        let secs = t
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        i64::try_from(secs).unwrap_or(0) + offset
    };
    let (t, n) = (local(then), local(now));
    let (day, today) = (t.div_euclid(86_400), n.div_euclid(86_400));
    let minute = t.rem_euclid(86_400) / 60;
    if day == today {
        format!("{:02}:{:02}", minute / 60, minute % 60)
    } else if day == today - 1 {
        "yesterday".into()
    } else {
        let (y, m, d) = crawrad_os::clock::civil_from_days(day);
        format!("{y:04}-{m:02}-{d:02}")
    }
}

#[note("recover-copies")]
fn copies(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut todo = vec![dir.to_path_buf()];
    while let Some(d) = todo.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                todo.push(p);
            } else if let Ok(rel) = p.strip_prefix(dir) {
                out.push(rel.to_path_buf());
            }
        }
    }
    out
}

#[cfg(test)]
#[path = "test/recover/test.rs"]
mod tests;
