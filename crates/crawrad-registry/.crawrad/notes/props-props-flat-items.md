---
id: props-props-flat-items
kind: doc
target_file: src/props.rs
line_hint: 656
author: 
created: 
title: "Props::flat_items"
tags: []
signature: "pub fn flat_items () -> PropDesc"
---

Flat collection of items (`Items` for ListBox / ComboBox).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::flat_items();
assert_eq!(p.key, "items");
```
