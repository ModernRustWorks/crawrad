---
id: connection-dapconnection-send-request
kind: doc
target_file: src/connection.rs
line_hint: 123
author: 
created: 
title: "DapConnection::send_request"
tags: []
signature: "pub fn send_request (& self , command : & str , arguments : Value ,) -> Result < PendingResponse , DapError >"
---

Sends a request and returns without waiting for its answer.

# Examples

```rust
use serde_json::json;
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.send_request("threads", json!({})).is_ok());
```

# Errors

Returns [`DapError::Closed`] if the connection is dead, or I/O errors
on failure to write the request.

# Panics

Panics if internal mutex synchronization is poisoned.
