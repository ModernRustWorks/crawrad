---
id: client-lspclient-is-alive
kind: doc
target_file: src/client.rs
line_hint: 314
author: 
created: 
title: "LspClient::is_alive"
tags: []
signature: "pub fn is_alive (& self) -> bool"
---

Returns `true` if the server process is currently running.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.is_alive());
```
