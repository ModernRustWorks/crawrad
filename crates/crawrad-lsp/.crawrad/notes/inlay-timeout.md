---
id: inlay-timeout
kind: doc
target_file: src/inlay.rs
line_hint: 26
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (5)"
---

How long one request may take.

# Examples

```rust
use crawrad_lsp::inlay::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(5));
```
