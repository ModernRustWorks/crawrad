---
id: permission-permissionoption
kind: doc
target_file: src/permission.rs
line_hint: 69
author: 
created: 
title: "PermissionOption"
tags: []
signature: "pub struct PermissionOption"
---

A concrete permission option offered in the ACP request.

# Examples

```rust
use crawrad_agent::{PermissionOption, PermissionOptionKind};

let opt = PermissionOption::new("allow_once", "Allow", PermissionOptionKind::AllowOnce);
assert_eq!(opt.option_id, "allow_once");
```
