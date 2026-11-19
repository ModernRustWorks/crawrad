---
id: client-lspclient-is-open
kind: doc
target_file: src/client.rs
line_hint: 837
author: 
created: 
title: "LspClient::is_open"
tags: []
signature: "pub fn is_open (& self , file_path : & Path) -> bool"
---

Checks whether `file_path` is currently tracked as open on this client.

Was `file_path` opened on this client, and not closed since?

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(!client.is_open(Path::new("src/lib.rs")));
```
