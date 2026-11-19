---
id: permission-permissionqueue-clear-and-cancel-all
kind: doc
target_file: src/permission.rs
line_hint: 320
author: 
created: 
title: "PermissionQueue::clear_and_cancel_all"
tags: []
signature: "pub fn clear_and_cancel_all (& mut self)"
---

Clears the queue, cancelling every pending request.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let mut queue = PermissionQueue::new();
queue.clear_and_cancel_all();
assert!(queue.is_empty());
```
