---
id: code-action-timeout
kind: doc
target_file: src/code_action.rs
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
use crawrad_lsp::code_action::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(3));
```
