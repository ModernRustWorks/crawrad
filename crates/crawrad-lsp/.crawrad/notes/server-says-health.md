---
id: server-says-health
kind: doc
target_file: src/server_says.rs
line_hint: 41
author:
created:
title: "Health"
tags: []
signature: "pub enum Health"
---

What the server last told us about its own health.

`Fine` is the default, and the verdict a working server leaves behind.

# Examples

```rust
use crawrad_lsp::server_says::Health;

assert_eq!(Health::default(), Health::Fine);
```
