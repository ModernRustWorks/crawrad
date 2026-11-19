---
id: permission-permissionoption-default-options
kind: doc
target_file: src/permission.rs
line_hint: 90
author: 
created: 
title: "PermissionOption::default_options"
tags: []
signature: "pub fn default_options () -> Vec < Self >"
---

Creates the 4 canonical default options of the ACP protocol.

# Examples

```rust
use crawrad_agent::PermissionOption;

let opts = PermissionOption::default_options();
assert_eq!(opts.len(), 4);
```
