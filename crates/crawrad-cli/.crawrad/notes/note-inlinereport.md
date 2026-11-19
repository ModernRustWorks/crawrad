---
id: note-inlinereport
kind: doc
target_file: src/note.rs
line_hint: 194
author: 
created: 
title: "InlineReport"
tags: []
signature: "pub struct InlineReport"
---

Report summarizing all inlined notes and skipped anchors.

# Examples

```rust
use crawrad_cli::note::InlineReport;

let report = InlineReport {
    inlined: Vec::new(),
    skipped: Vec::new(),
};
assert!(report.inlined.is_empty());
```
