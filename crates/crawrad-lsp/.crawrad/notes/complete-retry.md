---
id: complete-retry
kind: doc
target_file: src/complete.rs
line_hint: 29
author: 
created: 
title: "RETRY"
tags: []
signature: "pub const RETRY : Duration = Duration :: from_millis (400)"
---

The pause between two such asks.

# Examples

```rust
use crawrad_lsp::complete::RETRY;
use std::time::Duration;

assert_eq!(RETRY, Duration::from_millis(400));
```
