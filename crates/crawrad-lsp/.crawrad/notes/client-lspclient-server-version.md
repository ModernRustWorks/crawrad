---
id: client-lspclient-server-version
kind: doc
target_file: src/client.rs
line_hint: 1084
author: 
created: 
title: "LspClient::server_version"
tags: []
signature: "pub fn server_version (& self) -> Option < String >"
---

The server's name and version, as it answered `initialize`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.server_version().is_some());
```
