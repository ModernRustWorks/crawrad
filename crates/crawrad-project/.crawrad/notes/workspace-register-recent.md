---
id: workspace-register-recent
kind: doc
target_file: src/workspace.rs
line_hint: 301
author: 
created: 
title: "register_recent"
tags: []
signature: "pub fn register_recent (list : & [PathBuf] , new_path : & Path) -> Vec < PathBuf >"
---

Places a folder at the top of the recent folders list.

Without duplicates and without growing: reopening the same folder for the fifth
time must keep the list the same size, and at the top — which is where the hand
returns to look. Pure so it can be proven without disk, and where the three
classic defects of this list live (duplicating, pushing to end, growing without limit).

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::workspace::register_recent;

let list = register_recent(&[], Path::new("proj"));
assert_eq!(list, vec![PathBuf::from("proj")]);
```
