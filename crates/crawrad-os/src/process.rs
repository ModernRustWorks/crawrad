//! Child processes: the one place in the product that starts one.
#![forbid(unsafe_code)]
//!
//! # Why this module exists
//!
//! Launching `cargo` used to live inside `main.rs`, tangled with the `egui`
//! shell. That made it unreachable for the Makepad shell, which cannot build
//! or run anything today for no better reason than "the launcher is in the
//! other file". Nothing here knows about a framework: no `egui`, no `makepad`,
//! not even a window. It takes a description of a call, starts it, and hands
//! back what came out.
//!
//! # The defect this module refuses to inherit
//!
//! *"the Build panel swallows what cargo said and shows `nothing to
//! show`"*. The cause was not a missing feature, it was a discard.
//! `Command::output()` already holds **both** streams whole in memory, and the
//! old code kept a single line of `stderr` — the first one starting with
//! `error` — and dropped the rest on the floor. When cargo failed without that
//! word (a broken manifest, a dependency that will not resolve, a linker that
//! is not installed) the shell had nothing left to say.
//!
//! So [`Outcome`] carries `stdout` **and** `stderr` whole, and
//! [`Outcome::report`] never returns `None` when the child wrote anything at
//! all. Keeping the raw text costs the six lines the struct takes; throwing it
//! away cost an issue.
//!
//! # Progress without a framework
//!
//! A build takes minutes, and a shell that blocks on it is a frozen shell. Background
//! tasks run on a thread and wake the shell via `crawrad_os::tasks`.
//!
//! # What must never come back here
//!
//! **`cargo run` does not belong on the build thread.** It was there once, and
//! the call only returns when the user's application *closes* — so the status
//! bar kept saying `building…` with the app already on screen. Building waits;
//! launching detaches ([`detach`]).

use crawrad_note::note;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

#[note("process-no-console")]
pub fn no_console(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let cmd = Command::new(program);
    #[cfg(windows)]
    let cmd = {
        use std::os::windows::process::CommandExt;
        let mut cmd = cmd;
        cmd.creation_flags(0x0800_0000);
        cmd
    };
    cmd
}

#[note("process-cargocall")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoCall {
    args: Vec<String>,
    dir: PathBuf,
}

impl CargoCall {
    #[note("process-cargocall-new")]
    pub fn new<S: Into<String>>(
        args: impl IntoIterator<Item = S>,
        dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            args: args.into_iter().map(Into::into).collect(),
            dir: dir.into(),
        }
    }

    #[note("process-cargocall-build")]
    pub fn build(package: &str, dir: impl Into<PathBuf>) -> Self {
        Self::new(["build", "-p", package, "--message-format=json"], dir)
    }

    #[note("process-cargocall-check")]
    pub fn check(package: &str, dir: impl Into<PathBuf>) -> Self {
        Self::new(["check", "-p", package, "--message-format=json"], dir)
    }

    #[note("process-cargocall-clean")]
    pub fn clean(package: &str, dir: impl Into<PathBuf>) -> Self {
        Self::new(["clean", "-p", package], dir)
    }

    #[note("process-cargocall-run")]
    pub fn run(package: &str, dir: impl Into<PathBuf>) -> Self {
        Self::new(["run", "-p", package], dir)
    }

    #[note("process-cargocall-metadata")]
    pub fn metadata(dir: impl Into<PathBuf>) -> Self {
        Self::new(["metadata", "--format-version", "1", "--no-deps"], dir)
    }

    #[note("process-cargocall-command")]
    pub fn command(&self) -> Command {
        let mut cmd = no_console("cargo");
        cmd.args(&self.args)
            // The user's project lives outside ours; the variables of the cargo
            // that launched the IDE would confuse theirs.
            .env_remove("CARGO_TARGET_DIR")
            .env_remove("RUSTFLAGS")
            // **The output here is READ, not displayed** — so cargo is asked not
            // to paint it. Without this line, whoever exported
            // `CARGO_TERM_COLOR=always` gets ANSI escapes glued to every word:
            // the `Adding` of workspace adoption stops matching and someone
            // else's file is edited in silence, and the `error:` of an occupied
            // folder stops matching and the refusal loses its *"open it
            // instead"*. Both were measured on 2026-09-08 with cargo 1.95, and
            // the first one kept `main` red for four merges — CI sets that
            // variable so the GitHub log comes out coloured, and `always`
            // colours even when writing into a pipe.
            //
            // `env` and not `env_remove` on purpose: removing the variable hands
            // the decision back to cargo, which colours when it thinks it is
            // talking to a terminal. We do not want its guess here, we want
            // text.
            .env("CARGO_TERM_COLOR", "never")
            .current_dir(&self.dir);
        cmd
    }

    #[note("process-cargocall-progress-command")]
    pub fn progress_command(&self) -> Command {
        let mut cmd = self.command();
        cmd.env("CARGO_TERM_PROGRESS_WHEN", "always")
            .env("CARGO_TERM_PROGRESS_WIDTH", "200");
        cmd
    }
}

#[note("process-outcome")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Outcome {
    /// Did it exit with status zero?
    pub success: bool,
    /// Was it cancelled before completion?
    pub cancelled: bool,
    /// Standard output, untouched. For `--message-format=json` this is the
    /// diagnostics stream.
    pub stdout: String,
    /// Standard error, untouched. This is where cargo speaks for itself —
    /// `Creating`, `Adding`, `error: could not compile`, progress.
    pub stderr: String,
}

impl Outcome {
    #[note("process-outcome-merged")]
    pub fn merged(&self) -> String {
        format!("{}{}", self.stderr, self.stdout)
    }

    #[note("process-outcome-first-error-line")]
    pub fn first_error_line(&self) -> Option<&str> {
        self.stderr
            .lines()
            .map(str::trim_end)
            .find(|l| l.starts_with("error"))
    }

    #[note("process-outcome-report")]
    pub fn report(&self) -> Option<String> {
        if let Some(erro) = self.first_error_line() {
            return Some(erro.to_owned());
        }
        last_meaningful_line(&self.stderr)
            .or_else(|| last_meaningful_line(&self.stdout))
            .map(str::to_owned)
    }
}

#[note("process-nothing-to-show")]
pub const NOTHING_TO_SHOW: &str = "build failed, with nothing to show";

#[note("process-afterbuild")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AfterBuild {
    /// It compiled. The application is launched, [`detach`]ed.
    Launch,
    /// It did not, and this is the sentence for the status bar. Nothing is
    /// launched — running the binary from the previous build while the code on
    /// screen does not compile is the kind of lie that costs an hour.
    Report(String),
}

#[note("process-after-build")]
pub fn after_build(verdict: Result<Outcome, String>) -> AfterBuild {
    match verdict {
        Ok(o) if o.success => AfterBuild::Launch,
        Ok(o) => AfterBuild::Report(o.report().unwrap_or_else(|| NOTHING_TO_SHOW.to_owned())),
        Err(e) => AfterBuild::Report(e),
    }
}

#[note("process-last-meaningful-line")]
fn last_meaningful_line(text: &str) -> Option<&str> {
    text.lines().map(str::trim).rfind(|l| !l.is_empty())
}

#[note("process-wait")]
pub fn wait(cmd: &mut Command) -> Result<Outcome, String> {
    let raw = cmd
        .output()
        .map_err(|e| format!("could not run {}: {e}", cmd.get_program().to_string_lossy()))?;
    Ok(Outcome {
        success: raw.status.success(),
        cancelled: false,
        // `from_utf8_lossy`: a compiler that prints a path with an invalid byte
        // in it must not take the verdict down with it.
        stdout: String::from_utf8_lossy(&raw.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&raw.stderr).into_owned(),
    })
}

#[note("process-pipe")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Stdout,
    Stderr,
}

#[note("process-stream")]
pub fn stream(
    cmd: &mut Command,
    mut on_line: impl FnMut(Pipe, &str),
    cancelled: impl Fn() -> bool,
) -> Result<Outcome, String> {
    cmd.stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    // Its own process group, so Cancel can end the grandchildren too (rustc
    // under cargo, `sleep` under `sh`): they inherit the pipes, and while one
    // lives the readers below never see the end of them.
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(cmd, 0);

    let mut child = cmd
        .spawn()
        .map_err(|e| format!("could not run {}: {e}", cmd.get_program().to_string_lossy()))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "stdout pipe not available".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "stderr pipe not available".to_string())?;

    let (tx, rx) = std::sync::mpsc::channel();
    let tx_out = tx.clone();
    let handle_out = std::thread::spawn(move || {
        read_pipe(stdout, Pipe::Stdout, tx_out);
    });
    let tx_err = tx;
    let handle_err = std::thread::spawn(move || {
        read_pipe(stderr, Pipe::Stderr, tx_err);
    });

    let mut outcome = Outcome::default();
    let mut was_cancelled = false;

    let append_line = |outcome: &mut Outcome, pipe: Pipe, line: &str| match pipe {
        Pipe::Stdout => {
            outcome.stdout.push_str(line);
            outcome.stdout.push('\n');
        }
        Pipe::Stderr => {
            if !line.trim().starts_with("Building [") {
                outcome.stderr.push_str(line);
                outcome.stderr.push('\n');
            }
        }
    };

    loop {
        match rx.recv_timeout(std::time::Duration::from_millis(50)) {
            Ok((pipe, line)) => {
                on_line(pipe, &line);
                append_line(&mut outcome, pipe, &line);
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                break;
            }
        }

        if cancelled() {
            was_cancelled = true;
            kill_child(&mut child);
            break;
        }

        if let Ok(Some(status)) = child.try_wait() {
            outcome.success = status.success();
            break;
        }
    }

    let _ = handle_out.join();
    let _ = handle_err.join();

    while let Ok((pipe, line)) = rx.try_recv() {
        on_line(pipe, &line);
        append_line(&mut outcome, pipe, &line);
    }

    if let Ok(status) = child.wait()
        && !was_cancelled
    {
        outcome.success = status.success();
    }

    if was_cancelled {
        outcome.success = false;
        outcome.cancelled = true;
    }

    Ok(outcome)
}

fn read_pipe<R: Read>(mut reader: R, pipe: Pipe, tx: std::sync::mpsc::Sender<(Pipe, String)>) {
    let mut buf = [0u8; 1024];
    let mut accum = Vec::new();
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        for &byte in &buf[..n] {
            let is_delim = match pipe {
                Pipe::Stderr => byte == b'\r' || byte == b'\n',
                Pipe::Stdout => byte == b'\n',
            };
            if is_delim {
                let text = String::from_utf8_lossy(&accum);
                let fragment = text.strip_suffix('\r').unwrap_or(&text);
                if !fragment.is_empty() {
                    let _ = tx.send((pipe, fragment.to_string()));
                }
                accum.clear();
            } else {
                accum.push(byte);
            }
        }
    }
    if !accum.is_empty() {
        let text = String::from_utf8_lossy(&accum);
        let fragment = text.strip_suffix('\r').unwrap_or(&text);
        if !fragment.is_empty() {
            let _ = tx.send((pipe, fragment.to_string()));
        }
    }
}

fn kill_child(child: &mut std::process::Child) {
    #[cfg(windows)]
    {
        let pid = child.id();
        let _ = no_console("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .output();
    }
    // `stream` made the child a group leader; the negative pid names the group.
    // The shell's own `kill`: slim images ship no `/bin/kill`.
    #[cfg(unix)]
    {
        let script = format!("kill -9 -{}", child.id());
        let _ = Command::new("sh").args(["-c", &script]).output();
    }
    let _ = child.kill();
}

#[note("process-detach")]
pub fn detach(cmd: &mut Command) -> Result<(), String> {
    cmd.spawn().map(|_| ()).map_err(|e| {
        format!(
            "could not launch {}: {e}",
            cmd.get_program().to_string_lossy()
        )
    })
}

// `#[cfg(test)]` BEFORE `#[path]`, and the order is not taste. The
// `a_casca_ainda_muta_o_documento_por_fora_do_catalogo` guard over in
// `crawrad-actions` reads the test region of a file and looks for the `#[path]`
// *inside* it. With `#[path]` on top it sees a bare `mod tests;`, goes looking
// for `launcher/tests.rs`, does not find it, and accuses the whole test file of
// being production code. Measured on this very round, the other way round.
#[cfg(test)]
#[path = "test/process/test.rs"]
mod tests;
