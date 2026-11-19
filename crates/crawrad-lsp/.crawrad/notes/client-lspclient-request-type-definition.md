---
id: client-lspclient-request-type-definition
kind: doc
target_file: src/client.rs
line_hint: 971
author: 
created: 
title: "LspClient::request_type_definition"
tags: []
signature: "pub fn request_type_definition (& self , file_path : & Path , at : Position , timeout : Duration ,) -> Result < Vec < Location > , LspError >"
---

Where the type of the symbol at `at` is defined.

Where the type of the symbol at `at` is defined
(`textDocument/typeDefinition`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let locs = client.request_type_definition(Path::new("src/lib.rs"), Position::new(0, 0), Duration::from_secs(1)).unwrap();
assert!(locs.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
