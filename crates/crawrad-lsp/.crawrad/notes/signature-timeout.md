---
id: signature-timeout
kind: doc
target_file: src/signature.rs
line_hint: 31
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (3)"
---

How long the server has to answer.

# Examples

```rust
use crawrad_lsp::signature::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(3));
```
