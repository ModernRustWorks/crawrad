---
id: permission-permissionqueue-current
kind: doc
target_file: src/permission.rs
line_hint: 281
author: 
created: 
title: "PermissionQueue::current"
tags: []
signature: "pub fn current (& self) -> Option < Arc < PermissionRequest > >"
---

Returns the request currently at the top of the queue without removing it.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert!(queue.current().is_none());
```
