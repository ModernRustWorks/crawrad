---
id: references-referencesworker-set-waker
kind: doc
target_file: src/references.rs
line_hint: 56
author: 
created: 
title: "ReferencesWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::references::ReferencesWorker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

let worker = ReferencesWorker::new();
let flag = Arc::new(AtomicBool::new(false));
let flag_clone = Arc::clone(&flag);
worker.set_waker(move || flag_clone.store(true, Ordering::SeqCst));
assert!(!flag.load(Ordering::SeqCst));
```
