---
id: server-says-watch-new
kind: doc
target_file: src/server_says.rs
line_hint: 80
author:
created:
title: "new"
tags: []
signature: "pub fn new () -> Self"
---

A watch that has heard nothing yet, and so assumes the server is working.

# Examples

```rust
use crawrad_lsp::server_says::{Health, Watch};

assert_eq!(Watch::new().health(), Health::Fine);
```
