---
id: permission-permissionqueue-clear
kind: doc
target_file: src/permission.rs
line_hint: 315
author: 
created: 
title: "PermissionQueue::clear"
tags: []
signature: "pub fn clear (& mut self)"
---

Clears the queue, cancelling every pending request.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let mut queue = PermissionQueue::new();
queue.clear();
assert!(queue.is_empty());
```
