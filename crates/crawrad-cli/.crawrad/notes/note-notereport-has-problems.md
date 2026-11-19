---
id: note-notereport-has-problems
kind: doc
target_file: src/note.rs
line_hint: 106
author: 
created: 
title: "NoteReport::has_problems"
tags: []
signature: "pub fn has_problems (& self) -> bool"
---

True when any issue was found.

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
assert!(!report.has_problems());
```
