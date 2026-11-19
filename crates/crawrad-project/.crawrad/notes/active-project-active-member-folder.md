---
id: active-project-active-member-folder
kind: doc
target_file: src/active_project.rs
line_hint: 20
author: 
created: 
title: "active_member_folder"
tags: []
signature: "pub fn active_member_folder (root : & Path) -> Option < PathBuf >"
---

Locates the active member folder in a workspace or single project.

# Examples

```rust
use crawrad_project::active_project::*;
assert!(active_member_folder(std::path::Path::new("no/such/folder")).is_none());
```
