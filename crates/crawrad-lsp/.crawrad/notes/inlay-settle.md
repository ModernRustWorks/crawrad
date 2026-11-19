---
id: inlay-settle
kind: doc
target_file: src/inlay.rs
line_hint: 23
author: 
created: 
title: "SETTLE"
tags: []
signature: "pub const SETTLE : Duration = Duration :: from_millis (300)"
---

How long the worker waits for typing to settle before asking.

# Examples

```rust
use crawrad_lsp::inlay::SETTLE;
use std::time::Duration;

assert_eq!(SETTLE, Duration::from_millis(300));
```
