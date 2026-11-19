---
id: code-lens-codelensworker-set-waker
kind: doc
target_file: src/code_lens.rs
line_hint: 225
author: 
created: 
title: "CodeLensWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLensWorker;

let worker = CodeLensWorker::new();
worker.set_waker(|| {});
assert!(worker.shown().is_empty());
```
