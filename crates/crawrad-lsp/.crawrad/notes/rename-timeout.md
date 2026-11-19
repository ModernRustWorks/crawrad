---
id: rename-timeout
kind: doc
target_file: src/rename.rs
line_hint: 20
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (10)"
---

How long rename requests wait before timing out.

A rename walks the workspace: more time than a definition.

# Examples

```rust
use crawrad_lsp::rename::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(10));
```
