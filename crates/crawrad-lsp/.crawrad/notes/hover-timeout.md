---
id: hover-timeout
kind: doc
target_file: src/hover.rs
line_hint: 21
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (3)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::hover::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(3));
```
