---
id: diagnostics-model-severity
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 36
author: 
created: 
title: "Severity"
tags: []
signature: "pub enum Severity"
---

Error or warning.

# Examples

```rust
use crawrad_project::diagnostics_model::Severity;

assert!(Severity::Error < Severity::Warning);
```
