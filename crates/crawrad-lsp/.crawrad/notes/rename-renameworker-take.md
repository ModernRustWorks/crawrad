---
id: rename-renameworker-take
kind: doc
target_file: src/rename.rs
line_hint: 141
author: 
created: 
title: "RenameWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < RenameAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::rename::RenameWorker;

let worker = RenameWorker::new();
assert!(worker.take(42).is_none());
```
