---
id: permission-permissionrequest-is-expired
kind: doc
target_file: src/permission.rs
line_hint: 200
author: 
created: 
title: "PermissionRequest::is_expired"
tags: []
signature: "pub fn is_expired (& self) -> bool"
---

Whether the request has exceeded its waiting time limit.

# Examples

```rust
use crawrad_agent::PermissionRequest;
use std::sync::mpsc::channel;
use std::time::Duration;

let (tx, _rx) = channel();
let req = PermissionRequest::new(1, "s1", "read_file", None, "args", vec![], tx, Duration::from_secs(10));
assert!(!req.is_expired());
```
