---
id: locate-serversource
kind: doc
target_file: src/locate.rs
line_hint: 37
author: 
created: 
title: "ServerSource"
tags: []
signature: "pub enum ServerSource"
---

Which of the three places the server came from.

# Examples

```rust
use crawrad_lsp::locate::ServerSource;

assert_eq!(ServerSource::Bundled.status_name(), "rust-analyzer");
```
