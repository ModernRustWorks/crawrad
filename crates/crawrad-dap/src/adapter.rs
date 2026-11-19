//! Where the IDE finds its debug adapter.
//!
//! The IDE uses the adapter it ships, never one from the `PATH`:
//! an `lldb-dap` found there is whatever the machine happens to have —
//! every ready-made LLDB for Windows needs Python, and a different version
//! reads values differently. The IDE's own sits in [`ADAPTER_DIR`] beside its
//! executable, the way installed components sit in `components/`. It is built
//! without Python by `tools/lldb/build-lldb.ps1`.
//!
//! `CRAWRAD_LLDB_DAP` overrides the place, for development and for the tests that
//! run against a real adapter.

use crawrad_note::note;
use std::fmt;
use std::path::{Path, PathBuf};

#[note("adapter-adapter-dir")]
pub const ADAPTER_DIR: &str = "lldb";

#[note("adapter-adapter-env")]
pub const ADAPTER_ENV: &str = "CRAWRAD_LLDB_DAP";

#[note("adapter-adapter-file-name")]
pub fn adapter_file_name() -> &'static str {
    if cfg!(windows) {
        "lldb-dap.exe"
    } else {
        "lldb-dap"
    }
}

#[note("adapter-adaptermissing")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterMissing {
    /// Every place looked at, in order.
    pub looked_at: Vec<PathBuf>,
}

impl fmt::Display for AdapterMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The debugger that comes with the IDE is not installed: {} was not found",
            adapter_file_name()
        )?;
        match self.looked_at.as_slice() {
            [] => Ok(()),
            [only] => write!(f, " in {}", only.display()),
            many => {
                let places: Vec<String> = many.iter().map(|p| p.display().to_string()).collect();
                write!(f, " in {}", places.join(", "))
            }
        }
    }
}

impl std::error::Error for AdapterMissing {}

#[note("adapter-locate")]
pub fn locate() -> Result<PathBuf, AdapterMissing> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(Path::to_path_buf));
    locate_from(
        std::env::var_os(ADAPTER_ENV).map(PathBuf::from),
        exe_dir.as_deref(),
    )
}

#[note("adapter-locate-from")]
pub fn locate_from(
    overridden: Option<PathBuf>,
    exe_dir: Option<&Path>,
) -> Result<PathBuf, AdapterMissing> {
    let mut looked_at = Vec::new();
    if let Some(path) = overridden.filter(|p| !p.as_os_str().is_empty()) {
        if path.is_file() {
            return Ok(path);
        }
        looked_at.push(path);
    }
    if let Some(dir) = exe_dir {
        let bundled = dir.join(ADAPTER_DIR).join(adapter_file_name());
        if bundled.is_file() {
            return Ok(bundled);
        }
        looked_at.push(bundled);
    }
    Err(AdapterMissing { looked_at })
}
