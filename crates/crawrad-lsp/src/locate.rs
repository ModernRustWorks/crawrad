//! Where the IDE finds `rust-analyzer` (ADR 0050).
//!
//! The IDE uses the server it ships, the way it uses its own compiler
//! (ADR 0034) and its own `lldb-dap` (ADR 0048): the version the IDE was
//! tested with is the one that runs. It sits in [`SERVER_DIR`] beside the
//! executable, fetched by `tools/rust-analyzer/fetch-rust-analyzer.ps1`.
//!
//! In order:
//! 1. [`SERVER_ENV`] — an explicit path, for development and the tests that
//!    run against a real server;
//! 2. the IDE's own, beside the executable;
//! 3. **last resort**, the machine's: the `PATH`, then `~/.cargo/bin`. Kept
//!    so a checkout of the repository still has a server (owner's call,
//!    01/10/2026), but reported as [`ServerSource::System`] so the status bar
//!    can say it is not the IDE's.

use crawrad_note::note;
use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

#[note("locate-server-dir")]
pub const SERVER_DIR: &str = "rust-analyzer";

#[note("locate-server-env")]
pub const SERVER_ENV: &str = "CRAWRAD_RUST_ANALYZER";

#[note("locate-server-file-name")]
pub fn server_file_name() -> &'static str {
    if cfg!(windows) {
        "rust-analyzer.exe"
    } else {
        "rust-analyzer"
    }
}

#[note("locate-serversource")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerSource {
    /// [`SERVER_ENV`] named it.
    Override,
    /// The IDE's own, beside the executable.
    Bundled,
    /// The machine's `PATH` or `~/.cargo/bin`: whatever version it has.
    System,
}

impl ServerSource {
    #[note("locate-serversource-status-name")]
    pub fn status_name(self) -> &'static str {
        match self {
            Self::System => "rust-analyzer (system copy)",
            Self::Override | Self::Bundled => "rust-analyzer",
        }
    }
}

#[note("locate-locatedserver")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedServer {
    pub path: PathBuf,
    pub source: ServerSource,
}

#[note("locate-servermissing")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerMissing {
    /// Every place looked at, in order (the `PATH` folders not listed).
    pub looked_at: Vec<PathBuf>,
}

impl fmt::Display for ServerMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} was not found", server_file_name())?;
        if !self.looked_at.is_empty() {
            let places: Vec<String> = self
                .looked_at
                .iter()
                .map(|p| p.display().to_string())
                .collect();
            write!(f, " in {}", places.join(", "))?;
        }
        write!(f, " nor in the PATH")
    }
}

impl std::error::Error for ServerMissing {}

#[note("locate-locate")]
pub fn locate() -> Result<LocatedServer, ServerMissing> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));
    let home = if cfg!(windows) {
        std::env::var_os("USERPROFILE")
    } else {
        std::env::var_os("HOME")
    };
    locate_from(
        std::env::var_os(SERVER_ENV).map(PathBuf::from),
        exe_dir.as_deref(),
        std::env::var_os("PATH"),
        home.map(PathBuf::from),
    )
}

#[note("locate-locate-from")]
pub fn locate_from(
    overridden: Option<PathBuf>,
    exe_dir: Option<&Path>,
    path_var: Option<OsString>,
    home: Option<PathBuf>,
) -> Result<LocatedServer, ServerMissing> {
    let found = |path: PathBuf, source| Ok(LocatedServer { path, source });
    let mut looked_at = Vec::new();
    if let Some(path) = overridden.filter(|p| !p.as_os_str().is_empty()) {
        if path.is_file() {
            return found(path, ServerSource::Override);
        }
        looked_at.push(path);
    }
    if let Some(dir) = exe_dir {
        let bundled = dir.join(SERVER_DIR).join(server_file_name());
        if bundled.is_file() {
            return found(bundled, ServerSource::Bundled);
        }
        looked_at.push(bundled);
    }
    if let Some(paths) = path_var {
        for dir in std::env::split_paths(&paths) {
            let candidate = dir.join(server_file_name());
            if candidate.is_file() {
                return found(candidate, ServerSource::System);
            }
        }
    }
    if let Some(home) = home {
        let cargo_bin = home.join(".cargo").join("bin").join(server_file_name());
        if cargo_bin.is_file() {
            return found(cargo_bin, ServerSource::System);
        }
        looked_at.push(cargo_bin);
    }
    Err(ServerMissing { looked_at })
}

#[cfg(test)]
#[path = "test/locate/test.rs"]
mod tests;
