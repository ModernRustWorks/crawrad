---
id: complete-timeout
kind: doc
target_file: src/complete.rs
line_hint: 23
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (30)"
---

How long one request may take: the first one on a cold project is slow.

# Examples

```rust
use crawrad_lsp::complete::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(30));
```
