---
id: definition-timeout
kind: doc
target_file: src/definition.rs
line_hint: 18
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (5)"
---

How long the server has to answer before the editor says so.

# Examples

```rust
use crawrad_lsp::definition::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(5));
```
