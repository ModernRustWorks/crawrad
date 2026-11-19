---
id: note-notereport
kind: doc
target_file: src/note.rs
line_hint: 85
author: 
created: 
title: "NoteReport"
tags: []
signature: "pub struct NoteReport"
---

Report of note validation across source files and note definitions.

The report of all notes, orphan notes, dangling anchors, mismatches, stale signatures, and rustdoc problems.

# Examples

```rust
use crawrad_cli::note::NoteReport;

let report = NoteReport {
    notes: Vec::new(),
    orphans: Vec::new(),
    dangling: Vec::new(),
    mismatches: Vec::new(),
    stale: Vec::new(),
    doc_problems: Vec::new(),
};
assert!(report.is_clean());
```
