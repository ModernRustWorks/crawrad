//! The Problems panel: the last build's errors and warnings, one flat list.
//!
//! Errors first, then warnings, each in file and line order. A filter chip
//! narrows the list, and the `Output` chip swaps it for the build's full log.
//! When the app Run started panics, its panics lead the list: the
//! message, the spot, and the project's own backtrace frames.
//! Pure model in the domain crate; the shell only draws the rows.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::navigation::Place;
use crawrad_project::panic_report::{PanicSite, SourceSpot};

pub use crawrad_project::diagnostics_model::{Diagnostic, DiagnosticLocation, Severity};

/// What the list shows: every problem, one severity, or the build's log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Filter {
    #[default]
    All,
    Errors,
    Warnings,
    Panics,
    Output,
}

/// One row of the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// A diagnostic: `index` is its position in [`Problems::items`].
    Problem {
        index: usize,
        severity: Severity,
        message: String,
        place: String,
        code: String,
    },
    /// A panic of the app: `site` is its position in [`Problems::panics`].
    Panic {
        site: usize,
        message: String,
        place: String,
        meta: String,
    },
    /// A quiet line between a panic's rows (`Called from (your code)`).
    Heading { text: String },
    /// One of the project's backtrace frames under a panic.
    Frame {
        site: usize,
        frame: usize,
        function: String,
        place: String,
    },
    /// A line of the build's full output.
    Output { line: String },
}

impl Row {
    /// The icon the row wears: `Some(true)` the error glyph, `Some(false)` the
    /// warning triangle (a panic wears it in red), `None` no icon.
    pub fn error_icon(&self) -> Option<bool> {
        match self {
            Self::Problem { severity, .. } => Some(*severity == Severity::Error),
            Self::Panic { .. } => Some(false),
            _ => None,
        }
    }

    /// Each label of the row's template, by its path, and the text it shows.
    pub fn labels(&self) -> Vec<(&'static str, String)> {
        match self.clone() {
            Self::Problem {
                message,
                place,
                code,
                ..
            } => vec![
                ("face.message", message),
                ("face.place", place),
                ("face.code", code),
            ],
            Self::Panic {
                message,
                place,
                meta,
                ..
            } => vec![
                ("face.text.head.message", message),
                ("face.text.head.place", place),
                ("face.text.meta", meta),
            ],
            Self::Frame {
                function, place, ..
            } => vec![("face.function", function), ("face.place", place)],
            Self::Heading { text } | Self::Output { line: text } => vec![("line", text)],
        }
    }

    /// The list template that draws this row.
    pub fn template(&self) -> &'static str {
        match self {
            Self::Problem {
                severity: Severity::Error,
                ..
            } => "ProblemError",
            Self::Problem { .. } => "ProblemWarning",
            Self::Panic { .. } => "ProblemPanic",
            Self::Heading { .. } => "ProblemHeading",
            Self::Frame { .. } => "ProblemFrame",
            Self::Output { .. } => "ProblemOutput",
        }
    }
}

/// The panel's state: the diagnostics, the filter, why the run stopped, and the log.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Problems {
    /// Errors first, then warnings, by file and line.
    pub items: Vec<Diagnostic>,
    pub filter: Filter,
    /// Why the last build stopped (`Run stopped: the build failed — …`), or empty.
    pub reason: String,
    /// The panics of the app the last Run started, newest run only.
    pub panics: Vec<PanicSite>,
    /// The last build's full output, shown by the `Output` chip.
    pub log: Vec<String>,
    /// The folder cargo's relative paths start from.
    pub root: Option<PathBuf>,
    /// The rows the list drew, with their rectangles, for MCP.
    pub rects: Vec<(usize, [f64; 4])>,
}

impl Problems {
    /// Takes a build's diagnostics; `root` is the folder their paths are relative to.
    pub fn populate(&mut self, diagnostics: &[Diagnostic], root: Option<&Path>) {
        let mut items = diagnostics.to_vec();
        items.sort_by_key(|d| {
            let at = d.location.as_ref();
            (
                d.severity,
                at.map(|l| l.file.clone()),
                at.map(|l| (l.line, l.column)),
            )
        });
        self.items = items;
        self.panics.clear();
        if self.filter == Filter::Panics {
            self.filter = Filter::All;
        }
        self.root = root.map(Path::to_path_buf);
    }

    /// The app ended in `panics`: they lead the list, the `Full output` chip
    /// shows its stderr, and the reason says the Run stopped.
    pub fn show_panics(&mut self, panics: Vec<PanicSite>, stderr: &str, root: &Path) {
        self.panics = panics;
        self.log = stderr.lines().map(str::to_string).collect();
        self.root = Some(root.to_path_buf());
        self.filter = Filter::Panics;
        self.reason = "The app panicked \u{2014} Run stopped".to_string();
    }

    fn count(&self, severity: Severity) -> usize {
        self.items.iter().filter(|d| d.severity == severity).count()
    }

    pub fn error_count(&self) -> usize {
        self.count(Severity::Error)
    }

    pub fn warning_count(&self) -> usize {
        self.count(Severity::Warning)
    }

    /// `1 error, 2 warnings`, or `No problems`.
    pub fn summary(&self) -> String {
        let plural = |n: usize, word: &str| match n {
            1 => format!("1 {word}"),
            _ => format!("{n} {word}s"),
        };
        match (self.error_count(), self.warning_count()) {
            (0, 0) => "No problems".to_string(),
            (e, 0) => plural(e, "error"),
            (0, w) => plural(w, "warning"),
            (e, w) => format!("{}, {}", plural(e, "error"), plural(w, "warning")),
        }
    }

    /// The line that says why a failed build stopped, as the mockup words it.
    pub fn stop_text(&self, run: bool, elapsed: Duration) -> String {
        let what = match run {
            true => "Run stopped: the build failed",
            false => "Build failed",
        };
        let seconds = elapsed.as_secs_f64();
        format!("{what} \u{2014} {} \u{00b7} {seconds:.1} s", self.summary())
    }

    /// A filter chip's text (`Errors 2`) and whether it is the active filter.
    pub fn chip(&self, filter: Filter) -> (String, bool) {
        let text = match filter {
            Filter::All => format!("All {}", self.items.len()),
            Filter::Errors => format!("Errors {}", self.error_count()),
            Filter::Warnings => format!("Warnings {}", self.warning_count()),
            Filter::Panics => format!("Panics {}", self.panics.len()),
            Filter::Output => "Full output".to_string(),
        };
        (text, self.filter == filter)
    }

    /// Picks a filter; picking the active one again goes back to all.
    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = match self.filter == filter {
            true => Filter::All,
            false => filter,
        };
    }

    pub fn rows(&self) -> Vec<Row> {
        if self.filter == Filter::Output {
            let line = |l: &String| Row::Output { line: l.clone() };
            return self.log.iter().map(line).collect();
        }
        let shown = |d: &Diagnostic| match self.filter {
            Filter::Errors => d.severity == Severity::Error,
            Filter::Warnings => d.severity == Severity::Warning,
            _ => true,
        };
        let row = |(index, d): (usize, &Diagnostic)| Row::Problem {
            index,
            severity: d.severity,
            message: detail(d),
            place: d.location.as_ref().map(place_text).unwrap_or_default(),
            code: code_of(d),
        };
        let mut rows = match self.filter {
            Filter::All | Filter::Panics => self.panic_rows(),
            _ => Vec::new(),
        };
        if self.filter == Filter::Panics {
            return rows;
        }
        let items = self.items.iter().enumerate().filter(|(_, d)| shown(d));
        rows.extend(items.map(row));
        rows
    }

    /// Each panic, then the project's frames that led to it, as the mockup draws them.
    fn panic_rows(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for (site, p) in self.panics.iter().enumerate() {
            let caller = p.frames.first().map(|f| short_function(&f.function));
            rows.push(Row::Panic {
                site,
                message: p.message.lines().next().unwrap_or_default().to_string(),
                place: spot_text(&p.spot),
                meta: match caller {
                    Some(f) => format!("thread '{}' \u{00b7} {f}", p.thread),
                    None => format!("thread '{}'", p.thread),
                },
            });
            if p.frames.is_empty() {
                continue;
            }
            let heading = |text: &str| Row::Heading { text: text.into() };
            rows.push(heading("CALLED FROM (YOUR CODE)"));
            rows.extend(p.frames.iter().enumerate().map(|(frame, f)| Row::Frame {
                site,
                frame,
                function: f.function.clone(),
                place: spot_text(&f.spot),
            }));
            rows.push(heading(
                "Library and toolchain frames hidden \u{2014} see Full output.",
            ));
        }
        rows
    }

    /// Where a click on `row` goes, if anywhere.
    pub fn row_place(&self, row: &Row) -> Option<Place> {
        match *row {
            Row::Problem { index, .. } => self.place(index),
            Row::Panic { site, .. } => self.panic_place(site, None),
            Row::Frame { site, frame, .. } => self.panic_place(site, Some(frame)),
            _ => None,
        }
    }

    /// The marks the panics leave on `file`: 0-based line and byte, and the
    /// message the editor writes at the end of that line.
    pub fn panic_marks(&self, file: &Path) -> Vec<(usize, usize, String)> {
        (0..self.panics.len())
            .filter_map(|site| {
                let at = self.panic_place(site, None)?;
                let first = self.panics[site].message.lines().next().unwrap_or_default();
                (at.file == file).then(|| (at.line, at.byte, format!("panicked: {first}")))
            })
            .collect()
    }

    /// Where a click on a panic (`frame: None`) or one of its frames goes.
    pub fn panic_place(&self, site: usize, frame: Option<usize>) -> Option<Place> {
        let p = self.panics.get(site)?;
        let spot = match frame {
            Some(i) => &p.frames.get(i)?.spot,
            None => &p.spot,
        };
        let root = self.root.as_deref().unwrap_or(Path::new("."));
        Some(Place {
            file: spot.resolve(root, root)?,
            line: (spot.line as usize).saturating_sub(1),
            byte: (spot.column as usize).saturating_sub(1),
        })
    }

    /// Where a click on the problem `index` goes: 0-based line and column.
    pub fn place(&self, index: usize) -> Option<Place> {
        let at = self.items.get(index)?.location.as_ref()?;
        let file = match &self.root {
            Some(root) if at.file.is_relative() => root.join(&at.file),
            _ => at.file.clone(),
        };
        Some(Place {
            file,
            line: at.line.saturating_sub(1),
            byte: at.column.saturating_sub(1),
        })
    }
}

/// A panic's spot as the row shows it, with forward slashes.
fn spot_text(spot: &SourceSpot) -> String {
    let label = spot.label().replace('\\', "/");
    label
        .strip_prefix("./")
        .map(str::to_string)
        .unwrap_or(label)
}

/// `FormMain::divide` from `calc::form_main::FormMain::divide`.
fn short_function(path: &str) -> String {
    let parts: Vec<&str> = path.split("::").collect();
    parts[parts.len().saturating_sub(2)..].join("::")
}

/// `src/main.rs:8:20`, with forward slashes on every system.
fn place_text(at: &DiagnosticLocation) -> String {
    let file = at.file.to_string_lossy().replace('\\', "/");
    format!("{file}:{}:{}", at.line, at.column)
}

/// The message, and for a short one the primary label that explains it:
/// `mismatched types: expected i32, found &str`. Backticks go: the row is prose.
fn detail(d: &Diagnostic) -> String {
    let message = d.message.lines().next().unwrap_or("").trim();
    let label = primary_label(&d.rendered);
    let label = label.filter(|l| !message.contains(':') && !l.starts_with("help"));
    let text = match label {
        Some(l) => format!("{message}: {l}"),
        None => message.to_string(),
    };
    text.replace('`', "")
}

/// The text after the `^^^` marks in cargo's rendered message.
fn primary_label(rendered: &str) -> Option<String> {
    rendered.lines().find_map(|l| {
        let after = l.split_once('|')?.1;
        let at = after.rfind('^')?;
        let text = after[at + 1..].trim();
        (!text.is_empty()).then(|| text.to_string())
    })
}

/// `E0308` from `error[E0308]`, or the lint name from `#[warn(unused_variables)]`.
fn code_of(d: &Diagnostic) -> String {
    let first = d.rendered.lines().next().unwrap_or("");
    if let Some((level, rest)) = first.split_once('[')
        && matches!(level, "error" | "warning")
        && let Some((code, _)) = rest.split_once(']')
    {
        return code.to_string();
    }
    let lint = d.rendered.split_once("#[warn(");
    let lint = lint.and_then(|(_, r)| r.split_once(")]"));
    lint.map(|(name, _)| name.to_string()).unwrap_or_default()
}

#[cfg(test)]
#[path = "test/problems/test.rs"]
mod tests;
