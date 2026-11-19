---
id: diagnostics-model-extract-artifacts
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 234
author: 
created: 
title: "extract_artifacts"
tags: []
signature: "pub fn extract_artifacts (output : & str) -> Vec < CompiledArtifact >"
---

Extracts compiled artifacts emitted by Cargo.

# Examples

```rust
use crawrad_project::diagnostics_model::extract_artifacts;

let arts = extract_artifacts("");
assert!(arts.is_empty());
```
