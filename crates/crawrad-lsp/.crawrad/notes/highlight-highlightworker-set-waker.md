---
id: highlight-highlightworker-set-waker
kind: doc
target_file: src/highlight.rs
line_hint: 155
author: 
created: 
title: "HighlightWorker::set_waker"
tags: []
signature: "pub fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

# Examples

```rust
use crawrad_lsp::highlight::HighlightWorker;

let worker = HighlightWorker::new();
worker.set_waker(|| {});
assert!(worker.poll(std::path::Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0)).is_none());
```
