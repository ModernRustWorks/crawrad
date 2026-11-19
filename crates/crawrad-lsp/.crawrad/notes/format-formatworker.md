---
id: format-formatworker
kind: doc
target_file: src/format.rs
line_hint: 36
author: 
created: 
title: "FormatWorker"
tags: []
signature: "pub struct FormatWorker (crate :: asker :: Asker < FormatAnswer >)"
---

The thread that asks for formatting ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::format::FormatWorker;

let worker = FormatWorker::new();
assert!(worker.take(1).is_none());
```
