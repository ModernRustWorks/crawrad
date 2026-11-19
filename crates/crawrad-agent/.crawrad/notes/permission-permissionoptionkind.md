---
id: permission-permissionoptionkind
kind: doc
target_file: src/permission.rs
line_hint: 33
author: 
created: 
title: "PermissionOptionKind"
tags: []
signature: "pub enum PermissionOptionKind"
---

Canonical permission option kinds per the ACP protocol.

# Examples

```rust
use crawrad_agent::PermissionOptionKind;

assert_eq!(PermissionOptionKind::AllowOnce.display_label(), "Allow Once");
```
