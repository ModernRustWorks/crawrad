//! **The gate that keeps `crawrad-ide` focused on presentation** — [ADR 0043].
//!
//! # Why this gate exists
//!
//! As documented in ADR 0043, `crawrad-ide` accumulated ~62,000 lines and 21 root files,
//! mixing UI composition with terminal logic, extension hosting, MCP protocol handling,
//! ACP agent communication, and project generation.
//!
//! This gate enforces four strict architectural invariants to ensure that:
//! 1. Non-visual crates never depend on `makepad-widgets`, neither directly nor transitively.
//! 2. No crate in the workspace depends on `crawrad-ide` (dependencies flow strictly inward).
//! 3. The root of `crates/crawrad-ide/src/` is an exact allowlist that only shrinks.
//! 4. The total line count of `crates/crawrad-ide/src/` (outside `test/`) is ratcheted down.
//! 5. Individual `.rs` files in `crates/crawrad-ide/src/` cannot exceed 1,500 lines (grandfathered files in an exact ratchet).
//!

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::path::{Path, PathBuf};

/// Crates authorized to declare `makepad-widgets` in their Cargo.toml.
///
/// Only crates providing graphical UI or visual runtime integration are permitted.
pub const ALLOWED_MAKEPAD_CRATES: &[&str] = &[
    "crawrad-ide",
    "makepad-code-editor",
    "crawrad-runtime-makepad",
];

/// The explicit allowlist of `.rs` files permitted in the root of `crates/crawrad-ide/src/`.
///
/// This list reflects the 21 baseline files measured on 28/09/2026.
/// Per ADR 0043, this list must EXACTLY match the root files: if any file is moved
/// or removed, it MUST be removed from this list in the same PR.
pub const ALLOWED_SHELL_ROOT_FILES: &[&str] = &["ide_app.rs", "main.rs"];

/// Maximum allowed source lines in `crates/crawrad-ide/src/` (outside `test/`).
///
/// Raised on 26/11/2026 from 33,065 to 33,451 (+386): the ACP diagnostic the owner asked
/// for ("I click there, it does nothing, it does not tell me anything, and it gives me no
/// option"). Only the SCREEN is here — `dialogs/acp_diagnostic.rs`, its rows and its five
/// buttons; the whole engine (the probes, the report and the action list) lives in
/// `crawrad_agent::acp_diagnostic`, and the chip/footer single verdict in
/// `crawrad_agent::acp_live::live_failure`. Also here: the Recent-projects limit row in
/// `dialogs/dsl/options_dialog.rs`, whose model is `crawrad_project::workspace`.
/// Tightened on 18/11/2026 to 33,343: the slash popup's keyboard paid for by table-driven
/// rows and shorter chat click chains.
/// Tightened on 17/11/2026 to 33,376: translating the shell's comments to English joined two
/// comment lines.
/// Previously tightened on 11/11/2026 at 32,054 lines: the code templates' Manage dialog.
/// Raised on 13/11/2026 at 32,327 lines: the New Component wizard — dialog shell in
/// `new_component`, rows in `project_dialogs`, model in `crawrad_project::new_component`; eighty-three
/// lines paid by moving layout notes to the shell layout notes.
/// Raised on 15/11/2026 to 33,172 lines (+15): the MCP drives the Toolbox filter
/// (`view.set_toolbox_filter`); its rows are built in `crawrad_designer::palette::to_json`.
/// Last tightened on 12/11/2026 at 32,042 lines: the Designer's language preview and `Run in a
/// language` — rows in `crawrad_actions::menu`, texts in `crawrad_designer::translations`;
/// fifty-two layout notes moved to the shell layout notes.
/// Before, on 11/11/2026 at 32,067 lines: the Designer bar's `Tab order` button; three layout
/// notes moved to the shell layout notes.
/// Before, on 11/11/2026 at 32,079 lines: the code templates — the shipped set and
/// their rows in `crawrad_insight::templates`, the user's file in `crawrad_project::code_templates`;
/// five layout notes moved to the shell layout notes.
/// Before, on 11/11/2026 at 32,091 lines: the Sizes tab — its toolbar, frames and
/// custom card live in `size_preview_view`, the model in `crawrad_designer::size_preview`.
/// Before, on 11/11/2026 at 32,103 lines: the bottom panels (Problems, Test output, Form check)
/// take the full card, and the Form check repaints after a layout load; one layout note moved to the spec.
/// Before, on 11/11/2026 at 32,115 lines: the pin's hover tip lives in
/// `form_check_view::FormCheckTip` and the pin hit-test in the canvas; five more theme design
/// notes moved to the shell layout notes.
/// Before, on 11/11/2026 at 32,135 lines: the form checker and its panel's state live
/// in `crawrad_designer::form_check*`, its drawing in `form_check_view` and the canvas; six theme
/// design notes moved to the shell layout notes.
/// Raised on 16/11/2026 from 33,249 to 33,388: the product is renamed CrawRAD; rustfmt split
/// lines that grew with the longer `crawrad_*` crate names. No new logic.
/// Raised on 16/11/2026 from 33,241 to 33,249: an installed IDE starts in its own folder, where
/// its fonts are (`main`, `tools/bundle/bundle.py`).
/// Raised on 16/11/2026 from 33,212 to 33,241: a web project's form is a white page in the
/// Designer (`sync_page_look`) with the WEB PAGE badge.
/// Raised on 16/11/2026 from 33,204 to 33,212: rustfmt split the Toolbox greying code over
/// more lines; no new logic.
/// Raised on 15/11/2026 from 33,176 to 33,204: a web project's Toolbox greys what cannot run
/// in the browser, with the reason as the tooltip (`IdeApp::hint_toolbox_row`).
/// Raised on 15/11/2026 from 33,172 to 33,176: the diff review bar and Tab/Esc follow the
/// shown source, not the form's `.crawform` tab (`IdeApp::reviewed_file`).
/// Before, on 11/11/2026 at 32,166 lines: the Tests panel's state lives in
/// `crawrad_project::test_run::TestPanel` and its drawing in `tests_panel_view`; the editor's
/// semantic/decoration readers moved to `CodeDocument`; two long layout notes moved to
/// the shell layout notes.
/// Before, on 11/11/2026 at 32,185 lines: the lists of `draw.rs` share
/// one `draw_rows!` frame, paying for the Run panic panel.
/// Before, on 11/11/2026 at 32,195 lines: dialogs bind their plain controls by
/// name through `crawrad_actions::dialog_fields` (the Build dialog's options, presets,
/// environment and tabs moved to `crawrad_project::build_fields`), paying for Project
/// Options › Log. Before, on 08/11/2026 at 32,230 lines: the double-tap test moved to
/// `crawrad_runtime_makepad::canvas::is_double_tap`, paying for the grids' design-time rows.
/// Before, on 31/10/2026 at 34,089 lines: the Items Editor's commands and field rules moved
/// to `crawrad_actions::items_editor`, its rows to `CrawradFieldRow`. Before, on 29/10/2026 at 34,129 lines: the class icon strip's cells and
/// pixels moved to `crawrad_project::icon_strip`. Before, on 03/10/2026 at
/// 36,906 lines: the Explorer's view model
/// (`crawrad_project::explorer`) joined its rows out of the shell, paying for the
/// editor's context menu and the menus' keyboard.
/// Before, at 33,323 lines: non-UI MCP server and tool dispatch logic moved to `crawrad-mcp`.
/// Lowered on 27/11/2026 from 33,063 to 32,794: the shell handed the REST of the MCP
/// dispatch back to `crawrad-mcp` — `mcp_host.rs` and its `project`, `designer`,
/// `editor` and `build_run` modules now only wire the IDE to the crate, and the
/// editor's buffer, footer and the IDE zoom had their long rationale blocks
/// condensed to what the code itself needs to say.
/// Ratchet rule: This threshold can ONLY DECREASE as code is extracted into crates.
/// The gate enforces equality: if lines decrease, the constant must be lowered in the same PR.
/// Raised on 27/11/2026 from 32,794 to 33,180, measured on this tree: the ACP
/// diagnostic dialog is a new 296-line module, and the three screens account for
/// the remaining 90. The ratchet normally only tightens; it rises here because the
/// shell genuinely gained a surface, not because something was moved into it.
// Raised on 27/11/2026 from 33,180 to 33,210, measured on this tree: the New
// Project dialog gained 30 lines — it now asks `default_project_parent` where to
// point and `make_project_parent` whether the folder can be written, instead of
// taking the working directory and falling through to a file picker. The decision
// itself lives in `crawrad-project`, so the shell holds the wiring and not the rule.
pub const MAX_SHELL_SOURCE_LINES: usize = 33_210;

/// Maximum allowed lines for any individual `.rs` file in `crates/crawrad-ide/src/` (outside `test/`).
///
/// Files exceeding this ceiling must be grandfathered in `SHELL_FILE_LINE_EXCEPTIONS` with their exact line counts.
pub const MAX_SHELL_FILE_LINES: usize = 1_500;

/// Grandfathered files in `crates/crawrad-ide/src/` (outside `test/`) that exceed `MAX_SHELL_FILE_LINES`.
///
/// Ratchet rules:
/// - Each entry records the exact measured line count.
/// - The gate enforces equality: if a file shrinks, its constant must be lowered in the same PR.
/// - If a file grows, the gate fails.
/// - When a file drops to <= `MAX_SHELL_FILE_LINES`, it must be removed from this list.
pub const SHELL_FILE_LINE_EXCEPTIONS: &[(&str, usize)] = &[];

/// Violation reported when a non-UI crate depends on `makepad-widgets`, directly or transitively.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MakepadDependencyViolation {
    pub crate_name: String,
    pub path_or_manifest: String,
    pub is_transitive: bool,
}

/// Violation reported when a crate declares a dependency on `crawrad-ide`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ShellDependencyViolation {
    pub crate_name: String,
    pub cargo_toml_path: String,
}

/// Discrepancy reported when files in the root of `crawrad-ide/src/` don't match the exact allowlist.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShellRootMismatch {
    pub unexpected_files: Vec<String>,
    pub missing_files: Vec<String>,
}

/// Violation reported when the shell line count differs from the ratchet.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LineRatchetMismatch {
    Exceeded { actual: usize, max_allowed: usize },
    Shrank { actual: usize, max_allowed: usize },
}

/// Discrepancy reported when an individual file in `crates/crawrad-ide/src/` violates line ceilings.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ShellFileLineMismatch {
    /// A file not in the exception list exceeds `MAX_SHELL_FILE_LINES`.
    UnregisteredFileExceeded {
        file: String,
        actual: usize,
        max_allowed: usize,
    },
    /// A grandfathered file in the exception list grew beyond its recorded limit.
    ExceptionExceeded {
        file: String,
        actual: usize,
        expected: usize,
    },
    /// A grandfathered file in the exception list shrank — ratchet must be tightened.
    ExceptionShrank {
        file: String,
        actual: usize,
        expected: usize,
    },
    /// An entry in the exception list does not match any file on disk.
    MissingFile { file: String },
}

/// Maximum allowed references to `IdeApp` inside `crates/crawrad-ide/src/ide_app/mcp_host.rs`
/// and any submodules in `crates/crawrad-ide/src/ide_app/mcp_host/**/*.rs`.
///
/// Baseline measured on 29/09/2026: 36 references (ADR 0044, phase 0 of 8).
///
/// # Ratchet Rules
/// - This threshold can ONLY DECREASE as MCP logic migrates to `crawrad-host`.
/// - ONLY references to `IdeApp` replaced by calls to `crawrad-host` count as a valid decrease.
/// - Moving MCP code to other files in `crawrad-ide` is strictly forbidden and does NOT constitute decoupling.
/// - Every PR that lowers this constant must demonstrate in its description the before and after of each migrated snippet.
/// - The gate enforces equality: if references decrease legitimately, the constant must be lowered in the same PR.
/// - In phase 8, `mcp_host.rs` will be deleted from `crawrad-ide` (no longer exists) and `crawrad-mcp` will have 0 `IdeApp` references.
/// - Phase 3 review (30/09/2026): migrated tools (editor.get_active_file, editor.get_selection,
///   editor.list_open_files, project.get_tree, project.get_dependencies, project.add_dependency,
///   build.check, build.build, build.get_diagnostics, run.start, project.build) to &HostModel.
pub const MAX_MCP_HOST_IDEAPP_REFS: usize = 17;

/// Maximum allowed references to `IdeApp` inside `crates/crawrad-extension-host/src/**/*.rs`.
///
/// Baseline measured on 29/09/2026: 0 references (ADR 0044, phase 0 of 8).
/// Ratchet rule: This threshold can ONLY DECREASE (or stay at 0).
/// Extension-host must never acquire `IdeApp` dependencies.
pub const MAX_EXTENSION_HOST_IDEAPP_REFS: usize = 0;

/// Violation reported when `IdeApp` reference counts exceed the ratchet or required paths are missing.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IdeAppRatchetMismatch {
    /// A required source path is missing, unreadable, or contains no `.rs` files.
    PathMissing { path: String, reason: String },
    /// The actual count is greater than the ratchet — coupling increased.
    Exceeded {
        file: String,
        actual: usize,
        max_allowed: usize,
    },
    /// The actual count is less than the ratchet — it MUST be lowered in the same PR.
    Shrank {
        file: String,
        actual: usize,
        max_allowed: usize,
    },
}

/// Counts occurrences of the literal text `IdeApp` in a single source file.
/// Returns an error if reading the file fails.
pub fn count_ideapp_refs_in_file(path: &Path) -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.matches("IdeApp").count())
}

/// Recursively counts occurrences of `IdeApp` across all `.rs` files under a directory tree.
/// Returns `Ok((total_ideapp_refs, total_rs_files_seen))`.
/// Returns an error if reading the directory fails.
pub fn count_ideapp_refs_in_dir(dir: &Path) -> Result<(usize, usize), std::io::Error> {
    let mut total_refs = 0;
    let mut rs_files = 0;
    count_ideapp_refs_in_dir_recursive(dir, &mut total_refs, &mut rs_files)?;
    Ok((total_refs, rs_files))
}

fn count_ideapp_refs_in_dir_recursive(
    dir: &Path,
    total_refs: &mut usize,
    rs_files: &mut usize,
) -> Result<(), std::io::Error> {
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            count_ideapp_refs_in_dir_recursive(&p, total_refs, rs_files)?;
        } else if p.is_file() && p.extension().is_some_and(|ext| ext == "rs") {
            *rs_files += 1;
            let content = std::fs::read_to_string(&p)?;
            *total_refs += content.matches("IdeApp").count();
        }
    }
    Ok(())
}

/// Counts occurrences of `IdeApp` across the entire MCP host tree:
/// `shell_src_dir/ide_app/mcp_host.rs` PLUS any `.rs` files under `shell_src_dir/ide_app/mcp_host/**`.
///
/// Fails with an error message if `mcp_host.rs` does not exist or is unreadable.
pub fn count_mcp_ideapp_tree(shell_src_dir: &Path) -> Result<usize, String> {
    let mcp_file = shell_src_dir.join("ide_app").join("mcp_host.rs");
    if !mcp_file.is_file() {
        return Err(format!(
            "File `{}` not found. The path has changed or the file was moved. If the file was moved or renamed, update the ratchet target path in this PR.",
            mcp_file.display().to_string().replace('\\', "/")
        ));
    }

    let mut total = count_ideapp_refs_in_file(&mcp_file).map_err(|e| {
        format!(
            "Failed to read `{}`: {e}",
            mcp_file.display().to_string().replace('\\', "/")
        )
    })?;

    let mcp_dir = shell_src_dir.join("ide_app").join("mcp_host");
    if mcp_dir.is_dir() {
        let (dir_refs, _) = count_ideapp_refs_in_dir(&mcp_dir).map_err(|e| {
            format!(
                "Failed to read directory `{}`: {e}",
                mcp_dir.display().to_string().replace('\\', "/")
            )
        })?;
        total += dir_refs;
    }

    Ok(total)
}

/// Counts occurrences of `IdeApp` across `crates/crawrad-extension-host/src/**/*.rs`.
///
/// Fails with an error message if the directory does not exist, cannot be read, or contains 0 `.rs` files.
pub fn count_extension_host_ideapp_tree(ext_host_src_dir: &Path) -> Result<usize, String> {
    if !ext_host_src_dir.is_dir() {
        return Err(format!(
            "Directory `{}` not found. The path has changed or the crate was moved. If the crate was moved, update the ratchet target path in this PR.",
            ext_host_src_dir.display().to_string().replace('\\', "/")
        ));
    }

    let (total, rs_files) = count_ideapp_refs_in_dir(ext_host_src_dir).map_err(|e| {
        format!(
            "Failed to read directory `{}`: {e}",
            ext_host_src_dir.display().to_string().replace('\\', "/")
        )
    })?;

    if rs_files == 0 {
        return Err(format!(
            "Directory `{}` contains no `.rs` files. The path has changed or source files were moved.",
            ext_host_src_dir.display().to_string().replace('\\', "/")
        ));
    }

    Ok(total)
}

/// Checks the `IdeApp` reference ratchet for `mcp_host.rs` (+ subfolder) and the extension-host crate.
///
/// Both targets must exist and have counts matching their declared constants:
/// - If path is missing or empty: rejected with `PathMissing` (never counted as 0).
/// - If actual > max_allowed: rejected with `Exceeded` (coupling increased).
/// - If actual < max_allowed: rejected with `Shrank` (ratchet must be lowered in the same PR).
pub fn check_mcp_host_ideapp_ratchet(
    shell_src_dir: &Path,
    extension_host_src_dir: &Path,
    max_mcp: usize,
    max_ext: usize,
) -> Vec<IdeAppRatchetMismatch> {
    let mut mismatches = Vec::new();

    let mcp_label = "crawrad-ide/src/ide_app/mcp_host.rs (and ide_app/mcp_host/**)".to_string();
    match count_mcp_ideapp_tree(shell_src_dir) {
        Ok(mcp_actual) => {
            if mcp_actual > max_mcp {
                mismatches.push(IdeAppRatchetMismatch::Exceeded {
                    file: mcp_label,
                    actual: mcp_actual,
                    max_allowed: max_mcp,
                });
            } else if mcp_actual < max_mcp {
                mismatches.push(IdeAppRatchetMismatch::Shrank {
                    file: mcp_label,
                    actual: mcp_actual,
                    max_allowed: max_mcp,
                });
            }
        }
        Err(reason) => {
            mismatches.push(IdeAppRatchetMismatch::PathMissing {
                path: mcp_label,
                reason,
            });
        }
    }

    let ext_label = "crawrad-extension-host/src/".to_string();
    match count_extension_host_ideapp_tree(extension_host_src_dir) {
        Ok(ext_actual) => {
            if ext_actual > max_ext {
                mismatches.push(IdeAppRatchetMismatch::Exceeded {
                    file: ext_label,
                    actual: ext_actual,
                    max_allowed: max_ext,
                });
            } else if ext_actual < max_ext {
                mismatches.push(IdeAppRatchetMismatch::Shrank {
                    file: ext_label,
                    actual: ext_actual,
                    max_allowed: max_ext,
                });
            }
        }
        Err(reason) => {
            mismatches.push(IdeAppRatchetMismatch::PathMissing {
                path: ext_label,
                reason,
            });
        }
    }

    mismatches
}

/// Extracts real package dependencies from a Cargo.toml text using full TOML parsing.
///
/// Handles normal dependencies, renamed dependencies (`alias = { package = "real-name", ... }`),
/// target-specific dependencies, dev-dependencies (if requested), and build-dependencies.
pub fn extract_effective_dependencies(
    cargo_toml_content: &str,
    include_dev: bool,
) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();
    let Ok(root_table) = toml::from_str::<toml::Table>(cargo_toml_content) else {
        return deps;
    };

    let sections = if include_dev {
        vec!["dependencies", "dev-dependencies", "build-dependencies"]
    } else {
        vec!["dependencies", "build-dependencies"]
    };

    for section in &sections {
        if let Some(tbl) = root_table.get(*section).and_then(|v| v.as_table()) {
            extract_deps_from_table(tbl, &mut deps);
        }
    }

    if let Some(target_tbl) = root_table.get("target").and_then(|v| v.as_table()) {
        for (_target_name, target_val) in target_tbl {
            if let Some(tbl) = target_val.as_table() {
                for section in &sections {
                    if let Some(dep_tbl) = tbl.get(*section).and_then(|v| v.as_table()) {
                        extract_deps_from_table(dep_tbl, &mut deps);
                    }
                }
            }
        }
    }

    deps
}

fn extract_deps_from_table(
    table: &toml::map::Map<String, toml::Value>,
    out: &mut BTreeSet<String>,
) {
    for (key, val) in table {
        if let Some(dep_table) = val.as_table()
            && let Some(pkg_name) = dep_table.get("package").and_then(|p| p.as_str())
        {
            out.insert(pkg_name.to_string());
            continue;
        }
        out.insert(key.clone());
    }
}

/// Checks that non-visual crates never depend on `makepad-widgets`, neither directly nor transitively.
///
/// Looks at production dependencies (dependencies, build-dependencies, target-dependencies).
pub fn check_makepad_dependencies(crates_dir: &Path) -> Vec<MakepadDependencyViolation> {
    let mut violations = Vec::new();
    let Ok(entries) = std::fs::read_dir(crates_dir) else {
        return violations;
    };

    let allowed: BTreeSet<&str> = ALLOWED_MAKEPAD_CRATES.iter().copied().collect();
    let mut graph: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut manifest_paths: BTreeMap<String, PathBuf> = BTreeMap::new();

    // 1. Read all crate manifests and extract real production dependency names
    for entry in entries.flatten() {
        let crate_path = entry.path();
        if !crate_path.is_dir() {
            continue;
        }
        let crate_name = crate_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        let manifest = crate_path.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(&manifest) else {
            continue;
        };

        let deps = extract_effective_dependencies(&content, false);
        graph.insert(crate_name.clone(), deps);
        manifest_paths.insert(crate_name, manifest);
    }

    // 2. Check each non-allowed crate for direct or transitive reachability to makepad-widgets
    for (crate_name, deps) in &graph {
        if allowed.contains(crate_name.as_str()) {
            continue;
        }

        // Direct check
        if deps.contains("makepad-widgets") {
            violations.push(MakepadDependencyViolation {
                crate_name: crate_name.clone(),
                path_or_manifest: manifest_paths
                    .get(crate_name)
                    .map(|p| short_path(p, crates_dir))
                    .unwrap_or_default(),
                is_transitive: false,
            });
            continue;
        }

        // Transitive check via BFS within the workspace
        if let Some(path) = find_makepad_path(crate_name, &graph) {
            violations.push(MakepadDependencyViolation {
                crate_name: crate_name.clone(),
                path_or_manifest: path.join(" -> "),
                is_transitive: true,
            });
        }
    }

    violations.sort();
    violations
}

/// Finds a directed dependency chain from `start_crate` to `makepad-widgets` through workspace crates.
fn find_makepad_path(
    start_crate: &str,
    graph: &BTreeMap<String, BTreeSet<String>>,
) -> Option<Vec<String>> {
    let mut queue: VecDeque<(String, Vec<String>)> = VecDeque::new();
    let mut visited: BTreeSet<String> = BTreeSet::new();

    queue.push_back((start_crate.to_string(), vec![start_crate.to_string()]));
    visited.insert(start_crate.to_string());

    while let Some((current, path)) = queue.pop_front() {
        if let Some(neighbors) = graph.get(&current) {
            if neighbors.contains("makepad-widgets") {
                let mut full_path = path;
                full_path.push("makepad-widgets".to_string());
                return Some(full_path);
            }

            for neighbor in neighbors {
                if graph.contains_key(neighbor) && !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    let mut next_path = path.clone();
                    next_path.push(neighbor.clone());
                    queue.push_back((neighbor.clone(), next_path));
                }
            }
        }
    }

    None
}

/// Checks that no crate in the workspace declares a dependency on `crawrad-ide`.
pub fn check_shell_dependencies(crates_dir: &Path) -> Vec<ShellDependencyViolation> {
    let mut violations = Vec::new();
    let Ok(entries) = std::fs::read_dir(crates_dir) else {
        return violations;
    };

    for entry in entries.flatten() {
        let crate_path = entry.path();
        if !crate_path.is_dir() {
            continue;
        }
        let crate_name = crate_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();

        if crate_name == "crawrad-ide" {
            continue;
        }

        let manifest = crate_path.join("Cargo.toml");
        if !manifest.is_file() {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(&manifest) else {
            continue;
        };

        let deps = extract_effective_dependencies(&content, true);
        if deps.contains("crawrad-ide") {
            violations.push(ShellDependencyViolation {
                crate_name,
                cargo_toml_path: short_path(&manifest, crates_dir),
            });
        }
    }

    violations.sort();
    violations
}

/// Checks that files in the root of `crates/crawrad-ide/src/` exactly match `ALLOWED_SHELL_ROOT_FILES`.
///
/// Both unexpected new files AND missing files that were removed without updating the list are rejected.
pub fn check_shell_root_files(shell_src_dir: &Path) -> Result<(), ShellRootMismatch> {
    let mut actual_files = BTreeSet::new();
    if let Ok(entries) = std::fs::read_dir(shell_src_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file()
                && p.extension().is_some_and(|ext| ext == "rs")
                && let Some(name) = p.file_name().and_then(|n| n.to_str())
            {
                actual_files.insert(name.to_string());
            }
        }
    }

    let allowed: BTreeSet<String> = ALLOWED_SHELL_ROOT_FILES
        .iter()
        .map(|s| s.to_string())
        .collect();

    let unexpected: Vec<String> = actual_files.difference(&allowed).cloned().collect();
    let missing: Vec<String> = allowed.difference(&actual_files).cloned().collect();

    if unexpected.is_empty() && missing.is_empty() {
        Ok(())
    } else {
        Err(ShellRootMismatch {
            unexpected_files: unexpected,
            missing_files: missing,
        })
    }
}

/// Counts total source lines in `crates/crawrad-ide/src/` outside any `test/` directory.
pub fn count_shell_source_lines(shell_src_dir: &Path) -> usize {
    let mut total = 0;
    count_lines_recursive(shell_src_dir, &mut total);
    total
}

/// Evaluates whether the shell line count strictly respects the ratchet.
///
/// If actual > max_allowed: rejected (code was added).
/// If actual < max_allowed: rejected (code was removed, ratchet must be lowered in the same PR).
pub fn check_shell_line_ratchet(
    shell_src_dir: &Path,
    max_allowed: usize,
) -> Result<usize, LineRatchetMismatch> {
    let actual_lines = count_shell_source_lines(shell_src_dir);
    if actual_lines > max_allowed {
        Err(LineRatchetMismatch::Exceeded {
            actual: actual_lines,
            max_allowed,
        })
    } else if actual_lines < max_allowed {
        Err(LineRatchetMismatch::Shrank {
            actual: actual_lines,
            max_allowed,
        })
    } else {
        Ok(actual_lines)
    }
}

/// Recursively traverses directory counting lines of `.rs` files, skipping any path component named `test`.
fn count_lines_recursive(dir: &Path, total: &mut usize) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let is_test_dir = p
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| name == "test");
            if !is_test_dir {
                count_lines_recursive(&p, total);
            }
        } else if p.is_file()
            && p.extension().is_some_and(|ext| ext == "rs")
            && let Ok(content) = std::fs::read_to_string(&p)
        {
            *total += content.lines().count();
        }
    }
}

/// Evaluates whether all individual `.rs` files in `crates/crawrad-ide/src/` (outside `test/`)
/// stay under `MAX_SHELL_FILE_LINES` (1,500 lines) or strictly equal their grandfathered exceptions.
pub fn check_shell_file_lines(
    shell_src_dir: &Path,
    max_file_lines: usize,
    exceptions: &[(&str, usize)],
) -> Vec<ShellFileLineMismatch> {
    let mut mismatches = Vec::new();
    let mut files = Vec::new();
    collect_shell_files(shell_src_dir, &mut files);

    let mut matched_exceptions = HashSet::new();

    for (rel_path, actual_lines) in &files {
        let matching_exc = exceptions.iter().find(|(path, _)| {
            let p = path.replace('\\', "/");
            rel_path == &p || rel_path.ends_with(&format!("/{p}"))
        });

        if let Some((exc_path, expected_lines)) = matching_exc {
            matched_exceptions.insert(*exc_path);
            if *actual_lines > *expected_lines {
                mismatches.push(ShellFileLineMismatch::ExceptionExceeded {
                    file: rel_path.clone(),
                    actual: *actual_lines,
                    expected: *expected_lines,
                });
            } else if *actual_lines < *expected_lines {
                mismatches.push(ShellFileLineMismatch::ExceptionShrank {
                    file: rel_path.clone(),
                    actual: *actual_lines,
                    expected: *expected_lines,
                });
            }
        } else if *actual_lines > max_file_lines {
            mismatches.push(ShellFileLineMismatch::UnregisteredFileExceeded {
                file: rel_path.clone(),
                actual: *actual_lines,
                max_allowed: max_file_lines,
            });
        }
    }

    // Check for entries in exceptions that were never found on disk
    for (exc_path, _) in exceptions {
        if !matched_exceptions.contains(exc_path) {
            mismatches.push(ShellFileLineMismatch::MissingFile {
                file: exc_path.to_string(),
            });
        }
    }

    mismatches
}

fn collect_shell_files(dir: &Path, out: &mut Vec<(String, usize)>) {
    collect_shell_files_recursive(dir, dir, out);
}

fn collect_shell_files_recursive(dir: &Path, root: &Path, out: &mut Vec<(String, usize)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let is_test_dir = p
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|name| name == "test");
            if !is_test_dir {
                collect_shell_files_recursive(&p, root, out);
            }
        } else if p.is_file()
            && p.extension().is_some_and(|ext| ext == "rs")
            && let Ok(content) = std::fs::read_to_string(&p)
        {
            let count = content.lines().count();
            let rel = p
                .strip_prefix(root)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            out.push((rel, count));
        }
    }
}

/// The path from `crates/`, with forward slashes so diagnostics read identically across platforms.
fn short_path(file: &Path, crates_dir: &Path) -> String {
    file.strip_prefix(crates_dir)
        .unwrap_or(file)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
#[path = "test/architecture_gate/test.rs"]
mod tests;
