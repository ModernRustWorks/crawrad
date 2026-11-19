---
id: locate-server-file-name
kind: doc
target_file: src/locate.rs
line_hint: 28
author: 
created: 
title: "server_file_name"
tags: []
signature: "pub fn server_file_name () -> & 'static str"
---

The server's file name on this platform.

# Examples

```rust
use crawrad_lsp::locate::server_file_name;

assert!(server_file_name().starts_with("rust-analyzer"));
```
