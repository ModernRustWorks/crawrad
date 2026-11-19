---
id: hover-hoverworker-set-waker
kind: doc
target_file: src/hover.rs
line_hint: 62
author: 
created: 
title: "HoverWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::hover::HoverWorker;

let worker = HoverWorker::new();
worker.set_waker(|| {});
assert!(worker.take(1).is_none());
```
