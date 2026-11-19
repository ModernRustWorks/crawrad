---
id: locate-servermissing
kind: doc
target_file: src/locate.rs
line_hint: 65
author: 
created: 
title: "ServerMissing"
tags: []
signature: "pub struct ServerMissing"
---

No server anywhere the IDE looks.

# Examples

```rust
use crawrad_lsp::locate::ServerMissing;

let missing = ServerMissing { looked_at: Vec::new() };
assert!(missing.looked_at.is_empty());
```
