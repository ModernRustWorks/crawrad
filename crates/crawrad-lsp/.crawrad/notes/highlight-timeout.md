---
id: highlight-timeout
kind: doc
target_file: src/highlight.rs
line_hint: 29
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (3)"
---

How long one request may take.

# Examples

```rust
use crawrad_lsp::highlight::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(3));
```
