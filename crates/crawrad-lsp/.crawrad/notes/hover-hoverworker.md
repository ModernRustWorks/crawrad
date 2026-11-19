---
id: hover-hoverworker
kind: doc
target_file: src/hover.rs
line_hint: 48
author: 
created: 
title: "HoverWorker"
tags: []
signature: "pub struct HoverWorker (crate :: asker :: Asker < HoverAnswer >)"
---

The thread that asks for hovers ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::hover::HoverWorker;

let worker = HoverWorker::new();
assert!(worker.take(1).is_none());
```
