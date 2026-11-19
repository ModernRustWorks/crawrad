---
id: code-lens-codelensworker-shown
kind: doc
target_file: src/code_lens.rs
line_hint: 255
author: 
created: 
title: "CodeLensWorker::shown"
tags: []
signature: "pub fn shown (& self) -> Vec < CodeLens >"
---

A copy of the lenses last returned by `poll`.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLensWorker;

let worker = CodeLensWorker::new();
assert!(worker.shown().is_empty());
```
