---
id: resolve-docworker
kind: doc
target_file: src/resolve.rs
line_hint: 32
author: 
created: 
title: "DocWorker"
tags: []
signature: "pub struct DocWorker (crate :: asker :: Asker < DocAnswer >)"
---

The thread that resolves items ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::resolve::DocWorker;

let worker = DocWorker::new();
assert!(worker.take(1).is_none());
```
