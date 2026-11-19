//! Tests for `crawrad note` scanning, anchor discovery, and report generation.

use crate::{broken_links_from_cargo_json, extract_notes, inline_notes, missing_docs, scan_notes};
use crawrad_note_format::{NoteFile, NoteKind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

struct TempCrate(PathBuf);

impl Drop for TempCrate {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn create_temp_crate(name: &str) -> TempCrate {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "crawrad-test-note-{name}-{}-{id}",
        std::process::id()
    ));
    if dir.exists() {
        let _ = std::fs::remove_dir_all(&dir);
    }
    std::fs::create_dir_all(dir.join("src")).expect("create src dir");
    std::fs::create_dir_all(dir.join(".crawrad").join("notes")).expect("create notes dir");
    std::fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"
"#
        ),
    )
    .expect("write Cargo.toml");
    TempCrate(dir)
}

#[test]
fn clean_crate_reports_no_problems() {
    let tc = create_temp_crate("clean");
    let note = r#"---
id: sum
kind: doc
title: "Sum"
---

Calculates sum.

# Examples

```rust
assert_eq!(sum(1, 2), 3);
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/sum.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("sum")]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.is_clean());
    assert!(!report.has_problems());
    assert_eq!(report.notes.len(), 1);
    assert_eq!(report.notes[0].id, "sum");
    assert_eq!(report.notes[0].kind, NoteKind::Doc);
    assert_eq!(report.notes[0].anchor.as_deref(), Some("src/lib.rs:1"));
    assert!(report.orphans.is_empty());
    assert!(report.dangling.is_empty());
    assert!(report.mismatches.is_empty());
    assert!(report.stale.is_empty());
    assert!(report.doc_problems.is_empty());
}

#[test]
fn orphan_note_reported() {
    let tc = create_temp_crate("orphan");
    let note = r#"---
id: orphan-note
kind: doc
title: "Orphan"
---

Unanchored note.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/orphan-note.md"), note).expect("write note");

    let lib = "pub fn unrelated() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.has_problems());
    assert_eq!(report.orphans.len(), 1);
    assert_eq!(report.orphans[0].id, "orphan-note");
    assert_eq!(report.notes[0].anchor, None);
}

#[test]
fn dangling_anchor_reported() {
    let tc = create_temp_crate("dangling");
    let lib = r#"#[crawrad_note::note("ghost")]
pub fn ghost() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.has_problems());
    assert_eq!(report.dangling.len(), 1);
    assert_eq!(report.dangling[0].id, "ghost");
    assert_eq!(report.dangling[0].line, 1);
}

#[test]
fn anchor_to_internal_note_reported_as_mismatch() {
    let tc = create_temp_crate("internal");
    let note = r#"---
id: internal-note
kind: internal
title: "Internal"
---

Internal note body.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/internal-note.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("internal-note")]
pub fn secret() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.has_problems());
    assert_eq!(report.mismatches.len(), 1);
    assert_eq!(report.mismatches[0].id, "internal-note");
    assert_eq!(report.mismatches[0].line, 1);
}

#[test]
fn stale_signature_reported() {
    let tc = create_temp_crate("stale");
    let note = r#"---
id: compute
kind: doc
title: "Compute"
signature: "fn f(a: i32)"
---

Compute body.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/compute.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("compute")]
fn f(a: i64) {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.has_problems());
    assert_eq!(report.stale.len(), 1);
    assert_eq!(report.stale[0].id, "compute");
    assert!(
        report.stale[0]
            .message
            .contains("was written for `fn f(a: i32)`")
    );
}

#[test]
fn reformatted_signature_is_not_stale() {
    let tc = create_temp_crate("reformat");
    let note = r#"---
id: compute
kind: doc
title: "Compute"
signature: "fn f(a: i32, b: i32) -> i32"
---

Compute body.

# Examples

```rust
let _ = f(1, 2);
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/compute.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("compute")]
fn f(
    a: i32,
    b: i32,
) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.stale.is_empty());
    assert!(report.is_clean());
}

#[test]
fn anchors_inside_impl_and_trait_are_found() {
    let tc = create_temp_crate("impl_trait");
    let note_trait = r#"---
id: trait-method
kind: doc
title: "Trait Method"
---

Trait method doc.

# Examples

```rust
let _ = 1;
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/trait-method.md"), note_trait).expect("write note");

    let note_impl = r#"---
id: impl-method
kind: doc
title: "Impl Method"
---

Impl method doc.

# Examples

```rust
let _ = 1;
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/impl-method.md"), note_impl).expect("write note");

    let lib = r#"trait Worker {
    #[crawrad_note::note("trait-method")]
    fn work(&self) -> &'static str;
}

struct Engine;

impl Worker for Engine {
    #[crawrad_note::note("impl-method")]
    fn work(&self) -> &'static str {
        "engine"
    }
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.is_clean());
    assert_eq!(report.notes.len(), 2);
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "trait-method" && n.anchor.is_some())
    );
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "impl-method" && n.anchor.is_some())
    );
}

#[test]
fn bare_note_and_crawrad_note_both_count() {
    let tc = create_temp_crate("prefixes");
    let note_bare = r#"---
id: bare-fn
kind: doc
title: "Bare Fn"
---

Bare doc.

# Examples

```rust
let _ = 1;
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/bare-fn.md"), note_bare).expect("write note");

    let note_crawrad = r#"---
id: crawrad-fn
kind: doc
title: "Crawrad Fn"
---

Crawrad doc.

# Examples

```rust
let _ = 1;
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/crawrad-fn.md"), note_crawrad).expect("write note");

    let lib = r#"use crawrad_note::note;

#[note("bare-fn")]
pub fn bare_fn() {}

#[crawrad_note::note("crawrad-fn")]
pub fn crawrad_fn() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.is_clean());
    assert_eq!(report.notes.len(), 2);
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "bare-fn" && n.anchor.is_some())
    );
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "crawrad-fn" && n.anchor.is_some())
    );
}

#[test]
fn consistency_scan_over_real_crawrad_registry() {
    let registry_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crawrad-registry");
    let report = scan_notes(&registry_dir).expect("scan real crawrad-registry");
    assert!(
        report.is_clean(),
        "expected crawrad-registry to be clean, got problems:\n{report}"
    );
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "registry-category" && n.anchor.is_some()),
        "registry-category must be found and anchored in crawrad-registry"
    );
    assert!(
        report
            .notes
            .iter()
            .any(|n| n.id == "registry-category-label" && n.anchor.is_some()),
        "registry-category-label must be found and anchored in crawrad-registry"
    );
}

#[test]
fn an_anchor_in_an_integration_test_is_not_an_orphan() {
    let tc = create_temp_crate("in-tests");
    let note = "---\nid: used\nkind: doc\ntitle: \"Used\"\n---\n\nUsed by a test.\n";
    std::fs::write(tc.0.join(".crawrad/notes/used.md"), note).expect("write note");
    std::fs::write(tc.0.join("src/lib.rs"), "pub fn nothing() {}\n").expect("write lib.rs");
    std::fs::create_dir_all(tc.0.join("tests")).expect("create tests dir");
    std::fs::write(
        tc.0.join("tests/it.rs"),
        "#[crawrad_note::note(\"used\")]\nfn used() {}\n",
    )
    .expect("write test");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.orphans.is_empty(), "{report}");
    assert_eq!(report.notes[0].anchor.as_deref(), Some("tests/it.rs:1"));
}

#[test]
fn doc_note_missing_errors_on_result_fn_fails_clean() {
    let tc = create_temp_crate("missing_errors");
    let note = r#"---
id: parse
kind: doc
title: "Parse"
---

Parses the input value.

# Examples

```rust
let _ = parse("123");
```
"#;
    std::fs::write(tc.0.join(".crawrad/notes/parse.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("parse")]
pub fn parse(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e| format!("{e}"))
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(!report.is_clean());
    assert!(report.has_problems());
    assert_eq!(report.doc_problems.len(), 1);
    assert_eq!(report.doc_problems[0].id, "parse");
    assert_eq!(report.doc_problems[0].line, 1);
    assert!(
        report.doc_problems[0]
            .message
            .contains("missing `# Errors`: returns Result")
    );
}

#[test]
fn anchor_to_comment_note_reported_as_mismatch() {
    let tc = create_temp_crate("comment_mismatch");
    let note = r#"---
id: review-comment
kind: comment
title: "Review Comment"
---

A review comment.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/review-comment.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("review-comment")]
pub fn reviewed() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = scan_notes(&tc.0).expect("scan succeeds");
    assert!(report.has_problems());
    assert!(!report.is_clean());
    assert_eq!(report.mismatches.len(), 1);
    assert_eq!(report.mismatches[0].id, "review-comment");
    assert_eq!(report.mismatches[0].kind, NoteKind::Comment);
    assert!(
        report
            .to_string()
            .contains("note is comment; only kind: doc notes become documentation")
    );
}

#[test]
fn crate_root_without_inner_doc_reported_and_with_not() {
    let tc = create_temp_crate("crate-root-doc");
    std::fs::write(tc.0.join("src/lib.rs"), "pub fn ok() {}\n").unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 2);
    assert_eq!(missing[0].kind, "crate");
    assert_eq!(missing[0].name, "crate-root-doc");
    assert_eq!(missing[0].line, 1);
    assert_eq!(missing[1].kind, "fn");
    assert_eq!(missing[1].name, "ok");

    std::fs::write(
        tc.0.join("src/lib.rs"),
        "//! Crate doc.\n/// Ok function.\npub fn ok() {}\n",
    )
    .unwrap();
    let missing2 = missing_docs(&tc.0).expect("scan succeeds");
    assert!(missing2.is_empty());
}

#[test]
fn pub_fn_documentation_status() {
    let tc = create_temp_crate("pub-fn-status");
    let lib = r#"//! Crate docs.
/// Documented function.
pub fn doc_fn() {}

#[crawrad_note::note("note-fn")]
pub fn note_fn() {}

#[note("short-note")]
pub fn short_note_fn() {}

pub fn undoc_fn() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].kind, "fn");
    assert_eq!(missing[0].name, "undoc_fn");
    assert_eq!(missing[0].line, 11);
}

#[test]
fn restricted_visibility_fn_skipped() {
    let tc = create_temp_crate("restricted-vis");
    let lib = r#"//! Crate docs.
pub(crate) fn crate_fn() {}
pub(super) fn super_fn() {}
pub(in crate) fn in_crate_fn() {}
fn private_fn() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert!(missing.is_empty());
}

#[test]
fn private_mod_contents_skipped_and_pub_mod_reported() {
    let tc = create_temp_crate("mod-vis");
    let lib = r#"//! Crate docs.
mod private_mod {
    pub fn secret() {}
}

/// Public module.
pub mod public_mod {
    pub fn visible() {}
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].kind, "fn");
    assert_eq!(missing[0].name, "visible");
    assert_eq!(missing[0].line, 8);
}

#[test]
fn struct_fields_enum_variants_trait_items() {
    let tc = create_temp_crate("types-check");
    let lib = r#"//! Crate docs.
/// A point.
pub struct Point {
    pub x: i32,
    y: i32,
    pub(crate) z: i32,
}

/// An action.
pub enum Action {
    Run,
    /// Stop action.
    Stop,
}

/// A driver trait.
pub trait Driver {
    fn drive(&self);
    type Fuel;
    const SPEED: u32;
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    let kinds_and_names: Vec<(&'static str, &str)> =
        missing.iter().map(|m| (m.kind, m.name.as_str())).collect();
    assert_eq!(
        kinds_and_names,
        vec![
            ("field", "x"),
            ("variant", "Run"),
            ("method", "drive"),
            ("assoc type", "Fuel"),
            ("assoc const", "SPEED"),
        ]
    );
}

#[test]
fn inherent_impl_pub_fn_reported_and_trait_impl_skipped() {
    let tc = create_temp_crate("impl-check");
    let lib = r#"//! Crate docs.
/// S struct.
pub struct S;

impl S {
    pub fn inherent_pub(&self) {}
    fn inherent_priv(&self) {}
    pub const LIMIT: u32 = 100;
    pub type Alias = u32;
}

/// T trait.
pub trait T {
    /// Trait method.
    fn trait_method(&self);
}

impl T for S {
    fn trait_method(&self) {}
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    let kinds_and_names: Vec<(&'static str, &str)> =
        missing.iter().map(|m| (m.kind, m.name.as_str())).collect();
    assert_eq!(
        kinds_and_names,
        vec![
            ("method", "inherent_pub"),
            ("assoc const", "LIMIT"),
            ("assoc type", "Alias"),
        ]
    );
}

#[test]
fn cfg_test_mod_skipped() {
    let tc = create_temp_crate("cfg-test");
    let lib = r#"//! Crate docs.
pub fn normal() {}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn helper() {}
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].kind, "fn");
    assert_eq!(missing[0].name, "normal");
}

#[test]
fn macro_export_reported() {
    let tc = create_temp_crate("macro-check");
    let lib = r#"//! Crate docs.
#[macro_export]
macro_rules! my_macro {
    () => {};
}

macro_rules! internal_macro {
    () => {};
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].kind, "macro");
    assert_eq!(missing[0].name, "my_macro");
    assert_eq!(missing[0].line, 2);
}

#[test]
fn line_numbers_point_at_first_attr_or_keyword() {
    let tc = create_temp_crate("line-check");
    let lib = r#"//! Crate docs.
#[inline]
#[must_use]
pub fn attributed() {}

pub fn plain() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 2);
    assert_eq!(missing[0].name, "attributed");
    assert_eq!(missing[0].line, 2);
    assert_eq!(missing[1].name, "plain");
    assert_eq!(missing[1].line, 6);
}

#[test]
fn external_pub_mod_and_file_scan() {
    let tc = create_temp_crate("ext-mod");
    std::fs::write(tc.0.join("src/lib.rs"), "//! Crate docs.\npub mod x;\n").unwrap();
    std::fs::write(tc.0.join("src/x.rs"), "pub fn inner() {}\n").unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert_eq!(missing.len(), 2);
    assert_eq!(missing[0].file, PathBuf::from("src/lib.rs"));
    assert_eq!(missing[0].kind, "mod");
    assert_eq!(missing[0].name, "x");
    assert_eq!(missing[0].line, 2);
    assert_eq!(missing[1].file, PathBuf::from("src/x.rs"));
    assert_eq!(missing[1].kind, "fn");
    assert_eq!(missing[1].name, "inner");
    assert_eq!(missing[1].line, 1);

    std::fs::write(
        tc.0.join("src/x.rs"),
        "//! Module docs.\n/// Inner docs.\npub fn inner() {}\n",
    )
    .unwrap();
    let missing2 = missing_docs(&tc.0).expect("scan succeeds");
    assert!(missing2.is_empty());
}

#[test]
fn cfg_not_test_and_hidden_items() {
    let tc = create_temp_crate("cfg-not-test");
    let lib = "//! Crate docs.\n#[cfg(not(test))]\npub fn shipped() {}\n\n#[doc(hidden)]\npub fn hidden() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    let names: Vec<&str> = missing.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, vec!["shipped"]);
}

#[test]
fn private_module_file_is_never_read() {
    let tc = create_temp_crate("private-file");
    std::fs::write(tc.0.join("src/lib.rs"), "//! Crate docs.\nmod inner;\n").unwrap();
    std::fs::write(tc.0.join("src/inner.rs"), "pub fn hidden_api() {}\n").unwrap();
    let missing = missing_docs(&tc.0).expect("scan succeeds");
    assert!(missing.is_empty(), "{missing:?}");
}

#[test]
fn doc_note_inlined_and_note_file_deleted() {
    let tc = create_temp_crate("inline-doc");
    let note = r#"---
id: sum
kind: doc
title: "Sum"
---

Calculates sum.
"#;
    let note_path = tc.0.join(".crawrad/notes/sum.md");
    std::fs::write(&note_path, note).expect("write note");

    let lib = r#"#[crawrad_note::note("sum")]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);
    assert_eq!(report.inlined[0].id, "sum");
    assert_eq!(report.inlined[0].file, "src/lib.rs");
    assert_eq!(report.inlined[0].line, 1);
    assert!(report.skipped.is_empty());

    assert!(!note_path.exists());

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read rewritten");
    let expected = r#"/// Calculates sum.
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    assert_eq!(rewritten, expected);
}

#[test]
fn indentation_inside_impl_kept() {
    let tc = create_temp_crate("inline-indent");
    let note = r#"---
id: calc
kind: doc
title: "Calc"
---

Performs calculation.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/calc.md"), note).expect("write note");

    let lib = r#"struct Math;

impl Math {
    #[crawrad_note::note("calc")]
    pub fn calc(&self) -> i32 {
        42
    }
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);
    assert!(report.skipped.is_empty());

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read rewritten");
    let expected = r#"struct Math;

impl Math {
    /// Performs calculation.
    pub fn calc(&self) -> i32 {
        42
    }
}
"#;
    assert_eq!(rewritten, expected);
}

#[test]
fn empty_body_lines_become_bare_triple_slash() {
    let tc = create_temp_crate("inline-bare-doc");
    let note = r#"---
id: multi
kind: doc
title: "Multi"
---

First line.

Second line.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/multi.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("multi")]
pub fn multi() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read rewritten");
    let expected = "/// First line.\n///\n/// Second line.\npub fn multi() {}\n";
    assert_eq!(rewritten, expected);
}

#[test]
fn internal_and_comment_notes_are_skipped() {
    let tc = create_temp_crate("inline-skip-kinds");
    let internal_note = r#"---
id: secret
kind: internal
title: "Secret"
---

Internal docs.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/secret.md"), internal_note).expect("write secret");

    let comment_note = r#"---
id: remark
kind: comment
title: "Remark"
---

Remark docs.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/remark.md"), comment_note).expect("write remark");

    let lib = r#"#[crawrad_note::note("secret")]
pub fn secret() {}

#[crawrad_note::note("remark")]
pub fn remark() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert!(report.inlined.is_empty());
    assert_eq!(report.skipped.len(), 2);
    assert_eq!(report.skipped[0].id, "secret");
    assert_eq!(
        report.skipped[0].reason,
        "note is internal; only doc notes inline"
    );
    assert_eq!(report.skipped[1].id, "remark");
    assert_eq!(
        report.skipped[1].reason,
        "note is comment; only doc notes inline"
    );

    assert!(tc.0.join(".crawrad/notes/secret.md").exists());
    assert!(tc.0.join(".crawrad/notes/remark.md").exists());
    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    assert_eq!(rewritten, lib);
}

#[test]
fn dangling_anchor_skipped() {
    let tc = create_temp_crate("inline-dangling");
    let lib = r#"#[crawrad_note::note("ghost")]
pub fn ghost() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert!(report.inlined.is_empty());
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].id, "ghost");
    assert_eq!(report.skipped[0].reason, "no note file");

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    assert_eq!(rewritten, lib);
}

#[test]
fn attribute_sharing_line_with_code_skipped() {
    let tc = create_temp_crate("inline-sharing-line");
    let note = r#"---
id: a
kind: doc
title: "A"
---

Doc for a.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/a.md"), note).expect("write note");

    let lib = "#[note(\"a\")] pub fn f() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert!(report.inlined.is_empty());
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].id, "a");
    assert_eq!(
        report.skipped[0].reason,
        "attribute shares its line with code"
    );

    assert!(tc.0.join(".crawrad/notes/a.md").exists());
    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    assert_eq!(rewritten, lib);
}

#[test]
fn two_notes_in_one_file_both_inlined_with_correct_lines() {
    let tc = create_temp_crate("inline-two-notes");
    let note1 = r#"---
id: first
kind: doc
title: "First"
---

First note body.
Line 2 of first.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/first.md"), note1).expect("write note1");

    let note2 = r#"---
id: second
kind: doc
title: "Second"
---

Second note body.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/second.md"), note2).expect("write note2");

    let lib = r#"#[crawrad_note::note("first")]
pub fn first() {}

#[crawrad_note::note("second")]
pub fn second() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 2);
    assert_eq!(report.inlined[0].id, "first");
    assert_eq!(report.inlined[0].line, 1);
    assert_eq!(report.inlined[1].id, "second");
    assert_eq!(report.inlined[1].line, 4);

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    let expected = r#"/// First note body.
/// Line 2 of first.
pub fn first() {}

/// Second note body.
pub fn second() {}
"#;
    assert_eq!(rewritten, expected);
}

#[test]
fn use_crawrad_note_removed_when_no_notes_remain_and_kept_when_one_does() {
    let tc_clean = create_temp_crate("inline-remove-use");
    let note_doc = r#"---
id: doc-note
kind: doc
title: "Doc"
---

Doc note.
"#;
    std::fs::write(tc_clean.0.join(".crawrad/notes/doc-note.md"), note_doc).expect("write note");
    let lib_clean = r#"use crawrad_note::note;

#[note("doc-note")]
pub fn f() {}
"#;
    std::fs::write(tc_clean.0.join("src/lib.rs"), lib_clean).expect("write lib");

    let report = inline_notes(&tc_clean.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);
    let rewritten_clean = std::fs::read_to_string(tc_clean.0.join("src/lib.rs")).expect("read lib");
    assert!(!rewritten_clean.contains("use crawrad_note::note;"));
    assert!(rewritten_clean.contains("/// Doc note.\npub fn f() {}"));

    let tc_kept = create_temp_crate("inline-keep-use");
    let note_internal = r#"---
id: internal-note
kind: internal
title: "Internal"
---

Internal note.
"#;
    std::fs::write(
        tc_kept.0.join(".crawrad/notes/internal-note.md"),
        note_internal,
    )
    .expect("write note");
    let lib_kept = r#"use crawrad_note::note;

#[note("internal-note")]
pub fn g() {}
"#;
    std::fs::write(tc_kept.0.join("src/lib.rs"), lib_kept).expect("write lib");

    let report_kept = inline_notes(&tc_kept.0, true).expect("inline succeeds");
    assert_eq!(report_kept.skipped.len(), 1);
    let rewritten_kept = std::fs::read_to_string(tc_kept.0.join("src/lib.rs")).expect("read lib");
    assert!(rewritten_kept.contains("use crawrad_note::note;"));
}

#[test]
fn crlf_file_stays_crlf() {
    let tc = create_temp_crate("inline-crlf");
    let note = r#"---
id: win
kind: doc
title: "Win"
---

Windows doc.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/win.md"), note).expect("write note");

    let lib = "#[crawrad_note::note(\"win\")]\r\npub fn win() {}\r\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    assert!(rewritten.contains("\r\n"));
    assert_eq!(rewritten, "/// Windows doc.\r\npub fn win() {}\r\n");
}

#[test]
fn check_mode_leaves_files_untouched_and_reports_same() {
    let tc = create_temp_crate("inline-check-mode");
    let note = r#"---
id: check-doc
kind: doc
title: "Check Doc"
---

Check doc body.
"#;
    let note_path = tc.0.join(".crawrad/notes/check-doc.md");
    std::fs::write(&note_path, note).expect("write note");

    let lib = "#[crawrad_note::note(\"check-doc\")]\npub fn check_fn() {}\n";
    let lib_path = tc.0.join("src/lib.rs");
    std::fs::write(&lib_path, lib).expect("write lib");

    let report_check = inline_notes(&tc.0, false).expect("check succeeds");
    assert_eq!(report_check.inlined.len(), 1);
    assert_eq!(report_check.inlined[0].id, "check-doc");

    assert!(note_path.exists());
    let current_lib = std::fs::read_to_string(&lib_path).expect("read lib");
    assert_eq!(current_lib, lib);

    let report_write = inline_notes(&tc.0, true).expect("write succeeds");
    assert_eq!(report_check, report_write);
    assert!(!note_path.exists());
}

#[test]
fn rewritten_file_parses_with_syn_and_doc_equals_body() {
    let tc = create_temp_crate("inline-syn-parse");
    let note_body = "Documentation for complex item.\n\n# Section\n\nMore text.";
    let note = format!(
        r#"---
id: complex
kind: doc
title: "Complex"
---

{note_body}
"#
    );
    std::fs::write(tc.0.join(".crawrad/notes/complex.md"), note).expect("write note");

    let lib = r#"#[crawrad_note::note("complex")]
pub fn complex_fn() -> i32 {
    1
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib");

    let report = inline_notes(&tc.0, true).expect("inline succeeds");
    assert_eq!(report.inlined.len(), 1);

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    let syn_file = syn::parse_file(&rewritten).expect("rewritten file parses with syn::parse_file");

    let doc_text = syn_file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Fn(f) => Some(
                f.attrs
                    .iter()
                    .filter(|a| a.path().is_ident("doc"))
                    .map(|a| match &a.meta {
                        syn::Meta::NameValue(nv) => match &nv.value {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(s),
                                ..
                            }) => {
                                let v = s.value();
                                v.strip_prefix(' ').unwrap_or(&v).to_string()
                            }
                            _ => String::new(),
                        },
                        _ => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            _ => None,
        })
        .expect("found fn item");

    assert_eq!(doc_text, note_body);
}

#[test]
fn broken_link_on_note_anchor_line_maps_to_note() {
    let tc = create_temp_crate("broken-link-note");
    let note = r#"---
id: note-sum
kind: doc
title: "Note Sum"
---

Sum function with unresolved link.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/note-sum.md"), note).expect("write note");
    let lib = r#"#[crawrad_note::note("note-sum")]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let json = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::broken_intra_doc_links" },
            "message": "unresolved link to `nonexistent`",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let links = broken_links_from_cargo_json(&json, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].file, "src/lib.rs");
    assert_eq!(links[0].line, 1);
    assert_eq!(links[0].note.as_deref(), Some("note-sum"));
    assert_eq!(links[0].message, "unresolved link to `nonexistent`");
}

#[test]
fn broken_link_on_doc_comment_gives_none_note() {
    let tc = create_temp_crate("broken-link-plain");
    let note = r#"---
id: note-sum
kind: doc
title: "Note Sum"
---

Sum function.
"#;
    std::fs::write(tc.0.join(".crawrad/notes/note-sum.md"), note).expect("write note");
    let lib = r#"#[crawrad_note::note("note-sum")]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// Plain doc comment on line 6.
pub fn plain() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let json = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::broken_intra_doc_links" },
            "message": "unresolved link in plain doc",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 6,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let links = broken_links_from_cargo_json(&json, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].file, "src/lib.rs");
    assert_eq!(links[0].line, 6);
    assert_eq!(links[0].note, None);
    assert_eq!(links[0].message, "unresolved link in plain doc");
}

#[test]
fn other_lint_codes_and_non_json_lines_are_ignored() {
    let tc = create_temp_crate("broken-link-filters");
    let lib = "pub fn f() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let non_json = "   Compiling test-pkg v0.1.0\nRunning `rustdoc`";
    let non_object = "[1, 2, 3]";
    let artifact = serde_json::json!({
        "reason": "compiler-artifact",
        "package_id": "test-pkg 0.1.0"
    })
    .to_string();
    let other_lint = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "unused_variables" },
            "message": "unused variable",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();
    let bare_urls = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::bare_urls" },
            "message": "bare URL found",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();
    let private_intra_doc = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::private_intra_doc_links" },
            "message": "public item refers to private item",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let stdout = format!(
        "{non_json}\n{non_object}\n{artifact}\n{other_lint}\n{bare_urls}\n{private_intra_doc}\n"
    );
    let links = broken_links_from_cargo_json(&stdout, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].message, "public item refers to private item");
}

#[test]
fn windows_backslash_paths_normalised() {
    let tc = create_temp_crate("broken-link-win-path");
    let lib = "pub fn f() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let json = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::broken_intra_doc_links" },
            "message": "unresolved link",
            "spans": [{
                "file_name": "src\\lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let links = broken_links_from_cargo_json(&json, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].file, "src/lib.rs");
}

#[test]
fn duplicates_collapsed() {
    let tc = create_temp_crate("broken-link-dedup");
    let lib = "pub fn f() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let msg = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::broken_intra_doc_links" },
            "message": "duplicate error",
            "spans": [{
                "file_name": "src/lib.rs",
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let stdout = format!("{msg}\n{msg}\n");
    let links = broken_links_from_cargo_json(&stdout, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
}

#[test]
fn absolute_path_made_relative() {
    let tc = create_temp_crate("broken-link-abs-path");
    let lib = "pub fn f() {}\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let abs_file = tc.0.join("src/lib.rs");
    let abs_str = abs_file.to_string_lossy();

    let json = serde_json::json!({
        "reason": "compiler-message",
        "message": {
            "code": { "code": "rustdoc::broken_intra_doc_links" },
            "message": "unresolved link",
            "spans": [{
                "file_name": abs_str,
                "line_start": 1,
                "is_primary": true
            }]
        }
    })
    .to_string();

    let links = broken_links_from_cargo_json(&json, &tc.0).expect("parse broken links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].file, "src/lib.rs");
}

#[test]
fn extract_fn_with_rustdoc_pattern() {
    let tc = create_temp_crate("extract-fn");
    let lib = r#"/// Computes sum of two numbers.
///
/// # Examples
///
/// ```rust
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert!(report.skipped.is_empty());
    assert_eq!(report.extracted[0].id, "add");
    assert_eq!(report.extracted[0].file, "src/lib.rs");
    assert_eq!(report.extracted[0].item, "add");

    let note_path = tc.0.join(".crawrad/notes/add.md");
    assert!(note_path.is_file());
    let md = std::fs::read_to_string(&note_path).expect("read note file");
    let parsed = NoteFile::parse(&md).expect("parse note file");
    assert_eq!(parsed.kind, NoteKind::Doc);
    assert_eq!(
        parsed.body,
        "Computes sum of two numbers.\n\n# Examples\n\n```rust\nassert_eq!(add(1, 2), 3);\n```"
    );
    assert_eq!(
        parsed.signature,
        crawrad_note_format::item_signature(&quote::quote!(
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        ))
    );

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read rewritten");
    assert!(rewritten.contains(r#"#[note("add")]"#));
    assert!(rewritten.contains("use crawrad_note::note;"));
}

#[test]
fn extract_and_inline_round_trip() {
    let sample = r#"use std::fmt::Debug;

/// A 2D point representation.
///
/// # Examples
///
/// ```rust
/// let p = Point { x: 1, y: 2 };
/// ```
#[derive(Debug)]
pub struct Point {
    /// Field x coordinate.
    pub x: i32,
    /// Field y coordinate.
    pub y: i32,
}

impl Point {
    /// Constructs a origin point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let p = Point::origin();
    /// ```
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

mod helpers {
    /// Helper calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// helpers::calc();
    /// ```
    pub fn calc() {}
}
"#;

    // Run 1: LF
    {
        let tc = create_temp_crate("round-trip-lf");
        std::fs::write(tc.0.join("src/lib.rs"), sample).expect("write lf lib");

        let extract_rep = extract_notes(&tc.0, true, false).expect("extract succeeds");
        assert_eq!(extract_rep.extracted.len(), 3);

        let inline_rep = inline_notes(&tc.0, true).expect("inline succeeds");
        assert_eq!(inline_rep.inlined.len(), 3);

        let after = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read after");
        assert_eq!(after, sample);
    }

    // Run 2: CRLF
    {
        let tc = create_temp_crate("round-trip-crlf");
        let sample_crlf = sample.replace('\n', "\r\n");
        std::fs::write(tc.0.join("src/lib.rs"), &sample_crlf).expect("write crlf lib");

        let extract_rep = extract_notes(&tc.0, true, false).expect("extract succeeds");
        assert_eq!(extract_rep.extracted.len(), 3);

        let inline_rep = inline_notes(&tc.0, true).expect("inline succeeds");
        assert_eq!(inline_rep.inlined.len(), 3);

        let after = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read after");
        assert_eq!(after, sample_crlf);
    }
}

#[test]
fn skips_reported_with_reasons() {
    let tc = create_temp_crate("extract-skips");
    let lib = r#"//! Module inner doc.

#[doc = "custom attribute doc"]
pub fn doc_attr() {}

/// Summary with no period
///
/// # Examples
///
/// ```rust
/// test();
/// ```
pub fn no_period() {}

pub struct Container {
    /// Field doc stays untouched.
    pub field: i32,
}

#[cfg(test)]
mod tests {
    /// Helper in test.
    ///
    /// # Examples
    ///
    /// ```rust
    /// helper();
    /// ```
    pub fn helper() {}
}

/// Already has note.
///
/// # Examples
///
/// ```rust
/// already();
/// ```
#[note("already")]
pub fn already() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert!(report.extracted.is_empty());
    assert_eq!(report.skipped.len(), 4);

    let doc_attr_skip = report
        .skipped
        .iter()
        .find(|s| s.item == "doc_attr")
        .unwrap();
    assert_eq!(
        doc_attr_skip.reason,
        "doc attribute is not a /// line comment"
    );

    let no_period_skip = report
        .skipped
        .iter()
        .find(|s| s.item == "no_period")
        .unwrap();
    assert!(no_period_skip.reason.contains("ending with a period"));

    let test_skip = report.skipped.iter().find(|s| s.item == "helper").unwrap();
    assert_eq!(test_skip.reason, "inside #[cfg(test)] module");

    let already_skip = report.skipped.iter().find(|s| s.item == "already").unwrap();
    assert_eq!(already_skip.reason, "already has #[note]");

    // Container's field doc is untouched and not in skipped
    assert!(
        !report
            .skipped
            .iter()
            .any(|s| s.item == "field" || s.item == "Container::field")
    );

    // When --all is passed, no_period is extracted
    let report_all = extract_notes(&tc.0, true, true).expect("extract --all succeeds");
    assert_eq!(report_all.extracted.len(), 1);
    assert_eq!(report_all.extracted[0].item, "no_period");
}

#[test]
fn id_collision_increments_suffix() {
    let tc = create_temp_crate("extract-collision");
    std::fs::write(tc.0.join("src/lib.rs"), "mod doc;\n").expect("write lib.rs");
    std::fs::write(
        tc.0.join(".crawrad/notes/doc-parse-date.md"),
        "---\nid: doc-parse-date\nkind: doc\ntitle: \"Existing\"\n---\n\nExisting.",
    )
    .expect("write existing note");

    let doc_rs = r#"/// Parses date string.
///
/// # Examples
///
/// ```rust
/// parse_date();
/// ```
pub fn parse_date() {}
"#;
    std::fs::write(tc.0.join("src/doc.rs"), doc_rs).expect("write doc.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert_eq!(report.extracted[0].id, "doc-parse-date-2");
    assert!(tc.0.join(".crawrad/notes/doc-parse-date-2.md").is_file());
}

#[test]
fn dry_run_leaves_files_identical_and_creates_no_notes() {
    let tc = create_temp_crate("extract-dry-run");
    let lib = r#"/// Simple function.
///
/// # Examples
///
/// ```rust
/// simple();
/// ```
pub fn simple() {}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");
    let original_lib = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    let original_cargo = std::fs::read_to_string(tc.0.join("Cargo.toml")).expect("read cargo");

    let report = extract_notes(&tc.0, false, false).expect("extract dry run succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert_eq!(report.extracted[0].id, "simple");
    assert!(report.cargo_toml_changed);

    let current_lib = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib");
    let current_cargo = std::fs::read_to_string(tc.0.join("Cargo.toml")).expect("read cargo");
    assert_eq!(current_lib, original_lib);
    assert_eq!(current_cargo, original_cargo);

    let notes_dir = tc.0.join(".crawrad/notes");
    let md_count = std::fs::read_dir(&notes_dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
                .count()
        })
        .unwrap_or(0);
    assert_eq!(md_count, 0);
}

#[test]
fn cargo_toml_dependency_handling() {
    // 6a: Standalone crate gets version string, comments survive, running twice adds once
    {
        let tc = create_temp_crate("extract-cargo-standalone");
        let initial_cargo = r#"# Top comment in Cargo.toml
[package]
name = "extract-cargo-standalone"
version = "0.1.0"
edition = "2024"

# Section comment
[dependencies]
# Existing dep comment
serde = "1.0"
"#;
        std::fs::write(tc.0.join("Cargo.toml"), initial_cargo).expect("write cargo");
        let lib = r#"/// Simple item.
///
/// # Examples
///
/// ```rust
/// test();
/// ```
pub fn test() {}
"#;
        std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib");

        // First run: adds crawrad-note = "0.1.0"
        let rep1 = extract_notes(&tc.0, true, false).expect("first extract succeeds");
        assert!(rep1.cargo_toml_changed);
        let cargo1 = std::fs::read_to_string(tc.0.join("Cargo.toml")).expect("read cargo1");
        assert!(cargo1.contains("# Top comment in Cargo.toml"));
        assert!(cargo1.contains("# Section comment"));
        assert!(cargo1.contains("# Existing dep comment"));
        assert!(cargo1.contains("crawrad-note = \"0.1.0\""));

        // Second run: dependency already present, not added twice
        let rep2 = extract_notes(&tc.0, true, false).expect("second extract succeeds");
        assert!(!rep2.cargo_toml_changed);
        let cargo2 = std::fs::read_to_string(tc.0.join("Cargo.toml")).expect("read cargo2");
        assert_eq!(cargo1, cargo2);
        assert_eq!(cargo2.matches("crawrad-note").count(), 1);
    }

    // 6b: Workspace crate chooses workspace = true when root declares crawrad-note in [workspace.dependencies]
    {
        static WS_COUNTER: AtomicU64 = AtomicU64::new(100);
        let ws_id = WS_COUNTER.fetch_add(1, Ordering::Relaxed);
        let ws_dir =
            std::env::temp_dir().join(format!("crawrad-test-ws-{}-{ws_id}", std::process::id()));
        let pkg_dir = ws_dir.join("member");
        std::fs::create_dir_all(pkg_dir.join("src")).expect("create pkg src");
        std::fs::create_dir_all(pkg_dir.join(".crawrad/notes")).expect("create notes");

        std::fs::write(
            ws_dir.join("Cargo.toml"),
            r#"[workspace]
members = ["member"]

[workspace.dependencies]
crawrad-note = { version = "0.1.0", path = "dummy" }
"#,
        )
        .expect("write ws cargo");

        std::fs::write(
            pkg_dir.join("Cargo.toml"),
            r#"[package]
name = "member"
version = "0.1.0"
edition = "2024"
"#,
        )
        .expect("write pkg cargo");

        let lib = r#"/// In member pkg.
///
/// # Examples
///
/// ```rust
/// member_fn();
/// ```
pub fn member_fn() {}
"#;
        std::fs::write(pkg_dir.join("src/lib.rs"), lib).expect("write member lib");

        let rep = extract_notes(&pkg_dir, true, false).expect("extract in ws succeeds");
        assert!(rep.cargo_toml_changed);

        let cargo_pkg =
            std::fs::read_to_string(pkg_dir.join("Cargo.toml")).expect("read pkg cargo");
        assert!(cargo_pkg.contains("crawrad-note = { workspace = true }"));

        let _ = std::fs::remove_dir_all(&ws_dir);
    }
}

#[test]
fn cfg_test_out_of_line_mod_is_never_entered() {
    let tc = create_temp_crate("cfg-test-out-of-line");
    let lib = "#[cfg(test)]\nmod tests;\n";
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let tests_content = r#"/// Private helper in tests.
fn helper() {}
"#;
    std::fs::write(tc.0.join("src/tests.rs"), tests_content).expect("write tests.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert!(report.extracted.is_empty());
    assert!(report.skipped.is_empty());

    let tests_after = std::fs::read_to_string(tc.0.join("src/tests.rs")).expect("read tests.rs");
    assert_eq!(tests_after, tests_content);
}

#[test]
fn documented_rustdoc_clean_private_fn_is_extracted() {
    let tc = create_temp_crate("clean-private-fn");
    let lib = r#"/// Calculates the internal sum.
fn internal_sum(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert!(report.skipped.is_empty());
    assert_eq!(report.extracted[0].id, "internal-sum");
    assert_eq!(report.extracted[0].item, "internal_sum");

    let note_path = tc.0.join(".crawrad/notes/internal-sum.md");
    assert!(note_path.is_file());
    let md = std::fs::read_to_string(&note_path).expect("read note");
    let parsed = NoteFile::parse(&md).expect("parse note");
    assert_eq!(parsed.kind, NoteKind::Doc);
    assert_eq!(parsed.body, "Calculates the internal sum.");

    let rewritten = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read rewritten");
    assert!(rewritten.contains(r#"#[note("internal-sum")]"#));
    assert!(rewritten.contains("use crawrad_note::note;"));
}

#[test]
fn pub_fn_without_examples_is_skipped_with_reason() {
    let tc = create_temp_crate("pub-fn-no-examples");
    let lib = r#"/// Public calculation function.
pub fn public_calc(a: i32) -> i32 {
    a * 2
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert!(report.extracted.is_empty());
    assert_eq!(report.skipped.len(), 1);
    assert_eq!(report.skipped[0].item, "public_calc");
    assert!(
        report.skipped[0].reason.contains("missing `# Examples`"),
        "expected reason to contain 'missing `# Examples`', got: {}",
        report.skipped[0].reason
    );
}

#[test]
fn module_reached_through_path_attribute_uses_module_name() {
    let tc = create_temp_crate("path-attr-mod");
    let lib = r#"#[path = "other_name.rs"]
mod real;
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let other_content = r#"/// Helper inside renamed module.
fn helper() {}
"#;
    std::fs::write(tc.0.join("src/other_name.rs"), other_content).expect("write other_name.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert!(report.skipped.is_empty());
    assert_eq!(report.extracted[0].id, "real-helper");
    assert_eq!(report.extracted[0].file, "src/other_name.rs");
}

#[test]
fn bin_crate_root_items_start_with_bin_prefix() {
    let tc = create_temp_crate("bin-prefix");
    std::fs::create_dir_all(tc.0.join("src/bin")).expect("create src/bin");
    let bin_content = r#"/// Entry point helper.
fn start() {}
"#;
    std::fs::write(tc.0.join("src/bin/tool.rs"), bin_content).expect("write src/bin/tool.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1);
    assert_eq!(report.extracted[0].id, "bin-tool-start");
    assert_eq!(report.extracted[0].file, "src/bin/tool.rs");
}

#[test]
fn lib_and_main_both_walked_and_shared_mod_visited_once() {
    let tc = create_temp_crate("lib-and-main");
    let lib = "mod shared;\n/// Lib function.\nfn lib_fn() {}\n";
    let main = "mod shared;\n/// Main function.\nfn main_fn() {}\n";
    let shared = "/// Shared function.\nfn shared_fn() {}\n";

    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");
    std::fs::write(tc.0.join("src/main.rs"), main).expect("write main.rs");
    std::fs::write(tc.0.join("src/shared.rs"), shared).expect("write shared.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 3);
    let ids: Vec<&str> = report.extracted.iter().map(|e| e.id.as_str()).collect();
    assert!(ids.contains(&"lib-fn"));
    assert!(ids.contains(&"main-fn"));
    assert!(ids.contains(&"shared-shared-fn"));
}

#[test]
fn an_impl_on_an_array_type_gets_a_valid_id() {
    let tc = create_temp_crate("extract-array-impl");
    let lib = r#"pub struct Color;

impl From<Color> for [u8; 4] {
    /// Converts the colour into its four channels.
    fn from(_: Color) -> Self {
        [0; 4]
    }
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 1, "{report:?}");
    assert_eq!(report.extracted[0].id, "u8-4-from");
    assert!(crawrad_note_format::validate_id(&report.extracted[0].id).is_ok());
}

#[test]
fn cfg_twin_modules_each_get_the_use_line() {
    let tc = create_temp_crate("extract-cfg-twins");
    let lib = r#"#[cfg(windows)]
mod window {
    /// Opens the window.
    pub(crate) fn open() {}
}

#[cfg(not(windows))]
mod window {
    /// Opens nothing.
    pub(crate) fn open() {}
}
"#;
    std::fs::write(tc.0.join("src/lib.rs"), lib).expect("write lib.rs");

    let report = extract_notes(&tc.0, true, false).expect("extract succeeds");
    assert_eq!(report.extracted.len(), 2, "{report:?}");
    let text = std::fs::read_to_string(tc.0.join("src/lib.rs")).expect("read lib.rs");
    assert_eq!(text.matches("use crawrad_note::note;").count(), 2, "{text}");
}
