---
id: ide-app-aisessionperms
kind: doc
target_file: src/ide_app.rs
line_hint: 358
author: 
created: 
title: "AiSessionPerms"
tags: []
signature: "pub type AiSessionPerms = std :: sync :: Arc < std :: sync :: Mutex < crawrad_agent :: SessionPermissions > >"
---

Accumulated session permissions.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let perms: AiSessionPerms = std::sync::Arc::new(std::sync::Mutex::new(crawrad_agent::SessionPermissions::default()));
assert!(perms.lock().is_ok());
```
