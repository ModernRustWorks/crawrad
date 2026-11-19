---
id: resolve-docworker-set-waker
kind: doc
target_file: src/resolve.rs
line_hint: 46
author: 
created: 
title: "DocWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::resolve::DocWorker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

let worker = DocWorker::new();
let flag = Arc::new(AtomicBool::new(false));
let flag_clone = Arc::clone(&flag);
worker.set_waker(move || flag_clone.store(true, Ordering::SeqCst));
assert!(!flag.load(Ordering::SeqCst));
```
