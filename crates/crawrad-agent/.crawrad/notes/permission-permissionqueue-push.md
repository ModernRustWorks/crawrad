---
id: permission-permissionqueue-push
kind: doc
target_file: src/permission.rs
line_hint: 276
author: 
created: 
title: "PermissionQueue::push"
tags: []
signature: "pub fn push (& mut self , req : impl Into < Arc < PermissionRequest > >)"
---

Enqueues a new request.

# Examples

```rust
use crawrad_agent::{PermissionQueue, PermissionRequest};
use std::sync::mpsc::channel;
use std::time::Duration;

let mut queue = PermissionQueue::new();
let (tx, _rx) = channel();
queue.push(PermissionRequest::new(1, "s1", "tool", None, "", vec![], tx, Duration::from_secs(1)));
assert_eq!(queue.len(), 1);
```
