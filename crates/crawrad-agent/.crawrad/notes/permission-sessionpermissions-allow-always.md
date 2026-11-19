---
id: permission-sessionpermissions-allow-always
kind: doc
target_file: src/permission.rs
line_hint: 218
author: 
created: 
title: "SessionPermissions::allow_always"
tags: []
signature: "pub fn allow_always (& mut self , tool : impl Into < String > , target : Option < String >)"
---

Grants continuous permission for a tool and target in this session.

# Examples

```rust
use crawrad_agent::SessionPermissions;

let mut perms = SessionPermissions::new();
perms.allow_always("bash", None);
assert!(perms.is_allowed("bash", None));
```
