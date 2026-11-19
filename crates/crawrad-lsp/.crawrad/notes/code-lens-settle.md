---
id: code-lens-settle
kind: doc
target_file: src/code_lens.rs
line_hint: 16
author: 
created: 
title: "SETTLE"
tags: []
signature: "pub const SETTLE : Duration = Duration :: from_millis (300)"
---

How long the worker waits for typing to settle before asking.

# Examples

```rust
use crawrad_lsp::code_lens::SETTLE;
use std::time::Duration;

assert_eq!(SETTLE, Duration::from_millis(300));
```
