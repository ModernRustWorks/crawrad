---
id: lsp-cell-server
kind: doc
target_file: src/lsp_cell.rs
line_hint: 7
author: 
created: 
title: "SERVER"
tags: []
signature: "pub const SERVER : & str = \"rust-analyzer\""
---

Default language server name when no client exists.

# Examples

```rust
use crawrad_insight::lsp_cell::SERVER;

assert_eq!(SERVER, "rust-analyzer");
```
