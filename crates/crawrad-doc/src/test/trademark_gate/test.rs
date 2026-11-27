//! The gate itself, plus the proof it can go red.

use super::{MAX_TRADEMARK_MENTIONS, mentions_by_file, mentions_in};
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The files git tracks — what the project ships. A file only on this
/// machine (a build folder, a scratch note) is not the project's.
///
/// Prefers `.gate-tracked` recorded by `gate.ps1` before container runs,
/// then falls back to `git ls-files -z`. Panics if both fail.
fn tracked_files(root: &Path) -> Vec<String> {
    let snapshot = root.join(".gate-tracked");
    if snapshot.is_file()
        && let Ok(bytes) = std::fs::read(&snapshot)
    {
        return String::from_utf8_lossy(&bytes)
            .split('\0')
            .filter(|f| !f.is_empty())
            .map(str::to_string)
            .collect();
    }

    let mut git = std::process::Command::new("git");
    git.arg("-C").arg(root).args(["ls-files", "-z"]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        git.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    match git.output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .split('\0')
            .filter(|f| !f.is_empty())
            .map(str::to_string)
            .collect(),
        _ => panic!(
            "could not obtain repository tracked files: neither .gate-tracked nor \
             `git ls-files -z` succeeded. Without tracked files the trademark \
             measurement is invalid. `gate.ps1` records .gate-tracked before \
             running container tests."
        ),
    }
}

/// **The mentions only go down.**
#[test]
fn trademark_mentions_only_go_down() {
    let root = repo_root();
    let found = mentions_by_file(&root, &tracked_files(&root));
    let total: usize = found.iter().map(|(_, n)| n).sum();
    let mut worst = found.clone();
    worst.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
    let top: Vec<String> = worst
        .iter()
        .take(15)
        .map(|(f, n)| format!("  {n:4} {f}"))
        .collect();
    // An `if`, not `assert!(total <= MAX)`: at the floor of zero that is a
    // comparison clippy rejects as always-true-or-equal.
    if total > MAX_TRADEMARK_MENTIONS {
        panic!(
            "{total} mentions of another vendor's marks, above the ratchet of \
         {MAX_TRADEMARK_MENTIONS}. The owner's rule: none, not even in a \
         file name. Write the idea without the name (\"the classic RAD IDE\").\n{}",
            top.join("\n")
        );
    }
    assert!(
        total == MAX_TRADEMARK_MENTIONS,
        "the mentions went down to {total}: lower MAX_TRADEMARK_MENTIONS from \
         {MAX_TRADEMARK_MENTIONS} to {total} in this PR."
    );
}

/// **A mention is caught, in any case; the library's name only as a word.**
#[test]
fn a_mention_is_counted_and_a_fragment_is_not() {
    let ide = ["Del", "phi"].concat();
    let lib = ["V", "CL"].concat();
    assert_eq!(
        mentions_in(&format!("as in {ide} and {}", ide.to_uppercase())),
        2
    );
    assert_eq!(mentions_in(&format!("a {lib} form, {lib}-like")), 2);
    // Inside a word it is not the library: no false alarm.
    assert_eq!(mentions_in(&format!("x{}y", lib.to_lowercase())), 0);
    assert_eq!(mentions_in("nothing here"), 0);
}

/// **A new mention in a file makes the gate red.**
#[test]
fn a_new_file_with_a_mention_is_caught() {
    let dir = std::env::temp_dir().join(format!("crawrad-tm-gate-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let ide = ["Del", "phi"].concat();
    std::fs::write(dir.join("a.rs"), format!("// the F9 of {ide}\n")).unwrap();
    std::fs::write(dir.join("b.md"), "clean\n").unwrap();
    let found = mentions_by_file(&dir, &["a.rs".into(), "b.md".into(), "gone.rs".into()]);
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(found, vec![("a.rs".to_string(), 1)]);
}

/// **Our own product name is not the vendor's product line; the rest is.**
#[test]
fn our_studio_is_not_counted_and_the_other_marks_are() {
    let line = ["RAD St", "udio"].concat();
    assert_eq!(mentions_in(&format!("Craw{line} IDE")), 0);
    assert_eq!(mentions_in(&format!("measured in {line} 13")), 1);
    let ui = ["F", "MX"].concat();
    assert_eq!(mentions_in(&format!("a {ui} form")), 1);
    assert_eq!(mentions_in(&format!("x{}y", ui.to_lowercase())), 0);
    let tests = ["DUn", "itX"].concat();
    let data = ["Fire", "DAC"].concat();
    assert_eq!(mentions_in(&format!("{tests} and {data}")), 2);
}

/// **A mark in a file's NAME is caught, even in a clean file.**
#[test]
fn a_mark_in_the_file_name_is_caught() {
    let dir = std::env::temp_dir().join(format!("crawrad-tm-path-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let name = format!("keys-{}.md", ["del", "phi"].concat());
    std::fs::write(dir.join(&name), "clean\n").unwrap();
    let found = mentions_by_file(&dir, std::slice::from_ref(&name));
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(found, vec![(name, 1)]);
}
