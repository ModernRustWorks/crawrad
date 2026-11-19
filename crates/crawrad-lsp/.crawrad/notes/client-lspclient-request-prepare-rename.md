---
id: client-lspclient-request-prepare-rename
kind: doc
target_file: src/client.rs
line_hint: 1029
author: 
created: 
title: "LspClient::request_prepare_rename"
tags: []
signature: "pub fn request_prepare_rename (& self , file_path : & Path , at : Position , timeout : Duration ,) -> Result < Option < Range > , LspError >"
---

Checks whether the symbol at `at` can be renamed, returning its span.

Whether the symbol at `at` can be renamed, and the span of its name
(`textDocument/prepareRename`). `None` = nothing renameable there.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let span = client.request_prepare_rename(Path::new("src/lib.rs"), Position::new(0, 0), Duration::from_secs(1)).unwrap();
assert_eq!(span, None);
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
