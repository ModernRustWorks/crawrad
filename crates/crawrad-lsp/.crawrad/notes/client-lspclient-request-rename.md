---
id: client-lspclient-request-rename
kind: doc
target_file: src/client.rs
line_hint: 1046
author: 
created: 
title: "LspClient::request_rename"
tags: []
signature: "pub fn request_rename (& self , file_path : & Path , at : Position , new_name : & str , timeout : Duration ,) -> Result < crate :: edit :: WorkspaceEdit , LspError >"
---

Computes workspace edits to rename the symbol at `at` to `new_name`.

The edits that rename the symbol at `at` to `new_name`, by file
(`textDocument/rename`). Nothing is applied here.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let edit = client.request_rename(Path::new("src/lib.rs"), Position::new(0, 0), "foo", Duration::from_secs(1)).unwrap();
assert!(edit.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
