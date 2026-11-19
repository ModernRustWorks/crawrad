---
id: code-lens-codelensworker
kind: doc
target_file: src/code_lens.rs
line_hint: 194
author: 
created: 
title: "CodeLensWorker"
tags: []
signature: "pub struct CodeLensWorker"
---

The thread that asks for and resolves code lenses.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLensWorker;

let worker = CodeLensWorker::new();
assert!(worker.shown().is_empty());
```
