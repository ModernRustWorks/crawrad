---
id: tree-row-of
kind: doc
target_file: src/tree.rs
line_hint: 116
author: 
created: 
title: "row_of"
tags: []
signature: "pub fn row_of (rows : & [TreeRow] , selected : Option < usize >) -> Option < usize >"
---

The position in `rows` of the node `selected`, if it is visible.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_runtime::tree::{row_of, visible_rows, TreeNode};

let nodes = vec![TreeNode::leaf("Item")];
let rows = visible_rows(&nodes, &BTreeSet::new());
assert_eq!(row_of(&rows, Some(0)), Some(0));
assert_eq!(row_of(&rows, None), None);
```
