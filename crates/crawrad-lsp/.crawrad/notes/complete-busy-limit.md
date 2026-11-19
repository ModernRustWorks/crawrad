---
id: complete-busy-limit
kind: doc
target_file: src/complete.rs
line_hint: 26
author: 
created: 
title: "BUSY_LIMIT"
tags: []
signature: "pub const BUSY_LIMIT : Duration = Duration :: from_secs (90)"
---

How long to keep asking a busy server that answers empty.

# Examples

```rust
use crawrad_lsp::complete::BUSY_LIMIT;
use std::time::Duration;

assert_eq!(BUSY_LIMIT, Duration::from_secs(90));
```
