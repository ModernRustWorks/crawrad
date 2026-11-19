---
id: tree-count
kind: doc
target_file: src/tree.rs
line_hint: 122
author: 
created: 
title: "count"
tags: []
signature: "pub fn count (nodes : & [TreeNode]) -> usize"
---

How many nodes `nodes` holds, in pre-order (the next free index).

# Examples

```rust
use crawrad_runtime::tree::{count, TreeNode};

let nodes = vec![TreeNode::branch("Root", vec![TreeNode::leaf("Child")])];
assert_eq!(count(&nodes), 2);
```
