---
id: client-lspclient-send-notification
kind: doc
target_file: src/client.rs
line_hint: 405
author: 
created: 
title: "LspClient::send_notification"
tags: []
signature: "pub fn send_notification (& self , method : & str , params : serde_json :: Value ,) -> Result < () , LspError >"
---

Sends a one-way notification to the server without expecting a response.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use serde_json::json;
use std::path::Path;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
assert!(client.send_notification("initialized", json!({})).is_ok());
```

# Errors

Returns [`LspError::ChannelClosed`] if the background writer thread is down.
