---
id: connection-dapconnection-new
kind: doc
target_file: src/connection.rs
line_hint: 81
author: 
created: 
title: "DapConnection::new"
tags: []
signature: "pub fn new < R , W > (reader : R , writer : W) -> Self where R : Read + Send + 'static , W : Write + Send + 'static ,"
---

Starts a DAP conversation over reader and writer streams.

Starts the conversation over `reader` (what the adapter writes) and
`writer` (what the adapter reads).

# Examples

```rust
use crawrad_dap::DapConnection;

let (client_reader, _to_client) = std::io::pipe().unwrap();
let (_from_client, client_writer) = std::io::pipe().unwrap();
let conn = DapConnection::new(client_reader, client_writer);
assert!(conn.is_alive());
```

# Panics

Panics if the background reader thread cannot be spawned.
