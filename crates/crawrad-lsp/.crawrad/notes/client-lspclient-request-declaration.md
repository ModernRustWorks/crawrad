---
id: client-lspclient-request-declaration
kind: doc
target_file: src/client.rs
line_hint: 961
author: 
created: 
title: "LspClient::request_declaration"
tags: []
signature: "pub fn request_declaration (& self , file_path : & Path , at : Position , timeout : Duration ,) -> Result < Vec < Location > , LspError >"
---

Where the symbol at `at` is declared (`textDocument/declaration`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let locs = client.request_declaration(Path::new("src/lib.rs"), Position::new(0, 0), Duration::from_secs(1)).unwrap();
assert!(locs.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
