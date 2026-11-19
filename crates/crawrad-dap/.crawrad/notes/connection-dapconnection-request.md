---
id: connection-dapconnection-request
kind: doc
target_file: src/connection.rs
line_hint: 150
author: 
created: 
title: "DapConnection::request"
tags: []
signature: "pub fn request (& self , command : & str , arguments : Value , timeout : Duration ,) -> Result < Value , DapError >"
---

Sends a request and waits for its answer.

# Examples

```rust
use std::time::Duration;
use serde_json::json;
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.request("threads", json!({}), Duration::from_millis(5)).is_err());
```

# Errors

Returns [`DapError`] if sending fails, times out, or the adapter returns an error.
