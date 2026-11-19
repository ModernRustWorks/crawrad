---
id: connection-pendingresponse-wait
kind: doc
target_file: src/connection.rs
line_hint: 58
author: 
created: 
title: "PendingResponse::wait"
tags: []
signature: "pub fn wait (self , timeout : Duration) -> Result < Value , DapError >"
---

The response body, or why there is none.

# Examples

```rust
use std::time::Duration;
use serde_json::json;
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
let pending = conn.send_request("threads", json!({})).unwrap();
assert!(pending.wait(Duration::from_millis(10)).is_err());
```

# Errors

Returns [`DapError::Timeout`] if no response arrives in time, or
[`DapError::Failed`] if the adapter rejected the request.
