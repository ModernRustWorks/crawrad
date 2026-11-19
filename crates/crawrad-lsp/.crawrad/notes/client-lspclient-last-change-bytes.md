---
id: client-lspclient-last-change-bytes
kind: doc
target_file: src/client.rs
line_hint: 780
author: 
created: 
title: "LspClient::last_change_bytes"
tags: []
signature: "pub fn last_change_bytes (& self) -> u64"
---

Bytes of the last `didChange` sent.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert_eq!(client.last_change_bytes(), 0);
```
