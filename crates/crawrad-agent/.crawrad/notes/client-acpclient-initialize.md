---
id: client-acpclient-initialize
kind: doc
target_file: src/client.rs
line_hint: 1011
author: 
created: 
title: "AcpClient::initialize"
tags: []
signature: "pub fn initialize (& self) -> Result < serde_json :: Value , String >"
---

Performs the ACP `initialize` handshake negotiating protocol version.

# Errors

Returns an error if communication fails or the server rejects initialization.

# Examples

```rust,no_run
// Performs ACP initialization handshake with external process.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.initialize().is_ok());
```
