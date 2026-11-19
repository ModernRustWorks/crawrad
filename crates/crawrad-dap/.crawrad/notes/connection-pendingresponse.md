---
id: connection-pendingresponse
kind: doc
target_file: src/connection.rs
line_hint: 51
author: 
created: 
title: "PendingResponse"
tags: []
signature: "pub struct PendingResponse"
---

A request on its way: wait on it when the answer is needed.

`launch` is the case that needs it: its response only comes after the
client has sent `setBreakpoints` and `configurationDone`.

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
