---
id: client-acpclient-allow-always
kind: doc
target_file: src/client.rs
line_hint: 1453
author: 
created: 
title: "AcpClient::allow_always"
tags: []
signature: "pub fn allow_always (& self , tool : impl Into < String > , target : Option < String >)"
---

Grants continuous session-wide permission for a tool and optional target.

# Examples

```rust,no_run
// Grants session permission on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
client.allow_always("read", None);
```
