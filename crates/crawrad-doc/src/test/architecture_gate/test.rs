use crate::architecture_gate::{
    ALLOWED_MAKEPAD_CRATES, IdeAppRatchetMismatch, LineRatchetMismatch,
    MAX_EXTENSION_HOST_IDEAPP_REFS, MAX_MCP_HOST_IDEAPP_REFS, MAX_SHELL_FILE_LINES,
    MAX_SHELL_SOURCE_LINES, SHELL_FILE_LINE_EXCEPTIONS, ShellFileLineMismatch,
    check_makepad_dependencies, check_mcp_host_ideapp_ratchet, check_shell_dependencies,
    check_shell_file_lines, check_shell_line_ratchet, check_shell_root_files,
    count_ideapp_refs_in_dir, count_ideapp_refs_in_file, extract_effective_dependencies,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn next_temp_dir(tag: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "crawrad-arch-gate-{tag}-{}-{id}",
        std::process::id()
    ))
}

fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn shell_src_dir() -> PathBuf {
    crates_dir().join("crawrad-ide").join("src")
}

// =========================================================================
// Actual Invariant Checks on the Workspace
// =========================================================================

/// **Rule 1 (ADR 0043): Only visual surface crates may depend on `makepad-widgets` (directly or transitively).**
#[test]
fn only_authorized_crates_declare_makepad_widgets() {
    let violations = check_makepad_dependencies(&crates_dir());
    assert!(
        violations.is_empty(),
        "\n\n[ARCHITECTURE GATE - ADR 0043] Non-visual crates depending on `makepad-widgets`:\n{}\n\n\
         How to fix:\n\
         - Makepad is only allowed in: {:?}.\n\
         - Pure logic crates (terminal, mcp, extension-host, agent, project, etc.) cannot have widgets or windows,\n\
           nor depend on crates that have them (clean transitive closure).\n\
         - Move code dependent on Makepad to `crates/crawrad-ide` or use abstract types.\n",
        violations
            .iter()
            .map(|v| {
                if v.is_transitive {
                    format!(
                        "  - Crate: {}\n    Transitive path: {}",
                        v.crate_name, v.path_or_manifest
                    )
                } else {
                    format!(
                        "  - Crate: {}\n    Manifest: {}",
                        v.crate_name, v.path_or_manifest
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
        ALLOWED_MAKEPAD_CRATES
    );
}

/// **Rule 2 (ADR 0043): No crate in the workspace may depend on `crawrad-ide`.**
#[test]
fn no_crate_depends_on_crawrad_ide() {
    let violations = check_shell_dependencies(&crates_dir());
    assert!(
        violations.is_empty(),
        "\n\n[ARCHITECTURE GATE - ADR 0043] Crates declaring dependency on `crawrad-ide`:\n{}\n\n\
         How to fix:\n\
         - The dependency arrow always points inwards:\n\
           crawrad-ide -> (agent, mcp, terminal, extension-host, project) -> (actions, codegen, registry, doc, ir, os)\n\
         - No crate may depend on `crawrad-ide`. Remove the dependency on `crawrad-ide` from the Cargo.toml above.\n",
        violations
            .iter()
            .map(|v| format!(
                "  - Crate: {}\n    Manifest: {}",
                v.crate_name, v.cargo_toml_path
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// **Rule 3 (ADR 0043): The root of `crates/crawrad-ide/src/` matches the allowlist exactly.**
#[test]
fn shell_root_files_match_allowlist_exactly() {
    let result = check_shell_root_files(&shell_src_dir());
    if let Err(mismatch) = result {
        let mut msg = String::from(
            "\n\n[ARCHITECTURE GATE - ADR 0043] Discrepancy at the root of `crates/crawrad-ide/src/`:\n",
        );
        if !mismatch.unexpected_files.is_empty() {
            msg.push_str(&format!(
                "  Unauthorized files found at root:\n    {}\n\
                 How to fix: Move files to semantic folders (e.g. view_models/, support/) or to dedicated crates.\n",
                mismatch.unexpected_files.join(", ")
            ));
        }
        if !mismatch.missing_files.is_empty() {
            msg.push_str(&format!(
                "  Inventory files that no longer exist at root:\n    {}\n\
                 How to fix: The ratchet tightens! Remove these files from `ALLOWED_SHELL_ROOT_FILES` in `architecture_gate.rs` in this PR.\n",
                mismatch.missing_files.join(", ")
            ));
        }
        panic!("{msg}");
    }
}

/// **Rule 4 (ADR 0043): Total line count in `crates/crawrad-ide/src/` obeys the exact ratchet.**
#[test]
fn shell_source_line_ratchet() {
    let result = check_shell_line_ratchet(&shell_src_dir(), MAX_SHELL_SOURCE_LINES);
    match result {
        Ok(actual) => {
            println!(
                "crawrad-ide line ratchet (excluding test/): {} lines (matching ratchet exactly: {})",
                actual, MAX_SHELL_SOURCE_LINES
            );
        }
        Err(LineRatchetMismatch::Exceeded {
            actual,
            max_allowed,
        }) => {
            panic!(
                "\n\n[ARCHITECTURE GATE - ADR 0043] Line ratchet for `crawrad-ide` exceeded upwards!\n\
                 Current lines (excluding test/): {}\n\
                 Maximum ratchet limit:          {}\n\
                 Excess:                         {} lines over!\n\n\
                 How to fix:\n\
                 - `crawrad-ide` is under a downward ratchet. New logic development must\n\
                   be done in domain crates (`crawrad-terminal`, `crawrad-mcp`, `crawrad-agent`, `crawrad-project`, etc.).\n",
                actual,
                max_allowed,
                actual.saturating_sub(max_allowed)
            );
        }
        Err(LineRatchetMismatch::Shrank {
            actual,
            max_allowed,
        }) => {
            panic!(
                "\n\n[ARCHITECTURE GATE - ADR 0043] The ratchet must tighten! `crawrad-ide` shrank!\n\
                 Current lines (excluding test/): {}\n\
                 Previous ratchet:               {}\n\
                 Reduction achieved:             {} lines fewer!\n\n\
                 How to fix:\n\
                 - Update `MAX_SHELL_SOURCE_LINES` from {} to {} in `architecture_gate.rs` in this PR.\n\
                 - Do not leave slack for lines to return to the shell.\n",
                actual,
                max_allowed,
                max_allowed.saturating_sub(actual),
                max_allowed,
                actual
            );
        }
    }
}

/// **Rule 5: No individual `.rs` file in `crates/crawrad-ide/src/` exceeds 1,500 lines, unless grandfathered.**
#[test]
fn shell_file_line_ceiling_ratchet() {
    let mismatches = check_shell_file_lines(
        &shell_src_dir(),
        MAX_SHELL_FILE_LINES,
        SHELL_FILE_LINE_EXCEPTIONS,
    );
    if !mismatches.is_empty() {
        let mut msg = String::from(
            "\n\n[ARCHITECTURE GATE] Line ceiling per file in `crawrad-ide` violated!\n",
        );
        for m in &mismatches {
            match m {
                ShellFileLineMismatch::UnregisteredFileExceeded {
                    file,
                    actual,
                    max_allowed,
                } => {
                    msg.push_str(&format!(
                        "  - New/unlisted file `{file}` has {actual} lines, exceeding the ceiling of {max_allowed} lines!\n\
                         How to fix: Slice the file into smaller submodules or extract logic to domain crates.\n"
                    ));
                }
                ShellFileLineMismatch::ExceptionExceeded {
                    file,
                    actual,
                    expected,
                } => {
                    msg.push_str(&format!(
                        "  - Grandfathered file `{file}` grew: {actual} lines (registered ceiling: {expected})!\n\
                         How to fix: Legacy files may only shrink. Revert additions or extract code.\n"
                    ));
                }
                ShellFileLineMismatch::ExceptionShrank {
                    file,
                    actual,
                    expected,
                } => {
                    if *actual <= MAX_SHELL_FILE_LINES {
                        msg.push_str(&format!(
                            "  - Grandfathered file `{file}` shrank to {actual} lines (<= {MAX_SHELL_FILE_LINES})!\n\
                             How to fix: The file reached the general ceiling! Remove `{file}` from `SHELL_FILE_LINE_EXCEPTIONS` in `architecture_gate.rs` in this PR.\n"
                        ));
                    } else {
                        msg.push_str(&format!(
                            "  - Grandfathered file `{file}` shrank: {actual} lines (previous ceiling: {expected})!\n\
                             How to fix: The ratchet tightens! Update entry for `{file}` to {actual} in `SHELL_FILE_LINE_EXCEPTIONS` in `architecture_gate.rs` in this PR.\n"
                        ));
                    }
                }
                ShellFileLineMismatch::MissingFile { file } => {
                    msg.push_str(&format!(
                        "  - File registered in `SHELL_FILE_LINE_EXCEPTIONS` was not found on disk: `{file}`\n\
                         How to fix: Remove nonexistent entry from `SHELL_FILE_LINE_EXCEPTIONS` in `architecture_gate.rs` in this PR.\n"
                    ));
                }
            }
        }
        panic!("{msg}");
    }
}

// =========================================================================
// Mutation Proofs (Proving the gate goes red when violated)
// =========================================================================

/// **Mutation Proof 1 (Aefos finding 1a)**: When a file is removed from the shell root, the gate fails
/// demanding that `ALLOWED_SHELL_ROOT_FILES` be updated to drop that file.
#[test]
fn mutation_proof_root_file_removed_demands_allowlist_shrink() {
    let base = next_temp_dir("shrank-root");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();

    // Create all files except `main.rs`
    for file in crate::architecture_gate::ALLOWED_SHELL_ROOT_FILES {
        if *file == "main.rs" {
            continue; // Simulate moving or deleting main.rs
        }
        std::fs::write(base.join(file), "// dummy\n").unwrap();
    }

    let result = check_shell_root_files(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert!(result.is_err());
    let mismatch = result.unwrap_err();
    assert_eq!(mismatch.missing_files, vec!["main.rs".to_string()]);
    assert!(mismatch.unexpected_files.is_empty());
}

/// **Mutation Proof 2 (Aefos finding 1b)**: When the shell shrinks, the gate fails demanding
/// that `MAX_SHELL_SOURCE_LINES` be lowered in the same PR.
#[test]
fn mutation_proof_line_ratchet_demands_shrink() {
    let base = next_temp_dir("shrank-lines");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();

    // Create 100 lines of source
    let content = "pub fn f() {}\n".repeat(100);
    std::fs::write(base.join("source.rs"), &content).unwrap();

    // If the ratchet is 150 (greater than actual 100), the gate MUST fail demanding lower MAX
    let result = check_shell_line_ratchet(&base, 150);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(
        result,
        Err(LineRatchetMismatch::Shrank {
            actual: 100,
            max_allowed: 150
        })
    );
}

/// **Mutation Proof 3 (Aefos finding 2)**: Renamed dependency `mk = { package = "makepad-widgets" }`
/// in an unauthorized crate fails the gate.
#[test]
fn mutation_proof_renamed_makepad_dependency_is_caught() {
    let base = next_temp_dir("renamed-makepad");
    let _ = std::fs::remove_dir_all(&base);

    let lsp_crate = base.join("crawrad-lsp");
    std::fs::create_dir_all(&lsp_crate).unwrap();

    std::fs::write(
        lsp_crate.join("Cargo.toml"),
        r#"
[package]
name = "crawrad-lsp"
version = "0.1.0"

[dependencies]
mk = { package = "makepad-widgets", version = "1.0.0" }
"#,
    )
    .unwrap();

    let violations = check_makepad_dependencies(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].crate_name, "crawrad-lsp");
    assert!(!violations[0].is_transitive);
}

/// **Mutation Proof 4 (Aefos finding 2b)**: Renamed dependency `sh = { package = "crawrad-ide" }`
/// fails the gate.
#[test]
fn mutation_proof_renamed_shell_dependency_is_caught() {
    let base = next_temp_dir("renamed-shell");
    let _ = std::fs::remove_dir_all(&base);

    let rogue = base.join("crawrad-agent");
    std::fs::create_dir_all(&rogue).unwrap();

    std::fs::write(
        rogue.join("Cargo.toml"),
        r#"
[package]
name = "crawrad-agent"
version = "0.1.0"

[dependencies]
sh = { package = "crawrad-ide", path = "../crawrad-ide" }
"#,
    )
    .unwrap();

    let violations = check_shell_dependencies(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].crate_name, "crawrad-agent");
}

/// **Mutation Proof 5 (Aefos finding 3)**: Transitive dependency on Makepad (`crawrad-lsp -> crawrad-runtime-makepad`)
/// fails the gate with the exact dependency chain.
#[test]
fn mutation_proof_transitive_makepad_dependency_is_caught() {
    let base = next_temp_dir("transitive-makepad");
    let _ = std::fs::remove_dir_all(&base);

    let lsp = base.join("crawrad-lsp");
    let runtime_makepad = base.join("crawrad-runtime-makepad");
    std::fs::create_dir_all(&lsp).unwrap();
    std::fs::create_dir_all(&runtime_makepad).unwrap();

    // crawrad-runtime-makepad is allowed to have makepad-widgets directly
    std::fs::write(
        runtime_makepad.join("Cargo.toml"),
        r#"
[package]
name = "crawrad-runtime-makepad"
version = "0.1.0"

[dependencies]
makepad-widgets = "1.0.0"
"#,
    )
    .unwrap();

    // crawrad-lsp is NOT allowed to have makepad-widgets, neither directly nor transitively
    std::fs::write(
        lsp.join("Cargo.toml"),
        r#"
[package]
name = "crawrad-lsp"
version = "0.1.0"

[dependencies]
crawrad-runtime-makepad = { path = "../crawrad-runtime-makepad" }
"#,
    )
    .unwrap();

    let violations = check_makepad_dependencies(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].crate_name, "crawrad-lsp");
    assert!(violations[0].is_transitive);
    assert_eq!(
        violations[0].path_or_manifest,
        "crawrad-lsp -> crawrad-runtime-makepad -> makepad-widgets"
    );
}

/// **Mutation Proof 6**: Direct `makepad-widgets` in unauthorized crate fails.
#[test]
fn mutation_proof_direct_makepad_fails() {
    let base = next_temp_dir("direct-makepad");
    let _ = std::fs::remove_dir_all(&base);

    let terminal = base.join("crawrad-terminal");
    std::fs::create_dir_all(&terminal).unwrap();

    std::fs::write(
        terminal.join("Cargo.toml"),
        "[package]\nname = \"crawrad-terminal\"\n[dependencies]\nmakepad-widgets = \"1.0.0\"\n",
    )
    .unwrap();

    let violations = check_makepad_dependencies(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].crate_name, "crawrad-terminal");
    assert!(!violations[0].is_transitive);
}

/// **Mutation Proof 7**: Unexpected file in shell root fails.
#[test]
fn mutation_proof_unexpected_shell_root_file_fails() {
    let base = next_temp_dir("unexpected-root");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();

    for file in crate::architecture_gate::ALLOWED_SHELL_ROOT_FILES {
        std::fs::write(base.join(file), "// dummy\n").unwrap();
    }
    std::fs::write(base.join("qualquer.rs"), "pub fn rogue() {}\n").unwrap();

    let result = check_shell_root_files(&base);
    let _ = std::fs::remove_dir_all(&base);

    assert!(result.is_err());
    let mismatch = result.unwrap_err();
    assert_eq!(mismatch.unexpected_files, vec!["qualquer.rs".to_string()]);
    assert!(mismatch.missing_files.is_empty());
}

/// **Mutation Proof 8**: Exceeding lines (> MAX) fails.
#[test]
fn mutation_proof_exceeding_lines_fails() {
    let base = next_temp_dir("exceeded-lines");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).unwrap();

    let content = "pub fn f() {}\n".repeat(300);
    std::fs::write(base.join("source.rs"), &content).unwrap();

    let result = check_shell_line_ratchet(&base, 200);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(
        result,
        Err(LineRatchetMismatch::Exceeded {
            actual: 300,
            max_allowed: 200
        })
    );
}

/// Unit test for extract_effective_dependencies parser.
#[test]
fn test_effective_dependencies_parser() {
    let toml = r#"
[package]
name = "demo"

[dependencies]
crawrad-ir.workspace = true
plain = "1.0.0"
renamed = { package = "real-crate", version = "2.0" }

[dev-dependencies]
dev-plain = "1.0"
dev-renamed = { package = "real-dev", version = "1.0" }

[target.'cfg(windows)'.dependencies]
win-dep = { package = "windows-sys", version = "0.52" }
"#;

    let deps = extract_effective_dependencies(toml, true);
    assert!(deps.contains("crawrad-ir"));
    assert!(deps.contains("plain"));
    assert!(deps.contains("real-crate"));
    assert!(!deps.contains("renamed"));
    assert!(deps.contains("dev-plain"));
    assert!(deps.contains("real-dev"));
    assert!(!deps.contains("dev-renamed"));
    assert!(deps.contains("windows-sys"));
    assert!(!deps.contains("win-dep"));
}

// =========================================================================
// IdeApp Ratchet (ADR 0044 / crawrad-host)
// =========================================================================

/// **Invariant (ADR 0044 PR 0):** The number of `IdeApp` references in the MCP host tree
/// (`ide_app/mcp_host.rs` + `ide_app/mcp_host/**`) and `crawrad-extension-host/` must be exactly
/// equal to their declared ratchet constants.
///
/// Baseline at 29/09/2026: mcp_host.rs = 36, extension-host/ = 0.
/// Each subsequent PR of Issue #242 MUST lower the mcp_host.rs count and update
/// `MAX_MCP_HOST_IDEAPP_REFS` in the same PR. Reaching 0 and deleting `mcp_host.rs` is the PR 8 goal.
#[test]
fn ideapp_reference_ratchet() {
    let shell_src = shell_src_dir();
    let ext_host_src = crates_dir().join("crawrad-extension-host").join("src");
    let mismatches = check_mcp_host_ideapp_ratchet(
        &shell_src,
        &ext_host_src,
        MAX_MCP_HOST_IDEAPP_REFS,
        MAX_EXTENSION_HOST_IDEAPP_REFS,
    );

    assert!(
        mismatches.is_empty(),
        "\n\n[ARCHITECTURE GATE - ADR 0044] IdeApp ratchet violated!\n\
         Mismatches found:\n{}\n\n\
         How to fix:\n\
         - If MISSING PATH: the file/folder was moved or renamed; point the ratchet to the new path.\n\
         - If the count GREW: remove references to IdeApp (migrate logic to crawrad-host).\n\
         - If the count SHRANK: ensure the decrease came from migration to crawrad-host (and not from moving code to another shell file) and update MAX_MCP_HOST_IDEAPP_REFS in this PR.\n\
         - If extension-host/ gained IdeApp references: remove them (strict violation of ADR 0044).\n",
        mismatches
            .iter()
            .map(|m| match m {
                IdeAppRatchetMismatch::PathMissing { path, reason } =>
                    format!("  MISSING/CHANGED PATH {path}: {reason}"),
                IdeAppRatchetMismatch::Exceeded {
                    file,
                    actual,
                    max_allowed,
                } => format!("  GREW   {file}: {actual} refs (limit: {max_allowed})"),
                IdeAppRatchetMismatch::Shrank {
                    file,
                    actual,
                    max_allowed,
                } => format!(
                    "  SHRANK {file}: {actual} refs (old limit: {max_allowed}) -> update constant"
                ),
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// **Mutation Proof A (ADR 0044):** If a new `IdeApp` reference appears in `mcp_host.rs`,
/// the gate must go red (coupling increased).
#[test]
fn mutation_proof_ideapp_mcp_host_exceeded() {
    let base = next_temp_dir("ideapp-exceeded");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    std::fs::create_dir_all(&ide_app_dir).unwrap();

    // 3 refs to IdeApp in the stub file, but ratchet allows only 2
    std::fs::write(
        ide_app_dir.join("mcp_host.rs"),
        "impl IdeApp { fn a() {} }\nfn b(app: &IdeApp) {}\nstruct Foo(IdeApp);\n",
    )
    .unwrap();

    let ext_dir = base.join("ext_src");
    std::fs::create_dir_all(&ext_dir).unwrap();
    std::fs::write(ext_dir.join("lib.rs"), "// clean\n").unwrap();

    let result = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 2, 0);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        result[0],
        IdeAppRatchetMismatch::Exceeded {
            actual: 3,
            max_allowed: 2,
            ..
        }
    ));
}

/// **Mutation Proof B (ADR 0044):** If `IdeApp` references decrease without updating the constant,
/// the gate must go red (ratchet must be lowered in the same PR).
#[test]
fn mutation_proof_ideapp_mcp_host_shrank() {
    let base = next_temp_dir("ideapp-shrank");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    std::fs::create_dir_all(&ide_app_dir).unwrap();

    // Only 1 ref, but ratchet says 3
    std::fs::write(ide_app_dir.join("mcp_host.rs"), "fn foo(app: &IdeApp) {}\n").unwrap();

    let ext_dir = base.join("ext_src");
    std::fs::create_dir_all(&ext_dir).unwrap();
    std::fs::write(ext_dir.join("lib.rs"), "// clean\n").unwrap();

    let result = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 3, 0);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        result[0],
        IdeAppRatchetMismatch::Shrank {
            actual: 1,
            max_allowed: 3,
            ..
        }
    ));
}

/// **Mutation Proof C (ADR 0044):** If `IdeApp` appears in `crawrad-extension-host/`,
/// the gate must go red (extension-host must never acquire IdeApp coupling).
#[test]
fn mutation_proof_ideapp_extension_host_exceeded() {
    let base = next_temp_dir("ideapp-ext");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    std::fs::create_dir_all(&ide_app_dir).unwrap();

    // mcp_host.rs with 0 refs (fine) — clean stub
    std::fs::write(ide_app_dir.join("mcp_host.rs"), "// clean stub\n").unwrap();

    // extension-host dir with 1 IdeApp ref (should fail)
    let ext_dir = base.join("ext_src");
    std::fs::create_dir_all(&ext_dir).unwrap();
    std::fs::write(ext_dir.join("host.rs"), "fn call(app: &IdeApp) {}\n").unwrap();

    let result = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 0, 0);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(result.len(), 1);
    assert!(matches!(
        result[0],
        IdeAppRatchetMismatch::Exceeded {
            actual: 1,
            max_allowed: 0,
            ..
        }
    ));
}

/// **Mutation Proof D (ADR 0044):** If the extension-host directory is missing or empty,
/// the gate MUST fail with `PathMissing`, NOT silently pass as zero.
#[test]
fn mutation_proof_ideapp_extension_host_missing_fails() {
    let base = next_temp_dir("ideapp-ext-missing");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    std::fs::create_dir_all(&ide_app_dir).unwrap();
    std::fs::write(ide_app_dir.join("mcp_host.rs"), "// clean stub\n").unwrap();

    // Non-existent directory
    let non_existent_ext_dir = base.join("non_existent_ext_src");

    let result = check_mcp_host_ideapp_ratchet(&base, &non_existent_ext_dir, 0, 0);
    assert_eq!(result.len(), 1);
    assert!(matches!(
        &result[0],
        IdeAppRatchetMismatch::PathMissing { path, reason }
        if path.contains("crawrad-extension-host") && reason.contains("not found")
    ));

    // Empty directory (contains 0 .rs files)
    let empty_ext_dir = base.join("empty_ext_src");
    std::fs::create_dir_all(&empty_ext_dir).unwrap();

    let result_empty = check_mcp_host_ideapp_ratchet(&base, &empty_ext_dir, 0, 0);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(result_empty.len(), 1);
    assert!(matches!(
        &result_empty[0],
        IdeAppRatchetMismatch::PathMissing { path, reason }
        if path.contains("crawrad-extension-host") && reason.contains("contains no `.rs` files")
    ));
}

/// **Mutation Proof E (ADR 0044):** If `mcp_host.rs` is missing/renamed, the gate MUST fail
/// with `PathMissing` and NOT report "DESCEU" (Shrank) which would falsely suggest lowering the constant.
#[test]
fn mutation_proof_ideapp_mcp_host_missing_fails() {
    let base = next_temp_dir("ideapp-mcp-missing");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    std::fs::create_dir_all(&ide_app_dir).unwrap();
    // Notice: mcp_host.rs is NOT created in ide_app_dir

    let ext_dir = base.join("ext_src");
    std::fs::create_dir_all(&ext_dir).unwrap();
    std::fs::write(ext_dir.join("lib.rs"), "// clean\n").unwrap();

    let result = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 36, 0);
    let _ = std::fs::remove_dir_all(&base);

    assert_eq!(result.len(), 1);
    assert!(
        matches!(&result[0], IdeAppRatchetMismatch::PathMissing { path, reason }
            if path.contains("mcp_host.rs") && reason.contains("not found")),
        "Expected PathMissing for missing mcp_host.rs, but got: {:?}",
        result[0]
    );
}

/// **Mutation Proof F (ADR 0044):** `IdeApp` references located in submodules under
/// `ide_app/mcp_host/**/*.rs` MUST be included in the total MCP count.
#[test]
fn mutation_proof_ideapp_mcp_host_subfolder_counted() {
    let base = next_temp_dir("ideapp-mcp-subfolder");
    let _ = std::fs::remove_dir_all(&base);
    let ide_app_dir = base.join("ide_app");
    let mcp_sub_dir = ide_app_dir.join("mcp_host");
    std::fs::create_dir_all(&mcp_sub_dir).unwrap();

    // 2 references in mcp_host.rs
    std::fs::write(
        ide_app_dir.join("mcp_host.rs"),
        "struct Foo(IdeApp);\nfn f(app: &IdeApp) {}\n",
    )
    .unwrap();

    // 3 references in mcp_host/tools.rs
    std::fs::write(
        mcp_sub_dir.join("tools.rs"),
        "impl IdeApp { fn a() {} }\nfn b(a: &IdeApp) {}\nfn c(a: &mut IdeApp) {}\n",
    )
    .unwrap();

    let ext_dir = base.join("ext_src");
    std::fs::create_dir_all(&ext_dir).unwrap();
    std::fs::write(ext_dir.join("lib.rs"), "// clean\n").unwrap();

    // Total actual is 2 + 3 = 5 refs.
    // If ratchet allows 5: perfectly matches (0 mismatches)
    let result_ok = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 5, 0);
    assert!(
        result_ok.is_empty(),
        "Expected 0 mismatches for exact match 5, got: {:?}",
        result_ok
    );

    // If ratchet is 4: Exceeded with actual = 5
    let result_exceeded = check_mcp_host_ideapp_ratchet(&base, &ext_dir, 4, 0);
    assert_eq!(result_exceeded.len(), 1);
    assert!(matches!(
        result_exceeded[0],
        IdeAppRatchetMismatch::Exceeded {
            actual: 5,
            max_allowed: 4,
            ..
        }
    ));

    let _ = std::fs::remove_dir_all(&base);
}

/// Unit test for `count_ideapp_refs_in_file`.
#[test]
fn count_ideapp_refs_counts_correctly() {
    let dir = next_temp_dir("count-ideapp");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("test.rs");
    std::fs::write(
        &file,
        "struct IdeApp;\nimpl IdeApp { fn f(&self) {} }\nfn g(a: &mut IdeApp) {}\n",
    )
    .unwrap();
    let count = count_ideapp_refs_in_file(&file).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(count, 3);
}

/// Unit test for `count_ideapp_refs_in_dir`.
#[test]
fn count_ideapp_refs_in_dir_recurses() {
    let dir = next_temp_dir("count-ideapp-dir");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.rs"), "fn f(app: &IdeApp) {}\n").unwrap();
    let sub = dir.join("sub");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(
        sub.join("b.rs"),
        "struct IdeApp;\nlet _x: IdeApp = todo!();\n",
    )
    .unwrap();
    let (count, files) = count_ideapp_refs_in_dir(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(count, 3);
    assert_eq!(files, 2);
}

/// **Mutation Proof (File Ceiling 1,501 lines)**: A new file with 1,501 lines fails the gate.
#[test]
fn mutation_proof_new_file_with_1501_lines_fails() {
    let temp = next_temp_dir("ceiling-1501");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();

    let file = temp.join("novo_modulo.rs");
    let content = "// dummy line\n".repeat(1501);
    std::fs::write(&file, content).unwrap();

    let mismatches = check_shell_file_lines(&temp, MAX_SHELL_FILE_LINES, &[]);
    let _ = std::fs::remove_dir_all(&temp);

    assert_eq!(
        mismatches,
        vec![ShellFileLineMismatch::UnregisteredFileExceeded {
            file: "novo_modulo.rs".to_string(),
            actual: 1501,
            max_allowed: 1500,
        }],
        "The gate should have failed for a file with 1,501 lines!"
    );
}

/// **Mutation Proof (File Ceiling Exact 1,500 lines)**: A file with exactly 1,500 lines passes.
#[test]
fn mutation_proof_file_with_exact_1500_lines_passes() {
    let temp = next_temp_dir("ceiling-1500");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();

    let file = temp.join("modulo_limite.rs");
    let content = "// dummy line\n".repeat(1500);
    std::fs::write(&file, content).unwrap();

    let mismatches = check_shell_file_lines(&temp, MAX_SHELL_FILE_LINES, &[]);
    let _ = std::fs::remove_dir_all(&temp);

    assert!(
        mismatches.is_empty(),
        "A file with exactly 1,500 lines should pass!"
    );
}

/// **Mutation Proof (Exception Shrink)**: When a grandfathered file shrinks, the gate fails demanding ratchet tightening.
#[test]
fn mutation_proof_exception_file_shrank_demands_update() {
    let temp = next_temp_dir("ceiling-shrank");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(&temp).unwrap();

    let file = temp.join("ide_app.rs");
    let content = "// dummy line\n".repeat(10_000);
    std::fs::write(&file, content).unwrap();

    let mismatches = check_shell_file_lines(&temp, MAX_SHELL_FILE_LINES, &[("ide_app.rs", 11_030)]);
    let _ = std::fs::remove_dir_all(&temp);

    assert_eq!(
        mismatches,
        vec![ShellFileLineMismatch::ExceptionShrank {
            file: "ide_app.rs".to_string(),
            actual: 10_000,
            expected: 11_030,
        }],
        "The gate should have failed demanding ratchet tightening when the file shrank!"
    );
}
