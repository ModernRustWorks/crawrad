//! The Preview tab and the Test example button of the Note dialog,
//! headless.
//!
//! The preview is the markdown `cargo doc` would show for the item: its
//! signature on top, a rule, then the note body. The test runs the item's
//! doctests through `cargo test --doc` and reads cargo's answer.

use crawrad_note::note;
#[note("note-preview-preview-markdown")]
pub fn preview_markdown(signature: &str, body: &str) -> String {
    let mut out = String::new();
    if !signature.trim().is_empty() {
        out.push_str("```rust\n");
        out.push_str(signature.trim());
        out.push_str("\n```\n\n---\n\n");
    }
    out.push_str(&demote_headings(&unlink_intra_doc(body)));
    out
}

#[note("note-preview-demote-headings")]
pub fn demote_headings(body: &str) -> String {
    let mut fenced = false;
    let mut out = String::with_capacity(body.len() + 16);
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            fenced = !fenced;
        } else if !fenced && trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|c| *c == '#').count();
            if level <= 3 && trimmed[level..].starts_with([' ', '\t']) {
                out.push_str("###");
            }
        }
        out.push_str(line);
    }
    out
}

#[note("note-preview-note-at-definition")]
pub fn note_at_definition(source: &str, line: usize) -> Option<String> {
    crate::doc_note::doc_target_at(source, line)?.existing_note
}

#[note("note-preview-unlink-intra-doc")]
fn unlink_intra_doc(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut rest = body;
    while let Some(start) = rest.find("[`") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let closed = after.find("`]").filter(|end| !after[..*end].contains('\n'));
        match closed {
            Some(end) => {
                let tail = &after[end + 2..];
                if tail.starts_with('(') || tail.starts_with('[') {
                    out.push_str(&rest[start..start + 2 + end + 2]);
                } else {
                    out.push('`');
                    out.push_str(&after[..end]);
                    out.push('`');
                }
                rest = tail;
            }
            None => {
                out.push_str("[`");
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

#[note("note-preview-has-example")]
pub fn has_example(body: &str) -> bool {
    // Only an OPENING fence says what the block is; the closing one is bare.
    let mut in_block = false;
    for line in body.lines() {
        let Some(info) = line.trim_start().strip_prefix("```") else {
            continue;
        };
        if in_block {
            in_block = false;
            continue;
        }
        in_block = true;
        let info = info.trim();
        let runs = info.is_empty()
            || info.split(',').map(str::trim).all(|tag| {
                matches!(
                    tag,
                    "rust" | "should_panic" | "no_run" | "edition2021" | "edition2024"
                )
            });
        if runs {
            return true;
        }
    }
    false
}

#[note("note-preview-package-name")]
pub fn package_name(cargo_toml: &str) -> Option<String> {
    let mut in_package = false;
    for line in cargo_toml.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        let Some(value) = line.strip_prefix("name") else {
            continue;
        };
        let Some(value) = value.trim_start().strip_prefix('=') else {
            continue;
        };
        let value = value.trim().trim_matches('"');
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

#[note("note-preview-doctestrun")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctestRun {
    pub package: String,
    /// `Owner::name`, or `name` for a free item.
    pub path: String,
}

impl DoctestRun {
    pub fn new(package: &str, owner: Option<&str>, name: &str) -> Self {
        let path = match owner {
            Some(owner) if !owner.is_empty() => format!("{owner}::{name}"),
            _ => name.to_string(),
        };
        Self {
            package: package.to_string(),
            path,
        }
    }

    #[note("note-preview-doctestrun-args")]
    pub fn args(&self) -> Vec<String> {
        vec![
            "test".into(),
            "--doc".into(),
            "-p".into(),
            self.package.clone(),
            "--".into(),
            self.path.clone(),
        ]
    }
}

#[note("note-preview-doctestoutcome")]
#[derive(Debug, Clone, PartialEq)]
pub enum DoctestOutcome {
    Passed {
        count: usize,
        secs: f64,
    },
    Failed {
        failed: usize,
        detail: String,
    },
    /// Cargo ran but no doctest of the item was among the tests.
    NoTest,
    /// Cargo did not get to run the tests (the crate does not build, …).
    Error(String),
}

impl DoctestOutcome {
    #[note("note-preview-doctestoutcome-label")]
    pub fn label(&self) -> String {
        match self {
            Self::Passed { count: 1, secs } => format!("Doctest passed · {secs:.1} s"),
            Self::Passed { count, secs } => format!("{count} doctests passed · {secs:.1} s"),
            Self::Failed { failed, detail } if detail.is_empty() => {
                format!("{failed} doctest(s) failed")
            }
            Self::Failed { failed, detail } => format!("{failed} doctest(s) failed: {detail}"),
            Self::NoTest => {
                "No doctest ran: save the note so the build sees the example".to_string()
            }
            Self::Error(e) => format!("Could not run the doctest: {e}"),
        }
    }

    pub fn passed(&self) -> bool {
        matches!(self, Self::Passed { .. })
    }
}

#[note("note-preview-is-doctest-of")]
fn is_doctest_of(name: &str, path: &str) -> bool {
    name.split_once(" - ")
        .and_then(|(_, rest)| rest.rsplit_once(" (line "))
        .is_some_and(|(item, _)| item == path)
}

#[note("note-preview-parse-doctest-output")]
pub fn parse_doctest_output(output: &str, run: &DoctestRun) -> DoctestOutcome {
    let Some(result) = output
        .lines()
        .rev()
        .find(|l| l.trim_start().starts_with("test result:"))
    else {
        let error = output
            .lines()
            .find(|l| l.trim_start().starts_with("error"))
            .unwrap_or("cargo stopped before running the tests")
            .trim();
        return DoctestOutcome::Error(error.to_string());
    };
    let (mut passed, mut failed) = (0, 0);
    for line in output.lines() {
        let Some(rest) = line.strip_prefix("test ") else {
            continue;
        };
        let Some((name, verdict)) = rest.rsplit_once(" ... ") else {
            continue;
        };
        if !is_doctest_of(name, &run.path) {
            continue;
        }
        match verdict.trim() {
            "ok" => passed += 1,
            "FAILED" => failed += 1,
            _ => {}
        }
    }
    if failed > 0 {
        // The first error or panic inside this item's own failure section.
        let header = output.lines().position(|l| {
            l.strip_prefix("---- ")
                .and_then(|l| l.strip_suffix(" stdout ----"))
                .is_some_and(|name| is_doctest_of(name, &run.path))
        });
        let detail = header
            .and_then(|at| {
                output.lines().skip(at + 1).find(|l| {
                    let l = l.trim_start();
                    l.starts_with("error") || l.contains("panicked at")
                })
            })
            .unwrap_or("")
            .trim()
            .to_string();
        return DoctestOutcome::Failed { failed, detail };
    }
    if passed == 0 {
        return DoctestOutcome::NoTest;
    }
    // The time is the whole run's: the neighbours the filter let in are in it.
    let secs = result
        .split("finished in")
        .nth(1)
        .and_then(|s| s.trim().trim_end_matches('s').trim().parse().ok())
        .unwrap_or(0.0);
    DoctestOutcome::Passed {
        count: passed,
        secs,
    }
}

#[note("note-preview-run-doctest")]
pub fn run_doctest(run: &DoctestRun, dir: &std::path::Path) -> DoctestOutcome {
    let call = crawrad_os::process::CargoCall::new(run.args(), dir);
    match crawrad_os::process::wait(&mut call.command()) {
        Ok(out) => parse_doctest_output(&out.merged(), run),
        Err(e) => DoctestOutcome::Error(e),
    }
}

#[note("note-preview-doctestrunner")]
#[derive(Default)]
pub struct DoctestRunner {
    tasks: crawrad_os::tasks::TaskManager<DoctestOutcome>,
}

impl DoctestRunner {
    pub fn is_running(&self) -> bool {
        self.tasks.is_running("doctest")
    }

    #[note("note-preview-doctestrunner-start")]
    pub fn start(&mut self, note: &mut crate::note_dialog::NoteDialogModel) {
        let (Some(run), Some(dir)) = (note.doctest_run(), note.crate_dir.clone()) else {
            return;
        };
        if self.is_running() {
            return;
        }
        note.testing();
        self.tasks.spawn("doctest", move |reporter| {
            reporter.done(run_doctest(&run, &dir));
        });
    }

    #[note("note-preview-doctestrunner-poll")]
    pub fn poll(&mut self, note: Option<&mut crate::note_dialog::NoteDialogModel>) -> bool {
        let events = self.tasks.drain();
        let Some(note) = note else {
            return false;
        };
        let mut answered = false;
        for (_, event) in events {
            let outcome = match event {
                crawrad_os::tasks::TaskEvent::Done(outcome) => outcome,
                crawrad_os::tasks::TaskEvent::Failed(e) => DoctestOutcome::Error(e),
                crawrad_os::tasks::TaskEvent::Progress(_) => continue,
            };
            note.tested(&outcome);
            answered = true;
        }
        answered
    }
}

#[cfg(test)]
#[path = "test/note_preview/test.rs"]
mod test;
