---
id: package-manager-no-project
kind: doc
target_file: src/package_manager.rs
line_hint: 596
author: 
created: 
title: "NO_PROJECT"
tags: []
signature: "pub const NO_PROJECT : & str = \"No project is currently open.\""
---

What either tab says when there is no project to add a package TO.

# Examples

```rust
use crawrad_project::package_manager::NO_PROJECT;

assert_eq!(NO_PROJECT, "No project is currently open.");
```
