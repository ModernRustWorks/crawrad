//! **WHAT `cargo` RESPONDED**, as data — without UI.

use std::path::{Path, PathBuf};

use crawrad_note::note;
use serde::Deserialize;

// ------------------------------------------------------------------ the format

#[note("diagnostics-model-cargoline")]
#[derive(Deserialize)]
struct CargoLine {
    reason: String,
    message: Option<CargoMessage>,
}

#[derive(Deserialize)]
struct CargoMessage {
    level: String,
    message: String,
    rendered: Option<String>,
    #[serde(default)]
    spans: Vec<CargoSpan>,
}

#[derive(Deserialize)]
struct CargoSpan {
    file_name: String,
    line_start: usize,
    column_start: usize,
    is_primary: bool,
}

// --------------------------------------------------------------- the diagnostic

#[note("diagnostics-model-severity")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Comes first in list: what prevents running.
    Error,
    Warning,
}

impl Severity {
    #[note("diagnostics-model-severity-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }

    fn from_level(level: &str) -> Option<Self> {
        match level {
            "error" => Some(Self::Error),
            "warning" => Some(Self::Warning),
            _ => None,
        }
    }
}

#[note("diagnostics-model-diagnosticlocation")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLocation {
    pub file: PathBuf,
    /// Base 1, as cargo gives.
    pub line: usize,
    /// Base 1, in characters.
    pub column: usize,
}

#[note("diagnostics-model-diagnostic")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Option<DiagnosticLocation>,
    pub rendered: String,
}

#[note("diagnostics-model-marks-in")]
pub fn marks_in(diags: &[Diagnostic], name: &str) -> Vec<(usize, usize, bool)> {
    diags
        .iter()
        .filter_map(|d| {
            let at = d.location.as_ref()?;
            let named = at.file.file_name().and_then(|n| n.to_str()) == Some(name);
            let error = d.severity == Severity::Error;
            named.then(|| {
                (
                    at.line.saturating_sub(1),
                    at.column.saturating_sub(1),
                    error,
                )
            })
        })
        .collect()
}

#[note("diagnostics-model-parse-diagnostics")]
pub fn parse_diagnostics(output: &str) -> Vec<Diagnostic> {
    let mut found: Vec<Diagnostic> = output.lines().filter_map(line_diagnostic).collect();
    found.sort_by_key(|d| d.severity);
    found
}

fn line_diagnostic(line: &str) -> Option<Diagnostic> {
    let raw: CargoLine = serde_json::from_str(line).ok()?;
    if raw.reason != "compiler-message" {
        return None;
    }
    let m = raw.message?;
    let severity = Severity::from_level(&m.level)?;
    Some(Diagnostic {
        severity,
        message: plain_note_warning(m.message),
        location: location_from_span(&m.spans),
        rendered: m.rendered.unwrap_or_default(),
    })
}

/// The doc-note checks warn through a deprecated marker constant; show what
/// they mean instead of rustc's "use of deprecated constant `…`".
fn plain_note_warning(message: String) -> String {
    const MARKERS: [(&str, &str); 2] = [
        (
            "NOTE_NOT_RUSTDOC_STYLE`: ",
            "Doc note is not in rustdoc style: ",
        ),
        ("NOTE_SIGNATURE_CHANGED`: ", "Note is out of date: "),
    ];
    MARKERS
        .iter()
        .find_map(|(marker, plain)| {
            let at = message.find(marker)?;
            Some(format!("{plain}{}", &message[at + marker.len()..]))
        })
        .unwrap_or(message)
}

#[note("diagnostics-model-location-from-span")]
fn location_from_span(spans: &[CargoSpan]) -> Option<DiagnosticLocation> {
    let s = spans.iter().find(|s| s.is_primary)?;
    Some(DiagnosticLocation {
        file: PathBuf::from(&s.file_name),
        line: s.line_start,
        column: s.column_start,
    })
}

// ------------------------------------------------------------------- destination

#[note("diagnostics-model-destination")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination {
    /// `<form>.rs` — user unit. Code tab, cursor on line.
    Unit { line: usize, column: usize },
    /// `<form>.designer.rs` — generated source.
    FormSource { line: usize, column: usize },
    /// Other member file.
    InMember { relative: PathBuf, line: usize },
    /// Outside member.
    Outside { path: PathBuf },
}

pub fn destination(
    location: &DiagnosticLocation,
    workspace_root: &Path,
    member: &Path,
    unit: Option<&Path>,
    designer: Option<&Path>,
) -> Destination {
    let full = workspace_root.join(&location.file);

    if unit.is_some_and(|u| same_file(&full, u)) {
        return Destination::Unit {
            line: location.line,
            column: location.column,
        };
    }
    if designer.is_some_and(|d| same_file(&full, d)) {
        return Destination::FormSource {
            line: location.line,
            column: location.column,
        };
    }
    match relative_to(&full, member) {
        Some(relative) => Destination::InMember {
            relative,
            line: location.line,
        },
        None => Destination::Outside { path: full },
    }
}

fn suffix_after(full: &Path, base: &Path) -> Option<PathBuf> {
    let mut c = full.components();
    for b in base.components() {
        let current = c.next()?;
        let (x, y) = (
            current.as_os_str().to_string_lossy(),
            b.as_os_str().to_string_lossy(),
        );
        let equal = if cfg!(windows) {
            x.eq_ignore_ascii_case(&y)
        } else {
            x == y
        };
        if !equal {
            return None;
        }
    }
    Some(c.collect())
}

fn same_file(a: &Path, b: &Path) -> bool {
    suffix_after(a, b).is_some_and(|r| r.as_os_str().is_empty())
}

fn relative_to(full: &Path, base: &Path) -> Option<PathBuf> {
    suffix_after(full, base).filter(|r| !r.as_os_str().is_empty())
}

// ------------------------------------------------------------------ artifacts

#[note("diagnostics-model-compiledartifact")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledArtifact {
    pub package_name: String,
    pub kind: Vec<String>,
    pub manifest_path: Option<PathBuf>,
    pub executable: Option<PathBuf>,
    pub filenames: Vec<PathBuf>,
}

#[derive(Deserialize)]
struct CargoArtifactLine {
    reason: String,
    manifest_path: Option<PathBuf>,
    target: Option<CargoArtifactTarget>,
    executable: Option<PathBuf>,
    #[serde(default)]
    filenames: Vec<PathBuf>,
}

#[derive(Deserialize)]
struct CargoArtifactTarget {
    name: String,
    #[serde(default)]
    kind: Vec<String>,
}

#[note("diagnostics-model-extract-artifacts")]
pub fn extract_artifacts(output: &str) -> Vec<CompiledArtifact> {
    let mut artifacts = Vec::new();
    for line in output.lines() {
        let l = line.trim();
        if l.starts_with('{')
            && l.ends_with('}')
            && let Ok(art) = serde_json::from_str::<CargoArtifactLine>(l)
            && art.reason == "compiler-artifact"
        {
            let (name, kind) = match art.target.as_ref() {
                Some(t) => (t.name.clone(), t.kind.clone()),
                None => (String::new(), Vec::new()),
            };
            artifacts.push(CompiledArtifact {
                package_name: name,
                kind,
                manifest_path: art.manifest_path,
                executable: art.executable,
                filenames: art.filenames,
            });
        }
    }
    artifacts
}

#[note("diagnostics-model-format-file-size")]
pub fn format_file_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

// -------------------------------------------------------------------- summary

#[note("diagnostics-model-summary")]
pub fn summary(diags: &[Diagnostic]) -> String {
    let count = |s: Severity| diags.iter().filter(|d| d.severity == s).count();
    let plural = |n: usize, name: &str| format!("{n} {name}{}", if n == 1 { "" } else { "s" });
    let mut parts = Vec::new();
    let errors = count(Severity::Error);
    if errors > 0 {
        parts.push(plural(errors, "error"));
    }
    let warnings = count(Severity::Warning);
    if warnings > 0 {
        parts.push(plural(warnings, "warning"));
    }
    parts.join("  ·  ")
}

/// Errors and warnings: the LSP's totals when it has them, else the build's
/// diagnostics, else `unmeasured` for both.
pub fn count_problems_or_diagnostics(
    lsp_totals: Option<(usize, usize)>,
    diags: Option<&[Diagnostic]>,
    unmeasured: &str,
) -> (String, String) {
    if let Some((e, w)) = lsp_totals {
        return (e.to_string(), w.to_string());
    }
    let Some(diags) = diags else {
        return (unmeasured.to_string(), unmeasured.to_string());
    };
    let errors = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let warnings = diags
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    (errors.to_string(), warnings.to_string())
}

#[cfg(test)]
#[path = "test/diagnostics_model/test.rs"]
mod tests;
