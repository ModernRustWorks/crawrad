---
id: resolve-timeout
kind: doc
target_file: src/resolve.rs
line_hint: 22
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (2)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::resolve::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(2));
```
