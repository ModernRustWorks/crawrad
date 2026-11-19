---
id: permission-sessionpermissions-clear
kind: doc
target_file: src/permission.rs
line_hint: 258
author: 
created: 
title: "SessionPermissions::clear"
tags: []
signature: "pub fn clear (& mut self)"
---

Clears every accumulated permission (e.g. a project switch or /clear).

# Examples

```rust
use crawrad_agent::SessionPermissions;

let mut perms = SessionPermissions::new();
perms.allow_always("tool", None);
perms.clear();
assert!(!perms.is_allowed("tool", None));
```
