---
id: definition-retry
kind: doc
target_file: src/definition.rs
line_hint: 126
author: 
created: 
title: "RETRY"
tags: []
signature: "pub const RETRY : Duration = Duration :: from_millis (300)"
---

The pause before asking again after [`CONTENT_MODIFIED`].

# Examples

```rust
use crawrad_lsp::definition::RETRY;
use std::time::Duration;

assert_eq!(RETRY, Duration::from_millis(300));
```
