---
id: complete-completionworker
kind: doc
target_file: src/complete.rs
line_hint: 74
author: 
created: 
title: "CompletionWorker"
tags: []
signature: "pub struct CompletionWorker"
---

The thread that asks for completions.

# Examples

```rust
use crawrad_lsp::complete::CompletionWorker;

let worker = CompletionWorker::new();
assert!(worker.take(1).is_none());
```
