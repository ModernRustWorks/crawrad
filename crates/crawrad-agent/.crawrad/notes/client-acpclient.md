---
id: client-acpclient
kind: doc
target_file: src/client.rs
line_hint: 55
author: 
created: 
title: "AcpClient"
tags: []
signature: "pub struct AcpClient"
---

Continuous client connection to an ACP agent subprocess.

# Examples

```rust,no_run
// Spawns external agent process.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(client.is_alive());
```
