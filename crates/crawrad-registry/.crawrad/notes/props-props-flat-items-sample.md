---
id: props-props-flat-items-sample
kind: doc
target_file: src/props.rs
line_hint: 670
author: 
created: 
title: "Props::flat_items_sample"
tags: []
signature: "pub fn flat_items_sample (samples : & [& str]) -> PropDesc"
---

Flat collection of items with default samples (`Items` for ListBox / ComboBox).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::flat_items_sample(&["Apple", "Banana"]);
assert_eq!(p.key, "items");
```
