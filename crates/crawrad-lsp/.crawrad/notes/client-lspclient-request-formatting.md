---
id: client-lspclient-request-formatting
kind: doc
target_file: src/client.rs
line_hint: 1063
author: 
created: 
title: "LspClient::request_formatting"
tags: []
signature: "pub fn request_formatting (& self , file_path : & Path , tab_size : u32 , insert_spaces : bool , timeout : Duration ,) -> Result < Vec < crate :: edit :: TextEdit > , LspError >"
---

The edits that format the whole document (`textDocument/formatting`).

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let edits = client.request_formatting(Path::new("src/lib.rs"), 4, true, Duration::from_secs(1)).unwrap();
assert!(edits.is_empty());
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
