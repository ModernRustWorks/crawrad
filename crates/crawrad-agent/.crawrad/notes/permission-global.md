---
id: permission-global
kind: doc
target_file: src/permission.rs
line_hint: 414
author: 
created: 
title: "global"
tags: []
signature: "pub fn global () -> & 'static Arc < Mutex < PermissionManager > >"
---

Synchronised global access to the permission manager.

# Examples

```rust
use crawrad_agent::permission::global;

let mgr = global();
assert!(mgr.lock().unwrap().active().is_none());
```
