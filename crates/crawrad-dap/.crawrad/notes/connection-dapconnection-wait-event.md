---
id: connection-dapconnection-wait-event
kind: doc
target_file: src/connection.rs
line_hint: 174
author: 
created: 
title: "DapConnection::wait_event"
tags: []
signature: "pub fn wait_event (& self , name : & str , timeout : Duration) -> Result < Event , DapError >"
---

Waits for the event named `name`, dropping the others on the way.

# Examples

```rust
use std::time::Duration;
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.wait_event("stopped", Duration::from_millis(5)).is_err());
```

# Errors

Returns [`DapError::Timeout`] if the named event does not arrive within `timeout`.
