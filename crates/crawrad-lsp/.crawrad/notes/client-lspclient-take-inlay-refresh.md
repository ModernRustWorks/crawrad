---
id: client-lspclient-take-inlay-refresh
kind: doc
target_file: src/client.rs
line_hint: 663
author: 
created: 
title: "LspClient::take_inlay_refresh"
tags: []
signature: "pub fn take_inlay_refresh (& self) -> bool"
---

Whether the server asked for the inlay hints again since the last call.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(!client.take_inlay_refresh());
```
