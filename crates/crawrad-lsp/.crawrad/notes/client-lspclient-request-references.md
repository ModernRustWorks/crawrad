---
id: client-lspclient-request-references
kind: doc
target_file: src/client.rs
line_hint: 991
author: 
created: 
title: "LspClient::request_references"
tags: []
signature: "pub fn request_references (& self , file_path : & Path , at : Position , include_declaration : bool , timeout : Duration ,) -> Result < Vec < Location > , LspError >"
---

Finds references to the symbol at `at` across the workspace.

Every use of the symbol at `at` in the workspace
(`textDocument/references`), its declaration too when asked.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let refs = client.request_references(Path::new("src/lib.rs"), Position::new(0, 0), false, Duration::from_secs(1)).unwrap();
assert!(refs.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
