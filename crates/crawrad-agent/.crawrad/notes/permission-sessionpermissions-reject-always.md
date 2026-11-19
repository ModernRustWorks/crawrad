---
id: permission-sessionpermissions-reject-always
kind: doc
target_file: src/permission.rs
line_hint: 223
author: 
created: 
title: "SessionPermissions::reject_always"
tags: []
signature: "pub fn reject_always (& mut self , tool : impl Into < String > , target : Option < String >)"
---

Blocks a tool and target continuously in this session.

# Examples

```rust
use crawrad_agent::SessionPermissions;

let mut perms = SessionPermissions::new();
perms.reject_always("bash", None);
assert!(perms.is_rejected("bash", None));
```
