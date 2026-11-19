---
id: diagnostics-model-diagnostic
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 72
author: 
created: 
title: "Diagnostic"
tags: []
signature: "pub struct Diagnostic"
---

A line in the panel.

# Examples

```rust
use crawrad_project::diagnostics_model::{Diagnostic, Severity};

let diag = Diagnostic {
    severity: Severity::Error,
    message: "mismatched types".to_string(),
    location: None,
    rendered: String::new(),
};
assert_eq!(diag.severity, Severity::Error);
```
