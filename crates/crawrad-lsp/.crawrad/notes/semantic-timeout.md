---
id: semantic-timeout
kind: doc
target_file: src/semantic.rs
line_hint: 24
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (5)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::semantic::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(5));
```
