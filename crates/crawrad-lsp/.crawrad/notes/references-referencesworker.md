---
id: references-referencesworker
kind: doc
target_file: src/references.rs
line_hint: 42
author: 
created: 
title: "ReferencesWorker"
tags: []
signature: "pub struct ReferencesWorker (crate :: asker :: Asker < ReferencesAnswer >)"
---

The thread that asks for references ([`crate::asker`]).

# Examples

```rust
use crawrad_lsp::references::ReferencesWorker;

let worker = ReferencesWorker::new();
assert!(worker.take(1).is_none());
```
