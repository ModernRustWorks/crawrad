---
id: explorer-rows-rows-group-groupname
kind: doc
target_file: src/explorer_rows/rows/group.rs
line_hint: 12
author: 
created: 
title: "GroupName"
tags: []
signature: "pub struct GroupName (String , Option < std :: path :: PathBuf >)"
---

The name of the group folder, with the root drawing.

# Examples

```rust
use crawrad_project::explorer_rows::rows::group::GroupName;
use crawrad_project::explorer_rows::row::ExplorerRow;

let gn = GroupName::new("my_workspace");
assert_eq!(gn.text(), "my_workspace");
```
