---
id: props-props-selected-items
kind: doc
target_file: src/props.rs
line_hint: 686
author: 
created: 
title: "Props::selected_items"
tags: []
signature: "pub fn selected_items () -> PropDesc"
---

Selected items texts for `MultiSelect` (`SelectedItems`, #500).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::selected_items();
assert_eq!(p.key, "selected_items");
```
