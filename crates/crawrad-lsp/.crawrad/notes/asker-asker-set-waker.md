---
id: asker-asker-set-waker
kind: doc
target_file: src/asker.rs
line_hint: 39
author: 
created: 
title: "Asker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::asker::Asker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

let asker = Asker::<i32>::new("worker");
let called = Arc::new(AtomicBool::new(false));
let flag = Arc::clone(&called);
asker.set_waker(move || flag.store(true, Ordering::SeqCst));
assert!(!called.load(Ordering::SeqCst));
```
