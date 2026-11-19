---
id: inlay-inlayworker
kind: doc
target_file: src/inlay.rs
line_hint: 127
author: 
created: 
title: "InlayWorker"
tags: []
signature: "pub struct InlayWorker"
---

The thread that asks for the hints.

# Examples

```rust
use crawrad_lsp::inlay::InlayWorker;

let worker = InlayWorker::new();
assert!(worker.take().is_none());
```
