---
id: server-says-first-line
kind: doc
target_file: src/server_says.rs
line_hint: 142
author:
created:
title: "first_line"
tags: []
signature: "pub fn first_line (text : & str) -> String"
---

The first line of a server message, trimmed.

A server message can be a page long, and the panel has one line to say it in.

# Examples

```rust
use crawrad_lsp::server_says::first_line;

assert_eq!(first_line("  failed to load\nCaused by:\n  no such file  "), "failed to load");
assert_eq!(first_line(""), "");
```
