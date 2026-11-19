//! The user's app started by Run without the debugger, watched until it ends:
//! its stderr is kept (stdout still goes where it went) and read for panics
//! when the process is gone.

use crate::panic_report::{PanicSite, find_panics};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// How often the watcher looks, and how many quiet looks end a panic's report.
const QUIET_STEP: Duration = Duration::from_millis(250);
const QUIET_STEPS: u32 = 4;

/// How a watched app ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEnd {
    /// The package that ran.
    pub package: String,
    /// The workspace root its paths are relative to.
    pub root: PathBuf,
    /// The panics its stderr told, in order; empty when it ended cleanly.
    pub panics: Vec<PanicSite>,
    /// The whole stderr, for the panel's "Full output".
    pub stderr: String,
}

impl AppEnd {
    /// How `package` ended, from what it wrote to stderr.
    pub fn read(package: String, root: PathBuf, stderr: String) -> Self {
        let panics = find_panics(&stderr, &root);
        Self {
            package,
            root,
            panics,
            stderr,
        }
    }
}

/// The app processes Run started, each watched by a task.
#[derive(Default)]
pub struct AppRun {
    tasks: crawrad_os::tasks::TaskManager<AppEnd>,
}

impl AppRun {
    /// Starts `cmd` for `package` with a backtrace on, and watches it.
    /// The launch error comes back at once; the end arrives later by [`AppRun::pump`].
    pub fn start(&mut self, mut cmd: Command, package: &str, root: &Path) -> Result<(), String> {
        let program = cmd.get_program().to_string_lossy().into_owned();
        let mut child = cmd
            .env("RUST_BACKTRACE", "1")
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("could not launch {program}: {e}"))?;
        let (package, root) = (package.to_string(), root.to_path_buf());
        let said = Arc::new(Mutex::new(Vec::<u8>::new()));
        if let Some(mut err) = child.stderr.take() {
            let sink = Arc::clone(&said);
            std::thread::spawn(move || {
                let mut chunk = [0u8; 4096];
                while let Ok(n @ 1..) = err.read(&mut chunk) {
                    sink.lock()
                        .unwrap_or_else(|e| e.into_inner())
                        .extend_from_slice(&chunk[..n]);
                }
            });
        }
        self.tasks.spawn("app-run", move |reporter| {
            let text = || {
                String::from_utf8_lossy(&said.lock().unwrap_or_else(|e| e.into_inner()))
                    .into_owned()
            };
            // A panic in a callback that cannot unwind aborts, and Windows may hold
            // the dead app in its error report: the panic is told once stderr has
            // said it and gone quiet, not when the process is finally gone.
            let (mut seen, mut quiet) = (0, 0);
            loop {
                let exited = matches!(child.try_wait(), Ok(Some(_)) | Err(_));
                std::thread::sleep(QUIET_STEP);
                let now = text();
                quiet = if now.len() == seen { quiet + 1 } else { 0 };
                seen = now.len();
                if exited || (quiet >= QUIET_STEPS && now.contains("panicked at ")) {
                    return reporter.done(AppEnd::read(package, root, now));
                }
            }
        });
        Ok(())
    }

    /// The apps that ended since the last call.
    pub fn pump(&mut self) -> Vec<AppEnd> {
        self.tasks
            .drain()
            .into_iter()
            .filter_map(|(_, event)| match event {
                crawrad_os::tasks::TaskEvent::Done(end) => Some(end),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "test/app_run/test.rs"]
mod app_run_test;
