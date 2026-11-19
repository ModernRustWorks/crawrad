---
id: tree-treenode
kind: doc
target_file: src/tree.rs
line_hint: 11
author: 
created: 
title: "TreeNode"
tags: []
signature: "pub struct TreeNode"
---

One node of a tree the form holds at run time.

# Examples

```rust
use crawrad_runtime::tree::TreeNode;

let leaf = TreeNode::leaf("Item 1");
assert_eq!(leaf.text, "Item 1");
```
