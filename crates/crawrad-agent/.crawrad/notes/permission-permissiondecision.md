---
id: permission-permissiondecision
kind: doc
target_file: src/permission.rs
line_hint: 109
author: 
created: 
title: "PermissionDecision"
tags: []
signature: "pub enum PermissionDecision"
---

The decision taken for the permission request.

# Examples

```rust
use crawrad_agent::PermissionDecision;

let dec = PermissionDecision::Selected("allow_once".to_string());
assert!(matches!(dec, PermissionDecision::Selected(_)));
```
