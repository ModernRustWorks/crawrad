//! Where a relative SQLite url points, for a generated app (#Beta: relative db url).
//!
//! The Designer reads `sqlite:contacts.db` next to the project member's
//! `Cargo.toml`. The app must read the same file whatever its working
//! directory is, so a relative path is NEVER resolved against the working
//! directory. It is looked for, in order:
//!
//! 1. next to the running executable — where a deployed app keeps its data;
//! 2. in the project folder the app was built from (`CARGO_MANIFEST_DIR`,
//!    baked in at compile time) — the same file the Designer shows, so an
//!    app run from the IDE edits the very database the form was designed on
//!    (a copy next to the exe would silently diverge from it).
//!
//! When neither exists the url names the candidate that fits the run (the
//! project folder while it still exists, else next to the exe), so the
//! "database file not found" message carries a full path the user can check.

use std::path::{Path, PathBuf};

use crawrad_note::note;

/// The file a SQLite url names, or `None` for in-memory and URI forms.
#[note("db-url-sqlite-file")]
pub fn sqlite_file(url: &str) -> Option<&str> {
    let url = url.trim();
    let path = url
        .strip_prefix("sqlite://")
        .or_else(|| url.strip_prefix("sqlite:"))?;
    (!path.is_empty() && path != ":memory:" && !path.starts_with("file:")).then_some(path)
}

/// `url` with a relative SQLite file made absolute (see the module docs).
///
/// Anything else (absolute paths, in-memory, other databases) is returned as is.
#[note("db-url-resolve-sqlite-url")]
pub fn resolve_sqlite_url(url: &str, project_dir: &str) -> String {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|e| e.parent().map(Path::to_path_buf));
    resolve_with(url, exe_dir.as_deref(), Path::new(project_dir))
}

/// [`resolve_sqlite_url`] with the executable's folder given (for tests).
#[note("db-url-resolve-with")]
pub fn resolve_with(url: &str, exe_dir: Option<&Path>, project_dir: &Path) -> String {
    let Some(rel) = sqlite_file(url).filter(|p| Path::new(p).is_relative()) else {
        return url.to_string();
    };
    let by_exe = exe_dir.map(|d| d.join(rel));
    let by_project = (!project_dir.as_os_str().is_empty()).then(|| project_dir.join(rel));
    let chosen: Option<PathBuf> = [by_exe.clone(), by_project.clone()]
        .into_iter()
        .flatten()
        .find(|p| p.is_file())
        .or_else(|| by_project.filter(|_| project_dir.is_dir()))
        .or(by_exe);
    match chosen {
        Some(p) => format!("sqlite://{}", p.to_string_lossy().replace('\\', "/")),
        None => url.to_string(),
    }
}

#[cfg(test)]
#[path = "test/db_url/test.rs"]
mod tests;
