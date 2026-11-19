---
id: format-timeout
kind: doc
target_file: src/format.rs
line_hint: 16
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (10)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::format::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(10));
```
