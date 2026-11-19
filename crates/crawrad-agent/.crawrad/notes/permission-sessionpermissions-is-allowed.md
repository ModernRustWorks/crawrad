---
id: permission-sessionpermissions-is-allowed
kind: doc
target_file: src/permission.rs
line_hint: 228
author: 
created: 
title: "SessionPermissions::is_allowed"
tags: []
signature: "pub fn is_allowed (& self , tool : & str , target : Option < & str >) -> bool"
---

Checks if a tool and target are permanently allowed.

Whether the tool (and target) has already been permanently allowed in this session.

# Examples

```rust
use crawrad_agent::SessionPermissions;

let perms = SessionPermissions::new();
assert!(!perms.is_allowed("read", None));
```
