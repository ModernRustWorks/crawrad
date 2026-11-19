---
id: permission-permissionqueue-pop
kind: doc
target_file: src/permission.rs
line_hint: 291
author: 
created: 
title: "PermissionQueue::pop"
tags: []
signature: "pub fn pop (& mut self) -> Option < Arc < PermissionRequest > >"
---

Removes and returns the request at the top of the queue.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let mut queue = PermissionQueue::new();
assert!(queue.pop().is_none());
```
