---
id: active-project-project-name
kind: doc
target_file: src/active_project.rs
line_hint: 42
author: 
created: 
title: "project_name"
tags: []
signature: "pub fn project_name (folder : & Path) -> String"
---

Returns the package name of a project from its manifest or folder name.

# Examples

```rust
use crawrad_project::active_project::*;
assert_eq!(project_name(std::path::Path::new("no/such/app_x")), "app_x");
```
