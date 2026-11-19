---
id: client-lspclient-request-code-lenses
kind: doc
target_file: src/client.rs
line_hint: 594
author: 
created: 
title: "LspClient::request_code_lenses"
tags: []
signature: "pub fn request_code_lenses (& self , file_path : & Path , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < Vec < CodeLens > , LspError >"
---

Asks for the code lenses in `file_path`.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let lenses = client.request_code_lenses(Path::new("src/lib.rs"), Duration::from_secs(1), &mut || false).unwrap();
assert!(lenses.is_empty());
```

# Errors

Returns [`LspError::Cancelled`], [`LspError::Timeout`], or [`LspError::RpcError`].
