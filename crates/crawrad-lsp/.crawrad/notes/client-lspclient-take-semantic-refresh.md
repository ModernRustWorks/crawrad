---
id: client-lspclient-take-semantic-refresh
kind: doc
target_file: src/client.rs
line_hint: 673
author: 
created: 
title: "LspClient::take_semantic_refresh"
tags: []
signature: "pub fn take_semantic_refresh (& self) -> bool"
---

Whether semantic tokens should be refreshed since the last call.

Whether the semantic tokens should be asked again (#730): the server
said so, or finished its work; reading clears it.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(!client.take_semantic_refresh());
```
