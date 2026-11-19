---
id: inlay-inlayworker-take
kind: doc
target_file: src/inlay.rs
line_hint: 200
author: 
created: 
title: "InlayWorker::take"
tags: []
signature: "pub fn take (& self) -> Option < InlayAnswer >"
---

The newest answer that arrived since the last call, if any.

# Examples

```rust
use crawrad_lsp::inlay::InlayWorker;

let worker = InlayWorker::new();
assert!(worker.take().is_none());
```
