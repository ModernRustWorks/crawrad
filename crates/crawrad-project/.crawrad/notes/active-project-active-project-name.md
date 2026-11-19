---
id: active-project-active-project-name
kind: doc
target_file: src/active_project.rs
line_hint: 57
author: 
created: 
title: "active_project_name"
tags: []
signature: "pub fn active_project_name (root : & Path) -> Option < String >"
---

Returns the active project name under the workspace or single project root.

# Examples

```rust
use crawrad_project::active_project::*;
assert!(active_project_name(std::path::Path::new("no/such/folder")).is_none());
```
