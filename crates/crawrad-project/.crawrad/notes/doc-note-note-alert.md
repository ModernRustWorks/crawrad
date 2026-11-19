---
id: doc-note-note-alert
kind: doc
target_file: src/doc_note.rs
line_hint: 819
author: 
created: 
title: "note_alert"
tags: []
signature: "pub fn note_alert (source : & str , line : usize , note : & SidecarNote) -> bool"
---

Returns whether a documentation note anchor is in an alert state.

Whether an anchor is in alert: `true` when the note is `kind: doc` and
EITHER `stale_signature` reports a changed signature vs `note.signature`
OR `doc_problems` is non-empty. Internal and comment notes are never in alert.

# Examples

```rust
use crawrad_project::doc_note::note_alert;
use crawrad_project::sidecar_notes::SidecarNote;

let note = SidecarNote::new("my-note".into(), "src/lib.rs", 1);
assert!(!note_alert("fn foo() {}", 0, &note));
```
