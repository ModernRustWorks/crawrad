---
id: client-acpclient-is-permission-allowed
kind: doc
target_file: src/client.rs
line_hint: 1435
author: 
created: 
title: "AcpClient::is_permission_allowed"
tags: []
signature: "pub fn is_permission_allowed (& self , tool : & str , target : Option < & str >) -> bool"
---

Checks if a tool and optional target is permanently allowed in this session.

# Examples

```rust,no_run
// Checks if tool is allowed on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(!client.is_permission_allowed("bash", None));
```
