---
id: client-lspclient-did-save
kind: doc
target_file: src/client.rs
line_hint: 824
author: 
created: 
title: "LspClient::did_save"
tags: []
signature: "pub fn did_save (& self , file_path : & Path) -> Result < () , LspError >"
---

Notifies the language server that a document was saved to disk.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.did_save(Path::new("src/lib.rs")).is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if sending the notification fails.
