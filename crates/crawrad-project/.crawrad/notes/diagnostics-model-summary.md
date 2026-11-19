---
id: diagnostics-model-summary
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 275
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary (diags : & [Diagnostic]) -> String"
---

`2 errors · 1 warning`, and only non-zero counts.

# Examples

```rust
use crawrad_project::diagnostics_model::{summary, Diagnostic, Severity};

let diags = vec![Diagnostic {
    severity: Severity::Error,
    message: "err".to_string(),
    location: None,
    rendered: String::new(),
}];
assert_eq!(summary(&diags), "1 error");
```
