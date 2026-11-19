---
id: locate-server-dir
kind: doc
target_file: src/locate.rs
line_hint: 22
author: 
created: 
title: "SERVER_DIR"
tags: []
signature: "pub const SERVER_DIR : & str = \"rust-analyzer\""
---

The folder beside the IDE's executable that holds the server.

# Examples

```rust
use crawrad_lsp::locate::SERVER_DIR;

assert_eq!(SERVER_DIR, "rust-analyzer");
```
