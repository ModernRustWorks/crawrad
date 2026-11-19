---
id: client-acpclient-send-request
kind: doc
target_file: src/client.rs
line_hint: 969
author: 
created: 
title: "AcpClient::send_request"
tags: []
signature: "pub fn send_request (& self , method : & str , params : serde_json :: Value , timeout : Duration ,) -> Result < serde_json :: Value , String >"
---

Sends a JSON-RPC request and synchronously awaits the response.

# Errors

Returns an error if writing to stdin fails, flushing fails, or the request times out.

# Panics

Panics if the internal response channel mutex is poisoned.

# Examples

```rust,no_run
// Sends JSON-RPC request to external agent.
use crawrad_agent::AcpClient;
use std::path::Path;
use std::time::Duration;
use serde_json::json;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
let res = client.send_request("ping", json!({}), Duration::from_secs(1));
assert!(res.is_ok());
```
