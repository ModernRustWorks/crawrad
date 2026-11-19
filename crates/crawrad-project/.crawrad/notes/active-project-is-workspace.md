---
id: active-project-is-workspace
kind: doc
target_file: src/active_project.rs
line_hint: 12
author: 
created: 
title: "is_workspace"
tags: []
signature: "pub fn is_workspace (root : & Path) -> bool"
---

Tells whether the root directory is a Cargo workspace with members.

# Examples

```rust
use crawrad_project::active_project::*;
assert!(!is_workspace(std::path::Path::new("no/such/folder")));
```
