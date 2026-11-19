//! Probing the host machine and executing environment installation plans.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::environment::{Host, InstallStep, Probe};

/// Returns the path to cargo's `bin` directory (`$CARGO_HOME/bin` or `<home>/.cargo/bin`).
pub fn cargo_bin_dir() -> Option<PathBuf> {
    if let Some(cargo_home) = std::env::var_os("CARGO_HOME")
        && !cargo_home.is_empty()
    {
        return Some(PathBuf::from(cargo_home).join("bin"));
    }
    let home = if cfg!(windows) {
        std::env::var_os("USERPROFILE")
    } else {
        std::env::var_os("HOME")
    };
    home.filter(|h| !h.is_empty())
        .map(|h| PathBuf::from(h).join(".cargo").join("bin"))
}

/// Resolves a program executable name against an optional bin directory and platform executable suffix.
pub fn resolve_in(bin_dir: Option<&Path>, name: &str, exe_suffix: &str) -> PathBuf {
    if let Some(dir) = bin_dir {
        if !exe_suffix.is_empty() && !name.ends_with(exe_suffix) {
            let candidate_with_ext = dir.join(format!("{name}{exe_suffix}"));
            if candidate_with_ext.exists() {
                return candidate_with_ext;
            }
        }
        let candidate = dir.join(name);
        if candidate.exists() {
            return candidate;
        }
    }
    PathBuf::from(name)
}

/// Resolves a program executable against `cargo_bin_dir()` or falls back to bare name for PATH lookup.
pub fn resolve_program(name: &str) -> PathBuf {
    let exe_suffix = if cfg!(windows) { ".exe" } else { "" };
    resolve_in(cargo_bin_dir().as_deref(), name, exe_suffix)
}

/// System probe that inspects the real machine and executes tools.
#[derive(Clone, Copy, Debug)]
pub struct SystemProbe {
    /// Locator function for `rust-analyzer` (provided by LSP integration).
    pub rust_analyzer: fn() -> Option<PathBuf>,
    /// Locator function for `lldb-dap` (provided by DAP integration).
    pub lldb_dap: fn() -> Option<PathBuf>,
}

impl Default for SystemProbe {
    fn default() -> Self {
        Self {
            rust_analyzer: || None,
            lldb_dap: || None,
        }
    }
}

impl Probe for SystemProbe {
    fn output(&self, program: &str, args: &[&str]) -> Option<String> {
        let resolved = resolve_program(program);
        run_probe_output(&resolved, args, std::time::Duration::from_secs(15))
    }

    fn rust_analyzer(&self) -> Option<PathBuf> {
        if let Some(path) = (self.rust_analyzer)() {
            return Some(path);
        }
        let bin_dir = cargo_bin_dir()?;
        let exe_suffix = if cfg!(windows) { ".exe" } else { "" };
        let candidate_with_ext = bin_dir.join(format!("rust-analyzer{exe_suffix}"));
        if candidate_with_ext.exists() {
            return Some(candidate_with_ext);
        }
        let candidate_bare = bin_dir.join("rust-analyzer");
        if candidate_bare.exists() {
            return Some(candidate_bare);
        }
        None
    }

    fn lldb_dap(&self) -> Option<PathBuf> {
        (self.lldb_dap)()
    }

    fn msvc_installed(&self) -> bool {
        if !cfg!(windows) {
            return false;
        }
        let pf = std::env::var_os("ProgramFiles(x86)")
            .or_else(|| std::env::var_os("ProgramFiles"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(r"C:\Program Files (x86)"));
        let vswhere = pf
            .join("Microsoft Visual Studio")
            .join("Installer")
            .join("vswhere.exe");
        let args = [
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ];
        match run_probe_output(&vswhere, &args, std::time::Duration::from_secs(15)) {
            Some(out) => !out.trim().is_empty(),
            None => false,
        }
    }

    fn host(&self) -> Option<Host> {
        Host::current()
    }
}

fn run_probe_output(program: &Path, args: &[&str], timeout: std::time::Duration) -> Option<String> {
    let mut cmd = crawrad_os::process::no_console(program);
    cmd.args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null());

    let child_holder: Arc<Mutex<Option<std::process::Child>>> = Arc::new(Mutex::new(None));
    let holder = Arc::clone(&child_holder);

    let (tx, rx) = std::sync::mpsc::channel();
    let thread_handle = std::thread::spawn(move || {
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(_) => {
                let _ = tx.send(None);
                return;
            }
        };

        let mut stdout_pipe = match child.stdout.take() {
            Some(p) => p,
            None => {
                let _ = child.kill();
                let _ = tx.send(None);
                return;
            }
        };

        if let Ok(mut guard) = holder.lock() {
            *guard = Some(child);
        } else {
            return;
        }

        let mut stdout_bytes = Vec::new();
        let read_ok = stdout_pipe.read_to_end(&mut stdout_bytes).is_ok();

        let status = if let Ok(mut guard) = holder.lock() {
            if let Some(ref mut child) = *guard {
                child.wait().ok()
            } else {
                None
            }
        } else {
            None
        };

        if read_ok
            && let Some(st) = status
            && st.success()
            && let Ok(text) = String::from_utf8(stdout_bytes)
        {
            let _ = tx.send(Some(text));
            return;
        }
        let _ = tx.send(None);
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => {
            let _ = thread_handle.join();
            result
        }
        Err(_) => {
            if let Ok(mut guard) = child_holder.lock()
                && let Some(ref mut child) = *guard
            {
                let _ = child.kill();
                let _ = child.wait();
            }
            let _ = thread_handle.join();
            None
        }
    }
}

/// An event emitted during execution of an installation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallEvent {
    /// A step has started.
    Started { step: usize, title: String },
    /// Progress of a download step.
    Progress {
        step: usize,
        done: u64,
        total: Option<u64>,
    },
    /// A line of stdout or stderr from a running command.
    Line { step: usize, text: String },
    /// A step has finished.
    Finished {
        step: usize,
        ok: bool,
        message: String,
    },
}

/// Errors that can occur during installation plan execution.
#[derive(Debug, PartialEq, Eq)]
pub enum InstallError {
    /// The download URL does not match trusted sources.
    UntrustedUrl(String),
    /// Execution was cancelled.
    Cancelled,
    /// A step in the plan failed.
    StepFailed { step: usize, message: String },
}

impl std::fmt::Display for InstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstallError::UntrustedUrl(url) => write!(f, "untrusted download URL: {url}"),
            InstallError::Cancelled => write!(f, "installation cancelled"),
            InstallError::StepFailed { step, message } => {
                write!(f, "step {step} failed: {message}")
            }
        }
    }
}

impl std::error::Error for InstallError {}

/// Reports the step as failed and returns the error to propagate.
fn failed(step: usize, message: String, on_event: &mut dyn FnMut(InstallEvent)) -> InstallError {
    on_event(InstallEvent::Finished {
        step,
        ok: false,
        message: message.clone(),
    });
    InstallError::StepFailed { step, message }
}

/// Reports the step as cancelled and returns the error to propagate.
fn cancelled(step: usize, on_event: &mut dyn FnMut(InstallEvent)) -> InstallError {
    on_event(InstallEvent::Finished {
        step,
        ok: false,
        message: "cancelled".to_string(),
    });
    InstallError::Cancelled
}

fn create_ureq_agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .tls_config(
            ureq::tls::TlsConfig::builder()
                .provider(ureq::tls::TlsProvider::Rustls)
                .unversioned_rustls_crypto_provider(Arc::new(rustls_graviola::default_provider()))
                .build(),
        )
        .build()
        .new_agent()
}

fn execute_command(
    step: usize,
    program_path: &Path,
    args: &[&str],
    cancel: &AtomicBool,
    on_event: &mut dyn FnMut(InstallEvent),
) -> Result<(), InstallError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(cancelled(step, on_event));
    }

    let mut cmd = crawrad_os::process::no_console(program_path);
    cmd.args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let message = format!("failed to start {}: {e}", program_path.display());
            return Err(failed(step, message, on_event));
        }
    };

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let (line_tx, line_rx) = std::sync::mpsc::channel::<String>();

    let tx1 = line_tx.clone();
    let stdout_thread = std::thread::spawn(move || {
        if let Some(pipe) = stdout {
            let reader = BufReader::new(pipe);
            for line in reader.lines().map_while(Result::ok) {
                if tx1.send(line).is_err() {
                    break;
                }
            }
        }
    });

    let tx2 = line_tx;
    let stderr_thread = std::thread::spawn(move || {
        if let Some(pipe) = stderr {
            let reader = BufReader::new(pipe);
            for line in reader.lines().map_while(Result::ok) {
                if tx2.send(line).is_err() {
                    break;
                }
            }
        }
    });

    let exit_status;
    loop {
        if cancel.load(Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            let _ = stdout_thread.join();
            let _ = stderr_thread.join();
            return Err(cancelled(step, on_event));
        }

        match child.try_wait() {
            Ok(Some(status)) => {
                exit_status = Some(status);
                break;
            }
            Ok(None) => {}
            Err(e) => {
                let message = format!("failed waiting for process: {e}");
                let _ = child.kill();
                let _ = stdout_thread.join();
                let _ = stderr_thread.join();
                return Err(failed(step, message, on_event));
            }
        }

        match line_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(line) => {
                on_event(InstallEvent::Line { step, text: line });
                while let Ok(line) = line_rx.try_recv() {
                    on_event(InstallEvent::Line { step, text: line });
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }

    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    while let Ok(line) = line_rx.try_recv() {
        on_event(InstallEvent::Line { step, text: line });
    }

    let status = match exit_status {
        Some(s) => s,
        None => match child.wait() {
            Ok(s) => s,
            Err(e) => {
                let message = format!("failed waiting for process: {e}");
                return Err(failed(step, message, on_event));
            }
        },
    };

    if status.success() {
        on_event(InstallEvent::Finished {
            step,
            ok: true,
            message: String::new(),
        });
        Ok(())
    } else {
        let message = match status.code() {
            Some(code) => format!("process exited with code {code}"),
            None => "process terminated by signal".to_string(),
        };
        Err(failed(step, message, on_event))
    }
}

fn run_download_and_run(
    step: usize,
    url: &str,
    file_name: &str,
    args: &[&'static str],
    work_dir: &Path,
    cancel: &AtomicBool,
    on_event: &mut dyn FnMut(InstallEvent),
) -> Result<(), InstallError> {
    if !url.starts_with("https://static.rust-lang.org/") {
        return Err(InstallError::UntrustedUrl(url.to_string()));
    }

    if cancel.load(Ordering::Relaxed) {
        return Err(InstallError::Cancelled);
    }

    let name = file_name.strip_suffix(".exe").unwrap_or(file_name);
    let title = format!("Downloading {name}");
    on_event(InstallEvent::Started { step, title });

    if let Err(e) = std::fs::create_dir_all(work_dir) {
        let message = format!("failed to create directory {}: {e}", work_dir.display());
        return Err(failed(step, message, on_event));
    }

    let dest_path = work_dir.join(file_name);
    let agent = create_ureq_agent();
    let response = match agent.get(url).call() {
        Ok(resp) => resp,
        Err(e) => {
            let message = format!("download failed for {url}: {e}");
            return Err(failed(step, message, on_event));
        }
    };

    let total: Option<u64> = response
        .headers()
        .get("content-length")
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok());

    let mut file = match std::fs::File::create(&dest_path) {
        Ok(f) => f,
        Err(e) => {
            let message = format!("failed to create file {}: {e}", dest_path.display());
            return Err(failed(step, message, on_event));
        }
    };

    let mut reader = response.into_body().into_reader();
    let mut buffer = [0u8; 16384];
    let mut done: u64 = 0;
    let mut last_progress: u64 = 0;
    const PROGRESS_STEP: u64 = 256 * 1024; // 256 KiB

    loop {
        if cancel.load(Ordering::Relaxed) {
            drop(file);
            let _ = std::fs::remove_file(&dest_path);
            return Err(cancelled(step, on_event));
        }

        let n = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                drop(file);
                let _ = std::fs::remove_file(&dest_path);
                let message = format!("failed reading download stream: {e}");
                return Err(failed(step, message, on_event));
            }
        };

        if let Err(e) = file.write_all(&buffer[..n]) {
            drop(file);
            let _ = std::fs::remove_file(&dest_path);
            let message = format!("failed writing to {}: {e}", dest_path.display());
            return Err(failed(step, message, on_event));
        }

        done += n as u64;
        if done.saturating_sub(last_progress) >= PROGRESS_STEP {
            on_event(InstallEvent::Progress { step, done, total });
            last_progress = done;
        }
    }

    if done != last_progress {
        on_event(InstallEvent::Progress { step, done, total });
    }

    drop(file);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        if let Err(e) = std::fs::set_permissions(&dest_path, perms) {
            let message = format!("failed to set permissions on {}: {e}", dest_path.display());
            return Err(failed(step, message, on_event));
        }
    }

    execute_command(step, &dest_path, args, cancel, on_event)
}

fn run_command_step(
    step: usize,
    program: &'static str,
    args: &[&'static str],
    cancel: &AtomicBool,
    on_event: &mut dyn FnMut(InstallEvent),
) -> Result<(), InstallError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(InstallError::Cancelled);
    }

    let title = if args.is_empty() {
        program.to_string()
    } else {
        format!("{program} {}", args.join(" "))
    };
    on_event(InstallEvent::Started { step, title });

    let resolved = resolve_program(program);
    execute_command(step, &resolved, args, cancel, on_event)
}

/// Executes an installation plan sequentially, reporting progress events.
pub fn run_plan(
    plan: &[InstallStep],
    work_dir: &Path,
    cancel: &AtomicBool,
    on_event: &mut dyn FnMut(InstallEvent),
) -> Result<(), InstallError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(InstallError::Cancelled);
    }

    for (step, item) in plan.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err(InstallError::Cancelled);
        }

        match item {
            InstallStep::DownloadAndRun {
                url,
                file_name,
                args,
            } => {
                run_download_and_run(step, url, file_name, args, work_dir, cancel, on_event)?;
            }
            InstallStep::Run { program, args } => {
                run_command_step(step, program, args, cancel, on_event)?;
            }
            InstallStep::OpenUrl { url } => {
                let title = format!("Open URL: {url}");
                on_event(InstallEvent::Started { step, title });
                on_event(InstallEvent::Finished {
                    step,
                    ok: false,
                    message: (*url).to_string(),
                });
            }
            InstallStep::CopyCommand { command } => {
                let title = format!("Command: {command}");
                on_event(InstallEvent::Started { step, title });
                on_event(InstallEvent::Finished {
                    step,
                    ok: false,
                    message: (*command).to_string(),
                });
            }
            InstallStep::Manual { tool, how } => {
                let title = format!("Manual: {}", tool.title());
                on_event(InstallEvent::Started { step, title });
                on_event(InstallEvent::Finished {
                    step,
                    ok: false,
                    message: (*how).to_string(),
                });
            }
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/environment_install/test.rs"]
mod test;
