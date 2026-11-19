---
id: permission-permissionoptionkind-display-label
kind: doc
target_file: src/permission.rs
line_hint: 44
author: 
created: 
title: "PermissionOptionKind::display_label"
tags: []
signature: "pub fn display_label (& self) -> & 'static str"
---

Readable label for buttons and the interface.

# Examples

```rust
use crawrad_agent::PermissionOptionKind;

assert_eq!(PermissionOptionKind::AllowAlways.display_label(), "Always Allow");
```
