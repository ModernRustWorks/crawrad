---
id: code-lens-timeout
kind: doc
target_file: src/code_lens.rs
line_hint: 19
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (5)"
---

How long one request may take.

# Examples

```rust
use crawrad_lsp::code_lens::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(5));
```
