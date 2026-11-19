---
id: rename-renameworker
kind: doc
target_file: src/rename.rs
line_hint: 70
author: 
created: 
title: "RenameWorker"
tags: []
signature: "pub struct RenameWorker (crate :: asker :: Asker < RenameAnswer >)"
---

The thread that asks about renames.

# Examples

```rust
use crawrad_lsp::rename::RenameWorker;

let worker = RenameWorker::new();
assert!(worker.take(1).is_none());
```
