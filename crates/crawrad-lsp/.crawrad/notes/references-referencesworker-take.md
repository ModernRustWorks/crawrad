---
id: references-referencesworker-take
kind: doc
target_file: src/references.rs
line_hint: 89
author: 
created: 
title: "ReferencesWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < ReferencesAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::references::ReferencesWorker;

let worker = ReferencesWorker::new();
assert!(worker.take(42).is_none());
```
