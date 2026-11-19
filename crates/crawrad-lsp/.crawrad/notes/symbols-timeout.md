---
id: symbols-timeout
kind: doc
target_file: src/symbols.rs
line_hint: 17
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (5)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::symbols::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(5));
```
