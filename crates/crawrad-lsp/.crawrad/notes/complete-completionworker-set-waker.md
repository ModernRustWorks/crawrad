---
id: complete-completionworker-set-waker
kind: doc
target_file: src/complete.rs
line_hint: 134
author: 
created: 
title: "CompletionWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::complete::CompletionWorker;

let worker = CompletionWorker::new();
worker.set_waker(|| {});
assert!(worker.take(1).is_none());
```
