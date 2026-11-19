---
id: note-extractreport
kind: doc
target_file: src/note.rs
line_hint: 229
author: 
created: 
title: "ExtractReport"
tags: []
signature: "pub struct ExtractReport"
---

What `extract_notes` did (or would do).

# Examples

```rust
use crawrad_cli::note::ExtractReport;

let report = ExtractReport {
    extracted: Vec::new(),
    skipped: Vec::new(),
    cargo_toml_changed: false,
};
assert!(!report.cargo_toml_changed);
```
