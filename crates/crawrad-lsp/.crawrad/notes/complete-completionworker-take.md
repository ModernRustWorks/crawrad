---
id: complete-completionworker-take
kind: doc
target_file: src/complete.rs
line_hint: 157
author: 
created: 
title: "CompletionWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < CompletionAnswer >"
---

The answer for `ticket`, if it arrived; older tickets are dropped.

# Examples

```rust
use crawrad_lsp::complete::CompletionWorker;

let worker = CompletionWorker::new();
assert!(worker.take(1).is_none());
```
