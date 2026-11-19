//! A panic of the user's app, read from its stderr: where it happened, why, and
//! the backtrace frames that are the project's own code.
//!
//! Both message shapes are read — the current `thread 'main' panicked at
//! src/x.rs:12:5:` with the message on the next line, and the older
//! `panicked at 'msg', src/x.rs:12:5` — and a backtrace (`RUST_BACKTRACE=1`)
//! adds the frames whose `at` line points into the project.

use std::path::{Path, PathBuf};

/// A place in a source file, as the panic wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpot {
    /// The path as written: relative to the workspace root, or absolute.
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl SourceSpot {
    /// The file on disk: the path itself when absolute, else under `root`,
    /// else under the `member` — the first that exists.
    pub fn resolve(&self, root: &Path, member: &Path) -> Option<PathBuf> {
        let p = Path::new(&self.file);
        if p.is_absolute() {
            return p.is_file().then(|| p.to_path_buf());
        }
        [root.join(p), member.join(p)]
            .into_iter()
            .find(|c| c.is_file())
    }

    /// `file:line:column`, as the panel shows it.
    pub fn label(&self) -> String {
        format!("{}:{}:{}", self.file, self.line, self.column)
    }
}

/// One frame of the backtrace that is the project's code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    /// `crate::module::function`, as the backtrace names it.
    pub function: String,
    pub spot: SourceSpot,
}

/// One panic: the thread, the message, where, and the project's frames.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicSite {
    pub thread: String,
    pub message: String,
    pub spot: SourceSpot,
    pub frames: Vec<Frame>,
}

impl PanicSite {
    /// The status-bar line: `calc panicked: <first message line> — <spot>`.
    pub fn headline(&self, package: &str) -> String {
        let first = self.message.lines().next().unwrap_or_default();
        format!("{package} panicked: {first} — {}", self.spot.label())
    }
}

/// `path:line:column` at the end of `text`, if it ends with one. A backtrace
/// frame writes only `path:line`; its column is then 1.
fn spot_at_end(text: &str) -> Option<SourceSpot> {
    let text = text.trim().trim_end_matches(':');
    let (rest, last) = text.rsplit_once(':')?;
    let last: u32 = last.parse().ok()?;
    let (file, line, column) = match rest.rsplit_once(':') {
        Some((file, line)) if line.parse::<u32>().is_ok() => (file, line.parse().ok()?, last),
        _ => (rest, last, 1),
    };
    let file = file.trim().to_string();
    (!file.is_empty()).then_some(SourceSpot { file, line, column })
}

/// A file the designer rewrites: its frames are not the user's code.
fn is_generated(file: &str) -> bool {
    file.ends_with(".designer.rs")
}

/// A path the toolchain or a dependency owns, not the project.
fn is_outside(file: &str) -> bool {
    let f = file.replace('\\', "/");
    f.starts_with("/rustc/")
        || f.contains("/.cargo/registry/")
        || f.contains("/.cargo/git/")
        || f.contains("/.rustup/")
        || f.contains("/library/std/")
        || f.contains("/library/core/")
}

/// An absolute path outside `root`, or a toolchain path.
fn is_foreign(file: &str, root: &Path) -> bool {
    if is_outside(file) {
        return true;
    }
    let p = Path::new(file);
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/").to_lowercase();
    p.is_absolute() && !norm(p).starts_with(&norm(root))
}

/// The thread and the rest after `panicked at `, from a `thread '…' panicked at` line.
fn panic_line(line: &str) -> Option<(String, &str)> {
    let at = line.find("panicked at ")?;
    let thread = line[..at]
        .trim()
        .strip_prefix("thread '")
        .and_then(|t| t.split_once('\'').map(|(name, _)| name.to_string()))
        .unwrap_or_else(|| "main".to_string());
    Some((thread, &line[at + "panicked at ".len()..]))
}

/// Every panic in an app's stderr, in order.
pub fn find_panics(output: &str, root: &Path) -> Vec<PanicSite> {
    let lines: Vec<&str> = output.lines().collect();
    let mut found: Vec<PanicSite> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let Some((thread, rest)) = panic_line(lines[i]) else {
            if let Some(last) = found.last_mut() {
                i = read_frames(&lines, i, root, last);
            }
            i += 1;
            continue;
        };
        // Older shape: `panicked at 'message', src/x.rs:1:2`.
        let old = rest
            .strip_prefix('\'')
            .and_then(|r| r.rsplit_once("', "))
            .and_then(|(msg, s)| Some((msg.to_string(), spot_at_end(s)?)));
        let site = match old {
            Some((message, spot)) => Some((message, spot)),
            None => spot_at_end(rest).map(|spot| {
                let mut message = Vec::new();
                while let Some(next) = lines.get(i + 1) {
                    if next.starts_with("note: ")
                        || next.starts_with("stack backtrace:")
                        || next.is_empty()
                        || panic_line(next).is_some()
                    {
                        break;
                    }
                    message.push(next.trim_end());
                    i += 1;
                }
                (message.join("\n"), spot)
            }),
        };
        if let Some((message, spot)) = site {
            found.push(PanicSite {
                thread,
                message,
                spot,
                frames: Vec::new(),
            });
        }
        i += 1;
    }
    // A panic inside a callback that cannot unwind ends in a second panic in the
    // toolchain (`panic in a function that cannot unwind`): an echo, not news.
    if found.iter().any(|p| !is_foreign(&p.spot.file, root)) {
        found.retain(|p| !is_foreign(&p.spot.file, root));
    }
    found
}

/// Reads a backtrace frame at `i` (`  5: name` then `at path:l:c`) into `site`
/// when it is the project's; returns the last line it used.
fn read_frames(lines: &[&str], i: usize, root: &Path, site: &mut PanicSite) -> usize {
    let head = lines[i].trim_start();
    let Some((n, function)) = head.split_once(": ") else {
        return i;
    };
    if n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) {
        return i;
    }
    let Some(spot) = lines
        .get(i + 1)
        .and_then(|l| l.trim_start().strip_prefix("at "))
        .and_then(spot_at_end)
    else {
        return i;
    };
    if !is_foreign(&spot.file, root) && !is_generated(&spot.file) {
        site.frames.push(Frame {
            function: function.trim().to_string(),
            spot,
        });
    }
    i + 1
}

#[cfg(test)]
#[path = "test/panic_report/test.rs"]
mod panic_report_test;
