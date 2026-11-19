use super::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn next_temp_dir(tag: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "crawrad-duplication-{tag}-{}-{id}",
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
fn duplication_ratchet() {
    let r = root_dir();
    let result = check_duplication_ratchet(&r, MAX_DUPLICATED_WINDOWS);
    match result {
        Ok(count) => {
            println!(
                "Duplication ratchet: {count} duplicated windows (matches ratchet exactly: {MAX_DUPLICATED_WINDOWS})"
            );
        }
        Err(DuplicationRatchetMismatch::Exceeded {
            actual,
            max_allowed,
        }) => {
            let per_crate = scan_workspace_duplication(&r);
            let mut details = String::new();
            for (crate_name, windows) in per_crate {
                if windows.is_empty() {
                    continue;
                }
                details.push_str(&format!(
                    "\n[{crate_name}] ({} duplicated windows):\n",
                    windows.len()
                ));
                for w in windows.iter().take(10) {
                    let sample: Vec<&str> = w.places.iter().take(3).map(|s| s.as_str()).collect();
                    details.push_str(&format!(
                        "  - {} copies at: {}\n",
                        w.places.len(),
                        sample.join(", ")
                    ));
                }
            }

            panic!(
                "\n\n[DUPLICATION GATE] Duplicated code windows exceeded allowed baseline!\n\
                 Actual:      {actual}\n\
                 Max allowed: {max_allowed}\n\
                 Excess:      {} windows above threshold\n\
                 Duplications found per crate:{details}\n\
                 Turn the copies into a function, a DSL template or a macro + trait.\n",
                actual.saturating_sub(max_allowed)
            );
        }
        Err(DuplicationRatchetMismatch::Shrank {
            actual,
            max_allowed,
        }) => {
            panic!(
                "\n\n[DUPLICATION GATE] Duplication ratchet tightened! Duplicated windows decreased!\n\
                 Actual:            {actual}\n\
                 Previous ratchet:  {max_allowed}\n\
                 Reduction:         {} windows eliminated\n\n\
                 Update `MAX_DUPLICATED_WINDOWS` from {max_allowed} to {actual} in `duplication_gate.rs` in this PR.\n",
                max_allowed.saturating_sub(actual)
            );
        }
    }
}

/// A repeated 6-line window appearing 3 times qualifies as duplication.
#[test]
fn three_copies_make_a_window() {
    let temp = next_temp_dir("three-copies");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let block =
        "let a = 100;\nlet b = 200;\nlet c = 300;\nlet d = 400;\nlet e = 500;\nlet f = 600;";
    let content = format!("{block}\n\n{block}\n\n{block}\nlet trailing = 1;\n");
    let file = src_dir.join("a.rs");
    fs::write(&file, content).unwrap();

    let windows = duplicated_windows(&src_dir);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].places.len(), 3);
    assert_eq!(windows[0].places[0], "a.rs:1");
    assert_eq!(windows[0].places[1], "a.rs:8");
    assert_eq!(windows[0].places[2], "a.rs:15");
}

/// A repeated 6-line window appearing only twice does not meet the minimum copy threshold.
#[test]
fn two_copies_do_not() {
    let temp = next_temp_dir("two-copies");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let block =
        "let a = 100;\nlet b = 200;\nlet c = 300;\nlet d = 400;\nlet e = 500;\nlet f = 600;";
    let content = format!("{block}\n\n{block}\nlet trailing = 1;\n");
    let file = src_dir.join("a.rs");
    fs::write(&file, content).unwrap();

    let windows = duplicated_windows(&src_dir);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(windows.len(), 0);
}

/// Test directories/files and windows containing comment lines are excluded from duplication checks.
#[test]
fn tests_and_comments_are_not_counted() {
    let temp = next_temp_dir("tests-and-comments");
    let _ = fs::remove_dir_all(&temp);
    let src_dir = temp.join("src");

    // Case 1: 3 copies inside src/test/x/test.rs -> 0 windows
    let test_dir = src_dir.join("test").join("x");
    fs::create_dir_all(&test_dir).unwrap();
    let block =
        "let a = 100;\nlet b = 200;\nlet c = 300;\nlet d = 400;\nlet e = 500;\nlet f = 600;";
    let test_file = test_dir.join("test.rs");
    fs::write(
        &test_file,
        format!("{block}\n\n{block}\n\n{block}\nlet trailing = 1;\n"),
    )
    .unwrap();

    let windows = duplicated_windows(&src_dir);
    assert_eq!(windows.len(), 0);

    // Case 2: 3 copies of a window that contains a `// comment` line -> 0 windows
    let comment_block =
        "let a = 100;\n// comment line\nlet c = 300;\nlet d = 400;\nlet e = 500;\nlet f = 600;";
    let prod_file = src_dir.join("prod.rs");
    fs::write(
        &prod_file,
        format!("{comment_block}\n\n{comment_block}\n\n{comment_block}\nlet trailing = 1;\n"),
    )
    .unwrap();

    let windows = duplicated_windows(&src_dir);
    let _ = fs::remove_dir_all(&temp);

    assert_eq!(windows.len(), 0);
}
