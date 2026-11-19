---
id: inlay-inlayworker-set-waker
kind: doc
target_file: src/inlay.rs
line_hint: 168
author: 
created: 
title: "InlayWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::inlay::InlayWorker;

let worker = InlayWorker::new();
worker.set_waker(|| {});
assert!(worker.take().is_none());
```
