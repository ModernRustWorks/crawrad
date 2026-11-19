---
id: signature-signatureworker-set-waker
kind: doc
target_file: src/signature.rs
line_hint: 133
author: 
created: 
title: "SignatureWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::signature::SignatureWorker;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

let worker = SignatureWorker::new();
let flag = Arc::new(AtomicBool::new(false));
let flag_clone = Arc::clone(&flag);
worker.set_waker(move || flag_clone.store(true, Ordering::SeqCst));
assert!(!flag.load(Ordering::SeqCst));
```
