---
id: client-lspclient-request-document-highlight
kind: doc
target_file: src/client.rs
line_hint: 624
author: 
created: 
title: "LspClient::request_document_highlight"
tags: []
signature: "pub fn request_document_highlight (& self , file_path : & Path , at : Position , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < Vec < crate :: highlight :: DocumentHighlight > , LspError >"
---

Highlights every occurrence of the symbol at `at` in `file_path`.

Every place in `file_path` that names the symbol at `at`
(`textDocument/documentHighlight`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let hl = client.request_document_highlight(Path::new("src/lib.rs"), Position::new(0, 0), Duration::from_secs(1), &mut || false).unwrap();
assert!(hl.is_empty());
```

# Errors

Returns [`LspError::Cancelled`], [`LspError::Timeout`], or [`LspError::RpcError`].
