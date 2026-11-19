//! Test Layout and Architecture Gate
//!
//! Enforces the unified workspace test structure across all `crates/` and `components/`:
//!
//! 1. **Unit tests**: `src/test/<module_path>/test.rs`, connected at the end of `<module_path>.rs` via
//!    `#[cfg(test)] #[path = "test/<module_path>/test.rs"] mod tests;`.
//! 2. **Integration tests**: `tests/` at crate root (standard cargo integration tests for public API/adapters).
//!
//! # Forbidden Anti-Patterns
//! - `mod tests { ... }` inline in `.rs` source files.
//! - Loose `tests.rs` or `test.rs` anywhere in `src/` outside `src/test/`.
//! - Flat test files in `src/test/` without a dedicated module folder (e.g. `src/test/<module>.rs`).
//!
//! # Ratchet Rule
//! `MAX_TEST_LAYOUT_VIOLATIONS` can ONLY decrease as crates migrate their tests to the standard.
//! Equality is enforced: when violations decrease, this constant must be lowered in the same PR.

use std::path::Path;

/// Maximum allowed test layout violations across `crates/` and `components/`.
///
/// Baseline measured on 01/10/2026: 98 violations across the workspace.
/// Ratchet rule: This threshold can ONLY DECREASE as tests are standardized.
/// The gate enforces equality: when violations decrease, the constant must be lowered in the same PR.
pub const MAX_TEST_LAYOUT_VIOLATIONS: usize = 85;

/// Kinds of test layout violations.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TestLayoutViolationKind {
    /// An inline test module declared with `{ ... }` inside a source file in `src/`.
    InlineModTests,
    /// A loose `tests.rs` or `test.rs` file inside `src/` outside `src/test/`.
    LooseTestFile,
    /// A test file under `src/test/` that is not named `test.rs` inside a dedicated module directory.
    FlatTestFile,
}

impl TestLayoutViolationKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineModTests => "inline_mod_tests",
            Self::LooseTestFile => "loose_test_file",
            Self::FlatTestFile => "flat_test_file",
        }
    }
}

/// A specific violation of the workspace test layout standard.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TestLayoutViolation {
    pub file: String,
    pub line: usize,
    pub kind: TestLayoutViolationKind,
    pub description: String,
}

/// Discrepancy reported when the total count of test layout violations differs from the ratchet.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TestLayoutRatchetMismatch {
    Exceeded { actual: usize, max_allowed: usize },
    Shrank { actual: usize, max_allowed: usize },
}

/// Strips comments and string literals from Rust source while preserving newline positions.
pub fn strip_comments_and_strings(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let chars: Vec<char> = code.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        // Line comment
        if chars[i] == '/' && i + 1 < n && chars[i + 1] == '/' {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            if i < n {
                out.push('\n');
                i += 1;
            }
            continue;
        }

        // Block comment
        if chars[i] == '/' && i + 1 < n && chars[i + 1] == '*' {
            i += 2;
            let mut depth = 1;
            while i < n && depth > 0 {
                if chars[i] == '/' && i + 1 < n && chars[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if chars[i] == '*' && i + 1 < n && chars[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            continue;
        }

        // Raw string r#"..."#
        if chars[i] == 'r' && (i + 1 < n && (chars[i + 1] == '#' || chars[i + 1] == '"')) {
            i += 1;
            let mut hashes = 0;
            while i < n && chars[i] == '#' {
                hashes += 1;
                i += 1;
            }
            if i < n && chars[i] == '"' {
                i += 1;
                while i < n {
                    if chars[i] == '"' {
                        let mut match_hashes = 0;
                        while i + 1 + match_hashes < n
                            && chars[i + 1 + match_hashes] == '#'
                            && match_hashes < hashes
                        {
                            match_hashes += 1;
                        }
                        if match_hashes == hashes {
                            i += 1 + hashes;
                            break;
                        }
                    }
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
                out.push(' ');
                continue;
            }
        }

        // String literal "..."
        if chars[i] == '"' {
            i += 1;
            while i < n {
                if chars[i] == '\\' && i + 1 < n {
                    i += 2;
                } else if chars[i] == '"' {
                    i += 1;
                    break;
                } else {
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            out.push(' ');
            continue;
        }

        // Char literal 'c'
        if chars[i] == '\'' {
            out.push(' ');
            i += 1;
            while i < n && chars[i] != '\'' && chars[i] != '\n' {
                if chars[i] == '\\' && i + 1 < n {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if i < n && chars[i] == '\'' {
                i += 1;
            }
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    out
}

/// Recursively scans workspace `crates/` and `components/` for test layout violations.
pub fn scan_workspace_test_layout(root_dir: &Path) -> Vec<TestLayoutViolation> {
    let mut violations = Vec::new();
    let scan_dirs = ["crates", "components"];

    for scan_dir in scan_dirs {
        let dir_path = root_dir.join(scan_dir);
        if dir_path.is_dir() {
            walk_dir_for_test_layout(&dir_path, root_dir, &mut violations);
        }
    }

    violations.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));
    violations
}

fn walk_dir_for_test_layout(
    dir: &Path,
    root_dir: &Path,
    violations: &mut Vec<TestLayoutViolation>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip ignored directories
        if name == "target" || name == ".git" || name == "i18n" || name == "vendor" {
            continue;
        }

        if path.is_dir() {
            walk_dir_for_test_layout(&path, root_dir, violations);
        } else if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            let rel_path = path
                .strip_prefix(root_dir)
                .unwrap_or(&path)
                .display()
                .to_string()
                .replace('\\', "/");

            // Only check files inside a crate's `src/` directory (ignore tests/, benches/, examples/, build scripts)
            if !rel_path.contains("/src/") {
                continue;
            }

            // Case 2: Loose tests.rs or test.rs directly in src/ outside src/test/
            if !rel_path.contains("/src/test/") && (name == "tests.rs" || name == "test.rs") {
                violations.push(TestLayoutViolation {
                    file: rel_path.clone(),
                    line: 1,
                    kind: TestLayoutViolationKind::LooseTestFile,
                    description: format!(
                        "Loose `{name}` outside `src/test/` (unit tests must live in `src/test/<module>/test.rs`)"
                    ),
                });
                continue;
            }

            // Case 3: File in src/test/ that is NOT named test.rs (missing dedicated module directory)
            if rel_path.contains("/src/test/") && name != "test.rs" {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
                violations.push(TestLayoutViolation {
                    file: rel_path.clone(),
                    line: 1,
                    kind: TestLayoutViolationKind::FlatTestFile,
                    description: format!(
                        "Test file `{name}` in `src/test/` without dedicated directory (must be `src/test/{stem}/test.rs`)"
                    ),
                });
                continue;
            }

            // Case 1: Inline mod tests { ... } in production source files (outside src/test/)
            if !rel_path.contains("/src/test/")
                && let Ok(content) = std::fs::read_to_string(&path)
            {
                scan_inline_test_modules(&rel_path, &content, violations);
            }
        }
    }
}

/// Scans stripped source code for inline test module definitions `mod tests { ... }`.
fn scan_inline_test_modules(
    file_path: &str,
    code: &str,
    violations: &mut Vec<TestLayoutViolation>,
) {
    let stripped = strip_comments_and_strings(code);
    let lines: Vec<&str> = stripped.lines().collect();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Look for `mod <ident> {` or `pub mod <ident> {` or `pub(crate) mod <ident> {`
        if let Some(mod_pos) = find_mod_keyword(trimmed) {
            let after_mod = trimmed[mod_pos + 3..].trim_start();
            let ident_end = after_mod
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(after_mod.len());
            let mod_name = &after_mod[..ident_end];

            if !mod_name.is_empty() {
                let remainder = after_mod[ident_end..].trim_start();
                // Check if this module opens an inline block `{`
                let opens_block = remainder.starts_with('{') || {
                    // Check if opening brace is on next non-empty line
                    if remainder.is_empty() {
                        lines
                            .get(idx + 1)
                            .map(|l| l.trim().starts_with('{'))
                            .unwrap_or(false)
                    } else {
                        false
                    }
                };

                if opens_block {
                    let is_test_name = is_test_ident(mod_name);
                    let has_cfg_test_above = has_preceding_cfg_test(&lines, idx);

                    if is_test_name || has_cfg_test_above {
                        violations.push(TestLayoutViolation {
                            file: file_path.to_string(),
                            line: idx + 1,
                            kind: TestLayoutViolationKind::InlineModTests,
                            description: format!(
                                "Inline test module `mod {mod_name} {{` (unit tests must live in `src/test/<module>/test.rs`)"
                            ),
                        });
                    }
                }
            }
        }
    }
}

fn find_mod_keyword(s: &str) -> Option<usize> {
    for (i, _) in s.match_indices("mod ") {
        if i == 0 {
            return Some(i);
        }
        let prefix = &s[..i];
        let trimmed_prefix = prefix.trim();
        if trimmed_prefix.is_empty()
            || trimmed_prefix.ends_with("pub")
            || trimmed_prefix.ends_with(')')
        {
            return Some(i);
        }
    }
    None
}

fn is_test_ident(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower == "tests"
        || lower == "test"
        || lower == "testes"
        || lower.ends_with("_test")
        || lower.ends_with("_tests")
        || lower.ends_with("_testes")
        || lower.starts_with("test_")
        || lower.starts_with("tests_")
        || lower.starts_with("testes_")
}

fn has_preceding_cfg_test(lines: &[&str], current_idx: usize) -> bool {
    let mut check_idx = current_idx;
    while check_idx > 0 {
        check_idx -= 1;
        let line = lines[check_idx].trim();
        if line.is_empty() {
            continue;
        }
        if line.contains("cfg(test)") {
            return true;
        }
        if !line.starts_with('#') {
            break;
        }
    }
    false
}

/// Evaluates the test layout ratchet against the current workspace.
pub fn check_test_layout_ratchet(
    root_dir: &Path,
    max_allowed: usize,
) -> Result<usize, TestLayoutRatchetMismatch> {
    let violations = scan_workspace_test_layout(root_dir);
    let actual = violations.len();

    if actual > max_allowed {
        Err(TestLayoutRatchetMismatch::Exceeded {
            actual,
            max_allowed,
        })
    } else if actual < max_allowed {
        Err(TestLayoutRatchetMismatch::Shrank {
            actual,
            max_allowed,
        })
    } else {
        Ok(actual)
    }
}

#[cfg(test)]
#[path = "test/test_layout_gate/test.rs"]
mod tests;
