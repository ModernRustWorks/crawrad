---
id: client-acpclient-is-permission-rejected
kind: doc
target_file: src/client.rs
line_hint: 1444
author: 
created: 
title: "AcpClient::is_permission_rejected"
tags: []
signature: "pub fn is_permission_rejected (& self , tool : & str , target : Option < & str >) -> bool"
---

Checks if a tool and optional target is permanently rejected in this session.

# Examples

```rust,no_run
// Checks if tool is rejected on client.
use crawrad_agent::AcpClient;
use std::path::Path;

let client = AcpClient::spawn(Path::new("agy"), &[], None, false).unwrap();
assert!(!client.is_permission_rejected("bash", None));
```
