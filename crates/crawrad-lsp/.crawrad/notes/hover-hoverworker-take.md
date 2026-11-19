---
id: hover-hoverworker-take
kind: doc
target_file: src/hover.rs
line_hint: 125
author: 
created: 
title: "HoverWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < HoverAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::hover::HoverWorker;

let worker = HoverWorker::new();
assert!(worker.take(1).is_none());
```
