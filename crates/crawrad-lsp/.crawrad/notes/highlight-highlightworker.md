---
id: highlight-highlightworker
kind: doc
target_file: src/highlight.rs
line_hint: 108
author: 
created: 
title: "HighlightWorker"
tags: []
signature: "pub struct HighlightWorker"
---

The thread that asks for the occurrences.

# Examples

```rust
use crawrad_lsp::highlight::HighlightWorker;

let worker = HighlightWorker::new();
assert!(worker.poll(std::path::Path::new("src/lib.rs"), 1, "fn main() {}", (0, 0)).is_none());
```
