---
id: tree-visible-rows
kind: doc
target_file: src/tree.rs
line_hint: 55
author: 
created: 
title: "visible_rows"
tags: []
signature: "pub fn visible_rows (nodes : & [TreeNode] , collapsed : & BTreeSet < usize >) -> Vec < TreeRow >"
---

Computes visible tree rows skipping collapsed descendants.

The rows of `nodes`, skipping every node under one whose index is in
`collapsed`. Depth is capped at 32 (a file can hold any tree; a frame must
not overflow its stack).

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_runtime::tree::{visible_rows, TreeNode};

let nodes = vec![TreeNode::leaf("Item")];
let rows = visible_rows(&nodes, &BTreeSet::new());
assert_eq!(rows.len(), 1);
```
