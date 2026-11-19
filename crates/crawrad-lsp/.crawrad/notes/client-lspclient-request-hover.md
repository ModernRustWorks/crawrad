---
id: client-lspclient-request-hover
kind: doc
target_file: src/client.rs
line_hint: 910
author: 
created: 
title: "LspClient::request_hover"
tags: []
signature: "pub fn request_hover (& self , file_path : & Path , line : u32 , character : u32 , timeout : Duration ,) -> Result < Option < Hover > , LspError >"
---

Requests hover documentation at the line and character position.

Requests hover documentation information at the given line and character position.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let hover = client.request_hover(Path::new("src/lib.rs"), 0, 0, Duration::from_secs(1)).unwrap();
assert_eq!(hover, None);
```

# Errors

Returns [`LspError::Timeout`], [`LspError::RpcError`], or [`LspError::ChannelClosed`].
