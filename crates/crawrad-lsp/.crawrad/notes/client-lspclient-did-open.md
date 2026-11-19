---
id: client-lspclient-did-open
kind: doc
target_file: src/client.rs
line_hint: 678
author: 
created: 
title: "LspClient::did_open"
tags: []
signature: "pub fn did_open (& self , file_path : & Path , language_id : & str , version : i32 , text : & str ,) -> Result < () , LspError >"
---

Notifies the language server that a document was opened in the editor.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.did_open(Path::new("src/lib.rs"), "rust", 1, "fn main() {}").is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if sending the notification fails.
