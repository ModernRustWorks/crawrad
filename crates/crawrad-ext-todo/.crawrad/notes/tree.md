---
id: tree
kind: doc
target_file: src/lib.rs
line_hint: 175
author: 
created: 
title: "tree"
tags: []
signature: "pub fn tree (tagged : & [Todo]) -> SetTreeParams"
---

Builds tree view parameters grouping tagged comments by file.

The view's tree: a branch per file (its count as detail), a leaf per
tagged comment pointing at its line.

# Examples

```rust
use crawrad_ext_todo::tree;

let params = tree(&[]);
assert_eq!(params.view_id, "todos");
assert!(params.items.is_empty());
```
