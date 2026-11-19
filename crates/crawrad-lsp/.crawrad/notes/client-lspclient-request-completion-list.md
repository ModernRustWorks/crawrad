---
id: client-lspclient-request-completion-list
kind: doc
target_file: src/client.rs
line_hint: 886
author: 
created: 
title: "LspClient::request_completion_list"
tags: []
signature: "pub fn request_completion_list (& self , file_path : & Path , at : Position , context : serde_json :: Value , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < (Vec < CompletionItem > , bool) , LspError >"
---

The completion list and whether it is partial (`isIncomplete`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use crawrad_lsp::protocol::Position;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let (items, incomplete) = client.request_completion_list(Path::new("src/lib.rs"), Position::new(0, 0), json!({}), Duration::from_secs(1), &mut || false).unwrap();
assert!(!incomplete);
```

# Errors

Returns [`LspError::Cancelled`], [`LspError::Timeout`], or [`LspError::RpcError`].
