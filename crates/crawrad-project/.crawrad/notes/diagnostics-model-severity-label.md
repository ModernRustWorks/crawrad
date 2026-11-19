---
id: diagnostics-model-severity-label
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 45
author: 
created: 
title: "Severity::label"
tags: []
signature: "pub fn label (self) -> & 'static str"
---

UI label.

# Examples

```rust
use crawrad_project::diagnostics_model::Severity;

assert_eq!(Severity::Error.label(), "error");
```
