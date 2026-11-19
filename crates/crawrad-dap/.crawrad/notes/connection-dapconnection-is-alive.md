---
id: connection-dapconnection-is-alive
kind: doc
target_file: src/connection.rs
line_hint: 118
author: 
created: 
title: "DapConnection::is_alive"
tags: []
signature: "pub fn is_alive (& self) -> bool"
---

Whether the adapter is still talking.

# Examples

```rust
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.is_alive());
```
