//! # When the generated app cannot start
//!
//! A generated app is `windows_subsystem = "windows"` in release: there is no
//! console. If `main` returned the toolkit's `Result`, Rust's `Termination`
//! would print `Error: …` to a stderr nobody reads, and the program would just
//! not open — in front of OUR user's customer, who has no terminal and nobody
//! to ask. So the generated `main` hands the error here, and the person sees
//! it: a native message box, plus a log file next to the executable that can
//! be sent to whoever built the app.
//!
//! Native through `rfd`, which this crate already carries for the file dialogs:
//! the fix adds no dependency to the user's project.

use crawrad_note::note;
use std::fmt::Display;
#[cfg(not(target_arch = "wasm32"))]
use std::path::{Path, PathBuf};

#[note("startup-report-startup-failure")]
pub fn report_startup_failure(app: &str, error: &dyn Display) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let log = write_startup_log(&startup_log_path(), app, error);
        let text = startup_failure_text(app, error, log.as_deref());
        eprintln!("{text}");
        let _ = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title(app)
            .set_description(text)
            .set_buttons(rfd::MessageButtons::Ok)
            .show();
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (app, error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[note("startup-startup-log-path")]
fn startup_log_path() -> PathBuf {
    std::env::current_exe()
        .map(|exe| exe.with_extension("error.log"))
        .unwrap_or_else(|_| std::env::temp_dir().join("crawrad-app.error.log"))
}

#[cfg(not(target_arch = "wasm32"))]
#[note("startup-write-startup-log")]
pub(crate) fn write_startup_log(path: &Path, app: &str, error: &dyn Display) -> Option<PathBuf> {
    let body = format!("{app} could not start.\n\n{error}\n");
    if std::fs::write(path, &body).is_ok() {
        return Some(path.to_path_buf());
    }
    let fallback = std::env::temp_dir().join(path.file_name()?);
    std::fs::write(&fallback, &body).ok().map(|()| fallback)
}

#[cfg(not(target_arch = "wasm32"))]
#[note("startup-startup-failure-text")]
pub(crate) fn startup_failure_text(app: &str, error: &dyn Display, log: Option<&Path>) -> String {
    let mut text = format!("{app} could not start.\n\n{error}");
    if let Some(log) = log {
        text.push_str(&format!("\n\nDetails were saved to:\n{}", log.display()));
    }
    text
}
