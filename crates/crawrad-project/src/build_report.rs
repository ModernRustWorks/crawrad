//! Build report formatting and summaries for background compiler invocations.
//!
//! Extracted from `crawrad-ide` to keep UI modules thin and logic pure.

use std::path::{Path, PathBuf};

use crate::build_settings::{BuildMode, BuildProfile, TargetPlatform};
use crate::diagnostics_model::{CompiledArtifact, Diagnostic, Severity, format_file_size};
use crawrad_note::note;

#[note("build-report-manifest-path")]
pub fn manifest_path(root: Option<&Path>, package: &str, artifacts: &[CompiledArtifact]) -> String {
    root.and_then(|r| {
        let m1 = r.join(package).join("Cargo.toml");
        if m1.exists() {
            Some(m1)
        } else {
            let m2 = r.join("Cargo.toml");
            if m2.exists() { Some(m2) } else { None }
        }
    })
    .or_else(|| {
        artifacts
            .iter()
            .find(|a| a.package_name == package)
            .and_then(|a| a.manifest_path.clone())
    })
    .map(|p| p.to_string_lossy().to_string())
    .unwrap_or_else(|| format!("{package}/Cargo.toml"))
}

#[note("build-report-platform-label")]
pub fn platform_label(targets: &[TargetPlatform]) -> String {
    if targets.is_empty() {
        #[cfg(target_os = "windows")]
        let p = if cfg!(target_arch = "x86_64") {
            "Windows 64-bit (x64)"
        } else {
            "Windows 32-bit (x86)"
        };
        #[cfg(target_os = "linux")]
        let p = "Linux 64-bit (x64)";
        #[cfg(target_os = "macos")]
        let p = "macOS";
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        let p = "Host default";
        p.to_string()
    } else {
        targets
            .iter()
            .map(|t| t.label())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[note("build-report-executables")]
pub fn executables(
    root: Option<&Path>,
    package: &str,
    profile: BuildProfile,
    targets: &[TargetPlatform],
    artifacts: &[CompiledArtifact],
) -> Vec<PathBuf> {
    let mut exes: Vec<PathBuf> = artifacts
        .iter()
        .filter(|a| a.package_name == package || artifacts.len() == 1)
        .filter_map(|a| a.executable.clone())
        .collect();

    if exes.is_empty()
        && let Some(raiz) = root
    {
        let prof_sub = match profile {
            BuildProfile::Release => "release",
            BuildProfile::Debug => "debug",
        };
        let cand_exe = format!("{package}.exe");
        if targets.is_empty() {
            let c1 = raiz.join("target").join(prof_sub).join(&cand_exe);
            let c2 = raiz.join("target").join(prof_sub).join(package);
            if c1.exists() {
                exes.push(c1);
            } else if c2.exists() {
                exes.push(c2);
            }
        } else {
            for t in targets {
                let c1 = raiz
                    .join("target")
                    .join(t.triple())
                    .join(prof_sub)
                    .join(&cand_exe);
                let c2 = raiz
                    .join("target")
                    .join(t.triple())
                    .join(prof_sub)
                    .join(package);
                if c1.exists() {
                    exes.push(c1);
                } else if c2.exists() {
                    exes.push(c2);
                }
            }
        }
    }
    exes
}

#[note("build-report-report")]
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    pub manifest: String,
    pub platform: String,
    pub configuration: String,
    pub executables: Vec<(PathBuf, Option<u64>)>,
    pub mode: BuildMode,
    pub package: String,
    pub success: bool,
    pub errors: usize,
    pub warnings: usize,
    pub error_lines: Vec<String>,
}

#[note("build-report-summary")]
pub fn summary(r: &Report) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Project:   {}", r.manifest));
    lines.push(format!(
        "Platform:  {}   |   Configuration: {}",
        r.platform, r.configuration
    ));

    if !r.executables.is_empty() {
        if r.executables.len() == 1 {
            let (ref exe, size) = r.executables[0];
            let size_str = size
                .map(|len| format!(" ({})", format_file_size(len)))
                .unwrap_or_default();
            lines.push(format!("Output:    {}{size_str}", exe.display()));
        } else {
            for (i, (exe, size)) in r.executables.iter().enumerate() {
                let size_str = size
                    .map(|len| format!(" ({})", format_file_size(len)))
                    .unwrap_or_default();
                lines.push(format!("Output ({}): {}{size_str}", i + 1, exe.display()));
            }
        }
    } else if r.mode == BuildMode::Check {
        lines.push("Output:    [Check only — syntax & types verified, no binary generated]".into());
    } else if r.mode == BuildMode::Clean {
        lines.push(format!(
            "Output:    [Cleaned build artifacts for {}]",
            r.package
        ));
    } else {
        lines.push(format!("Finished for {}", r.package));
    }

    let status_badge = if r.success && r.errors == 0 {
        "✔ Success  (Done.)"
    } else {
        "❌ Failed"
    };
    lines.push(format!("Status:    {status_badge}"));
    lines.push(format!(
        "Result:    {} error(s), {} warning(s)",
        r.errors, r.warnings
    ));

    if !r.error_lines.is_empty() {
        lines.push(String::new());
        lines.push("Errors:".into());
        for line in &r.error_lines {
            lines.push(line.clone());
        }
    }

    lines
}

#[note("build-report-error-lines")]
pub fn error_lines(diags: &[Diagnostic]) -> Vec<String> {
    let mut lines = Vec::new();
    for e in diags.iter().filter(|d| d.severity == Severity::Error) {
        let loc = e
            .location
            .as_ref()
            .map(|l| format!(" ({}:{})", l.file.display(), l.line))
            .unwrap_or_default();
        lines.push(format!("  error: {}{loc}", e.message));
        if !e.rendered.is_empty() {
            for rl in e.rendered.lines() {
                lines.push(format!("    {rl}"));
            }
        }
    }
    lines
}

#[note("build-report-diagnostics-text")]
pub fn diagnostics_text(diags: &[Diagnostic], severity: Severity) -> String {
    let label = match severity {
        Severity::Warning => "warning",
        Severity::Error => "error",
    };
    let mut items = Vec::new();
    for d in diags.iter().filter(|d| d.severity == severity) {
        let loc = d
            .location
            .as_ref()
            .map(|l| format!(" ({}:{})", l.file.display(), l.line))
            .unwrap_or_default();
        items.push(format!("{label}: {}{loc}", d.message));
        if !d.rendered.is_empty() {
            for rl in d.rendered.lines() {
                items.push(format!("  {rl}"));
            }
        }
    }
    if items.is_empty() {
        match severity {
            Severity::Warning => "No warnings.".to_string(),
            Severity::Error => "No errors.".to_string(),
        }
    } else {
        items.join("\n")
    }
}

#[cfg(test)]
#[path = "test/build_report/test.rs"]
mod tests;
