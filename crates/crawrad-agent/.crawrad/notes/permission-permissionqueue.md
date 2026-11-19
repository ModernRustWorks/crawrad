---
id: permission-permissionqueue
kind: doc
target_file: src/permission.rs
line_hint: 265
author: 
created: 
title: "PermissionQueue"
tags: []
signature: "pub struct PermissionQueue"
---

FIFO queue of permission requests.

FIFO queue of permission requests.
Guarantees that no request answers or overrides another.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert!(queue.is_empty());
```
