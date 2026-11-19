---
id: diagnostics-model-diagnosticlocation
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 62
author: 
created: 
title: "DiagnosticLocation"
tags: []
signature: "pub struct DiagnosticLocation"
---

Where a diagnostic points.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::diagnostics_model::DiagnosticLocation;

let loc = DiagnosticLocation {
    file: PathBuf::from("src/main.rs"),
    line: 10,
    column: 5,
};
assert_eq!(loc.line, 10);
```
