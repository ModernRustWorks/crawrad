---
id: explorer-rows-rows-group-grouppath
kind: doc
target_file: src/explorer_rows/rows/group.rs
line_hint: 45
author: 
created: 
title: "GroupPath"
tags: []
signature: "pub struct GroupPath (String)"
---

The path of the group folder, spelled out and muted.

No icon, and the row above is the reason: the two are one statement, and a
second drawing under the first would read as a second thing.

# Examples

```rust
use crawrad_project::explorer_rows::rows::group::GroupPath;
use crawrad_project::explorer_rows::row::ExplorerRow;

let gp = GroupPath::new("/path/to/group");
assert_eq!(gp.text(), "/path/to/group");
```
