---
id: symbols-symbolworker-take
kind: doc
target_file: src/symbols.rs
line_hint: 66
author: 
created: 
title: "SymbolWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < SymbolAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::symbols::SymbolWorker;

let worker = SymbolWorker::default();
assert!(worker.take(42).is_none());
```
