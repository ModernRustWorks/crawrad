---
id: client-lspclient-send-request
kind: doc
target_file: src/client.rs
line_hint: 331
author: 
created: 
title: "LspClient::send_request"
tags: []
signature: "pub fn send_request (& self , method : & str , params : serde_json :: Value , timeout : Duration ,) -> Result < serde_json :: Value , LspError >"
---

Sends a request and returns the response within the given timeout.

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let res = client.send_request("shutdown", json!(null), Duration::from_secs(1)).unwrap();
assert!(res.is_null());
```

# Errors

Returns [`LspError::Timeout`] if no response is received before timeout,
[`LspError::RpcError`] if the server reports an error, or [`LspError::ChannelClosed`].
