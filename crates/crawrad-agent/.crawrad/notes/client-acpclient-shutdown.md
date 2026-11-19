---
id: client-acpclient-shutdown
kind: doc
target_file: src/client.rs
line_hint: 1472
author: 
created: 
title: "AcpClient::shutdown"
tags: []
signature: "pub fn shutdown (& mut self)"
---

Gracefully terminates the agent connection.

# Examples

```rust,no_run
// Shuts down client connection to external agent.
use crawrad_agent::AcpClient;
use std::path::Path;

let mut client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
client.shutdown();
```
