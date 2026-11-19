---
id: permission-permissionqueue-peek
kind: doc
target_file: src/permission.rs
line_hint: 286
author: 
created: 
title: "PermissionQueue::peek"
tags: []
signature: "pub fn peek (& self) -> Option < Arc < PermissionRequest > >"
---

Alias for `current`.

# Examples

```rust
use crawrad_agent::PermissionQueue;

let queue = PermissionQueue::new();
assert!(queue.peek().is_none());
```
