---
id: permission-permissionmanager
kind: doc
target_file: src/permission.rs
line_hint: 378
author: 
created: 
title: "PermissionManager"
tags: []
signature: "pub struct PermissionManager"
---

Unified permission state manager (compatibility with the IDE's calls).

# Examples

```rust
use crawrad_agent::PermissionManager;

let mgr = PermissionManager::new();
assert!(mgr.active().is_none());
```
