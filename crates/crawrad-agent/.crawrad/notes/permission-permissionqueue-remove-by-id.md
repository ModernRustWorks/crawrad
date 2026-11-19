---
id: permission-permissionqueue-remove-by-id
kind: doc
target_file: src/permission.rs
line_hint: 296
author: 
created: 
title: "PermissionQueue::remove_by_id"
tags: []
signature: "pub fn remove_by_id (& mut self , id : u64) -> Option < Arc < PermissionRequest > >"
---

Removes a specific request by its JSON-RPC id.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let mut queue = PermissionQueue::new();
assert!(queue.remove_by_id(42).is_none());
```
