---
id: permission-permissionrequest
kind: doc
target_file: src/permission.rs
line_hint: 118
author: 
created: 
title: "PermissionRequest"
tags: []
signature: "pub struct PermissionRequest"
---

A single permission request received through `session/request_permission`.

# Examples

```rust
use crawrad_agent::PermissionRequest;
use std::sync::mpsc::channel;
use std::time::Duration;

let (tx, _rx) = channel();
let req = PermissionRequest::new(1, "s1", "read_file", None, "args", vec![], tx, Duration::from_secs(10));
assert_eq!(req.id, 1);
```
