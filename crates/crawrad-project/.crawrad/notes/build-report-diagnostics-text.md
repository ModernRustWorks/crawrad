---
id: build-report-diagnostics-text
kind: doc
target_file: src/build_report.rs
line_hint: 201
author: 
created: 
title: "diagnostics_text"
tags: []
signature: "pub fn diagnostics_text (diags : & [Diagnostic] , severity : Severity) -> String"
---

Formats diagnostics into tab text for Warnings or Errors.

# Examples

```rust
use crawrad_project::build_report::diagnostics_text;
use crawrad_project::diagnostics_model::Severity;

let text = diagnostics_text(&[], Severity::Error);
assert_eq!(text, "No errors.");
```
