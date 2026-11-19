---
id: format-formatworker-take
kind: doc
target_file: src/format.rs
line_hint: 80
author: 
created: 
title: "FormatWorker::take"
tags: []
signature: "pub fn take (& self , ticket : u64) -> Option < FormatAnswer >"
---

The answer for `ticket`, if it arrived; any other is dropped.

# Examples

```rust
use crawrad_lsp::format::FormatWorker;

let worker = FormatWorker::new();
assert!(worker.take(1).is_none());
```
