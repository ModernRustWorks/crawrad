---
id: code-action-codeactionworker-set-waker
kind: doc
target_file: src/code_action.rs
line_hint: 144
author: 
created: 
title: "CodeActionWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::code_action::CodeActionWorker;

let worker = CodeActionWorker::new();
worker.set_waker(|| {});
assert!(worker.take(1).is_none());
```
