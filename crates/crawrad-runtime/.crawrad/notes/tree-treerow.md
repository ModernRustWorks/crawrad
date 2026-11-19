---
id: tree-treerow
kind: doc
target_file: src/tree.rs
line_hint: 42
author: 
created: 
title: "TreeRow"
tags: []
signature: "pub struct TreeRow"
---

A row the tree shows.

# Examples

```rust
use crawrad_runtime::tree::TreeRow;

let row = TreeRow {
    index: 0,
    level: 0,
    text: "Root".into(),
    has_children: false,
    collapsed: false,
    symbol: String::new(),
};
assert_eq!(row.text, "Root");
```
