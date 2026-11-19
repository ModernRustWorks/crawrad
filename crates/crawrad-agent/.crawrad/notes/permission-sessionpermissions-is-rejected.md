---
id: permission-sessionpermissions-is-rejected
kind: doc
target_file: src/permission.rs
line_hint: 244
author: 
created: 
title: "SessionPermissions::is_rejected"
tags: []
signature: "pub fn is_rejected (& self , tool : & str , target : Option < & str >) -> bool"
---

Checks if a tool and target are permanently rejected.

Whether the tool (and target) has already been permanently rejected in this session.

# Examples

```rust
use crawrad_agent::SessionPermissions;

let perms = SessionPermissions::new();
assert!(!perms.is_rejected("read", None));
```
