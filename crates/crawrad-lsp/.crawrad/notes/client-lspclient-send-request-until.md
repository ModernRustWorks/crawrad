---
id: client-lspclient-send-request-until
kind: doc
target_file: src/client.rs
line_hint: 341
author: 
created: 
title: "LspClient::send_request_until"
tags: []
signature: "pub fn send_request_until (& self , method : & str , params : serde_json :: Value , timeout : Duration , superseded : & mut dyn FnMut () -> bool ,) -> Result < serde_json :: Value , LspError >"
---

Sends a request that cancels early when `superseded` returns true.

A request that gives up when `superseded` says a newer one replaced
it: the server is told with `$/cancelRequest` (LSP 3.17, *Cancellation
Support*) and the caller gets [`LspError::Cancelled`]. A late answer
for the cancelled id — the result, or `RequestCancelled (-32800)` — is
dropped. `superseded` is asked every [`CANCEL_POLL`].

# Examples

```rust,no_run
// Interacting with a running language server requires a spawned process.
use crawrad_lsp::LspClient;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

let client = LspClient::spawn(None, Path::new(".")).unwrap();
let res = client.send_request_until("shutdown", json!(null), Duration::from_secs(1), &mut || false).unwrap();
assert!(res.is_null());
```

# Errors

Returns [`LspError::Cancelled`] if superseded, [`LspError::Timeout`] on timeout,
[`LspError::RpcError`] on server error, or [`LspError::ChannelClosed`].

# Panics

Panics if the internal mutex protecting pending requests is poisoned.
