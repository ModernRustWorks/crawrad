---
id: permission-permissionoptionkind-parse-kind
kind: doc
target_file: src/permission.rs
line_hint: 54
author: 
created: 
title: "PermissionOptionKind::parse_kind"
tags: []
signature: "pub fn parse_kind (raw : & str) -> Self"
---

Recognises the kind from an arbitrary string coming from the agent.

# Examples

```rust
use crawrad_agent::PermissionOptionKind;

assert_eq!(PermissionOptionKind::parse_kind("always allow"), PermissionOptionKind::AllowAlways);
```
