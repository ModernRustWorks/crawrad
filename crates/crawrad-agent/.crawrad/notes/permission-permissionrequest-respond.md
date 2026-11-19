---
id: permission-permissionrequest-respond
kind: doc
target_file: src/permission.rs
line_hint: 189
author: 
created: 
title: "PermissionRequest::respond"
tags: []
signature: "pub fn respond (& self , decision : PermissionDecision) -> bool"
---

Answers this request with the user's decision or a cancellation.

Answers this request with the user's decision or a cancellation.
Returns `true` if the decision was sent, or `false` if it had already been answered.

# Examples

```rust
use crawrad_agent::{PermissionDecision, PermissionRequest};
use std::sync::mpsc::channel;
use std::time::Duration;

let (tx, rx) = channel();
let req = PermissionRequest::new(1, "s1", "read_file", None, "args", vec![], tx, Duration::from_secs(10));
assert!(req.respond(PermissionDecision::Cancelled));
assert_eq!(rx.recv(), Ok(PermissionDecision::Cancelled));
```
