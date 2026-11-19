---
id: client-acpclient-clear-live-buffers
kind: doc
target_file: src/client.rs
line_hint: 1413
author: 
created: 
title: "AcpClient::clear_live_buffers"
tags: []
signature: "pub fn clear_live_buffers (& self)"
---

Clears all registered in-memory editor buffers.

# Examples

```rust,no_run
// Clears live buffers on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
client.clear_live_buffers();
```
