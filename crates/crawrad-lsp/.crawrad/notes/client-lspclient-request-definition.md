---
id: client-lspclient-request-definition
kind: doc
target_file: src/client.rs
line_hint: 938
author: 
created: 
title: "LspClient::request_definition"
tags: []
signature: "pub fn request_definition (& self , file_path : & Path , line : u32 , character : u32 , timeout : Duration ,) -> Result < Vec < Location > , LspError >"
---

Requests definition location (Go to Definition) for the symbol under cursor.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let locs = client.request_definition(Path::new("src/lib.rs"), 0, 0, Duration::from_secs(1)).unwrap();
assert!(locs.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
