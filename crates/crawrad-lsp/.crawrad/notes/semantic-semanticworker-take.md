---
id: semantic-semanticworker-take
kind: doc
target_file: src/semantic.rs
line_hint: 266
author: 
created: 
title: "SemanticWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < SemanticAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::semantic::SemanticWorker;

let worker = SemanticWorker::new();
assert!(worker.take(42).is_none());
```
