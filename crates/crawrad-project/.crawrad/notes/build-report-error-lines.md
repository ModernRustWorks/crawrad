---
id: build-report-error-lines
kind: doc
target_file: src/build_report.rs
line_hint: 182
author: 
created: 
title: "error_lines"
tags: []
signature: "pub fn error_lines (diags : & [Diagnostic]) -> Vec < String >"
---

Extracts formatted error lines from compilation diagnostics.

# Examples

```rust
use crawrad_project::build_report::error_lines;

let lines = error_lines(&[]);
assert!(lines.is_empty());
```
