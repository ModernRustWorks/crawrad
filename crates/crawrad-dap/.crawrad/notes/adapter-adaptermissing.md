---
id: adapter-adaptermissing
kind: doc
target_file: src/adapter.rs
line_hint: 32
author: 
created: 
title: "AdapterMissing"
tags: []
signature: "pub struct AdapterMissing"
---

No adapter where the IDE looks: what is said instead of a process error.

# Examples

```rust
use crawrad_dap::AdapterMissing;

let err = AdapterMissing { looked_at: Vec::new() };
assert!(err.looked_at.is_empty());
```
