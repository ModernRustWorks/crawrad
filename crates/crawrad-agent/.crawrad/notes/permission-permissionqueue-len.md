---
id: permission-permissionqueue-len
kind: doc
target_file: src/permission.rs
line_hint: 305
author: 
created: 
title: "PermissionQueue::len"
tags: []
signature: "pub fn len (& self) -> usize"
---

Number of requests currently waiting for a decision.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert_eq!(queue.len(), 0);
```
