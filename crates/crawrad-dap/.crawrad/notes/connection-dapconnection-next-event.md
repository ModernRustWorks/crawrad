---
id: connection-dapconnection-next-event
kind: doc
target_file: src/connection.rs
line_hint: 160
author: 
created: 
title: "DapConnection::next_event"
tags: []
signature: "pub fn next_event (& self , timeout : Duration) -> Result < Event , DapError >"
---

The next event, waiting up to `timeout`.

# Examples

```rust
use std::time::Duration;
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.next_event(Duration::from_millis(5)).is_err());
```

# Errors

Returns [`DapError::Timeout`] if no event arrives, or [`DapError::Closed`].

# Panics

Panics if the internal events lock is poisoned.
