---
id: props-props-tree-items
kind: doc
target_file: src/props.rs
line_hint: 517
author: 
created: 
title: "Props::tree_items"
tags: []
signature: "pub fn tree_items () -> PropDesc"
---

Hierarchical node collection (`Items` for TreeView).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tree_items();
assert_eq!(p.key, "items");
```
