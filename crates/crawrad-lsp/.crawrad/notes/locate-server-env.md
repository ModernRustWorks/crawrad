---
id: locate-server-env
kind: doc
target_file: src/locate.rs
line_hint: 25
author: 
created: 
title: "SERVER_ENV"
tags: []
signature: "pub const SERVER_ENV : & str = \"CRAWRAD_RUST_ANALYZER\""
---

The variable that names a server explicitly.

# Examples

```rust
use crawrad_lsp::locate::SERVER_ENV;

assert_eq!(SERVER_ENV, "CRAWRAD_RUST_ANALYZER");
```
