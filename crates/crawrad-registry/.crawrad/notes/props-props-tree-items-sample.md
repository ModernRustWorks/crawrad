---
id: props-props-tree-items-sample
kind: doc
target_file: src/props.rs
line_hint: 609
author: 
created: 
title: "Props::tree_items_sample"
tags: []
signature: "pub fn tree_items_sample () -> PropDesc"
---

Initial hierarchical sample nodes for `TreeView`.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tree_items_sample();
assert_eq!(p.key, "items");
```
