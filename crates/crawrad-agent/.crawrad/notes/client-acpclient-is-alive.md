---
id: client-acpclient-is-alive
kind: doc
target_file: src/client.rs
line_hint: 1420
author: 
created: 
title: "AcpClient::is_alive"
tags: []
signature: "pub fn is_alive (& self) -> bool"
---

Checks if the agent subprocess is still alive.

# Examples

```rust,no_run
// Checks if external agent child process is alive.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.is_alive());
```
