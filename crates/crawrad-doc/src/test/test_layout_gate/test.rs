use super::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn next_temp_dir(tag: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "crawrad-test-layout-{tag}-{}-{id}",
        std::process::id()
    ))
}

fn root_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("cargo workspace root")
        .to_path_buf()
}

#[test]
fn test_layout_ratchet() {
    let r = root_dir();
    let violations = scan_workspace_test_layout(&r);
    let result = check_test_layout_ratchet(&r, MAX_TEST_LAYOUT_VIOLATIONS);
    match result {
        Ok(count) => {
            println!(
                "Test layout ratchet: {} violations (matching ratchet exactly: {})",
                count, MAX_TEST_LAYOUT_VIOLATIONS
            );
        }
        Err(TestLayoutRatchetMismatch::Exceeded {
            actual,
            max_allowed,
        }) => {
            let mut by_crate: BTreeMap<String, Vec<&TestLayoutViolation>> = BTreeMap::new();
            for v in &violations {
                let parts: Vec<&str> = v.file.split('/').collect();
                let crate_name = if parts.len() >= 2 {
                    format!("{}/{}", parts[0], parts[1])
                } else {
                    v.file.clone()
                };
                by_crate.entry(crate_name).or_default().push(v);
            }

            let mut details = String::new();
            for (crate_name, items) in by_crate {
                details.push_str(&format!("\n[{crate_name}] ({} violations):\n", items.len()));
                for it in items {
                    details.push_str(&format!(
                        "  - [{}] {}:{} -> {}\n",
                        it.kind.as_str(),
                        it.file,
                        it.line,
                        it.description
                    ));
                }
            }

            panic!(
                "\n\n[TEST LAYOUT GATE] Test pattern violations increased!\n\
                 Current: {actual}\n\
                 Maximum: {max_allowed}\n\
                 Excess:  {} violations over!\n\
                 Violations found by crate:{details}\n\
                 Required pattern:\n\
                 - Unit test: src/test/<module_path>/test.rs\n\
                 - Integration test: tests/ at crate root\n\
                 - Forbidden: inline mod tests {{, loose src/tests.rs, and src/test/<module>.rs without a folder.\n",
                actual.saturating_sub(max_allowed)
            );
        }
        Err(TestLayoutRatchetMismatch::Shrank {
            actual,
            max_allowed,
        }) => {
            panic!(
                "\n\n[TEST LAYOUT GATE] The ratchet must tighten! Violations reduced!\n\
                 Current:           {actual}\n\
                 Previous ratchet:  {max_allowed}\n\
                 Reduction reached: {} violations fewer!\n\n\
                 Update `MAX_TEST_LAYOUT_VIOLATIONS` from {max_allowed} to {actual} in `test_layout_gate.rs` in this PR.\n",
                max_allowed.saturating_sub(actual)
            );
        }
    }
}

/// **Mutation Proof 1**: An inline `mod tests { ... }` in a production `.rs` file triggers `InlineModTests`.
#[test]
fn mutation_proof_inline_mod_tests_fails() {
    let temp = next_temp_dir("mutation-inline");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("crates").join("dummy").join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let dummy_file = src_dir.join("service.rs");
    fs::write(
        &dummy_file,
        "pub fn execute() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_execute() {}\n}\n",
    )
    .unwrap();

    let violations = scan_workspace_test_layout(&temp);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].kind, TestLayoutViolationKind::InlineModTests);
    assert!(violations[0].file.ends_with("crates/dummy/src/service.rs"));
    assert_eq!(violations[0].line, 4);
}

/// **Mutation Proof 2**: A loose `tests.rs` or `test.rs` file directly in `src/` triggers `LooseTestFile`.
#[test]
fn mutation_proof_loose_tests_file_fails() {
    let temp = next_temp_dir("mutation-loose");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("crates").join("dummy").join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let loose_file = src_dir.join("tests.rs");
    fs::write(&loose_file, "// loose tests.rs\n").unwrap();

    let violations = scan_workspace_test_layout(&temp);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].kind, TestLayoutViolationKind::LooseTestFile);
    assert!(violations[0].file.ends_with("crates/dummy/src/tests.rs"));
}

/// **Mutation Proof 3**: A test file in `src/test/` not named `test.rs` (flat, without folder) triggers `FlatTestFile`.
#[test]
fn mutation_proof_flat_test_file_fails() {
    let temp = next_temp_dir("mutation-flat");
    let _ = fs::remove_dir_all(&temp);
    let test_dir = temp.join("crates").join("dummy").join("src").join("test");
    fs::create_dir_all(&test_dir).unwrap();

    let flat_file = test_dir.join("inlay.rs");
    fs::write(&flat_file, "// flat inlay test\n").unwrap();

    let violations = scan_workspace_test_layout(&temp);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].kind, TestLayoutViolationKind::FlatTestFile);
    assert!(
        violations[0]
            .file
            .ends_with("crates/dummy/src/test/inlay.rs")
    );
}

/// **Mutation Proof 4**: When violations decrease below ratchet, `check_test_layout_ratchet` returns `Shrank`.
#[test]
fn mutation_proof_ratchet_shrank_demands_update() {
    let temp = next_temp_dir("mutation-shrank");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("crates").join("dummy").join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // 0 violations, but ratchet expects 5
    let result = check_test_layout_ratchet(&temp, 5);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(
        result,
        Err(TestLayoutRatchetMismatch::Shrank {
            actual: 0,
            max_allowed: 5,
        })
    );
}
