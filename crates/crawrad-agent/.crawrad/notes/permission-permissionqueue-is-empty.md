---
id: permission-permissionqueue-is-empty
kind: doc
target_file: src/permission.rs
line_hint: 310
author: 
created: 
title: "PermissionQueue::is_empty"
tags: []
signature: "pub fn is_empty (& self) -> bool"
---

Whether the queue is empty.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert!(queue.is_empty());
```
