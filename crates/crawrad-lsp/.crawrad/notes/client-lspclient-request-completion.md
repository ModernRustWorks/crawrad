---
id: client-lspclient-request-completion
kind: doc
target_file: src/client.rs
line_hint: 858
author: 
created: 
title: "LspClient::request_completion"
tags: []
signature: "pub fn request_completion (& self , file_path : & Path , line : u32 , character : u32 , timeout : Duration ,) -> Result < Vec < CompletionItem > , LspError >"
---

Requests completion items at the zero-based line and character position.

Requests autocompletion items at the given zero-based line and character cursor position.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let items = client.request_completion(Path::new("src/lib.rs"), 0, 0, Duration::from_secs(1)).unwrap();
assert!(items.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
