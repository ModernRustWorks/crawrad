//! Live compiler progress tracking for background builds.
//!
//! # Why this module exists
//!
//! When cargo runs in the background, the user needs continuous feedback on what
//! is happening: which crate is currently compiling, how many total crates are
//! in the build queue, how many are completed, elapsed time, and error/warning counts.
//! This model parses streamed lines from cargo (both JSON diagnostic messages on
//! stdout and progress bar fragments / human-readable notices on stderr) and
//! presents a clean, reactive snapshot of the active build state.

use crawrad_note::note;
use std::time::Duration;

#[note("build-progress-progressstate")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// The process is actively compiling.
    Running,
    /// Build finished with exit code zero and without cancellation.
    Succeeded,
    /// Build finished with a non-zero exit code or compilation error.
    Failed,
    /// Build was cancelled by the user.
    Cancelled,
}

#[note("build-progress-finish")]
#[derive(Debug, Clone, PartialEq)]
pub struct Finish {
    /// Whether the process exited with status zero.
    pub success: bool,
    /// Whether the process was killed due to user cancellation.
    pub cancelled: bool,
    /// Total number of compilation errors encountered.
    pub errors: usize,
    /// Total number of compilation warnings encountered.
    pub warnings: usize,
    /// Total duration elapsed from start to completion.
    pub elapsed: Duration,
}

#[note("build-progress-buildprogress")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildProgress {
    /// The verb being run: "Building", "Checking", "Cleaning", "Running".
    verb: String,
    /// The package the user asked for.
    package: String,
    /// Last `Compiling foo v1.2.3` (or `Checking foo v1.2.3`) seen: "foo v1.2.3".
    current: Option<String>,
    /// The action verb associated with `current`: "Compiling" or "Checking".
    current_action: Option<String>,
    done: usize,
    total: Option<usize>,
    /// Lines that belong in the Output console (not progress-bar fragments,
    /// not JSON).
    log: Vec<String>,
    finished: Option<Finish>,
}

impl BuildProgress {
    #[note("build-progress-buildprogress-new")]
    pub fn new(verb: &str, package: &str, command_line: &str) -> Self {
        Self {
            verb: verb.to_string(),
            package: package.to_string(),
            current: None,
            current_action: None,
            done: 0,
            total: None,
            log: vec![format!("> {command_line}")],
            finished: None,
        }
    }

    #[note("build-progress-buildprogress-feed")]
    pub fn feed(&mut self, pipe: crawrad_os::process::Pipe, line: &str) {
        match pipe {
            crawrad_os::process::Pipe::Stderr => {
                let trimmed = line.trim();
                if trimmed.starts_with("Building [") {
                    if let Some((_, after_bracket)) = trimmed.split_once(']') {
                        let after = after_bracket.trim_start();
                        let token = after.split([':', ' ']).next().unwrap_or("");
                        if let Some((n_str, m_str)) = token.split_once('/')
                            && let (Ok(n), Ok(m)) = (n_str.parse::<usize>(), m_str.parse::<usize>())
                        {
                            self.done = n;
                            self.total = Some(m);
                        }
                    }
                } else if let Some((action, rest, logged)) = [
                    ("Compiling", "Compiling ", true),
                    ("Checking", "Checking ", true),
                    ("Compiling", "Fresh ", false),
                ]
                .iter()
                .find_map(|&(action, prefix, logged)| {
                    trimmed
                        .strip_prefix(prefix)
                        .map(|rest| (action, rest, logged))
                }) {
                    let target = rest.split_once(" (").map_or(rest, |(before, _)| before);
                    self.current = Some(target.trim().to_string());
                    self.current_action = Some(action.to_string());
                    if logged {
                        self.log.push(line.trim_end().to_string());
                    }
                } else {
                    self.log.push(line.trim_end().to_string());
                }
            }
            crawrad_os::process::Pipe::Stdout => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(obj) = value.as_object()
                        && obj.get("reason").and_then(|r| r.as_str()) == Some("compiler-artifact")
                        && self.total.is_none()
                    {
                        self.done += 1;
                    }
                } else {
                    self.log.push(line.trim_end().to_string());
                }
            }
        }
    }

    #[note("build-progress-buildprogress-finish")]
    pub fn finish(&mut self, f: Finish) {
        self.finished = Some(f);
    }

    #[note("build-progress-buildprogress-is-running")]
    pub fn is_running(&self) -> bool {
        self.finished.is_none()
    }

    #[note("build-progress-buildprogress-fraction")]
    pub fn fraction(&self) -> Option<f64> {
        if let Some(ref fin) = self.finished
            && fin.success
        {
            return Some(1.0);
        }
        self.total.map(|total| {
            if total == 0 {
                1.0
            } else {
                ((self.done as f64) / (total as f64)).clamp(0.0, 1.0)
            }
        })
    }

    #[note("build-progress-buildprogress-headline")]
    pub fn headline(&self) -> String {
        if let Some(ref fin) = self.finished {
            if fin.cancelled {
                format!("{} {} cancelled", self.verb, self.package)
            } else if !fin.success {
                format!("{} {} failed", self.verb, self.package)
            } else {
                match self.verb.as_str() {
                    "Building" => format!("Built {}", self.package),
                    "Checking" => format!("Checked {}", self.package),
                    "Cleaning" => format!("Cleaned {}", self.package),
                    "Running" => format!("Ran {}", self.package),
                    other => format!("{other} {}", self.package),
                }
            }
        } else if let Some(ref curr) = self.current {
            let action = self.current_action.as_deref().unwrap_or("Compiling");
            format!("{} {} — {} {}", self.verb, self.package, action, curr)
        } else {
            format!("{} {}…", self.verb, self.package)
        }
    }

    #[note("build-progress-buildprogress-counter")]
    pub fn counter(&self) -> Option<String> {
        if let Some(ref fin) = self.finished {
            let count = self.total.unwrap_or(self.done);
            let err_part = if fin.errors == 1 {
                "1 error".to_string()
            } else {
                format!("{} errors", fin.errors)
            };
            let warn_part = if fin.warnings == 1 {
                "1 warning".to_string()
            } else {
                format!("{} warnings", fin.warnings)
            };
            Some(format!("{count} crates · {err_part} · {warn_part}"))
        } else {
            self.total
                .map(|total| format!("{} / {total} crates", self.done))
        }
    }

    #[note("build-progress-buildprogress-clock")]
    pub fn clock(elapsed: Duration) -> String {
        let secs = elapsed.as_secs();
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let rem_secs = secs % 60;
        if hours > 0 {
            format!("{hours}:{mins:02}:{rem_secs:02}")
        } else {
            format!("{mins:02}:{rem_secs:02}")
        }
    }

    #[note("build-progress-buildprogress-log")]
    pub fn log(&self) -> &[String] {
        &self.log
    }

    #[note("build-progress-buildprogress-append-log-line")]
    pub fn append_log_line(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }

    #[note("build-progress-buildprogress-append-log-lines")]
    pub fn append_log_lines<S: Into<String>>(&mut self, lines: impl IntoIterator<Item = S>) {
        for line in lines {
            self.log.push(line.into());
        }
    }

    #[note("build-progress-buildprogress-clear-log")]
    pub fn clear_log(&mut self) {
        self.log.clear();
    }

    #[note("build-progress-buildprogress-log-text")]
    pub fn log_text(&self) -> String {
        if self.log.is_empty() {
            return String::new();
        }
        let mut out = self.log.join("\n");
        out.push('\n');
        out
    }

    #[note("build-progress-buildprogress-status-line")]
    pub fn status_line(&self, elapsed: Duration) -> String {
        // a Run compiles before it runs: while cargo works, the bar says so
        let verb = match self.verb.as_str() {
            "Running" => "Building",
            other => other,
        };
        if let Some(ref fin) = self.finished {
            format!("{} · {}", self.headline(), Self::clock(fin.elapsed))
        } else if let Some(total) = self.total {
            format!(
                "{} {} · {} / {total} · {}",
                verb,
                self.package,
                self.done,
                Self::clock(elapsed)
            )
        } else {
            format!("{} {} · {}", verb, self.package, Self::clock(elapsed))
        }
    }

    #[note("build-progress-buildprogress-state")]
    pub fn state(&self) -> ProgressState {
        match self.finished {
            None => ProgressState::Running,
            Some(ref f) => {
                if f.cancelled {
                    ProgressState::Cancelled
                } else if f.success {
                    ProgressState::Succeeded
                } else {
                    ProgressState::Failed
                }
            }
        }
    }

    #[note("build-progress-buildprogress-verb")]
    pub fn verb(&self) -> &str {
        &self.verb
    }

    #[note("build-progress-buildprogress-package")]
    pub fn package(&self) -> &str {
        &self.package
    }

    #[note("build-progress-buildprogress-current")]
    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }

    pub fn current_action(&self) -> Option<&str> {
        self.current_action.as_deref()
    }

    #[note("build-progress-buildprogress-done")]
    pub fn done(&self) -> usize {
        self.done
    }

    #[note("build-progress-buildprogress-total")]
    pub fn total(&self) -> Option<usize> {
        self.total
    }

    #[note("build-progress-buildprogress-finished")]
    pub fn finished(&self) -> Option<&Finish> {
        self.finished.as_ref()
    }
}

#[note("build-progress-encode-line")]
pub fn encode_line(pipe: crawrad_os::process::Pipe, line: &str) -> String {
    match pipe {
        crawrad_os::process::Pipe::Stdout => format!("o {line}"),
        crawrad_os::process::Pipe::Stderr => format!("e {line}"),
    }
}

#[note("build-progress-decode-line")]
pub fn decode_line(text: &str) -> Option<(crawrad_os::process::Pipe, &str)> {
    if let Some(rest) = text.strip_prefix("o ") {
        Some((crawrad_os::process::Pipe::Stdout, rest))
    } else if let Some(rest) = text.strip_prefix("e ") {
        Some((crawrad_os::process::Pipe::Stderr, rest))
    } else {
        None
    }
}

#[cfg(test)]
#[path = "test/build_progress/test.rs"]
mod tests;
