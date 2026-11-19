---
id: explorer-rows-rows-group-groupname-at
kind: doc
target_file: src/explorer_rows/rows/group.rs
line_hint: 21
author: 
created: 
title: "GroupName::at"
tags: []
signature: "pub fn at (self , root : impl Into < std :: path :: PathBuf >) -> Self"
---

Sets the group's workspace root folder.

The group's folder: a right click on its NAME is about the workspace
root — Add Project, Add Existing Project (#471) — as on its path below.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::explorer_rows::rows::group::GroupName;
use crawrad_project::explorer_rows::row::ExplorerRow;

let gn = GroupName::new("my_ws").at(PathBuf::from("/root"));
assert_eq!(gn.text(), "my_ws");
```
