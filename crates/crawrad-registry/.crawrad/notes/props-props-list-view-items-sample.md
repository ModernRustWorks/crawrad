---
id: props-props-list-view-items-sample
kind: doc
target_file: src/props.rs
line_hint: 634
author: 
created: 
title: "Props::list_view_items_sample"
tags: []
signature: "pub fn list_view_items_sample () -> PropDesc"
---

Sample hierarchical items for `ListView` (#500): 2 rows, 1 sub-item each.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::list_view_items_sample();
assert_eq!(p.key, "items");
```
