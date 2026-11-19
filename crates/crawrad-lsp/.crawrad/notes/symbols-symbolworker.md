---
id: symbols-symbolworker
kind: doc
target_file: src/symbols.rs
line_hint: 38
author: 
created: 
title: "SymbolWorker"
tags: []
signature: "pub struct SymbolWorker (crate :: asker :: Asker < SymbolAnswer >)"
---

The thread that asks for items by name ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::symbols::SymbolWorker;

let worker = SymbolWorker::default();
assert!(worker.take(1).is_none());
```
