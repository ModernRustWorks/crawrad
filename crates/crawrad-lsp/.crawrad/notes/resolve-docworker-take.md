---
id: resolve-docworker-take
kind: doc
target_file: src/resolve.rs
line_hint: 63
author: 
created: 
title: "DocWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < DocAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::resolve::DocWorker;

let worker = DocWorker::new();
assert!(worker.take(42).is_none());
```
