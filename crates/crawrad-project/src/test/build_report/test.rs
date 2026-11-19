use std::path::{Path, PathBuf};

use super::*;
use crate::build_settings::{BuildMode, BuildProfile, TargetPlatform};
use crate::diagnostics_model::{CompiledArtifact, Diagnostic, DiagnosticLocation, Severity};

#[test]
fn test_manifest_path() {
    let tmp = std::env::temp_dir().join(format!("crawrad_test_manifest_{}", std::process::id()));
    let pkg_dir = tmp.join("my_pkg");
    let _ = std::fs::create_dir_all(&pkg_dir);
    let pkg_manifest = pkg_dir.join("Cargo.toml");
    let _ = std::fs::write(&pkg_manifest, "");

    // 1. Found in root/package/Cargo.toml
    let p1 = manifest_path(Some(&tmp), "my_pkg", &[]);
    assert_eq!(p1, pkg_manifest.to_string_lossy());

    // 2. Found in root/Cargo.toml
    let root_manifest = tmp.join("Cargo.toml");
    let _ = std::fs::write(&root_manifest, "");
    let p2 = manifest_path(Some(&tmp), "other_pkg", &[]);
    assert_eq!(p2, root_manifest.to_string_lossy());

    // 3. Artifact fallback
    let art = CompiledArtifact {
        package_name: "art_pkg".to_string(),
        kind: vec!["bin".to_string()],
        manifest_path: Some(PathBuf::from("/some/path/Cargo.toml")),
        executable: None,
        filenames: vec![],
    };
    let p3 = manifest_path(None, "art_pkg", &[art]);
    assert_eq!(p3, Path::new("/some/path/Cargo.toml").to_string_lossy());

    // 4. Default fallback
    let p4 = manifest_path(None, "unknown", &[]);
    assert_eq!(p4, "unknown/Cargo.toml");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_platform_label() {
    // Empty targets -> host default
    let host = platform_label(&[]);
    assert!(!host.is_empty());

    // Single target
    let single = platform_label(&[TargetPlatform::Linux64]);
    assert_eq!(single, "Linux 64-bit (x64)");

    // Multiple targets
    let multi = platform_label(&[TargetPlatform::Win64, TargetPlatform::Wasm32]);
    assert_eq!(multi, "Windows 64-bit (x64), WebAssembly (wasm32)");
}

#[test]
fn test_executables_and_cache_fallback() {
    let tmp = std::env::temp_dir().join(format!("crawrad_test_exes_{}", std::process::id()));
    let target_debug = tmp.join("target").join("debug");
    let _ = std::fs::create_dir_all(&target_debug);

    // 1. Artifacts take precedence
    let art = CompiledArtifact {
        package_name: "app".to_string(),
        kind: vec!["bin".to_string()],
        manifest_path: None,
        executable: Some(PathBuf::from("/direct/bin/app")),
        filenames: vec![],
    };
    let exes = executables(Some(&tmp), "app", BuildProfile::Debug, &[], &[art]);
    assert_eq!(exes, vec![PathBuf::from("/direct/bin/app")]);

    // 2. Cache fallback under target/debug/app.exe or app
    let cand = if cfg!(windows) {
        target_debug.join("app.exe")
    } else {
        target_debug.join("app")
    };
    let _ = std::fs::write(&cand, "dummy binary");
    let exes_cached = executables(Some(&tmp), "app", BuildProfile::Debug, &[], &[]);
    assert_eq!(exes_cached, vec![cand]);

    // 3. Cache fallback for triple target under release
    let target_triple_rel = tmp
        .join("target")
        .join(TargetPlatform::Linux64.triple())
        .join("release");
    let _ = std::fs::create_dir_all(&target_triple_rel);
    let triple_cand = target_triple_rel.join("app");
    let _ = std::fs::write(&triple_cand, "dummy binary");
    let exes_triple = executables(
        Some(&tmp),
        "app",
        BuildProfile::Release,
        &[TargetPlatform::Linux64],
        &[],
    );
    assert_eq!(exes_triple, vec![triple_cand]);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn test_summary_single_executable_success() {
    let report = Report {
        manifest: "C:/proj/Cargo.toml".to_string(),
        platform: "Windows 64-bit (x64)".to_string(),
        configuration: "Debug".to_string(),
        executables: vec![(PathBuf::from("C:/proj/target/debug/app.exe"), Some(1024))],
        mode: BuildMode::Build,
        package: "app".to_string(),
        success: true,
        errors: 0,
        warnings: 1,
        error_lines: vec![],
    };
    let lines = summary(&report);
    assert_eq!(lines[0], "Project:   C:/proj/Cargo.toml");
    assert_eq!(
        lines[1],
        "Platform:  Windows 64-bit (x64)   |   Configuration: Debug"
    );
    assert!(lines[2].starts_with("Output:    C:/proj/target/debug/app.exe (1.0 KB)"));
    assert_eq!(lines[3], "Status:    ✔ Success  (Done.)");
    assert_eq!(lines[4], "Result:    0 error(s), 1 warning(s)");
}

#[test]
fn test_summary_multiple_executables() {
    let report = Report {
        manifest: "C:/proj/Cargo.toml".to_string(),
        platform: "Multi".to_string(),
        configuration: "Release".to_string(),
        executables: vec![
            (PathBuf::from("C:/out/app1.exe"), Some(512)),
            (PathBuf::from("C:/out/app2.exe"), None),
        ],
        mode: BuildMode::Build,
        package: "app".to_string(),
        success: true,
        errors: 0,
        warnings: 0,
        error_lines: vec![],
    };
    let lines = summary(&report);
    assert_eq!(lines[2], "Output (1): C:/out/app1.exe (512 B)");
    assert_eq!(lines[3], "Output (2): C:/out/app2.exe");
    assert_eq!(lines[4], "Status:    ✔ Success  (Done.)");
}

#[test]
fn test_summary_check_and_clean_modes() {
    let r_check = Report {
        manifest: "Cargo.toml".into(),
        platform: "Host".into(),
        configuration: "Debug".into(),
        executables: vec![],
        mode: BuildMode::Check,
        package: "app".into(),
        success: true,
        errors: 0,
        warnings: 0,
        error_lines: vec![],
    };
    let lines_chk = summary(&r_check);
    assert_eq!(
        lines_chk[2],
        "Output:    [Check only — syntax & types verified, no binary generated]"
    );

    let r_clean = Report {
        manifest: "Cargo.toml".into(),
        platform: "Host".into(),
        configuration: "Debug".into(),
        executables: vec![],
        mode: BuildMode::Clean,
        package: "app".into(),
        success: true,
        errors: 0,
        warnings: 0,
        error_lines: vec![],
    };
    let lines_clean = summary(&r_clean);
    assert_eq!(
        lines_clean[2],
        "Output:    [Cleaned build artifacts for app]"
    );
}

#[test]
fn test_summary_failure_with_errors() {
    let report = Report {
        manifest: "Cargo.toml".into(),
        platform: "Host".into(),
        configuration: "Debug".into(),
        executables: vec![],
        mode: BuildMode::Build,
        package: "app".into(),
        success: false,
        errors: 2,
        warnings: 0,
        error_lines: vec![
            "  error: cannot find value `foo` (src/main.rs:10)".into(),
            "    | let x = foo;".into(),
        ],
    };
    let lines = summary(&report);
    assert_eq!(lines[2], "Finished for app");
    assert_eq!(lines[3], "Status:    ❌ Failed");
    assert_eq!(lines[4], "Result:    2 error(s), 0 warning(s)");
    assert_eq!(lines[5], "");
    assert_eq!(lines[6], "Errors:");
    assert_eq!(
        lines[7],
        "  error: cannot find value `foo` (src/main.rs:10)"
    );
    assert_eq!(lines[8], "    | let x = foo;");
}

#[test]
fn test_diagnostics_text_and_error_lines() {
    assert_eq!(diagnostics_text(&[], Severity::Warning), "No warnings.");
    assert_eq!(diagnostics_text(&[], Severity::Error), "No errors.");

    let diags = vec![
        Diagnostic {
            severity: Severity::Warning,
            message: "unused variable `x`".into(),
            location: Some(DiagnosticLocation {
                file: PathBuf::from("src/lib.rs"),
                line: 5,
                column: 9,
            }),
            rendered: "warning: unused variable\n --> src/lib.rs:5:9".into(),
        },
        Diagnostic {
            severity: Severity::Error,
            message: "mismatched types".into(),
            location: Some(DiagnosticLocation {
                file: PathBuf::from("src/main.rs"),
                line: 12,
                column: 4,
            }),
            rendered: "expected usize, found bool".into(),
        },
    ];

    let warn_text = diagnostics_text(&diags, Severity::Warning);
    assert!(warn_text.contains("warning: unused variable `x` (src/lib.rs:5)"));
    // The compiler's own indentation is kept, two spaces deeper.
    assert!(warn_text.contains("  warning: unused variable\n   --> src/lib.rs:5:9"));

    let err_text = diagnostics_text(&diags, Severity::Error);
    assert!(err_text.contains("error: mismatched types (src/main.rs:12)"));
    assert!(err_text.contains("  expected usize, found bool"));

    let elines = error_lines(&diags);
    assert_eq!(elines.len(), 2);
    assert_eq!(elines[0], "  error: mismatched types (src/main.rs:12)");
    assert_eq!(elines[1], "    expected usize, found bool");
}
