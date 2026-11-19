---
id: scan-project
kind: doc
target_file: src/lib.rs
line_hint: 126
author: 
created: 
title: "scan_project"
tags: []
signature: "pub fn scan_project (root : & Path) -> Vec < Todo >"
---

Every tagged comment of the project at `root`, by file then line.

# Examples

```rust
use std::path::Path;
use crawrad_ext_todo::scan_project;

let todos = scan_project(Path::new("non_existent_folder_xyz"));
assert!(todos.is_empty());
```
