---
id: client-lspclient-did-close
kind: doc
target_file: src/client.rs
line_hint: 843
author: 
created: 
title: "LspClient::did_close"
tags: []
signature: "pub fn did_close (& self , file_path : & Path) -> Result < () , LspError >"
---

Notifies the language server that a document was closed.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.did_close(Path::new("src/lib.rs")).is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if sending the notification fails.
