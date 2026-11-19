---
id: note-notereport-is-clean
kind: doc
target_file: src/note.rs
line_hint: 97
author: 
created: 
title: "NoteReport::is_clean"
tags: []
signature: "pub fn is_clean (& self) -> bool"
---

Returns true when the report found no issues or broken anchors.

True when there are no orphans, dangling anchors, kind mismatches, stale signatures, or rustdoc problems.

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
