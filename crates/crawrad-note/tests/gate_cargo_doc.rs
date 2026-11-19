#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// A `cargo` runner inside the pilot project, without the outer cargo's target dir and rustflags.
fn cargo(dest: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(dest)
        .output()
        .expect("run cargo inside the pilot project");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

#[test]
#[ignore = "runs cargo doc on a pilot crate; the gate runs it with --include-ignored"]
fn cargo_doc_embeds_note_and_tracks_rebuild() {
    let pilot_dir: PathBuf = Path::new(env!("CARGO_TARGET_TMPDIR")).join("note-doc-pilot");
    let src_dir = pilot_dir.join("src");
    let notes_dir = pilot_dir.join(".crawrad").join("notes");

    std::fs::create_dir_all(&src_dir).expect("create src dir");
    std::fs::create_dir_all(&notes_dir).expect("create .crawrad/notes dir");

    // The `include` is load-bearing, measured on cargo 1.95: rustdoc writes no
    // dep-info, so `cargo doc` decides "fresh" from the package's file list.
    // A hidden `.crawrad/` outside git is not on that list, and without the
    // `include` the second run below keeps the OLD sentence. `cargo build` is
    // not affected: there the macro's `include_str!` is in the dep-info.
    let crawrad_note_path = env!("CARGO_MANIFEST_DIR").replace('\\', "/");
    let cargo_toml = format!(
        r#"[package]
name = "note-doc-pilot"
version = "0.1.0"
edition = "2024"
include = ["src/**", ".crawrad/notes/*.md", "Cargo.toml"]

[workspace]

[dependencies]
crawrad-note = {{ path = "{crawrad_note_path}" }}
"#
    );
    std::fs::write(pilot_dir.join("Cargo.toml"), cargo_toml).expect("write pilot Cargo.toml");

    let outer_lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.lock");
    if outer_lock.exists() {
        std::fs::copy(&outer_lock, pilot_dir.join("Cargo.lock")).expect("copy Cargo.lock");
    }

    let note_path = notes_dir.join("pilot-sum.md");
    let initial_sentence = "The pilot sentence that only the note carries.";
    let initial_note = format!(
        r#"---
id: pilot-sum
kind: doc
title: "Pilot"
---

{initial_sentence}

# Examples

```rust
use note_doc_pilot::pilot_sum;

assert_eq!(pilot_sum(2, 3), 5);
```
"#
    );
    std::fs::write(&note_path, initial_note).expect("write initial pilot note");

    let lib_rs = r#"#![forbid(unsafe_code)]

#[crawrad_note::note("pilot-sum")]
pub fn pilot_sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(src_dir.join("lib.rs"), lib_rs).expect("write pilot lib.rs");

    let (ok, out) = cargo(&pilot_dir, &["doc", "--no-deps"]);
    assert!(ok, "`cargo doc --no-deps` failed:\n{out}");

    let html_path = pilot_dir
        .join("target")
        .join("doc")
        .join("note_doc_pilot")
        .join("fn.pilot_sum.html");
    assert!(
        html_path.exists(),
        "expected doc HTML at {}",
        html_path.display()
    );
    let html = std::fs::read_to_string(&html_path).expect("read doc html");
    assert!(
        html.contains(initial_sentence),
        "initial sentence missing from doc HTML:\n{html}"
    );
    assert!(
        !html.contains("kind: doc"),
        "doc HTML must not contain front matter `kind: doc`"
    );
    assert!(
        !html.contains("id: pilot-sum"),
        "doc HTML must not contain front matter `id: pilot-sum`"
    );

    // Ensure mtime ticks forward before rewriting
    sleep(Duration::from_millis(50));

    let updated_sentence = "The updated sentence after the rewrite.";
    let updated_note = format!(
        r#"---
id: pilot-sum
kind: doc
title: "Pilot"
---

{updated_sentence}

# Examples

```rust
use note_doc_pilot::pilot_sum;

assert_eq!(pilot_sum(10, 20), 30);
```
"#
    );
    std::fs::write(&note_path, updated_note).expect("write updated pilot note");

    let (ok2, out2) = cargo(&pilot_dir, &["doc", "--no-deps"]);
    assert!(ok2, "second `cargo doc --no-deps` failed:\n{out2}");

    let html2 = std::fs::read_to_string(&html_path).expect("read updated doc html");
    assert!(
        html2.contains(updated_sentence),
        "updated sentence missing from doc HTML:\n{html2}"
    );
    assert!(
        !html2.contains(initial_sentence),
        "old sentence still present in doc HTML:\n{html2}"
    );
}
