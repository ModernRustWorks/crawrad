---
id: permission-sessionpermissions
kind: doc
target_file: src/permission.rs
line_hint: 206
author: 
created: 
title: "SessionPermissions"
tags: []
signature: "pub struct SessionPermissions"
---

Permanent permission rules accumulated in the current session.

Permanent permission rules accumulated in the current session.
Keyed by tool AND target (when there is one).

# Examples

```rust
use crawrad_agent::SessionPermissions;

let mut perms = SessionPermissions::new();
perms.allow_always("read", None);
assert!(perms.is_allowed("read", None));
```
